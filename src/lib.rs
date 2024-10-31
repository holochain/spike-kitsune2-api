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

use futures_util::future::BoxFuture;
use std::sync::Arc;

/// Kitsune2 result
pub type Result<T> = std::io::Result<T>;

/// Kitsune2 agent (This is a type=String for the spike).
pub type Agent = String;

/// Kitsune2 peer url (This is a type=String for the spike).
pub type PeerUrl = String;

/// Kitsune2 location (This is a type=u32 for the spike).
pub type Loc = u32;

/// Kitsune2 agent metadata.
/// This is forshortened for the spike, but should also:
/// - be cryptographically signed
/// - include created at and expiration info
/// - include a generic map for extendable data fields
pub struct AgentInfo {
    /// Kitsune2 agent.
    pub agent: Agent,

    /// Kitsune2 peer url.
    pub url: PeerUrl,

    /// Kitsune2 loc.
    pub loc: Loc,
}

/// A meta-op.
pub struct MetaOp {
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
pub trait Kitsune2PeerStore: 'static + Send + Sync {
    /// Inject agent_info from an external source (e.g. bootstrap).
    /// (May be ignored if it is expired, or we have a more recent version).
    fn ingest_agent_info_list(
        &self,
        info: Vec<AgentInfo>,
    ) -> BoxFuture<'_, Result<()>>;

    /// Pull an agent info if we have one.
    fn get_agent(&self, agent: Agent) -> BoxFuture<'_, Option<AgentInfo>>;

    /// NOTE THIS IS FAKE, sharding is disabled, so we just get everyone.
    /// We don't even know the datatype for representing arcs right now.
    fn list_agents_for_arc(&self, arc: ()) -> BoxFuture<'_, Vec<AgentInfo>>;
}

/// Trait-object version of kitsune2 peer store.
pub type DynKitsune2PeerStore = Arc<dyn Kitsune2PeerStore + Send + Sync>;

/// The previous version of kitsune mixed these into the generic host api
/// making it very difficult to abstract for testing.
///
/// With this separated out, we can implement a simple memory store for
/// testing that will be robust enough for simple small-scale production
/// use, and can have binary import/export handles for manual persistence.
pub trait Kitsune2OpStore: 'static + Send + Sync {
    /// Process incoming ops.
    fn ingest_op_list(&self, op_list: Vec<MetaOp>)
        -> BoxFuture<'_, Result<()>>;
}

/// Trait-object version of kitsune2 op store.
pub type DynKitsune2OpStore = Arc<dyn Kitsune2OpStore + Send + Sync>;

/// Trait representing a kitsune2 endpoint.
pub trait Kitsune2Endpoint: 'static + Send + Sync {
    /// Get access to the peer store.
    fn peer_store(&self) -> &DynKitsune2PeerStore;

    /// Get access to the op store.
    fn op_store(&self) -> &DynKitsune2OpStore;

    /// Close a connection to a peer, if open.
    fn close(&self, agent: Agent) -> BoxFuture<'_, ()>;

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
        agent: Agent,
        data: Vec<u8>,
    ) -> BoxFuture<'_, Result<Vec<u8>>>;

    /// Get a list of currently connected peers.
    fn connected_peers(&self) -> Vec<AgentInfo>;

    /// Attempt to discover a specific remote agent.
    fn discover_agent(&self, agent: Agent) -> BoxFuture<'_, Result<AgentInfo>>;

    /// Get a list of peers claiming authority over the provided location.
    fn discover_peers_for_loc(
        &self,
        loc: Loc,
    ) -> BoxFuture<'_, Result<Vec<AgentInfo>>>;
}

/// Trait-object version of kitsune2 endpoint.
pub type DynKitsune2Endpoint = Arc<dyn Kitsune2Endpoint + Send + Sync>;
