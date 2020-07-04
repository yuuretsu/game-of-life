use std::time::Instant;
use random_integer::random_u8 as rand_u8;

fn normalize(p: Vec2i, s: Vec2i) -> Vec2i {
    let x = if p.x >= 0 {p.x % s.x} else {s.x - ((-p.x) % s.x)};
    let y = if p.y >= 0 {p.y % s.y} else {s.y - ((-p.y) % s.y)};
    Vec2i::new(x, y)
}

fn get_sum(vec: Vec2i, world: &Vec<Vec<bool>>) -> u8 {
    let mut sum: u8 = 0;
    for i in 0..MOORE_NEIGHBORHOOD.len() {
        let point = normalize(vec + &MOORE_NEIGHBORHOOD[i], Vec2i::new(W as i32, H as i32));
        sum += world[point.x as usize][point.y as usize] as u8;
    }
    sum
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

const W: usize = 500;
const H: usize = 500;

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

fn step(world: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut world_next: Vec<Vec<bool>> = Vec::new();
    for x in 0..W {
        world_next.push(Vec::new());
        for y in 0..H {
            let here = world[x][y];
            let sum = get_sum(Vec2i::new(x as i32, y as i32), &world);
            let next: bool;
            match here {
                false => next = if sum == 3 {true} else {false},
                true => next = if sum == 2 || sum == 3 {true} else {false},
            }
            world_next[x].push(next);
        }
    }
    world_next
}

fn main() {
    let mut world: Vec<Vec<bool>> = Vec::new();
    for x in 0..W {
        world.push(Vec::new());
        for y in 0..H {
            world[x].push(if rand_u8(0, 1) == 0 {false} else {true});
        }
    }

    let start = Instant::now();
    for _ in 0..100 {
        world = step(world);
    }
    println!("{:?}", Instant::now() - start);
}