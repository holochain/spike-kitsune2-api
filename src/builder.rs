//! Kitsune2 builder.

use crate::types::*;
use crate::*;

/// Kitsune2 builder.
#[derive(Debug)]
pub struct Builder {
    /// The config map to be used by the resulting Kitsune2 instance.
    pub config_map: config::ConfigMap,

    /// The transport factory.
    pub tx: tx::DynTxFactory,

    /// The peer store factory.
    pub peer_store: peer_store::DynPeerStoreFactory,

    /// The kitsune factory.
    pub kitsune: kitsune::DynKitsune2Factory,

    /// The space factory.
    pub space: space::DynSpaceFactory,
}

impl Builder {
    /// Construct a builder with test-appropriate factories.
    ///
    /// The normal pattern would be to use this, then replace a single
    /// factory with a real module that you are attempting to validate.
    pub fn new_testing() -> Self {
        Self {
            config_map: config::ConfigMap::new(),
            tx: factories::TestTxFactory::create(),
            // The mem peer store is suitible for both testing and production.
            peer_store: factories::MemPeerStoreFactory::create(),
            kitsune: factories::CoreKitsuneFactory::create(),
            space: factories::CoreSpaceFactory::create(),
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
    pub fn mixin_defaults(&mut self) {
        use crate::config::*;

        self.config_map.mixin_defaults(self.tx.default_config());
        self.config_map
            .mixin_defaults(self.peer_store.default_config());
        self.config_map
            .mixin_defaults(self.kitsune.default_config());
        self.config_map.mixin_defaults(self.space.default_config());
    }

    /// Build a kitsune2 instance.
    pub fn build(
        mut self,
        handler: kitsune::DynKitsune2Handler,
    ) -> BoxFuture<'static, Result<kitsune::DynKitsune2>> {
        Box::pin(async move {
            self.mixin_defaults();

            let kitsune = self.kitsune.clone();
            let builder = Arc::new(self);
            kitsune.create_instance(builder, handler).await
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
