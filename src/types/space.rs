//! Kitsune2 space types.

use crate::types::*;

/// Handler for requests and events emitted by the space instance.
pub trait SpaceHandler: 'static + Send + Sync + std::fmt::Debug {
    /// A connection has sent us a request.
    /// Returning an Err result will close this connection.
    /// If you want to send an error back AS the response,
    /// please see the [Space::respond] api.
    fn incoming_request(
        &self,
        peer: DynHash,
        req_id: Bytes,
        data: Bytes,
    ) -> Result<()>;
}

/// Trait-object space handler.
pub type DynSpaceHandler = Arc<dyn SpaceHandler>;

/// Abstract kitsune2 space instance.
pub trait Space: 'static + Send + Sync + std::fmt::Debug {
    /// Get access to the peer store used by this space.
    fn peer_store(&self) -> &peer_store::DynPeerStore;

    /// Join an agent to this space. If the agent is already joined,
    /// this is a no-op.
    fn agent_join(&self, agent: agent::DynLocalAgent);

    /// Cause an agent to leave this space. If this agent is not already
    /// in this space, this is a no-op.
    fn agent_leave(&self, agent: &DynHash);

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
        data: Bytes,
    ) -> BoxFuture<'_, Result<Bytes>>;

    /// Respond to a request previously made by a remote peer.
    fn respond(
        &self,
        req_id: Bytes,
        data: std::result::Result<Bytes, Bytes>,
    ) -> BoxFuture<'_, ()>;

    /// Attempt to discover a specific remote agent.
    fn discover_agent(
        &self,
        agent: DynHash,
    ) -> BoxFuture<'_, Result<agent::DynAgentInfo>>;

    /// Get a list of peers claiming authority over the provided location.
    fn discover_peers_for_loc(
        &self,
        loc: Loc,
    ) -> BoxFuture<'_, Result<Vec<agent::DynAgentInfo>>>;
}

/// Trait-object space instance.
pub type DynSpace = Arc<dyn Space>;

/// A factory to create a new space instance.
pub trait SpaceFactory: 'static + Send + Sync + std::fmt::Debug {
    /// Config options for the concrete Tx type.
    fn default_config(&self) -> &'static [crate::config::Config];

    /// Constructe a new space instance.
    fn create_instance(
        &self,
        builder: Arc<crate::builder::Builder>,
        handler: DynSpaceHandler,
        space: SpaceHash,
    ) -> BoxFuture<'static, Result<DynSpace>>;
}

/// Trait-object kitsune space factory.
pub type DynSpaceFactory = Arc<dyn SpaceFactory>;
