use std::collections::HashMap;
use random_integer::random_i32 as rand_int;
use macroquad::*;

fn normalize(p: Vec2i, s: Vec2i) -> Vec2i {
    let x = if p.x >= 0 {p.x % s.x} else {s.x - ((-p.x) % s.x)};
    let y = if p.y >= 0 {p.y % s.y} else {s.y - ((-p.y) % s.y)};
    Vec2i::new(x, y)
}

fn get_sum(vec: Vec2i, world: &HashMap<Vec2i, i32>) -> i32 {
    let mut sum = 0;
    for i in 0..MOORE_NEIGHBORHOOD.len() {
        sum += world.get(
            &normalize(vec + &MOORE_NEIGHBORHOOD[i], Vec2i::new(W, H))).unwrap_or(&0);
    }
    sum
}

fn draw_cell(x: i32, y: i32, color: Color) {
    draw_rectangle(
        (x * PX) as f32,
        (y * PX) as f32,
        PX as f32,
        PX as f32,
        color,
    );
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Vec2i {
    x: i32,
    y: i32,
}

impl Vec2i {
    fn new(x: i32, y: i32) -> Vec2i {
        Vec2i { x, y }
    }
}

impl std::ops::Add<&Vec2i> for Vec2i {
	type Output = Vec2i;

	#[inline]
	fn add(self, _rhs: &Vec2i) -> Vec2i {
		Vec2i { 
			x: self.x + _rhs.x, 
			y: self.y + _rhs.y
		}
	}
}

const W: i32 = 250;
const H: i32 = 250;
const PX: i32 = 2;

const MOORE_NEIGHBORHOOD: [Vec2i; 8] = [
    Vec2i { x: -1, y:  1 },
    Vec2i { x:  0, y:  1 },
    Vec2i { x:  1, y:  1 },
    Vec2i { x:  1, y:  0 },
    Vec2i { x:  1, y: -1 },
    Vec2i { x:  0, y: -1 },
    Vec2i { x: -1, y: -1 },
    Vec2i { x: -1, y:  0 },
];

#[macroquad::main("Game of Life")]
async fn main() {

    let mut world: HashMap<Vec2i, i32> = HashMap::new();
    for x in 0..W {
        for y in 0..H {
           world.insert(Vec2i::new(x, y), rand_int(0, 1));
        }
    }

    loop {
        clear_background(BLACK);
        let mut world_next: HashMap<Vec2i, i32> = HashMap::new();
        for x in 0..W {
            for y in 0..H {
                let here = world.get(&Vec2i::new(x, y)).unwrap_or(&0);
                let sum = get_sum(normalize(Vec2i::new(x, y), Vec2i::new(W, H)), &world);
                let next;
                match here {
                    0 => next = if sum == 3 {1} else {0},
                    1 => next = if sum == 2 || sum == 3 {1} else {0},
                    _ => {next = 0},
                }
                world_next.insert(Vec2i::new(x, y), next);
                if next == 1 {draw_cell(x, y, WHITE);}
            }
        }
        world = world_next;
        draw_text(&get_fps().to_string(), 5., 5., 40., RED);
        next_frame().await;
    }
}