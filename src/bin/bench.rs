use game_of_life_2::*;
use std::time::Instant;

fn main() {
    let mut world = init_world();
    let mut world_next = vec![vec![false; H]; W];

    let start = Instant::now();
    for _ in 0..100 {
        let worlds = step(world, world_next);
        world_next = worlds.0;
        world = worlds.1;
    }
    println!("{:?}", Instant::now() - start);
}
