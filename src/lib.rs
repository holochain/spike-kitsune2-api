#![deny(missing_docs)]
//! Kitsune2
//!
//! ## High-Level Open Questions
//!
//! - Do we want to re-tools arcs/quant to make it easier to reason about?
//! - Do we want to skip arcs/quant while we're not sharding?
//! - Do we want to move to bootstrap2 so we can use a more modern AgentInfo?
//! - Do we want to use generic loc, or hard-code u32?
//! - Do we want to use generic crypto or hard-code ed25519?
//! - Hashes should be
//!     - displayed in debugging in the holochain canonical fmt
//!       i.e. `uhCkblabla`
//!     - any size to support other crypto schemes
//!     - hashes cannot validate signatures... that fn in on info
//!       again to support other crypto schemes
//!
//! ## Modularization Domains
//!
//! - "Agent" identity+crypto + online-ness (join)
//! - Space management
//! - Peer Store
//! - Connectivity + Connection Blocking
//! - Bootstrapping
//! - Sharding / Arc Sizing aka authority domain or neighborhood management
//! - Op Store
//! - Gossip of op hashes loc-d within claimed authority
//! - Publishing of op hashes loc-d within claimed authority
//! - Fetching of op data
//! - Actual Kitsune, brining all of this together

pub mod types;

/*
/// A meta-op.
#[derive(Debug)]
pub struct MetaOp {
    /// The op hash.
    pub op_hash: DynHash,

    /// The op
    /// The actual op data.
    pub op_data: Vec<u8>,

    /// Crdt-style add-only opaque implementor-use flags.
    pub op_flags: std::collections::HashSet<String>,
}

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
pub trait Kitsune2PeerStore: 'static + Send + Sync + std::fmt::Debug {
    /// Inject agent_info from an external source (e.g. bootstrap).
    /// (May be ignored if it is expired, or we have a more recent version).
    fn ingest_agent_info_list(
        &self,
        info: Vec<DynAgentInfo>,
    ) -> BoxFuture<'_, Result<()>>;

    /// Pull an agent info if we have one.
    fn get_agent(&self, agent: DynHash) -> BoxFuture<'_, Option<DynAgentInfo>>;

    /// List all the agents within the specified arc.
    fn list_agents_for_arc(
        &self,
        arq: DynArq,
    ) -> BoxFuture<'_, Vec<DynAgentInfo>>;
}

/// Trait-object version of kitsune2 peer store.
pub type DynKitsune2PeerStore =
    Arc<dyn Kitsune2PeerStore + 'static + Send + Sync>;

/// The previous version of kitsune mixed these into the generic host api
/// making it very difficult to abstract for testing.
///
/// With this separated out, we can implement a simple memory store for
/// testing that will be robust enough for simple small-scale production
/// use, and can have binary import/export handles for manual persistence.
pub trait Kitsune2OpStore: 'static + Send + Sync + std::fmt::Debug {
    /// Process incoming ops.
    fn ingest_op_list(&self, op_list: Vec<MetaOp>)
        -> BoxFuture<'_, Result<()>>;
}

/// Trait-object version of kitsune2 op store.
pub type DynKitsune2OpStore = Arc<dyn Kitsune2OpStore + 'static + Send + Sync>;

/// Trait representing a kitsune2 endpoint.
pub trait Kitsune2Endpoint: 'static + Send + Sync + std::fmt::Debug {
    /// Get access to the peer store.
    fn peer_store(&self) -> &DynKitsune2PeerStore;

    /// Get access to the op store.
    fn op_store(&self) -> &DynKitsune2OpStore;

    /// Close a connection to a peer, if open.
    fn close(&self, agent: DynHash) -> BoxFuture<'_, ()>;

    /// Make a request of a peer, expecting a response. If we do not currently
    /// have an open connection to this peer, we will attempt to establish one.
    ///
    /// While connection establishment is active, messages will be queued.
    /// Individual messages will persist in the queue for the global timeout
    /// Duration. Once that is reached they will error with "PendingConnection",
    /// or the most recent connection error to that peer.
    ///
    /// QUESTION: Do we actually want to error immediately on the first error?
    ///
    /// Older versions of kitsune had various other ways of communicating
    /// with peers. These have all been removed in favor of this one method:
    /// - `notify`
    ///   - previously was a fire-and-forget method for sending data,
    ///     however, this led to many apis that would wait for an async
    ///     "response" that would just error in timeout because we didn't
    ///     know the original request was never succesful. The overhead
    ///     of having to send a response to every request is now accepted
    ///     as preferable to the debuggability headaches of the `notify` api.
    /// - `broadcast`
    ///   - there is no actual ability to broadcast on the backend. The
    ///     payload has to be copied and sent to every peer individually
    ///     anyways, so just do that if you need to. You can get the list
    ///     of connected peers via the `connected_peers` api.
    /// - `targeted broadcast`
    ///   - this is similar to the removed `broadcast` api. You can directly
    ///     call the `discover_peers_for_loc` api to get a list of peers
    ///     to send requests to directly.
    fn request(
        &self,
        agent: DynHash,
        data: Vec<u8>,
    ) -> BoxFuture<'_, Result<Vec<u8>>>;

    /// Get a list of currently connected peers.
    fn connected_peers(&self) -> Vec<DynAgentInfo>;

    /// Attempt to discover a specific remote agent.
    fn discover_agent(
        &self,
        agent: DynHash,
    ) -> BoxFuture<'_, Result<DynAgentInfo>>;

    /// Get a list of peers claiming authority over the provided location.
    fn discover_peers_for_loc(
        &self,
        loc: Loc,
    ) -> BoxFuture<'_, Result<Vec<DynAgentInfo>>>;
}

/// Trait-object version of kitsune2 endpoint.
pub type DynKitsune2Endpoint =
    Arc<dyn Kitsune2Endpoint + 'static + Send + Sync>;
*/
