use crate::episode::{Deed, DeedKind, SiteSnapshot};
use crate::model::{SiteIndex, World, Tick};

/// Capture a deed with pre/post snapshots around an action affecting one site.
pub fn log_unary_deed(
    world_before: &World,
    world_after: &World,
    tick: Tick,
    kind: DeedKind,
    i: SiteIndex,
    out: &mut Vec<Deed>,
) {
    if let (Some(pre), Some(post)) = (
        SiteSnapshot::from_world(world_before, i),
        SiteSnapshot::from_world(world_after, i),
    ) {
        let delta_church = vec![post.tokens.church - pre.tokens.church];
        let delta_fear = vec![post.tokens.fear - pre.tokens.fear];
        let delta_power = vec![post.tokens.power - pre.tokens.power];
        let delta_tech = vec![post.tokens.tech - pre.tokens.tech];
        let delta_bioload = vec![post.bioload - pre.bioload];
        let delta_trust_left = vec![post.left_trust - pre.left_trust];
        let delta_trust_right = vec![post.right_trust - pre.right_trust];

        out.push(Deed {
            tick,
            kind,
            actors: vec![i],
            pre: vec![pre],
            post: vec![post],
            delta_church,
            delta_fear,
            delta_power,
            delta_tech,
            delta_bioload,
            delta_trust_left,
            delta_trust_right,
        });
    }
}

/// Capture a deed with pre/post snapshots for a pairwise interaction (i, j).
pub fn log_binary_deed(
    world_before: &World,
    world_after: &World,
    tick: Tick,
    kind: DeedKind,
    i: SiteIndex,
    j: SiteIndex,
    out: &mut Vec<Deed>,
) {
    if let (Some(pre_i), Some(pre_j), Some(post_i), Some(post_j)) = (
        SiteSnapshot::from_world(world_before, i),
        SiteSnapshot::from_world(world_before, j),
        SiteSnapshot::from_world(world_after, i),
        SiteSnapshot::from_world(world_after, j),
    ) {
        let pre = vec![pre_i, pre_j];
        let post = vec![post_i, post_j];

        let delta_church = vec![
            post_i.tokens.church - pre_i.tokens.church,
            post_j.tokens.church - pre_j.tokens.church,
        ];
        let delta_fear = vec![
            post_i.tokens.fear - pre_i.tokens.fear,
            post_j.tokens.fear - pre_j.tokens.fear,
        ];
        let delta_power = vec![
            post_i.tokens.power - pre_i.tokens.power,
            post_j.tokens.power - pre_j.tokens.power,
        ];
        let delta_tech = vec![
            post_i.tokens.tech - pre_i.tokens.tech,
            post_j.tokens.tech - pre_j.tokens.tech,
        ];
        let delta_bioload = vec![
            post_i.bioload - pre_i.bioload,
            post_j.bioload - pre_j.bioload,
        ];
        let delta_trust_left = vec![
            post_i.left_trust - pre_i.left_trust,
            post_j.left_trust - pre_j.left_trust,
        ];
        let delta_trust_right = vec![
            post_i.right_trust - pre_i.right_trust,
            post_j.right_trust - pre_j.right_trust,
        ];

        out.push(Deed {
            tick,
            kind,
            actors: vec![i, j],
            pre,
            post,
            delta_church,
            delta_fear,
            delta_power,
            delta_tech,
            delta_bioload,
            delta_trust_left,
            delta_trust_right,
        });
    }
}
