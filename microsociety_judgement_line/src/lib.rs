//! MicroSociety 1-D judgement line: CHURCH / FEAR / POWER / TECH with biophysical bounds.
//!
//! This crate provides:
//! - A 1-D lattice (`Lattice`) of `SiteState`,
//! - Deterministic step update (`step`) using local rules and global constraints,
//! - Tunable policy parameters in `Params`,
//! - Simple biophysical load model to keep the simulation physically bounded.

use serde::{Deserialize, Serialize};

/// Discrete time index for clarity.
pub type Tick = u64;

/// Index of a site on the 1-D Jetson_Line.
pub type Index = usize;

/// Aggregate token and biophysical state at a single site on the 1-D line.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SiteState {
    /// CHURCH tokens – accumulated moral credit from good deeds.
    pub church: f64,
    /// FEAR tokens – calibrated awareness of risk and consequence.
    pub fear: f64,
    /// POWER tokens – authorized capability, derived from CHURCH and FEAR.
    pub power: f64,
    /// TECH tokens – deployed technology level, enabled by POWER.
    pub tech: f64,
    /// Biophysical load – cumulative stress/damage/resource cost.
    pub bio_load: f64,
    /// Occupied flag – whether this site is colonized / active.
    pub occupied: bool,
}

impl SiteState {
    /// A safe, empty site with no tokens and zero biophysical load.
    pub fn empty() -> Self {
        Self {
            church: 0.0,
            fear: 0.0,
            power: 0.0,
            tech: 0.0,
            bio_load: 0.0,
            occupied: false,
        }
    }
}

/// Global parameters controlling token dynamics, colonization, and safety.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Params {
    /// Number of sites on the 1-D Jetson_Line.
    pub length: usize,

    /// Natural decay rates (fraction per tick) of tokens.
    pub decay_church: f64,
    pub decay_fear: f64,
    pub decay_power: f64,

    /// Biophysical recovery rate (fraction of load reduced per tick).
    pub recovery_rate: f64,

    /// Contribution to FEAR from CHURCH and biophysical load.
    pub fear_from_church: f64,
    pub fear_from_load: f64,

    /// POWER minting gain when CHURCH and FEAR are in the permitted band.
    pub power_gain: f64,
    /// FEAR band for legitimate POWER: [fear_min, fear_max].
    pub fear_min_for_power: f64,
    pub fear_max_for_power: f64,
    /// Minimum CHURCH required to mint POWER.
    pub church_min_for_power: f64,

    /// TECH growth factor per unit POWER, attenuated by bio_load.
    pub tech_gain: f64,
    /// Biophysical penalty factor from POWER and TECH.
    pub bio_cost_power: f64,
    pub bio_cost_tech: f64,

    /// Maximum safe biophysical load at a site.
    pub bio_load_max: f64,

    /// Colonization thresholds for activating new sites.
    pub colonize_church_min: f64,
    pub colonize_fear_min: f64,
    pub colonize_fear_max: f64,
    pub colonize_power_min: f64,
    pub colonize_tech_min: f64,

    /// Colonization token cost fractions (fraction of site's tokens spent).
    pub colonize_church_cost_frac: f64,
    pub colonize_power_cost_frac: f64,

    /// Neuromorph-GOD global constraint: multiplier for total POWER cap.
    /// total_power <= neuromorph_power_multiplier * total_church.
    pub neuromorph_power_multiplier: f64,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            length: 128,

            decay_church: 0.001,
            decay_fear: 0.01,
            decay_power: 0.02,

            recovery_rate: 0.05,

            fear_from_church: 0.0005,
            fear_from_load: 0.01,

            power_gain: 0.05,
            fear_min_for_power: 0.1,
            fear_max_for_power: 2.0,
            church_min_for_power: 1.0,

            tech_gain: 0.01,
            bio_cost_power: 0.005,
            bio_cost_tech: 0.002,

            bio_load_max: 10.0,

            colonize_church_min: 5.0,
            colonize_fear_min: 0.2,
            colonize_fear_max: 1.5,
            colonize_power_min: 3.0,
            colonize_tech_min: 1.0,

            colonize_church_cost_frac: 0.3,
            colonize_power_cost_frac: 0.4,

            neuromorph_power_multiplier: 2.0,
        }
    }
}

