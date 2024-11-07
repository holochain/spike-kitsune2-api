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
pub type DynArq = Arc<dyn Arq>;

/// Information about an agent.
pub trait AgentInfo: 'static + Send + Sync + std::fmt::Debug {
    /// Get the hash identifying this agent.
    fn hash(&self) -> &DynHash;

    /// Get the raw encoded bytes of this agent_info.
    fn encoded(&self) -> Bytes;

    /// Get the signature over this agent_info's raw bytes.
    fn signature(&self) -> Bytes;

    /// Get the space under which to file this agent info.
    fn space(&self) -> &SpaceHash;

    /// Get the peer url of this agent.
    fn peer_url(&self) -> &PeerUrl;

    /// Get the current storage arq claimed by this agent.
    fn storage_arq(&self) -> &DynArq;

    /// Get the timestamp at which this agent_info was signed.
    fn signed_at(&self) -> Timestamp;

    /// Get the timestamp at which this agent_info will expire.
    fn expires_at(&self) -> Timestamp;

    /// Validate a signature authored by this agent.
    fn validate_signature(&self, data: &[u8], sig: &[u8]) -> bool;
}

/// Trait-object version of agent-info.
pub type DynAgentInfo = Arc<dyn AgentInfo>;

/// Additional api required to support local agents.
pub trait LocalAgent: 'static + Send + Sync + std::fmt::Debug {
    /// Get the hash identifying this local agent.
    fn hash(&self) -> &DynHash;

    /// Provide some data to be signed by this local agent.
    fn sign(&self, data: &[u8]) -> Bytes;

    /// Generate a new signed agent info for this agent.
    fn create_agent_info(
        &self,
        space: SpaceHash,
        peer_url: PeerUrl,
        storage_arc: DynArq,
        signed_at: Timestamp,
        expires_at: Timestamp,
    ) -> DynAgentInfo;
}

/// Trait-object local agent.
pub type DynLocalAgent = Arc<dyn LocalAgent>;
