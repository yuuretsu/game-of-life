use game_of_life_2::*;
use std::time::Instant;

fn main() {
    let mut world = init_world();

    let start = Instant::now();
    for _ in 0..100 {
        world = step(world);
    }
    println!("{:?}", Instant::now() - start);
}