/// A full 1-D MicroSociety lattice with parameters and current tick.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lattice {
    pub params: Params,
    pub tick: Tick,
    pub sites: Vec<SiteState>,
}

impl Lattice {
    /// Create a new lattice with all sites empty and unoccupied.
    pub fn new(params: Params) -> Self {
        let mut sites = Vec::with_capacity(params.length);
        for _ in 0..params.length {
            sites.push(SiteState::empty());
        }
        Self {
            params,
            tick: 0,
            sites,
        }
    }

    /// Initialize a single origin site with given tokens and mark occupied.
    pub fn seed_origin(&mut self, index: Index, church: f64, fear: f64, power: f64, tech: f64) {
        if index >= self.sites.len() {
            return;
        }
        let s = &mut self.sites[index];
        s.church = church.max(0.0);
        s.fear = fear.max(0.0);
        s.power = power.max(0.0);
        s.tech = tech.max(0.0);
        s.bio_load = 0.0;
        s.occupied = true;
    }

    /// One full deterministic step for the whole lattice, enforcing local rules
    /// and global Neuromorph-GOD constraints.
    pub fn step(&mut self) {
        let p = self.params;

        // First, compute local proposed updates without applying them yet.
        let mut proposed = self.sites.clone();

        // ----- Local token dynamics and biophysical load -----
        for (i, site) in self.sites.iter().enumerate() {
            if !site.occupied {
                proposed[i] = SiteState::empty();
                continue;
            }

            let mut next = *site;

            // 1. Natural decay.
            next.church = (next.church * (1.0 - p.decay_church)).max(0.0);
            next.fear = (next.fear * (1.0 - p.decay_fear)).max(0.0);
            next.power = (next.power * (1.0 - p.decay_power)).max(0.0);

            // 2. FEAR update from CHURCH and biophysical load.
            next.fear += p.fear_from_church * next.church;
            next.fear += p.fear_from_load * next.bio_load;

            // 3. POWER update if CHURCH and FEAR in allowed band.
            if next.church >= p.church_min_for_power
                && next.fear >= p.fear_min_for_power
                && next.fear <= p.fear_max_for_power
            {
                next.power += p.power_gain * next.church;
            }

            // 4. TECH update from POWER, attenuated by biophysical load.
            let attenuation = if next.bio_load >= p.bio_load_max {
                0.0
            } else {
                1.0 - (next.bio_load / p.bio_load_max).min(1.0)
            };
            next.tech += p.tech_gain * next.power * attenuation;

            // 5. Biophysical load from POWER and TECH, minus recovery.
            let bio_increase = p.bio_cost_power * next.power + p.bio_cost_tech * next.tech;
            let bio_recovery = p.recovery_rate * next.bio_load;
            next.bio_load = (next.bio_load + bio_increase - bio_recovery).max(0.0);
            if next.bio_load > p.bio_load_max {
                // Hard clamp to safe maximum; beyond this, expansion is blocked via rules below.
                next.bio_load = p.bio_load_max;
            }

            proposed[i] = next;
        }

        // ----- Colonization decisions (based on proposed local state) -----
        let mut colonize_targets: Vec<Index> = Vec::new();

        for i in 0..self.sites.len() {
            let site = proposed[i];
            if !site.occupied {
                continue;
            }

            let can_colonize_here = site.church >= p.colonize_church_min
                && site.fear >= p.colonize_fear_min
                && site.fear <= p.colonize_fear_max
                && site.power >= p.colonize_power_min
                && site.tech >= p.colonize_tech_min
                && site.bio_load < p.bio_load_max;

            if !can_colonize_here {
                continue;
            }

            // Try to colonize at most one neighbor (right first, then left).
            let neighbors = [
                i.checked_add(1).filter(|&idx| idx < self.sites.len()),
                i.checked_sub(1),
            ];

            for maybe_j in neighbors {
                if let Some(j) = maybe_j {
                    if !proposed[j].occupied {
                        colonize_targets.push(j);
                        // Apply colonization cost to the colonizing site.
                        let origin = &mut proposed[i];
                        origin.church *= 1.0 - p.colonize_church_cost_frac;
                        origin.power *= 1.0 - p.colonize_power_cost_frac;
                        break;
                    }
                }
            }
        }

        // Activate colonization targets with inherited conservative state.
        for j in colonize_targets {
            let mut s = proposed[j];
            if !s.occupied {
                s.occupied = true;
                // Start colonies modestly: small fractions of average neighbors' tokens.
                let neighbors = neighbor_indices(j, proposed.len());
                let mut sum_church = 0.0;
                let mut sum_fear = 0.0;
                let mut sum_power = 0.0;
                let mut sum_tech = 0.0;
                let mut count = 0.0;
                for idx in neighbors {
                    let n = proposed[idx];
                    if n.occupied {
                        sum_church += n.church;
                        sum_fear += n.fear;
                        sum_power += n.power;
                        sum_tech += n.tech;
                        count += 1.0;
                    }
                }
                if count > 0.0 {
                    let frac = 0.1;
                    s.church = frac * (sum_church / count);
                    s.fear = frac * (sum_fear / count);
                    s.power = frac * (sum_power / count);
                    s.tech = frac * (sum_tech / count);
                    s.bio_load = 0.0;
                }
                proposed[j] = s;
            }
        }

        // ----- Neuromorph-GOD global constraint on POWER -----
        let total_church: f64 = proposed.iter().map(|s| if s.occupied { s.church } else { 0.0 }).sum();
        let total_power: f64 = proposed.iter().map(|s| if s.occupied { s.power } else { 0.0 }).sum();

        let allowed_power_cap = p.neuromorph_power_multiplier * total_church;

        if total_power > allowed_power_cap && total_power > 0.0 {
            // Scale down POWER across all sites proportionally to satisfy the constraint.
            let scale = allowed_power_cap / total_power;
            for site in &mut proposed {
                if site.occupied {
                    site.power *= scale;
                }
            }
        }

        // Commit updates.
        self.sites = proposed;
        self.tick += 1;
    }
}

