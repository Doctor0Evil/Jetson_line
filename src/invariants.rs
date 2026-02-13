use serde::{Deserialize, Serialize};

use crate::model::{World, SiteState};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum NeuromorphGodStatus {
    Ok,
    PowerRescaled,
    BioloadClamped,
    GlobalBioloadExceeded,
}

pub fn enforce_invariants(world: &mut World) -> NeuromorphGodStatus {
    let mut status = NeuromorphGodStatus::Ok;

    let mut total_church = 0.0;
    let mut total_power = 0.0;
    let mut total_bioload = 0.0;

    for s in &world.sites {
        if s.occupied {
            total_church += s.church;
            total_power += s.power;
            total_bioload += s.bioload;
        }
    }

    let allowed_power = world.constraints.neuromorph_power_multiplier * total_church;

    if total_power > allowed_power && allowed_power > 0.0 {
        let scale = allowed_power / total_power;
        for s in &mut world.sites {
            if s.occupied {
                let mut ss = *s;
                ss.power *= scale;
                *s = clamp_bioload(ss, &mut status);
            }
        }
        status = NeuromorphGodStatus::PowerRescaled;
    } else {
        for s in &mut world.sites {
            let ss = clamp_bioload(*s, &mut status);
            *s = ss;
        }
    }

    let mut total_bioload_after = 0.0;
    for s in &world.sites {
        if s.occupied {
            total_bioload_after += s.bioload;
        }
    }

    if total_bioload_after > world.constraints.total_bioload_max {
        status = NeuromorphGodStatus::GlobalBioloadExceeded;
    }

    status
}

fn clamp_bioload(mut s: SiteState, status: &mut NeuromorphGodStatus) -> SiteState {
    if s.bioload > s.bioload_max {
        s.bioload = s.bioload_max;
        *status = NeuromorphGodStatus::BioloadClamped;
    }
    if s.bioload < 0.0 {
        s.bioload = 0.0;
    }
    s
}
