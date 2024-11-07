//! Kitsune2 peer store types.

use crate::types::*;

/// Trait representing a kitsune2 peer store.
///
/// Note, we originally did a ton of work making this persisted,
/// then found out we needed a memory cache of this because it's used too
/// frequently, and we're calling out to the bootstrap server regularly
/// anyways. The recommendation going forward is to just have an in-memory
/// kitsune peer store only, and not do any persistence of it until we
/// find some real requirement for it.
///
/// This will let us have a single implementation in the kitsune crate
/// that is not dependent on holochain that is run both for testing
/// and production.
///
/// The in-memory store doesn't need its functions to be async,
/// but we want to suport async stores in general.
pub trait PeerStore: 'static + Send + Sync + std::fmt::Debug {
    /// Inject agent_info from an external source (e.g. bootstrap).
    /// (May be ignored if it is expired, or we have a more recent version).
    fn ingest_agent_info_list(
        &self,
        info: Vec<agent::DynAgentInfo>,
    ) -> BoxFuture<'_, Result<()>>;

    /// Pull an agent info if we have one.
    fn get_agent(
        &self,
        agent: DynHash,
    ) -> BoxFuture<'_, Option<agent::DynAgentInfo>>;

    /// List all the agents within the specified arc.
    fn list_agents_for_arc(
        &self,
        arq: agent::DynArq,
    ) -> BoxFuture<'_, Vec<agent::DynAgentInfo>>;
}

/// Trait-object version of kitsune2 peer store.
pub type DynPeerStore = Arc<dyn PeerStore>;

/// A factory to create a new peer store.
pub trait PeerStoreFactory: 'static + Send + Sync + std::fmt::Debug {
    /// Config options for the concrete PeerStore type.
    fn default_config(&self) -> &'static [crate::config::Config];

    /// Construct a new transport instance.
    fn create_instance(
        &self,
        builder: Arc<crate::builder::Builder>,
    ) -> BoxFuture<'static, Result<DynPeerStore>>;
}

/// Trait-object peer store factory.
pub type DynPeerStoreFactory = Arc<dyn PeerStoreFactory>;
