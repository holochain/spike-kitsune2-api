#![deny(missing_docs)]
//! Kitsune2 p2p sharded dht
//!
//! ## High-Level Open Questions
//!
//! - Do we want to re-tools arcs/quant to make it easier to reason about?
//! - Do we want to move to bootstrap2 so we can use a more modern AgentInfo?
//! - Hashes should be
//!     - displayed in debugging in the holochain canonical fmt
//!       i.e. `uhCkblabla`
//!     - any size to support other crypto schemes
//!     - hashes cannot validate signatures... that fn is on info
//!       again to support other crypto schemes
//!

use std::io::Result;
use std::sync::Arc;

use bytes::Bytes;

// Using BoxFuture because even though async trait has been partially stablized
// this is still the best/only way to get the bounds right and still be
// compatible with trait-objects.
use futures::future::BoxFuture;

pub mod builder;
pub mod config;
pub mod factories;
pub mod types;

#[cfg(test)]
mod test;
