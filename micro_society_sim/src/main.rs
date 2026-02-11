//! Micro‑Society Tree‑of‑Life 1D sandbox for cybernetic / nanoswarm research.
//! Non‑actuating, log‑only, CPU‑safe simulation core designed to integrate
//! with neuromorphic / nano‑swarm crates and higher‑level Cybercore‑Brain
//! orchestration layers.
//
//  Edition 2024 compatible.

#![forbid(unsafe_code)]
#![deny(warnings)]

use std::collections::VecDeque;
use std::fmt;
use std::time::{Duration, SystemTime};

/// Global system metadata, sanitized and embedded as a non‑actuating configuration.
#[derive(Debug, Clone)]
pub struct VirtaSysConfig {
    pub system_version: &'static str,
    pub architecture: &'static str,
    pub firmware_chain_id: &'static str,
    pub platform: &'static str,
    pub operation_mode: &'static str,
}

impl Default for VirtaSysConfig {
    fn default() -> Self {
        Self {
            system_version: "VirtaSys vX.11 [Core Release: ∞-AXIOM-fork]",
            architecture: "Dimensity-1000C",
            firmware_chain_id: "0x021948A3-D",
            platform: "VirtaSys [Release: 0xCARRIER-FRAME-88E]",
            operation_mode: "Live-World_Runtime (SIM-ONLY, NON‑ACTUATING)",
        }
    }
}

/// One‑dimensional lattice coordinate.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct LatticeIndex(pub usize);

/// Core scalar for Tree‑of‑Life biophysical rules, clamped to [0, 1].
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BioScalar(f32);

impl BioScalar {
    pub fn new_clamped(v: f32) -> Self {
        let v = if v.is_nan() { 0.0 } else { v.max(0.0).min(1.0) };
        Self(v)
    }

    pub fn value(self) -> f32 {
        self.0
    }

    pub fn lerp(self, other: BioScalar, t: f32) -> BioScalar {
        let t = t.max(0.0).min(1.0);
        BioScalar::new_clamped(self.0 + (other.0 - self.0) * t)
    }

    pub fn add_clamped(self, delta: f32) -> BioScalar {
        BioScalar::new_clamped(self.0 + delta)
    }

    pub fn mul_clamped(self, factor: f32) -> BioScalar {
        BioScalar::new_clamped(self.0 * factor)
    }
}

/// Simple biophysical state for an agent on the 1D lattice.
#[derive(Debug, Clone)]
pub struct BioState {
    pub energy: BioScalar,
    pub cohesion: BioScalar,
    pub adaptation: BioScalar,
    pub outreach: BioScalar,
}

impl BioState {
    pub fn new_random(seed: u64, idx: usize) -> Self {
        // Small deterministic hash for reproducible pseudo‑randomness.
        fn h(seed: u64, salt: u64) -> f32 {
            let mut x = seed ^ (salt.wrapping_mul(0x9E3779B97F4A7C15));
            x ^= x >> 33;
            x = x.wrapping_mul(0x62A9D9ED799705F5);
            x ^= x >> 28;
            x = x.wrapping_mul(0xCB24D0A5C88C35B3);
            x ^= x >> 32;
            let v = (x as f64 / u64::MAX as f64) as f32;
            v.max(0.0).min(1.0)
        }

        Self {
            energy: BioScalar::new_clamped(h(seed, idx as u64)),
            cohesion: BioScalar::new_clamped(h(seed, (idx as u64) ^ 0x11)),
            adaptation: BioScalar::new_clamped(h(seed, (idx as u64) ^ 0x22)),
            outreach: BioScalar::new_clamped(h(seed, (idx as u64) ^ 0x33)),
        }
    }
}

/// Rights and responsibilities structure for education / micro‑society logic.
#[derive(Debug, Clone)]
pub struct RightsProfile {
    pub learning_weight: BioScalar,
    pub responsibility_weight: BioScalar,
    pub media_influence: BioScalar,
}

