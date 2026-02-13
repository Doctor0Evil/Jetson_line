use serde::{Deserialize, Serialize};

pub type Tick = u64;
pub type Index = usize;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SiteState {
    pub church: f64,
    pub fear: f64,
    pub power: f64,
    pub tech: f64,
    pub bioload: f64,
    pub bioload_max: f64,
    pub trust_left: f64,
    pub trust_right: f64,
    pub occupied: bool,
}

impl SiteState {
    pub fn empty(bioload_max: f64) -> Self {
        Self {
            church: 0.0,
            fear: 0.0,
            power: 0.0,
            tech: 0.0,
            bioload: 0.0,
            bioload_max,
            trust_left: 0.0,
            trust_right: 0.0,
            occupied: false,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct WorldParams {
    pub length: usize,

    pub decay_church: f64,
    pub decay_fear: f64,
    pub decay_power: f64,
    pub decay_tech: f64,
    pub recovery_rate: f64,

    pub fear_from_load: f64,
    pub fear_from_trust: f64,

    pub power_gain: f64,
    pub fear_min_for_power: f64,
    pub fear_max_for_power: f64,
    pub church_min_for_power: f64,

    pub tech_gain: f64,
    pub biocost_power: f64,
    pub biocost_tech: f64,

    pub colonize_church_min: f64,
    pub colonize_fear_min: f64,
    pub colonize_fear_max: f64,
    pub colonize_power_min: f64,
    pub colonize_tech_min: f64,
    pub colonize_church_cost_frac: f64,
    pub colonize_power_cost_frac: f64,

    pub trust_help_gain: f64,
    pub trust_conflict_loss: f64,
    pub trust_repair_gain: f64,
}

impl Default for WorldParams {
    fn default() -> Self {
        Self {
            length: 128,
            decay_church: 0.001,
            decay_fear: 0.01,
            decay_power: 0.02,
            decay_tech: 0.005,
            recovery_rate: 0.05,
            fear_from_load: 0.1,
            fear_from_trust: -0.05,
            power_gain: 0.05,
            fear_min_for_power: 0.1,
            fear_max_for_power: 2.0,
            church_min_for_power: 1.0,
            tech_gain: 0.02,
            biocost_power: 0.01,
            biocost_tech: 0.005,
            colonize_church_min: 5.0,
            colonize_fear_min: 0.2,
            colonize_fear_max: 1.5,
            colonize_power_min: 3.0,
            colonize_tech_min: 1.0,
            colonize_church_cost_frac: 0.3,
            colonize_power_cost_frac: 0.4,
            trust_help_gain: 0.1,
            trust_conflict_loss: -0.2,
            trust_repair_gain: 0.15,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GlobalConstraints {
    pub neuromorph_power_multiplier: f64,
    pub total_bioload_max: f64,
    pub fear_min_action: f64,
    pub fear_max_action: f64,
}

impl Default for GlobalConstraints {
    fn default() -> Self {
        Self {
            neuromorph_power_multiplier: 2.0,
            total_bioload_max: 128.0,
            fear_min_action: 0.1,
            fear_max_action: 3.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub params: WorldParams,
    pub constraints: GlobalConstraints,
    pub sites: Vec<SiteState>,
    pub tick: Tick,
}

impl World {
    pub fn new(params: WorldParams, constraints: GlobalConstraints, bioload_max: f64) -> Self {
        let mut sites = Vec::with_capacity(params.length);
        for _ in 0..params.length {
            sites.push(SiteState::empty(bioload_max));
        }
        Self {
            params,
            constraints,
            sites,
            tick: 0,
        }
    }

    pub fn occupy_origin(&mut self, index: Index, church: f64) {
        if index < self.sites.len() {
            let s = &mut self.sites[index];
            s.occupied = true;
            s.church = church.max(0.0);
        }
    }
}
