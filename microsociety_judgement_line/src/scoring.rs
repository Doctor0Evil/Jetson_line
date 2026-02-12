use chrono::{Duration, Utc};

use crate::model::{Deed, DeedKind, MoralScore};

/// Parameters that shape how judgement is exercised.
/// These can be tuned based on research and governance decisions.
#[derive(Debug, Clone)]
pub struct ScoringParameters {
    pub good_deed_weight: f64,
    pub heroic_action_weight: f64,
    pub harmful_action_weight: f64,
    pub recency_half_life_hours: f64,
}

impl Default for ScoringParameters {
    fn default() -> Self {
        ScoringParameters {
            good_deed_weight: 1.0,
            heroic_action_weight: 3.0,
            harmful_action_weight: -2.0,
            recency_half_life_hours: 24.0 * 30.0, // ~1 month half-life
        }
    }
}

/// Compute a decay factor based on time difference.
/// Newer deeds have stronger influence; older ones decay
/// along a 1-D exponential curve.
fn time_decay_factor(
    occurred_at: chrono::DateTime<Utc>,
    now: chrono::DateTime<Utc>,
    half_life_hours: f64,
) -> f64 {
    let delta = now - occurred_at;
    let hours = delta.num_seconds() as f64 / 3600.0;
    if hours <= 0.0 || half_life_hours <= 0.0 {
        return 1.0;
    }
    let lambda = (0.5_f64).ln() / half_life_hours;
    (lambda * -hours).exp()
}

/// Assign a base score to a deed before time decay.
fn base_score_for_deed(deed: &Deed, params: &ScoringParameters) -> MoralScore {
    let w = match deed.kind {
        DeedKind::GoodDeed => params.good_deed_weight,
        DeedKind::HeroicAction => params.heroic_action_weight,
        DeedKind::NeutralAction => 0.0,
        DeedKind::HarmfulAction => params.harmful_action_weight,
    };
    MoralScore(w)
}

/// Compute the 1-D moral score for a sequence of deeds.
/// This function enforces a linear aggregation with time-decay,
/// keeping judgement interpretable and auditable.
pub fn compute_moral_score(
    deeds: &[Deed],
    params: &ScoringParameters,
    now: chrono::DateTime<Utc>,
) -> MoralScore {
    deeds.iter().fold(MoralScore::zero(), |acc, deed| {
        let base = base_score_for_deed(deed, params);
        let decay = time_decay_factor(deed.occurred_at, now, params.recency_half_life_hours);
        acc.add(MoralScore(base.0 * decay))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{CitizenId, DeedKind, BiophysicalEvidence};
    use chrono::TimeZone;

    #[test]
    fn test_moral_score_positive_for_good_deed() {
        let params = ScoringParameters::default();
        let now = Utc::now();

        let deed = Deed {
            id: 1,
            citizen: CitizenId("citizen-1".to_string()),
            kind: DeedKind::GoodDeed,
            description: "Helped a neighbor with groceries.".to_string(),
            evidence: vec![BiophysicalEvidence {
                source: "witness_signature".to_string(),
                reference: "proof-1".to_string(),
                recorded_at: now,
            }],
            occurred_at: now,
        };

        let score = compute_moral_score(&[deed], &params, now);
        assert!(score.0 > 0.0);
    }

    #[test]
    fn test_moral_score_negative_for_harmful_action() {
        let params = ScoringParameters::default();
        let now = Utc::now();

        let deed = Deed {
            id: 2,
            citizen: CitizenId("citizen-2".to_string()),
            kind: DeedKind::HarmfulAction,
            description: "Damaged shared property.".to_string(),
            evidence: vec![],
            occurred_at: now,
        };

        let score = compute_moral_score(&[deed], &params, now);
        assert!(score.0 < 0.0);
    }

    #[test]
    fn test_time_decay_reduces_old_deeds_influence() {
        let mut params = ScoringParameters::default();
        params.recency_half_life_hours = 1.0;

        let now = Utc.ymd(2026, 2, 11).and_hms(12, 0, 0);
        let earlier = now - Duration::hours(4);

        let deed_recent = Deed {
            id: 3,
            citizen: CitizenId("citizen-3".to_string()),
            kind: DeedKind::GoodDeed,
            description: "Recent good deed.".to_string(),
            evidence: vec![],
            occurred_at: now,
        };

        let deed_old = Deed {
            id: 4,
            citizen: CitizenId("citizen-3".to_string()),
            kind: DeedKind::GoodDeed,
            description: "Older good deed.".to_string(),
            evidence: vec![],
            occurred_at: earlier,
        };

        let score_recent = compute_moral_score(&[deed_recent], &params, now);
        let score_old = compute_moral_score(&[deed_old], &params, now);

        assert!(score_recent.0 > score_old.0);
    }
}
