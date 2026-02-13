use jetsonline_treeoflife::{
    World, WorldParams, GlobalConstraints, Episode, EpisodeConfig, step_world,
};

fn main() {
    let mut params = WorldParams::default();
    params.length = 64;

    // Encourage fair, slower colonization with low load
    params.colonize_church_min = 8.0;
    params.colonize_power_min = 4.0;
    params.colonize_tech_min = 1.5;
    params.colonize_church_cost_frac = 0.4;
    params.colonize_power_cost_frac = 0.5;

    params.biocost_power = 0.005;
    params.biocost_tech = 0.003;
    params.recovery_rate = 0.08;

    let mut constraints = GlobalConstraints::default();
    constraints.neuromorph_power_multiplier = 1.5;
    constraints.total_bioload_max = 80.0;

    let mut world = World::new(params, constraints, 8.0);
    let origin = params.length / 2;
    world.occupy_origin(origin, 15.0);

    let mut episode = Episode::new(
        EpisodeConfig { max_ticks: 200 },
        world,
    );

    episode.run(|w| {
        let result = step_world(w);
        result.deeds
    });

    let json = episode.to_json().expect("serialize episode");
    println!("{json}");
}
