//! Memory-backed test-only transport.

use crate::types::*;
use crate::*;

/// Memory-backed test-only transport.
#[derive(Debug)]
struct TestTx {}

impl TestTx {
    pub fn new() -> Self {
        Self {}
    }
}

impl tx::Tx for TestTx {
    fn close_peer(
        &self,
        _peer_url: PeerUrl,
        _reason: Option<String>,
    ) -> BoxFuture<'_, ()> {
        Box::pin(async move { todo!() })
    }

    fn request(
        &self,
        _peer_url: PeerUrl,
        _data: Bytes,
    ) -> BoxFuture<'_, Result<Bytes>> {
        Box::pin(async move { todo!() })
    }

    fn respond(
        &self,
        _req_id: Bytes,
        _data: std::result::Result<Bytes, Bytes>,
    ) -> BoxFuture<'_, ()> {
        Box::pin(async move { todo!() })
    }
}

/// Memory-backed test-only transport.
#[derive(Debug)]
pub struct TestTxFactory {}

impl TestTxFactory {
    /// Create a new memory-backed peer store.
    pub fn create() -> tx::DynTxFactory {
        let out: tx::DynTxFactory = Arc::new(Self {});
        out
    }
}

impl tx::TxFactory for TestTxFactory {
    fn default_config(&self) -> &'static [config::Config] {
        &[]
    }

    /// Construct a new transport instance.
    fn create_instance(
        &self,
        _config: Arc<config::ConfigMap>,
        _handler: tx::DynTxHandler,
    ) -> BoxFuture<'static, Result<tx::DynTx>> {
        Box::pin(async move {
            let out: tx::DynTx = Arc::new(TestTx::new());
            Ok(out)
        })
    }
}