/// 1D neighbor snapshot for Tree‑of‑Life style update rules.
#[derive(Debug, Clone)]
pub struct Neighborhood1D {
    pub left: Option<BioState>,
    pub center: BioState,
    pub right: Option<BioState>,
}

/// Simulation‑time step.
#[derive(Debug, Copy, Clone)]
pub struct Tick(pub u64);

/// Internal ledger event types.
#[derive(Debug, Clone)]
pub enum DeedKind {
    CycleAdvance { from: Tick, to: Tick },
    EnergyDiffusion { idx: usize, delta: f32 },
    CohesionAdjustment { idx: usize, new_value: f32 },
    OutreachBroadcast { idx: usize, strength: f32 },
    RightsRecomputed { idx: usize },
}

/// Hash‑linked ledger entry.
#[derive(Debug, Clone)]
pub struct DeedEvent {
    pub id: u64,
    pub timestamp: SystemTime,
    pub kind: DeedKind,
    pub prev_hash: u64,
    pub hash: u64,
}

impl DeedEvent {
    fn compute_hash(id: u64, ts: &SystemTime, kind: &DeedKind, prev_hash: u64) -> u64 {
        let dur = ts
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0));
        let t = dur.as_nanos() as u64;
        let mut h = id
            ^ t.rotate_left(7)
            ^ prev_hash.rotate_left(13)
            ^ (std::mem::discriminant(kind) as u64).rotate_left(3);
        // Mild avalanche.
        h ^= h >> 33;
        h = h.wrapping_mul(0xff51afd7ed558ccd);
        h ^= h >> 33;
        h = h.wrapping_mul(0xc4ceb9fe1a85ec53);
        h ^= h >> 33;
        h
    }

    pub fn new(id: u64, prev_hash: u64, kind: DeedKind) -> Self {
        let ts = SystemTime::now();
        let hash = Self::compute_hash(id, &ts, &kind, prev_hash);
        Self {
            id,
            timestamp: ts,
            kind,
            prev_hash,
            hash,
        }
    }
}

/// Append‑only ledger.
#[derive(Debug, Default)]
pub struct Ledger {
    events: Vec<DeedEvent>,
}

impl Ledger {
    pub fn push(&mut self, kind: DeedKind) {
        let (id, prev_hash) = match self.events.last() {
            Some(e) => (e.id + 1, e.hash),
            None => (0, 0),
        };
        let event = DeedEvent::new(id, prev_hash, kind);
        self.events.push(event);
    }

    pub fn tail_hash(&self) -> u64 {
        self.events.last().map(|e| e.hash).unwrap_or(0)
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }
}

/// Learning‑object types (educational kernels) for micro‑society exploration.
#[derive(Debug, Clone)]
pub enum LearningObjectKind {
    MediaLiteracy,
    EnergyStewardship,
    CooperationGame,
    OutreachCampaign,
    ThermoRegulationStudy,
}

#[derive(Debug, Clone)]
pub struct LearningObject {
    pub id: u64,
    pub title: String,
    pub kind: LearningObjectKind,
    pub difficulty: BioScalar,
    pub impact_on_energy: BioScalar,
    pub impact_on_cohesion: BioScalar,
    pub impact_on_outreach: BioScalar,
}

impl LearningObject {
    pub fn apply(&self, state: &mut BioState) {
        state.energy = state.energy.lerp(self.impact_on_energy, self.difficulty.value());
        state.cohesion = state
            .cohesion
            .lerp(self.impact_on_cohesion, self.difficulty.value());
        state.outreach = state
            .outreach
            .lerp(self.impact_on_outreach, self.difficulty.value());
    }
}

/// Micro‑society agent on the 1D lattice.
#[derive(Debug, Clone)]
pub struct Agent {
    pub id: String,
    pub index: LatticeIndex,
    pub bio: BioState,
    pub rights: RightsProfile,
    pub last_learning_ids: VecDeque<u64>,
}

