use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Unique identifier for an Augmented-Citizen within MicroSociety.
/// This is intentionally opaque: it can be mapped to DID/ALN/KYC
/// systems outside this crate.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CitizenId(pub String);

/// BiophysicalEvidence represents non-fictional, recorded proof
/// of an action. For example: a signed log entry, sensor hash,
/// or cryptographic reference into an external audit trail.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiophysicalEvidence {
    /// Human-readable description of the evidence source.
    pub source: String,
    /// Opaque reference to verifiable data (hash, URI, etc.).
    pub reference: String,
    /// Timestamp when the evidence was recorded.
    pub recorded_at: DateTime<Utc>,
}

/// DeedKind encodes the moral framing of a deed.
/// This is intentionally simple and can be extended with
/// additional variants backed by research and governance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeedKind {
    GoodDeed,
    HeroicAction,
    NeutralAction,
    HarmfulAction,
}

/// A Deed is a concrete, non-fictional action with evidence,
/// authored by an Augmented-Citizen, subject to judgement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deed {
    pub id: u64,
    pub citizen: CitizenId,
    pub kind: DeedKind,
    /// Short description focusing on observable, non-fictional behavior.
    pub description: String,
    /// Evidence supporting that this deed actually occurred.
    pub evidence: Vec<BiophysicalEvidence>,
    /// Time when the deed occurred (or was finalized).
    pub occurred_at: DateTime<Utc>,
}

/// MoralScore represents an aggregate evaluation along a 1-D line.
/// Positive values indicate life-affirming deeds; negative values
/// indicate harmful patterns; zero is neutral/uncertain.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MoralScore(pub f64);

impl MoralScore {
    pub fn zero() -> Self {
        MoralScore(0.0)
    }

    pub fn add(self, other: MoralScore) -> Self {
        MoralScore(self.0 + other.0)
    }
}
