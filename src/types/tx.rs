//! Transport abstraction.

use crate::types::*;

use bytes::Bytes;

/// A trait representing the handling of transport instance events.
pub trait TxHandler: 'static + Send + std::fmt::Debug {
    /// Gather preflight data to send to a new opening connection.
    /// Returning an Err result will close this connection.
    fn preflight_gather(&mut self, peer: DynHash) -> Result<Bytes>;

    /// Validate preflight data sent by a remote peer on a new connection.
    /// Returning an Err result will close this connection.
    fn preflight_validate(&mut self, peer: DynHash, data: Bytes) -> Result<()>;

    /// A connection to a peer has opened.
    fn connected(&mut self, peer: DynHash);

    /// A connection to a peer has closed.
    fn disconnected(&mut self, peer: DynHash);

    /// A connection has sent us a request.
    /// Returning an Err result will close this connection.
    fn incoming_request(
        &mut self,
        peer: DynHash,
        req_id: Bytes,
        data: Bytes,
    ) -> Result<()>;
}

/// Trait-object transport handler.
pub type DynTxHandler = Arc<dyn TxHandler>;

/// A trait representing a transport instance.
pub trait Tx: 'static + Send + Sync + std::fmt::Debug {
    /// Close a peer connection if it is open, optionally sending
    /// a close reason first.
    fn close_peer(
        &self,
        peer: DynHash,
        reason: Option<String>,
    ) -> BoxFuture<'_, ()>;

    /// Make a request of a remote peer.
    fn request(
        &self,
        peer: DynHash,
        data: Bytes,
    ) -> BoxFuture<'_, Result<Bytes>>;

    /// Respond to a request previously made by a remote peer.
    fn respond(
        &self,
        req_id: Bytes,
        data: std::result::Result<Bytes, Bytes>,
    ) -> BoxFuture<'_, ()>;
}

/// Trait-object transport instance.
pub type DynTx = Arc<dyn Tx>;

/// A factory to create a new transport instance.
pub trait TxFactory: 'static + Send + Sync + std::fmt::Debug {
    /// Get a generic json config blob for this transport.
    fn default_config(&self) -> serde_json::Value;

    /// Construct a new transport instance.
    fn create(
        &self,
        config: serde_json::Value,
        handler: DynTxHandler,
    ) -> BoxFuture<'static, Result<DynTx>>;
}

/// Trait-object transport factory.
pub type DynTxFactory = Arc<dyn TxFactory>;
