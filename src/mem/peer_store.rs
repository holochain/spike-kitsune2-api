//! Memory-backed peer store.

use crate::types::*;
use crate::*;

/// Memory-backed peer store.
#[derive(Debug)]
struct MemPeerStore {}

impl MemPeerStore {
    pub fn new() -> Self {
        Self {}
    }
}

impl peer_store::PeerStore for MemPeerStore {
    fn ingest_agent_info_list(
        &self,
        _info: Vec<types::agent::DynAgentInfo>,
    ) -> BoxFuture<'_, Result<()>> {
        Box::pin(async move { todo!() })
    }

    /// Pull an agent info if we have one.
    fn get_agent(
        &self,
        _agent: DynHash,
    ) -> BoxFuture<'_, Option<agent::DynAgentInfo>> {
        Box::pin(async move { todo!() })
    }

    /// List all the agents within the specified arc.
    fn list_agents_for_arc(
        &self,
        _arq: agent::DynArq,
    ) -> BoxFuture<'_, Vec<agent::DynAgentInfo>> {
        Box::pin(async move { todo!() })
    }
}

/// Memory-backed peer store.
#[derive(Debug)]
pub struct MemPeerStoreFactory {}

impl MemPeerStoreFactory {
    /// Create a new memory-backed peer store.
    pub fn create() -> peer_store::DynPeerStoreFactory {
        let out: peer_store::DynPeerStoreFactory = Arc::new(Self {});
        out
    }
}

impl peer_store::PeerStoreFactory for MemPeerStoreFactory {
    fn default_config(&self) -> &'static [config::Config] {
        &[]
    }

    /// Construct a new transport instance.
    fn create(
        &self,
        _config: Arc<config::ConfigMap>,
    ) -> BoxFuture<'static, Result<peer_store::DynPeerStore>> {
        Box::pin(async move {
            let out: peer_store::DynPeerStore = Arc::new(MemPeerStore::new());
            Ok(out)
        })
    }
}
