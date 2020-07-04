use macroquad::*;
use game_of_life_2::*;

const PX: u8 = 1;

fn draw_cell(x: usize, y: usize, color: Color) {
    draw_rectangle(
        (x * PX as usize) as f32,
        (y * PX as usize) as f32,
        PX as f32,
        PX as f32,
        color,
    );
}

#[macroquad::main("Game of Life")]
async fn main() {
    let mut world = init_world();
    let mut world_next = vec![vec![false; H]; W];

    loop {
        clear_background(BLACK);
        let worlds = step(world, world_next);
        world_next = worlds.0;
        world = worlds.1;
        for x in 0..W {
            for y in 0..H {
                let here = world[x][y];
                if here == true { draw_cell(x, y, WHITE); }
            }
        }
        draw_text(&get_fps().to_string(), 5., 5., 40., RED);
        next_frame().await;
    }
}
