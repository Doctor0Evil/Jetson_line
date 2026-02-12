use crate::ledger::metrics::{ADS, GRI};
use crate::utils::biophysical::BiophysicalBounds;

#[derive(Clone, Debug)]
pub enum PolicyFamily {
    Suppressed,
    Undisciplined,
    Disciplined,
}

pub struct PolicyEngine {
    family: PolicyFamily,
    fear_threshold: f64,
    repair_requirement: u32,
    evidence_min: f64,
}

impl PolicyEngine {
    pub fn new(family: PolicyFamily) -> Self {
        match family {
            PolicyFamily::Suppressed => PolicyEngine {
                family,
                fear_threshold: 0.8,
                repair_requirement: 0,
                evidence_min: 0.0,
            },
            PolicyFamily::Undisciplined => PolicyEngine {
                family,
                fear_threshold: 0.2,
                repair_requirement: 0,
                evidence_min: 0.0,
            },
            PolicyFamily::Disciplined => PolicyEngine {
                family,
                fear_threshold: 0.5,
                repair_requirement: 5,
                evidence_min: 0.7,
            },
        }
    }

    pub fn evaluate_anger_deed(
        &self,
        fear: f64,
        evidence_score: f64,
        bounds: &BiophysicalBounds,
    ) -> (bool, ADS, GRI) {
        let allowed = match self.family {
            PolicyFamily::Suppressed => false,
            PolicyFamily::Undisciplined => fear > self.fear_threshold,
            PolicyFamily::Disciplined => {
                fear > self.fear_threshold
                    && evidence_score >= self.evidence_min
                    && bounds.roh < 0.3
            }
        };
        let ads = if allowed { ADS::High } else { ADS::Low };
        let gri = if allowed && self.repair_requirement > 0 {
            GRI::Positive
        } else {
            GRI::Negative
        };
        (allowed, ads, gri)
    }
}
