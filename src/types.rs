//! Kitsune2 types and traits.

pub use std::io::Result;
use std::sync::Arc;

// Using BoxFuture because even though async trait has been partially stablized
// this is still the best/only way to get the bounds right and still be
// compatible with trait-objects.
use futures_util::future::BoxFuture;

/// Kitsune2 location.
pub type Loc = u32;

/// Kitsune2 space.
pub type SpaceHash = Arc<[u8]>;

/// Kitsune2 hash.
pub trait Hash:
    'static + Send + Sync + std::fmt::Display + std::fmt::Debug
{
    /// Get the core/raw hash bytes without any prefix decoration or
    /// location suffix.
    fn hash_bytes(&self) -> &[u8];

    /// Get the loc.
    fn loc(&self) -> Loc;
}

/// Trait-object version of hash.
pub type DynHash = Arc<dyn Hash + 'static + Send + Sync>;

pub mod agent;
pub mod tx;
pub mod space;
pub mod kitsune;
