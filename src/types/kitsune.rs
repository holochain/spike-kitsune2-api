//! Kitsune2 types.

use crate::types::*;

/// Handler for requests and events coming out of the kitsune runtime.
pub trait Kitsune2Handle: 'static + Send + std::fmt::Debug {
    /// Gather preflight data to send to a new opening connection.
    /// Returning an Err result will close this connection.
    fn preflight_gather(&mut self, peer_url: PeerUrl) -> Result<Bytes>;

    /// Validate preflight data sent by a remote peer on a new connection.
    /// Returning an Err result will close this connection.
    fn preflight_validate(
        &mut self,
        peer_url: PeerUrl,
        data: Bytes,
    ) -> Result<()>;

    /// Kitsune would like to construct a space. Provide a handler.
    fn create_space(&mut self, space: &SpaceHash) -> space::DynSpaceHandler;
}

/// Abstract kitsune2 instance.
pub trait Kitsune2: 'static + Send + Sync + std::fmt::Debug {
    /// Assert a space exists, and get a handle to it.
    fn space(&self, space: &SpaceHash) -> &space::DynSpace;

    /// Close a peer connection if it is open, optionally sending
    /// a close reason first.
    fn close_peer(&self, peer_url: PeerUrl, reason: Option<String>);
}

/// Trait-object kitsune2.
pub type DynKitsune2 = Arc<dyn Kitsune2>;
