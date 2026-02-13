use serde::{Deserialize, Serialize};

use crate::model::{Index, SiteState, Tick};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DeedKind {
    Help,
    Conflict,
    Colonize,
    Repair,
    DeployCleanTech,
    UseSupport,
    EmitPollution,
    UseHabit,
    BanEmission,
    RepairEnvironment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deed {
    pub tick: Tick,
    pub kind: DeedKind,
    pub origin: Index,
    pub target: Option<Index>,
    pub pre_origin: SiteState,
    pub pre_target: Option<SiteState>,
    pub post_origin: SiteState,
    pub post_target: Option<SiteState>,
}

impl Deed {
    pub fn colonize(
        tick: Tick,
        origin: Index,
        target: Index,
        pre_origin: SiteState,
        pre_target: SiteState,
        post_origin: SiteState,
        post_target: SiteState,
    ) -> Self {
        Self {
            tick,
            kind: DeedKind::Colonize,
            origin,
            target: Some(target),
            pre_origin,
            pre_target: Some(pre_target),
            post_origin,
            post_target: Some(post_target),
        }
    }
}