impl Agent {
    pub fn new(id: String, index: LatticeIndex, bio: BioState) -> Self {
        Self {
            id,
            index,
            bio,
            rights: RightsProfile {
                learning_weight: BioScalar::new_clamped(0.5),
                responsibility_weight: BioScalar::new_clamped(0.5),
                media_influence: BioScalar::new_clamped(0.5),
            },
            last_learning_ids: VecDeque::with_capacity(8),
        }
    }

    fn observe_neighbors(&self, lattice: &Lattice) -> Neighborhood1D {
        let idx = self.index.0;
        let left = if idx > 0 {
            lattice
                .agents
                .get(idx - 1)
                .map(|a| a.bio.clone())
        } else {
            None
        };
        let right = lattice
            .agents
            .get(idx + 1)
            .map(|a| a.bio.clone());
        Neighborhood1D {
            left,
            center: self.bio.clone(),
            right,
        }
    }

    fn update_rights_from_neighborhood(&mut self, nb: &Neighborhood1D) {
        let mut local_cohesion = nb.center.cohesion.value();
        let mut neighbors = 1.0;
        if let Some(l) = &nb.left {
            local_cohesion += l.cohesion.value();
            neighbors += 1.0;
        }
        if let Some(r) = &nb.right {
            local_cohesion += r.cohesion.value();
            neighbors += 1.0;
        }
        let avg_cohesion = local_cohesion / neighbors;
        let learning = BioScalar::new_clamped(avg_cohesion);
        let responsibility = BioScalar::new_clamped(1.0 - (nb.center.energy.value() - 0.5).abs());
        let media_influence =
            BioScalar::new_clamped((nb.center.outreach.value() + avg_cohesion) * 0.5);
        self.rights = RightsProfile {
            learning_weight: learning,
            responsibility_weight: responsibility,
            media_influence,
        };
    }
}

/// One‑dimensional lattice of agents.
#[derive(Debug, Clone)]
pub struct Lattice {
    pub agents: Vec<Agent>,
}

impl Lattice {
    pub fn new_1d(agent_count: usize, seed: u64) -> Self {
        let mut agents = Vec::with_capacity(agent_count);
        for i in 0..agent_count {
            let bio = BioState::new_random(seed, i);
            let id = format!("agent_{}", i);
            agents.push(Agent::new(id, LatticeIndex(i), bio));
        }
        Self { agents }
    }

    pub fn len(&self) -> usize {
        self.agents.len()
    }
}

/// Tree‑of‑Life 1D update rule: purely mathematical, non‑actuating.
fn apply_tree_of_life_rule(nb: &Neighborhood1D) -> BioState {
    let self_energy = nb.center.energy.value();
    let mut neighbor_energy_sum = 0.0;
    let mut neighbor_count = 0.0;

    if let Some(l) = &nb.left {
        neighbor_energy_sum += l.energy.value();
        neighbor_count += 1.0;
    }
    if let Some(r) = &nb.right {
        neighbor_energy_sum += r.energy.value();
        neighbor_count += 1.0;
    }

    let neighbor_mean = if neighbor_count > 0.0 {
        neighbor_energy_sum / neighbor_count
    } else {
        self_energy
    };

    let diffusion_strength = 0.25;
    let new_energy = BioScalar::new_clamped(
        self_energy + diffusion_strength * (neighbor_mean - self_energy),
    );

    let new_cohesion = nb.center
        .cohesion
        .lerp(BioScalar::new_clamped(neighbor_mean), 0.2);

    let new_adaptation = nb.center
        .adaptation
        .mul_clamped(0.98)
        .add_clamped(0.02 * neighbor_mean);

    let outreach_boost = if neighbor_mean > 0.6 { 0.05 } else { -0.02 };
    let new_outreach = nb.center.outreach.add_clamped(outreach_boost);

    BioState {
        energy: new_energy,
        cohesion: new_cohesion,
        adaptation: new_adaptation,
        outreach: new_outreach,
    }
}

/// Micro‑society simulation core.
pub struct MicroSocietySim {
    pub cfg: VirtaSysConfig,
    pub lattice: Lattice,
    pub learning_catalog: Vec<LearningObject>,
    pub ledger: Ledger,
    pub tick: Tick,
}