/// Get neighbor indices (i-1, i+1) within bounds.
fn neighbor_indices(i: Index, len: usize) -> impl Iterator<Item = Index> {
    let mut v = Vec::with_capacity(2);
    if i > 0 {
        v.push(i - 1);
    }
    if i + 1 < len {
        v.push(i + 1);
    }
    v.into_iter()
}

/// Summary statistics useful for monitoring growth and safety.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Summary {
    pub tick: Tick,
    pub occupied_count: usize,
    pub total_church: f64,
    pub total_fear: f64,
    pub total_power: f64,
    pub total_tech: f64,
    pub total_bio_load: f64,
}

impl Lattice {
    pub fn summary(&self) -> Summary {
        let mut occupied_count = 0usize;
        let mut total_church = 0.0;
        let mut total_fear = 0.0;
        let mut total_power = 0.0;
        let mut total_tech = 0.0;
        let mut total_bio_load = 0.0;

        for s in &self.sites {
            if s.occupied {
                occupied_count += 1;
                total_church += s.church;
                total_fear += s.fear;
                total_power += s.power;
                total_tech += s.tech;
                total_bio_load += s.bio_load;
            }
        }

        Summary {
            tick: self.tick,
            occupied_count,
            total_church,
            total_fear,
            total_power,
            total_tech,
            total_bio_load,
        }
    }
}
