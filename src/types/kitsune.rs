//! Kitsune2 types.

use crate::types::*;

/// Handler for requests and events coming out of the kitsune runtime.
pub trait Kitsune2Handle: 'static + Send + std::fmt::Debug {
    /// Kitsune would like to construct a space. Provide a handler.
    fn create_space(&mut self, space: &SpaceHash) -> space::DynSpaceHandler;
}

/// Abstract kitsune2 instance.
pub trait Kitsune2: 'static + Send + Sync + std::fmt::Debug {
    /// Assert a space exists, and get a handle to it.
    fn assert_space(&self, space: &SpaceHash) -> space::DynSpace;
}