impl MicroSocietySim {
    pub fn new(agent_count: usize, seed: u64) -> Self {
        let cfg = VirtaSysConfig::default();
        let lattice = Lattice::new_1d(agent_count, seed);
        let learning_catalog = Self::default_learning_objects();
        Self {
            cfg,
            lattice,
            learning_catalog,
            ledger: Ledger::default(),
            tick: Tick(0),
        }
    }

    fn default_learning_objects() -> Vec<LearningObject> {
        vec![
            LearningObject {
                id: 1,
                title: "Media Literacy: Signal vs Noise".to_string(),
                kind: LearningObjectKind::MediaLiteracy,
                difficulty: BioScalar::new_clamped(0.4),
                impact_on_energy: BioScalar::new_clamped(0.55),
                impact_on_cohesion: BioScalar::new_clamped(0.7),
                impact_on_outreach: BioScalar::new_clamped(0.65),
            },
            LearningObject {
                id: 2,
                title: "Energy Stewardship 101".to_string(),
                kind: LearningObjectKind::EnergyStewardship,
                difficulty: BioScalar::new_clamped(0.5),
                impact_on_energy: BioScalar::new_clamped(0.8),
                impact_on_cohesion: BioScalar::new_clamped(0.6),
                impact_on_outreach: BioScalar::new_clamped(0.4),
            },
            LearningObject {
                id: 3,
                title: "Cooperation Game: Shared Resources".to_string(),
                kind: LearningObjectKind::CooperationGame,
                difficulty: BioScalar::new_clamped(0.6),
                impact_on_energy: BioScalar::new_clamped(0.6),
                impact_on_cohesion: BioScalar::new_clamped(0.85),
                impact_on_outreach: BioScalar::new_clamped(0.7),
            },
            LearningObject {
                id: 4,
                title: "Outreach Campaign: Earth‑Saving Stories".to_string(),
                kind: LearningObjectKind::OutreachCampaign,
                difficulty: BioScalar::new_clamped(0.7),
                impact_on_energy: BioScalar::new_clamped(0.65),
                impact_on_cohesion: BioScalar::new_clamped(0.75),
                impact_on_outreach: BioScalar::new_clamped(0.9),
            },
            LearningObject {
                id: 5,
                title: "Thermo‑Regulation Study".to_string(),
                kind: LearningObjectKind::ThermoRegulationStudy,
                difficulty: BioScalar::new_clamped(0.5),
                impact_on_energy: BioScalar::new_clamped(0.7),
                impact_on_cohesion: BioScalar::new_clamped(0.55),
                impact_on_outreach: BioScalar::new_clamped(0.5),
            },
        ]
    }

    /// Single simulation step: Tree‑of‑Life rule + learning objects + rights recompute.
    pub fn step(&mut self) {
        let current_tick = self.tick;
        let next_tick = Tick(current_tick.0 + 1);
        self.ledger
            .push(DeedKind::CycleAdvance { from: current_tick, to: next_tick });

        let mut new_states: Vec<BioState> = Vec::with_capacity(self.lattice.len());

        // 1D rule for each agent.
        for idx in 0..self.lattice.len() {
            let agent = &self.lattice.agents[idx];
            let nb = agent.observe_neighbors(&self.lattice);
            let updated_bio = apply_tree_of_life_rule(&nb);

            let delta = updated_bio.energy.value() - agent.bio.energy.value();
            if delta.abs() > 0.0001 {
                self.ledger.push(DeedKind::EnergyDiffusion {
                    idx,
                    delta,
                });
            }

            if (updated_bio.cohesion.value() - agent.bio.cohesion.value()).abs() > 0.0001 {
                self.ledger.push(DeedKind::CohesionAdjustment {
                    idx,
                    new_value: updated_bio.cohesion.value(),
                });
            }

            new_states.push(updated_bio);
        }

        // Apply new bio states.
        for (idx, state) in new_states.into_iter().enumerate() {
            self.lattice.agents[idx].bio = state;
        }

        // Learning objects and rights recompute.
        for idx in 0..self.lattice.len() {
            let agent = &mut self.lattice.agents[idx];
            if let Some(obj) = self.select_learning_object_for(agent) {
                obj.apply(&mut agent.bio);
                agent.last_learning_ids.push_back(obj.id);
                if agent.last_learning_ids.len() > 8 {
                    agent.last_learning_ids.pop_front();
                }
            }

            let nb = agent.observe_neighbors(&self.lattice);
            agent.update_rights_from_neighborhood(&nb);
            self.ledger.push(DeedKind::RightsRecomputed { idx });

            if agent.bio.outreach.value() > 0.8 {
                self.ledger.push(DeedKind::OutreachBroadcast {
                    idx,
                    strength: agent.bio.outreach.value(),
                });
            }
        }

        self.tick = next_tick;
    }

