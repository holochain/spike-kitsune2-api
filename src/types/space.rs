//! Kitsune2 space types.

use crate::types::*;

/// Handler for requests and events emitted by the space instance.
pub trait SpaceHandler: 'static + Send + std::fmt::Debug {
}

/// Trait-object space handler.
pub type DynSpaceHandler = Arc<dyn SpaceHandler>;

/// Abstract kitsune2 space instance.
pub trait Space: 'static + Send + Sync + std::fmt::Debug {
}

/// Trait-object space instance.
pub type DynSpace = Arc<dyn Space>;

/// A factory to create a new space instance.
pub trait SpaceFactory: 'static + Send + Sync + std::fmt::Debug {
    /// Constructe a new space instance.
    fn create(
        &self,
        handler: DynSpaceHandler,
    ) -> BoxFuture<'static, Result<DynSpace>>;
}
