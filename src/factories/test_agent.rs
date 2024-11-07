//! Test agent types for testing.

use crate::types::*;
use crate::*;

struct TestHash(u64, Bytes);

impl std::fmt::Debug for TestHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TestHash").field(&self.0).finish()
    }
}

impl std::fmt::Display for TestHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Hash for TestHash {
    fn hash_bytes(&self) -> Bytes {
        self.1.clone()
    }

    fn loc(&self) -> Loc {
        0
    }
}

#[derive(Debug)]
struct TestAgentInfo {
    hash: DynHash,
    space: SpaceHash,
    peer_url: PeerUrl,
    storage_arc: agent::DynArq,
    signed_at: Timestamp,
    expires_at: Timestamp,
}

impl agent::AgentInfo for TestAgentInfo {
    fn hash(&self) -> &DynHash {
        &self.hash
    }

    fn encoded(&self) -> Bytes {
        Bytes::from_static(b"fake-encoded")
    }

    fn signature(&self) -> Bytes {
        Bytes::from_static(b"fake-signature")
    }

    fn space(&self) -> &SpaceHash {
        &self.space
    }

    fn peer_url(&self) -> &PeerUrl {
        &self.peer_url
    }

    fn storage_arq(&self) -> &agent::DynArq {
        &self.storage_arc
    }

    fn signed_at(&self) -> Timestamp {
        self.signed_at
    }

    fn expires_at(&self) -> Timestamp {
        self.expires_at
    }

    fn validate_signature(&self, _data: &[u8], _sig: &[u8]) -> bool {
        true
    }
}

/// Test local agent type for testing.
#[derive(Debug)]
pub struct TestLocalAgent(DynHash);

impl agent::LocalAgent for TestLocalAgent {
    fn hash(&self) -> &DynHash {
        &self.0
    }

    fn sign(&self, _data: &[u8]) -> Bytes {
        Bytes::from_static(b"fake-signature")
    }

    fn create_agent_info(
        &self,
        space: SpaceHash,
        peer_url: PeerUrl,
        storage_arc: agent::DynArq,
        signed_at: Timestamp,
        expires_at: Timestamp,
    ) -> agent::DynAgentInfo {
        let out: agent::DynAgentInfo = Arc::new(TestAgentInfo {
            hash: self.0.clone(),
            space,
            peer_url,
            storage_arc,
            signed_at,
            expires_at,
        });
        out
    }
}

impl TestLocalAgent {
    /// Create a new uniq local test agent.
    pub fn create() -> agent::DynLocalAgent {
        static UNIQ: std::sync::atomic::AtomicU64 =
            std::sync::atomic::AtomicU64::new(1);
        let id = UNIQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let hash = id.to_le_bytes();
        let hash = Bytes::copy_from_slice(&hash);
        let hash: DynHash = Arc::new(TestHash(id, hash));
        let out: agent::DynLocalAgent = Arc::new(TestLocalAgent(hash));
        out
    }
}
