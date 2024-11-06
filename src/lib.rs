#![deny(missing_docs)]
//! Kitsune2 p2p sharded dht
//!
//! ## High-Level Open Questions
//!
//! - Do we want to re-tools arcs/quant to make it easier to reason about?
//! - Do we want to skip arcs/quant while we're not sharding?
//! - Do we want to move to bootstrap2 so we can use a more modern AgentInfo?
//! - Do we want to use generic loc, or hard-code u32?
//! - Do we want to use generic crypto or hard-code ed25519?
//! - Hashes should be
//!     - displayed in debugging in the holochain canonical fmt
//!       i.e. `uhCkblabla`
//!     - any size to support other crypto schemes
//!     - hashes cannot validate signatures... that fn in on info
//!       again to support other crypto schemes
//!
//! ## Modularization Domains
//!
//! - "Agent" identity+crypto + online-ness (join)
//! - Space management
//! - Peer Store
//! - Connectivity + Connection Blocking
//! - Bootstrapping
//! - Sharding / Arc Sizing aka authority domain or neighborhood management
//! - Op Store
//! - Gossip of op hashes loc-d within claimed authority
//! - Publishing of op hashes loc-d within claimed authority
//! - Fetching of op data
//! - Actual Kitsune, brining all of this together

use std::io::Result;
use std::sync::Arc;

use bytes::Bytes;

// Using BoxFuture because even though async trait has been partially stablized
// this is still the best/only way to get the bounds right and still be
// compatible with trait-objects.
use futures_util::future::BoxFuture;

pub mod builder;
pub mod config;
pub mod mem;
pub mod types;

/// NOTE this module is NOT #[cfg(test)]!
/// It contains test factories that are used in integration tests,
/// and are useful outside of this crate. They are small, and if not
/// used in production projects, will be eliminated by the rust compiler.
pub mod test_factories;
