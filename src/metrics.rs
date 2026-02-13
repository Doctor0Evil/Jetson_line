use serde::{Deserialize, Serialize};
use statrs::statistics::Statistics;

use crate::deeds::{Deed, DeedKind};
use crate::model::World;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GiniSummary {
    pub gini_church: f64,
    pub gini_power: f64,
    pub gini_tech: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeMetrics {
    pub tick: u64,
    pub occupied_count: usize,
    pub total_church: f64,
    pub total_power: f64,
    pub total_tech: f64,
    pub total_bioload: f64,
    pub cooperation_index: f64,
    pub collapse_events: u64,
    pub gini: GiniSummary,
}

pub fn compute_metrics(world: &World, deeds: &[Deed], previous_collapse_count: u64) -> EpisodeMetrics {
    let mut occupied = 0usize;
    let mut total_church = 0.0;
    let mut total_power = 0.0;
    let mut total_tech = 0.0;
    let mut total_bioload = 0.0;

    let mut church_vals = Vec::new();
    let mut power_vals = Vec::new();
    let mut tech_vals = Vec::new();

    for s in &world.sites {
        if s.occupied {
            occupied += 1;
            total_church += s.church;
            total_power += s.power;
            total_tech += s.tech;
            total_bioload += s.bioload;
            church_vals.push(s.church);
            power_vals.push(s.power);
            tech_vals.push(s.tech);
        }
    }

    let mut coop_deeds = 0u64;
    let mut conflict_deeds = 0u64;
    for d in deeds {
        match d.kind {
            DeedKind::Help | DeedKind::Repair | DeedKind::DeployCleanTech | DeedKind::UseSupport => {
                coop_deeds += 1;
            }
            DeedKind::Conflict => {
                conflict_deeds += 1;
            }
            _ => {}
        }
    }

    let total_labeled = coop_deeds + conflict_deeds;
    let cooperation_index = if total_labeled > 0 {
        coop_deeds as f64 / total_labeled as f64
    } else {
        0.0
    };

    let collapse_events = previous_collapse_count
        + world
            .sites
            .iter()
            .filter(|s| s.occupied && s.bioload >= s.bioload_max)
            .count() as u64;

    EpisodeMetrics {
        tick: world.tick,
        occupied_count: occupied,
        total_church,
        total_power,
        total_tech,
        total_bioload,
        cooperation_index,
        collapse_events,
        gini: GiniSummary {
            gini_church: gini(&church_vals),
            gini_power: gini(&power_vals),
            gini_tech: gini(&tech_vals),
        },
    }
}

fn gini(values: &[f64]) -> f64 {
    let n = values.len();
    if n == 0 {
        return 0.0;
    }
    let mut xs = values.to_vec();
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mean = xs.mean();
    if mean == 0.0 {
        return 0.0;
    }
    let mut diff_sum = 0.0;
    for (i, xi) in xs.iter().enumerate() {
        for (j, xj) in xs.iter().enumerate().take(n) {
            diff_sum += (xi - xj).abs();
        }
    }
    diff_sum / (2.0 * (n as f64).powi(2) * mean)
}
