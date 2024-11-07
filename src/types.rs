//! Kitsune2 types and traits.

use crate::*;

/// Kitsune2 peer url.
pub type PeerUrl = String;

/// Kitsune2 location.
pub type Loc = u32;

/// Kitsune2 space.
pub type SpaceHash = Bytes;

/// Kitsune2 timestamp.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp(i64);

impl From<std::time::SystemTime> for Timestamp {
    fn from(t: std::time::SystemTime) -> Self {
        Self(
            t.duration_since(std::time::SystemTime::UNIX_EPOCH)
                .expect("invalid system time")
                .as_micros() as i64,
        )
    }
}

impl From<Timestamp> for std::time::SystemTime {
    fn from(t: Timestamp) -> Self {
        std::time::SystemTime::UNIX_EPOCH
            + std::time::Duration::from_micros(t.0 as u64)
    }
}

impl From<Timestamp> for i64 {
    fn from(t: Timestamp) -> Self {
        t.0
    }
}

impl From<i64> for Timestamp {
    fn from(i: i64) -> Self {
        Self(i)
    }
}

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
