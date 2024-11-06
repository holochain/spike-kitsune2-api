//! Kitsune2 builder.

use crate::types::*;
use crate::*;

use futures_util::future::BoxFuture;

/// Kitsune2 builder.
#[derive(Debug)]
pub struct Builder {
    /// The transport factory.
    pub tx: tx::DynTxFactory,

    /// The peer store factory.
    pub peer_store: peer_store::DynPeerStoreFactory,
}

impl Builder {
    /// Construct a builder with test-appropriate factories.
    ///
    /// The normal pattern would be to use this, then replace a single
    /// factory with a real module that you are attempting to validate.
    pub fn new_testing() -> Self {
        Self {
            tx: test_factories::tx::TestTxFactory::create(),
            // The mem peer store is suitible for both testing and production.
            peer_store: mem::peer_store::MemPeerStoreFactory::create(),
        }
    }

    /// Apply a different transport factory.
    pub fn with_tx(mut self, tx: tx::DynTxFactory) -> Self {
        self.tx = tx;
        self
    }

    /// Apply a different peer store factory.
    pub fn with_peer_store(
        mut self,
        peer_store: peer_store::DynPeerStoreFactory,
    ) -> Self {
        self.peer_store = peer_store;
        self
    }

    /// Mixin defaults, only adding entries that don't exist.
    ///
    /// Use this with a blank ConfigMap to generate a default/example config.
    /// There is no need to manually call this before build, it will be run
    /// automatically.
    pub fn mixin_defaults(&self, map: &mut config::ConfigMap) {
        use crate::config::*;

        map.mixin_defaults(self.tx.default_config());
        map.mixin_defaults(self.peer_store.default_config());
    }

    /// Build a kitsune2 instance.
    pub fn build(
        self,
        mut config: config::ConfigMap,
    ) -> BoxFuture<'static, Result<kitsune::DynKitsune2>> {
        Box::pin(async move {
            self.mixin_defaults(&mut config);

            todo!()
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn debug_testing_builder() {
        println!("{:?}", Builder::new_testing());
    }
}
