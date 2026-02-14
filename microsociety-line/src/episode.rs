use crate::model::{Tick, SiteIndex, World, TokenState};
use serde::{Deserialize, Serialize};

/// Core deed types on the Jetson-Line Tree-of-Life axis.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeedKind {
    LocalHelp,
    LocalConflict,
    Repair,
    Colonize,
    EmitPollution,
    DeployCleanTech,
    Abstain,
}

/// Minimal biophysical + social snapshot for a site at a deed boundary.
/// This intentionally mirrors your CHURCH/FEAR/POWER/TECH + load + trust view.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SiteSnapshot {
    pub index: SiteIndex,
    pub tokens: TokenState,
    pub bioload: f64,
    pub left_trust: f64,
    pub right_trust: f64,
}

impl SiteSnapshot {
    pub fn from_world(world: &World, idx: SiteIndex) -> Option<Self> {
        let site = world.sites.get(idx)?;
        Some(Self {
            index: idx,
            tokens: site.tokens,
            bioload: site.bio.load,
            left_trust: site.trust.lefttrust,
            right_trust: site.trust.righttrust,
        })
    }
}

/// A single logged deed crossing a moral / safety threshold.
/// This is the atomic unit for later judgement and causal chains.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deed {
    pub tick: Tick,
    pub kind: DeedKind,
    /// Sites directly involved in the deed (1 or 2 in the current 1-D model).
    pub actors: Vec<SiteIndex>,
    /// Pre-action snapshots for all involved sites.
    pub pre: Vec<SiteSnapshot>,
    /// Post-action snapshots for all involved sites.
    pub post: Vec<SiteSnapshot>,
    /// Token and biophysical deltas, parallel to `actors`.
    pub delta_church: Vec<f64>,
    pub delta_fear: Vec<f64>,
    pub delta_power: Vec<f64>,
    pub delta_tech: Vec<f64>,
    pub delta_bioload: Vec<f64>,
    pub delta_trust_left: Vec<f64>,
    pub delta_trust_right: Vec<f64>,
}

/// Rule ID or label for the judgement rule that fired.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum JudgmentRuleId {
    BiophysicalOverload,
    FearBandViolation,
    UnjustConflict,
    ProportionalDefense,
    RestorativeRepair,
    StewardshipColonization,
    PollutionExport,
    CleanTechStewardship,
    AbstainUnderRisk,
    Custom(u16),
}

/// Responsibility level in neurolaw-style tiers.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ResponsibilityLevel {
    Low,
    Medium,
    High,
}

/// High-level justification / norm label.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum JustificationType {
    Defensive,
    Offensive,
    Proportional,
    Disproportional,
    Repair,
    Abstain,
    Unknown,
}

/// Per-deed judgement record, computed by the ethical rules module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Judgment {
    pub deed_index: usize,
    pub rule_id: JudgmentRuleId,
    pub responsibility: ResponsibilityLevel,
    pub justification: JustificationType,
    /// Scalar moral score, e.g. [-1.0, 1.0] where >0 is net prosocial.
    pub moral_score: f64,
    /// Short human-readable rationale referencing thresholds and alternatives.
    pub rationale: String,
}

/// Aggregated justice metrics over an Episode window.
/// These mirror HPCC / ERG / TECR in your existing stack.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct EpisodeMetrics {
    pub hpcc: f64,      // habit–pollution coupling
    pub erg: f64,       // exposure–responsibility gap
    pub tecr: f64,      // token-enforced collapse rate
    pub collapse_count: u32,
    pub total_ticks: Tick,
}

/// Episode-level reflection summaries (What / So what / Now what).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EpisodeReflection {
    pub what_happened: String,
    pub so_what_it_meant: String,
    pub now_what_to_change: String,
}

/// Episode is the canonical knowledge-object for a single Jetson-Line run.
/// It binds World state, deeds, judgements, metrics, and reflections.
#[derive(Debug, Serialize, Deserialize)]
pub struct Episode {
    /// Human-readable label (policy family, scenario name, etc.).
    pub label: String,
    /// Full Jetson-Line world state and StepLog history.
    pub world: World,
    /// All morally relevant deeds logged during the run.
    pub deeds: Vec<Deed>,
    /// One judgement per logged deed (same ordering).
    pub judgments: Vec<Judgment>,
    /// Justice metrics over the Episode.
    pub metrics: EpisodeMetrics,
    /// Optional human / agent reflections.
    pub reflection: EpisodeReflection,
}

impl Episode {
    pub fn new<L: Into<String>>(label: L, world: World) -> Self {
        Self {
            label: label.into(),
            world,
            deeds: Vec::new(),
            judgments: Vec::new(),
            metrics: EpisodeMetrics::default(),
            reflection: EpisodeReflection::default(),
        }
    }

    /// Core stepping loop; you will wire this to your existing `step_world`.
    /// Deed logging hooks will be inserted around high-impact actions.
    pub fn run_for_ticks<F>(&mut self, ticks: Tick, mut step_fn: F)
    where
        F: FnMut(&mut World, Tick, &mut Vec<Deed>),
    {
        for t in 0..ticks {
            step_fn(&mut self.world, t, &mut self.deeds);
        }
        self.metrics.total_ticks = ticks;
    }

    /// Attach a judgement for a given deed index.
    pub fn push_judgment(&mut self, judgment: Judgment) {
        self.judgments.push(judgment);
    }

    /// Recompute Episode-level justice metrics from judgments and world history.
    /// You will implement the exact HPCC / ERG / TECR math in a separate module.
    pub fn recompute_metrics(&mut self) {
        // Placeholder wiring: real implementation should use your existing
        // HPCC / ERG / TECR definitions over deeds + world.history.
        self.metrics.collapse_count = 0;
        self.metrics.hpcc = 0.0;
        self.metrics.erg = 0.0;
        self.metrics.tecr = 0.0;
    }

    /// Serialize Episode to JSON for knowledge-object export.
    pub fn save_json(&self, path: &str) -> std::io::Result<()> {
        let data = serde_json::to_vec_pretty(self)
            .expect("serialize episode");
        std::fs::write(path, data)
    }
}
