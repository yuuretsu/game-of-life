use game_of_life_2::*;
use std::time::Instant;

fn main() {
    let mut worlds = init_worlds();
    let mut world = worlds.0;
    let mut world_next = worlds.1;

    let start = Instant::now();
    for _ in 0..100 {
        worlds = step(world, world_next);
        world = worlds.0;
        world_next = worlds.1;
    }
    println!("{:?}", Instant::now() - start);
}
