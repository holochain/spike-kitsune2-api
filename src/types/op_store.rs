//! Kitsune2 op store types.

use crate::types::*;

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

/// The previous version of kitsune mixed these into the generic host api
/// making it very difficult to abstract for testing.
///
/// With this separated out, we can implement a simple memory store for
/// testing that will be robust enough for simple small-scale production
/// use, and can have binary import/export handles for manual persistence.
pub trait OpStore: 'static + Send + Sync + std::fmt::Debug {
    /// Process incoming ops.
    fn ingest_op_list(&self, op_list: Vec<MetaOp>)
        -> BoxFuture<'_, Result<()>>;
}

/// Trait-object version of kitsune2 op store.
pub type DynOpStore = Arc<dyn OpStore>;
