use crate::model::{CitizenId, Deed};
use crate::scoring::{ScoringParameters, compute_moral_score};
use crate::model::MoralScore;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error types for the 1-D judgement line.
#[derive(Debug, Error)]
pub enum LineError {
    #[error("deed id {0} already exists on the line")]
    DuplicateDeedId(u64),
}

/// JetsonLine represents the 1-Dimensional ordered space
/// where deeds are appended and judged.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct JetsonLine {
    deeds: Vec<Deed>,
}

impl JetsonLine {
    /// Create an empty Jetson_Line.
    pub fn new() -> Self {
        JetsonLine { deeds: Vec::new() }
    }

    /// Append a new deed to the line, preserving order.
    /// Rejects duplicate IDs to maintain unambiguous judgement.
    pub fn append_deed(&mut self, deed: Deed) -> Result<(), LineError> {
        if self.deeds.iter().any(|d| d.id == deed.id) {
            return Err(LineError::DuplicateDeedId(deed.id));
        }
        self.deeds.push(deed);
        Ok(())
    }

    /// Return all deeds, in the order they were appended.
    pub fn deeds(&self) -> &[Deed] {
        &self.deeds
    }

    /// Compute the moral score for a specific citizen on this line.
    pub fn moral_score_for_citizen(
        &self,
        citizen: &CitizenId,
        params: &ScoringParameters,
    ) -> MoralScore {
        let now = Utc::now();
        let relevant: Vec<Deed> = self
            .deeds
            .iter()
            .filter(|d| &d.citizen == citizen)
            .cloned()
            .collect();
        compute_moral_score(&relevant, params, now)
    }

    /// Serialize the entire line as JSON for audit or external
    /// Tree-of-Life mapping. This keeps the 1-D structure exportable.
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}