    fn select_learning_object_for(&self, agent: &Agent) -> Option<LearningObject> {
        if self.learning_catalog.is_empty() {
            return None;
        }
        let energy = agent.bio.energy.value();
        let idx = if energy < 0.33 {
            1 // Energy Stewardship
        } else if energy < 0.66 {
            3 // Outreach Campaign
        } else {
            2 // Cooperation Game / Thermo etc.
        };
        let clamped_idx = idx.min(self.learning_catalog.len() - 1);
        Some(self.learning_catalog[clamped_idx].clone())
    }

    /// Compute a simple entropy‑like dispersion metric across the lattice.
    pub fn energy_dispersion(&self) -> f32 {
        if self.lattice.len() < 2 {
            return 0.0;
        }
        let mean: f32 = self
            .lattice
            .agents
            .iter()
            .map(|a| a.bio.energy.value())
            .sum::<f32>()
            / (self.lattice.len() as f32);

        let var: f32 = self
            .lattice
            .agents
            .iter()
            .map(|a| {
                let d = a.bio.energy.value() - mean;
                d * d
            })
            .sum::<f32>()
            / (self.lattice.len() as f32);

        var.sqrt()
    }

    /// Export a simple textual snapshot for diagnostics.
    pub fn snapshot(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "Tick {} | Agents: {} | Ledger: {} events | Tail hash: {:016x}\n",
            self.tick.0,
            self.lattice.len(),
            self.ledger.len(),
            self.ledger.tail_hash()
        ));
        for (i, a) in self.lattice.agents.iter().enumerate() {
            out.push_str(&format!(
                "#{:03} {} @{} | E:{:.3} C:{:.3} A:{:.3} O:{:.3} | L:{:.3} R:{:.3} M:{:.3}\n",
                i,
                a.id,
                a.index.0,
                a.bio.energy.value(),
                a.bio.cohesion.value(),
                a.bio.adaptation.value(),
                a.bio.outreach.value(),
                a.rights.learning_weight.value(),
                a.rights.responsibility_weight.value(),
                a.rights.media_influence.value()
            ));
        }
        out
    }
}

impl fmt::Debug for MicroSocietySim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MicroSocietySim")
            .field("cfg", &self.cfg)
            .field("tick", &self.tick)
            .field("agents", &self.lattice.len())
            .field("ledger_events", &self.ledger.len())
            .finish()
    }
}

/// Simple CLI‑style runner for demonstration purposes.
/// In a full Cargo project, this would be wired to real CLI argument parsing (e.g. `clap`).
fn main() {
    // Deterministic seed for reproducible micro‑society runs.
    let seed: u64 = 0x_6E75_726F_2E31_3131;
    let mut sim = MicroSocietySim::new(32, seed);

    // Run a fixed number of steps for demonstration.
    let total_steps = 64;
    for _ in 0..total_steps {
        sim.step();
    }

    println!("=== Micro‑Society 1D Tree‑of‑Life Snapshot ===");
    println!("{}", sim.snapshot());
    println!("Energy dispersion: {:.6}", sim.energy_dispersion());
}
