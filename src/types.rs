//! Kitsune2 types and traits.

pub use std::io::Result;
use std::sync::Arc;

use bytes::Bytes;

// Using BoxFuture because even though async trait has been partially stablized
// this is still the best/only way to get the bounds right and still be
// compatible with trait-objects.
use futures_util::future::BoxFuture;

/// Kitsune2 peer url.
pub type PeerUrl = String;

/// Kitsune2 location.
pub type Loc = u32;

/// Kitsune2 space.
pub type SpaceHash = Bytes;

/// Kitsune2 hash.
pub trait Hash:
    'static + Send + Sync + std::fmt::Display + std::fmt::Debug
{
    /// Get the core/raw hash bytes without any prefix decoration or
    /// location suffix.
    fn hash_bytes(&self) -> Bytes;

    /// Get the loc.
    fn loc(&self) -> Loc;
}

/// Trait-object version of hash.
pub type DynHash = Arc<dyn Hash + 'static + Send + Sync>;

pub mod agent;
pub mod kitsune;
pub mod op_store;
pub mod peer_store;
pub mod space;
pub mod tx;
