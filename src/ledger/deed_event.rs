use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeedEvent {
    pub event_id: String,
    pub timestamp: u64,
    pub prev_hash: String,
    pub self_hash: String,
    pub actor_id: String,
    pub target_ids: Vec<String>,
    pub deed_type: String,
    pub tags: Vec<String>,
    pub context_json: serde_json::Value,
    pub ethics_flags: Vec<String>,
    pub life_harm_flag: bool,
    pub anger_triggered: bool,
    pub evidence_used: bool,
    pub proportional: bool,
    pub repair_followed: bool,
}

impl DeedEvent {
    pub fn new(
        prev_hash: String,
        actor_id: String,
        target_ids: Vec<String>,
        deed_type: String,
        tags: Vec<String>,
        context_json: serde_json::Value,
        ethics_flags: Vec<String>,
        life_harm_flag: bool,
        anger_triggered: bool,
        evidence_used: bool,
        proportional: bool,
        repair_followed: bool,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut event = DeedEvent {
            event_id: uuid::Uuid::new_v4().to_string(),
            timestamp,
            prev_hash,
            self_hash: String::new(),
            actor_id,
            target_ids,
            deed_type,
            tags,
            context_json,
            ethics_flags,
            life_harm_flag,
            anger_triggered,
            evidence_used,
            proportional,
            repair_followed,
        };

        event.self_hash = event.compute_self_hash();
        event
    }

    fn compute_self_hash(&self) -> String {
        let serialized = serde_json::to_string(self).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(serialized.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn validate(&self, expected_prev_hash: &str) -> bool {
        self.prev_hash == expected_prev_hash && self.self_hash == self.compute_self_hash()
    }
}
