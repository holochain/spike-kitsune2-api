//! Kitsune2 types.

use crate::types::*;

/// Handler for requests and events coming out of the kitsune runtime.
pub trait Kitsune2Handler: 'static + Send + Sync + std::fmt::Debug {
    /// Gather preflight data to send to a new opening connection.
    /// Returning an Err result will close this connection.
    fn preflight_gather(&self, peer_url: PeerUrl) -> Result<Bytes>;

    /// Validate preflight data sent by a remote peer on a new connection.
    /// Returning an Err result will close this connection.
    fn preflight_validate(&self, peer_url: PeerUrl, data: Bytes) -> Result<()>;

    /// Kitsune would like to construct a space. Provide a handler.
    fn create_space(
        &self,
        space: SpaceHash,
    ) -> BoxFuture<'_, Result<space::DynSpaceHandler>>;
}

/// Trait-object kitsune2 handler.
pub type DynKitsune2Handler = Arc<dyn Kitsune2Handler>;

/// Abstract kitsune2 instance.
pub trait Kitsune2: 'static + Send + Sync + std::fmt::Debug {
    /// Assert a space exists, and get a handle to it.
    fn space(&self, space: SpaceHash) -> BoxFuture<'_, space::DynSpace>;

    /// Close a peer connection if it is open, optionally sending
    /// a close reason first.
    fn close_peer(&self, peer_url: PeerUrl, reason: Option<String>);
}

/// Trait-object kitsune2.
pub type DynKitsune2 = Arc<dyn Kitsune2>;

/// Kitsune factory.
pub trait Kitsune2Factory: 'static + Send + Sync + std::fmt::Debug {
    /// Config options for the concrete Tx type.
    fn default_config(&self) -> &'static [crate::config::Config];

    /// Construct a new transport instance.
    fn create_instance(
        &self,
        builder: Arc<crate::builder::Builder>,
        handler: kitsune::DynKitsune2Handler,
    ) -> BoxFuture<'static, Result<DynKitsune2>>;
}

/// Trait-object kitsune2 factory.
pub type DynKitsune2Factory = Arc<dyn Kitsune2Factory>;
