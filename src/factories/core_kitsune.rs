//! Core kitsune factory.

use crate::types::*;
use crate::*;

use std::collections::HashMap;

type PendingSpace = futures::future::Shared<
    BoxFuture<
        'static,
        std::result::Result<space::DynSpace, Arc<std::io::Error>>,
    >,
>;

#[derive(Debug)]
struct CoreKitsune {
    builder: Arc<crate::builder::Builder>,
    handler: kitsune::DynKitsune2Handler,
    spaces: std::sync::Mutex<HashMap<SpaceHash, PendingSpace>>,
}

impl kitsune::Kitsune2 for CoreKitsune {
    fn space(&self, space: SpaceHash) -> BoxFuture<'_, space::DynSpace> {
        Box::pin(async move {
            use std::collections::hash_map::Entry;
            // this will eventually succeed... we can loop forever.
            loop {
                let pend =
                    match self.spaces.lock().unwrap().entry(space.clone()) {
                        Entry::Vacant(e) => {
                            use futures::future::FutureExt;
                            let builder = self.builder.clone();
                            let handler = self.handler.clone();
                            let space = space.clone();
                            let pend = async move {
                                let space_handler = handler
                                    .create_space(space.clone())
                                    .await
                                    .map_err(Arc::new)?;
                                builder
                                    .space
                                    .create_instance(
                                        builder.clone(),
                                        space_handler,
                                        space,
                                    )
                                    .await
                                    .map_err(Arc::new)
                            }
                            .boxed()
                            .shared();
                            e.insert(pend.clone());
                            pend
                        }
                        Entry::Occupied(e) => e.get().clone(),
                    };
                match pend.await {
                    Ok(space) => return space,
                    _ => continue,
                }
            }
        })
    }

    fn close_peer(&self, _peer_url: PeerUrl, _reason: Option<String>) {
        todo!()
    }
}

/// This is the core kitsune factory included/builtin to the project.
/// But feel free to write your own!
#[derive(Debug)]
pub struct CoreKitsuneFactory {}

impl CoreKitsuneFactory {
    /// Create a new core kitsune factory.
    pub fn create() -> kitsune::DynKitsune2Factory {
        let out: kitsune::DynKitsune2Factory = Arc::new(Self {});
        out
    }
}

impl kitsune::Kitsune2Factory for CoreKitsuneFactory {
    fn default_config(&self) -> &'static [crate::config::Config] {
        &[]
    }

    fn create_instance(
        &self,
        builder: Arc<crate::builder::Builder>,
        handler: kitsune::DynKitsune2Handler,
    ) -> BoxFuture<'static, Result<kitsune::DynKitsune2>> {
        Box::pin(async move {
            let kitsune: kitsune::DynKitsune2 = Arc::new(CoreKitsune {
                builder,
                handler,
                spaces: std::sync::Mutex::new(HashMap::new()),
            });
            Ok(kitsune)
        })
    }
}
