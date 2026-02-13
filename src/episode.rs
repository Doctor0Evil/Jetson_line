use serde::{Deserialize, Serialize};

use crate::deeds::Deed;
use crate::judgement::{Judgment, MoralScores};
use crate::metrics::{EpisodeMetrics};
use crate::model::World;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WReflection {
    pub what: String,
    pub so_what: String,
    pub now_what: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeConfig {
    pub max_ticks: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub config: EpisodeConfig,
    pub world: World,
    pub deeds: Vec<Deed>,
    pub judgments: Vec<Judgment>,
    pub metrics: Vec<EpisodeMetrics>,
    pub reflection: Option<WReflection>,
}

impl Episode {
    pub fn new(config: EpisodeConfig, world: World) -> Self {
        Self {
            config,
            world,
            deeds: Vec::new(),
            judgments: Vec::new(),
            metrics: Vec::new(),
            reflection: None,
        }
    }

    pub fn run<F>(&mut self, mut step_fn: F)
    where
        F: FnMut(&mut World) -> Vec<Deed>,
    {
        let mut collapse_count = 0u64;

        for _ in 0..self.config.max_ticks {
            let step_deeds = step_fn(&mut self.world);
            let judgments = crate::judgement::judge_deeds(&self.world, &step_deeds);
            let metrics = crate::metrics::compute_metrics(&self.world, &step_deeds, collapse_count);
            collapse_count = metrics.collapse_events;

            self.deeds.extend(step_deeds);
            self.judgments.extend(judgments);
            self.metrics.push(metrics);
        }
    }

    pub fn set_reflection(&mut self, what: String, so_what: String, now_what: String) {
        self.reflection = Some(WReflection {
            what,
            so_what,
            now_what,
        });
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    pub fn to_cbor(&self) -> Result<Vec<u8>, serde_cbor::Error> {
        serde_cbor::to_vec(self)
    }
}
