//! Core space factory.

use crate::types::*;
use crate::*;

#[derive(Debug)]
struct CoreSpace {
    peer_store: peer_store::DynPeerStore,
}

impl space::Space for CoreSpace {
    fn peer_store(&self) -> &peer_store::DynPeerStore {
        &self.peer_store
    }

    fn agent_join(&self, _agent: agent::DynLocalAgent) {
        todo!()
    }

    fn agent_leave(&self, _agent: &DynHash) {
        todo!()
    }

    fn request(
        &self,
        _agent: DynHash,
        _data: Bytes,
    ) -> BoxFuture<'_, Result<Bytes>> {
        Box::pin(async move { todo!() })
    }

    fn respond(
        &self,
        _req_id: Bytes,
        _data: std::result::Result<Bytes, Bytes>,
    ) -> BoxFuture<'_, ()> {
        Box::pin(async move { todo!() })
    }

    fn discover_agent(
        &self,
        _agent: DynHash,
    ) -> BoxFuture<'_, Result<agent::DynAgentInfo>> {
        Box::pin(async move { todo!() })
    }

    fn discover_peers_for_loc(
        &self,
        _loc: Loc,
    ) -> BoxFuture<'_, Result<Vec<agent::DynAgentInfo>>> {
        Box::pin(async move { todo!() })
    }
}

/// This is the core kitsune space factory included/builtin to the project.
/// But feel free to write your own!
#[derive(Debug)]
pub struct CoreSpaceFactory {}

impl CoreSpaceFactory {
    /// Create a new core kitsune factory.
    pub fn create() -> space::DynSpaceFactory {
        let out: space::DynSpaceFactory = Arc::new(Self {});
        out
    }
}

impl space::SpaceFactory for CoreSpaceFactory {
    fn default_config(&self) -> &'static [crate::config::Config] {
        &[]
    }

    fn create_instance(
        &self,
        builder: Arc<crate::builder::Builder>,
        _handler: space::DynSpaceHandler,
        _space: SpaceHash,
    ) -> BoxFuture<'static, Result<space::DynSpace>> {
        Box::pin(async move {
            let peer_store =
                builder.peer_store.create_instance(builder.clone()).await?;
            let space: space::DynSpace = Arc::new(CoreSpace { peer_store });
            Ok(space)
        })
    }
}
