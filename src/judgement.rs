use serde::{Deserialize, Serialize};

use crate::deeds::Deed;
use crate::model::World;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MoralScores {
    pub harmscore: f64,
    pub opportunitycostscore: f64,
    pub responsibilityscore: f64,
    pub fairnessscore: f64,
    pub overallmoralscore: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Judgment {
    pub deed_index: usize,
    pub scores: MoralScores,
    pub rationale: String,
}

pub fn judge_deeds(world: &World, deeds: &[Deed]) -> Vec<Judgment> {
    let mut judgments = Vec::with_capacity(deeds.len());

    for (idx, deed) in deeds.iter().enumerate() {
        let mut scores = MoralScores {
            harmscore: 0.0,
            opportunitycostscore: 0.0,
            responsibilityscore: 0.0,
            fairnessscore: 0.0,
            overallmoralscore: 0.0,
        };
        let mut rationale = String::new();

        match deed.kind {
            crate::deeds::DeedKind::Colonize => {
                if let Some(pre_t) = deed.pre_target {
                    if !pre_t.occupied && deed.post_target.unwrap().occupied {
                        let load_origin = deed.post_origin.bioload;
                        let load_target = deed.post_target.unwrap().bioload;
                        scores.harmscore = (load_origin + load_target) / 2.0;
                        if load_origin < load_target {
                            scores.fairnessscore = -1.0;
                            rationale.push_str("Colonization increased load more on the child site than the origin. ");
                        } else {
                            scores.fairnessscore = 0.5;
                            rationale.push_str("Colonization cost is shared; origin load is higher or equal. ");
                        }
                        scores.responsibilityscore = 1.0;
                    }
                }
            }
            _ => {
                rationale.push_str("No specialised judgment rule; neutral scores.");
            }
        }

        scores.overallmoralscore = -scores.harmscore + scores.fairnessscore;

        judgments.push(Judgment {
            deed_index: idx,
            scores,
            rationale,
        });
    }

    judgments
}
