use jetsonline_treeoflife::{
    World, WorldParams, GlobalConstraints, Episode, EpisodeConfig, step_world,
};

fn main() {
    let mut params = WorldParams::default();
    params.length = 64;

    let constraints = GlobalConstraints::default();
    let mut world = World::new(params, constraints, 10.0);

    let origin = params.length / 2;
    world.occupy_origin(origin, 10.0);

    let mut episode = Episode::new(
        EpisodeConfig { max_ticks: 100 },
        world,
    );

    episode.run(|w| {
        let result = step_world(w);
        result.deeds
    });

    let json = episode.to_json().expect("serialize episode");
    println!("{json}");
}
