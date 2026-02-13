use crate::deeds::{Deed, DeedKind};
use crate::invariants::enforce_invariants;
use crate::model::{Index, SiteState, World};

pub struct StepResult {
    pub deeds: Vec<Deed>,
}

pub fn step_world(world: &mut World) -> StepResult {
    let params = world.params;
    let mut proposed = world.sites.clone();
    let mut deeds = Vec::new();

    for (i, site) in world.sites.iter().enumerate() {
        if !site.occupied {
            proposed[i] = *site;
            continue;
        }
        let mut next = *site;

        next.church *= (1.0 - params.decay_church).max(0.0);
        next.fear *= (1.0 - params.decay_fear).max(0.0);
        next.power *= (1.0 - params.decay_power).max(0.0);
        next.tech *= (1.0 - params.decay_tech).max(0.0);

        let trust_avg = 0.5 * (site.trust_left + site.trust_right);
        next.fear += params.fear_from_load * site.bioload + params.fear_from_trust * trust_avg;

        if next.church >= params.church_min_for_power
            && next.fear >= params.fear_min_for_power
            && next.fear <= params.fear_max_for_power
        {
            next.power += params.power_gain * next.church;
        }

        let attenuation = if site.bioload >= site.bioload_max {
            0.0
        } else {
            (1.0 - site.bioload / site.bioload_max).min(1.0).max(0.0)
        };
        next.tech += params.tech_gain * next.power * attenuation;

        let bio_increase = params.biocost_power * next.power + params.biocost_tech * next.tech;
        let bio_recovery = params.recovery_rate * site.bioload;
        next.bioload = (site.bioload + bio_increase - bio_recovery).max(0.0);

        proposed[i] = next;
    }

    let mut colonize_targets: Vec<(Index, Index)> = Vec::new();

    for i in 0..proposed.len() {
        let site = proposed[i];
        if !site.occupied {
            continue;
        }

        if can_colonize(&site, &world) {
            let neighbors = neighbor_indices(i, proposed.len());
            for j in neighbors {
                if !proposed[j].occupied {
                    colonize_targets.push((i, j));
                    break;
                }
            }
        }
    }

    for (origin, target) in colonize_targets {
        let before_origin = proposed[origin];
        let before_target = proposed[target];
        let mut o = before_origin;
        let mut t = before_target;

        o.church *= (1.0 - world.params.colonize_church_cost_frac).max(0.0);
        o.power *= (1.0 - world.params.colonize_power_cost_frac).max(0.0);
        o.bioload += 0.5 * o.bioload_max;

        let neighbor_idxs = neighbor_indices(target, proposed.len());
        let mut sum_church = 0.0;
        let mut sum_fear = 0.0;
        let mut sum_power = 0.0;
        let mut sum_tech = 0.0;
        let mut count = 0.0;

        for idx in neighbor_idxs {
            let n = proposed[idx];
            if n.occupied {
                sum_church += n.church;
                sum_fear += n.fear;
                sum_power += n.power;
                sum_tech += n.tech;
                count += 1.0;
            }
        }

        let frac = 0.1;
        if count > 0.0 {
            t.church = frac * sum_church / count;
            t.fear = frac * sum_fear / count;
            t.power = frac * sum_power / count;
            t.tech = frac * sum_tech / count;
        } else {
            t.church = 0.0;
            t.fear = 0.0;
            t.power = 0.0;
            t.tech = 0.0;
        }
        t.occupied = true;
        t.bioload = 0.5 * t.bioload_max;

        proposed[origin] = o;
        proposed[target] = t;

        deeds.push(Deed::colonize(
            world.tick,
            origin,
            target,
            before_origin,
            before_target,
            o,
            t,
        ));
    }

    world.sites = proposed;
    let _status = enforce_invariants(world);
    world.tick += 1;

    StepResult { deeds }
}

fn can_colonize(site: &SiteState, world: &World) -> bool {
    let p = world.params;
    if !site.occupied {
        return false;
    }
    if site.church < p.colonize_church_min {
        return false;
    }
    if site.fear < p.colonize_fear_min || site.fear > p.colonize_fear_max {
        return false;
    }
    if site.power < p.colonize_power_min {
        return false;
    }
    if site.tech < p.colonize_tech_min {
        return false;
    }
    if site.bioload >= site.bioload_max {
        return false;
    }
    true
}

fn neighbor_indices(i: Index, len: usize) -> Vec<Index> {
    let mut v = Vec::with_capacity(2);
    if i > 0 {
        v.push(i - 1);
    }
    if i + 1 < len {
        v.push(i + 1);
    }
    v
}
