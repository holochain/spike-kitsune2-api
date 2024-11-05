//! Agent identity, agent crypto, and agent state abstractions.

use crate::types::*;

/// Kitsune2 arq.
pub trait Arq: 'static + Send + Sync + std::fmt::Debug {
    // TODO - this isn't quantized, figure out what this should actually be.
    //        also, this is mixing arc and arc-set.
    /// Get the list of bounds that are included in this arq.
    /// - If the arq is empty, the slice will be len zero.
    /// - If the arq is full, the slice will be
    ///   `[(Included(0), Included(u32::MAX))]`.
    fn list_bounds(&self) -> &[(std::ops::Bound<Loc>, std::ops::Bound<Loc>)];
}

/// Trait-object version of Arq.
pub type DynArq = Arc<dyn Arq + 'static + Send + Sync>;

/// Kitsune2 peer url.
pub type PeerUrl = String;

/// Information about an agent.
pub trait AgentInfo: 'static + Send + Sync + std::fmt::Debug {
    /// Get the hash identifying this agent.
    fn hash(&self) -> &DynHash;

    /// Get the space under which to file this agent info.
    fn space(&self) -> &SpaceHash;

    /// Get the peer url of this agent.
    fn peer_url(&self) -> PeerUrl;

    /// Get the current storage arq claimed by this agent.
    fn storage_arq(&self) -> &DynArq;

    /// Validate a signature authored by this agent.
    fn validate_signature(&self, data: &[u8], sig: &[u8]) -> bool;
}

/// Trait-object version of agent-info.
pub type DynAgentInfo = Arc<dyn AgentInfo + 'static + Send + Sync>;
