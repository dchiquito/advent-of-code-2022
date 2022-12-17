use std::collections::VecDeque;

use crate::advent;


#[derive(Debug)]
enum Wind { Left, Right }


fn get_wind() -> Vec<Wind> {
    let lines = advent::read_input(17);
    let line = lines.get(0).unwrap();
    line.chars().map(|c| match c {
        '<' => Wind::Left,
        '>' => Wind::Right,
        _ => panic!(),
    }).collect()
}

#[derive(Debug, Clone)]
struct Shape {
    tiles: Vec<(i64, i64)>,
}

impl Shape {
    const fn new(tiles: Vec<(i64, i64)>) -> Shape {
        Shape { tiles }
    }
}

fn the_shapes() -> Vec<Shape> {
    vec![
        Shape::new(vec![(0,0), (1,0), (2,0), (3,0)]),
        Shape::new(vec![(0,1), (1,0), (2,1), (1,2)]), // TODO (1,1) is not included
        Shape::new(vec![(0,0), (1,0), (2,0), (2,1), (2,2)]),
        Shape::new(vec![(0,0), (0,1), (0,2), (0,3)]),
        Shape::new(vec![(0,0), (1,0), (0,1), (1,1)]),
    ]
}

#[derive(Debug)]
struct Rock {
    shape: Shape,
    position: (i64, i64)
}

impl Rock {
    fn new(shape: &Shape, cave: &Cave) -> Rock {
        Rock {
            shape: shape.clone(),
            position: (2, cave.max_y + 4),
        }
    }
    fn blow(&mut self, wind: &Wind, cave: &Cave) {
        match wind {
            Wind::Left => {
                for (dx,_dy) in self.shape.tiles.iter() {
                    if self.position.0 + dx <= 0 {
                        return;
                    }
                }
                self.position.0 -= 1;
                // abort the blow if we are currently colliding
                if cave.collides(self) {
                    self.position.0 += 1;
                }
            },
            Wind::Right => {
                for (dx,_dy) in self.shape.tiles.iter() {
                    if self.position.0 + dx >= 6 {
                        return;
                    }
                }
                self.position.0 += 1;
                // abort the blow if we are currently colliding
                if cave.collides(self) {
                    self.position.0 -= 1;
                }
            },
        }
    }
    fn lower(&mut self, cave: &Cave) -> bool {
        if self.position.1 == 0 {
            true
        } else {
            self.position.1 -= 1;
            if cave.collides(self) {
                self.position.1 += 1;
                true
            } else {
                false
            }
        }
    }
}

#[derive(Debug)]
struct Cave {
    rocks: VecDeque<Rock>,
    max_y: i64,
}

impl Cave {
    fn new() -> Cave {
        Cave { rocks: VecDeque::new(), max_y: -1 }
    }
    fn collides(&self, rock: &Rock) -> bool {
        for settled_rock in self.rocks.iter() {
            if (settled_rock.position.1 - rock.position.1).abs() > 3 {
                // physically can't collide, no need to check cells
                continue;
            }
            let (sx, sy) = settled_rock.position;
            let (x, y) = rock.position;
            for (sdx, sdy) in settled_rock.shape.tiles.iter() {
                for (dx, dy) in rock.shape.tiles.iter() {
                    if sx+sdx == x+dx && sy+sdy == y+dy {
                        return true;
                    }
                }
            }
                
        }
        false
    }
    fn settle(&mut self, rock: Rock) {
        for (_dx, dy) in rock.shape.tiles.iter() {
            self.max_y = self.max_y.max(rock.position.1 + dy);
        }
        self.rocks.push_front(rock);
        if self.rocks.len() > 50 {
            self.rocks.pop_back();
        }
    }
    fn _print(&self) {
        for y in (self.max_y-10..self.max_y+2).rev() {
            for x in 0..7 {
                let mut empty = true;
                for rock in self.rocks.iter() {
                    let (rx, ry) = rock.position;
                    for (dx, dy) in rock.shape.tiles.iter() {
                        if (x,y) == (rx+dx, ry+dy) {
                            empty = false;
                            break;
                        }
                    }
                }
                if empty {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            println!();
            if y == 9 {
                println!("XXXXXXX");
            }
        }
    }
    fn hash_state(&self) -> i64 {
        let mut s = 0;
        let mut i = 1;
        for rock in self.rocks.iter().take(8) { // TODO this might need increasing
            let (x, y) = rock.position;
            let y = self.max_y - y;
            s += i * (y*7 + x);
            i *= 103;
        }
        s
    }
}

fn solve_1(limit: i64) -> i64 {
    let winds = get_wind();
    let mut winds = winds.iter().cycle();
    let shapes = the_shapes();
    let mut shapes = shapes.iter().cycle();
    let mut cave = Cave::new();
    for _ in 0..limit {
        let shape = shapes.next().unwrap();
        let mut rock = Rock::new(shape, &cave);
        loop {
            let wind = winds.next().unwrap();
            rock.blow(wind, &cave);
            if rock.lower(&cave) {
                cave.settle(rock);
                break;
            }
        }
    }
    cave.max_y + 1
}

fn solve_2() {
    let winds = get_wind();
    let cycle_len = num::integer::lcm(winds.len(), 5);
    println!("{} {} -> {}", winds.len(), num::integer::lcm(winds.len(), 5), cycle_len);
    let mut winds = winds.iter().cycle();
    let shapes = the_shapes();
    let mut shapes = shapes.iter().cycle().peekable();
    let mut cave = Cave::new();
    let mut hashes = vec![];
    let mut heights = vec![];
    println!("Searching for a cycle...");
    for cycle in 0..1000 {
        for _ in 0..cycle_len {
            let shape = shapes.next().unwrap();
            let mut rock = Rock::new(shape, &cave);
            loop {
                let wind = winds.next().unwrap();
                rock.blow(wind, &cave);
                if rock.lower(&cave) {
                    cave.settle(rock);
                    break;
                }
            }
        }
        println!("{}", cycle);
        cave._print();
        let hash = cave.hash_state();
        if hashes.contains(&hash) {
            let initial_cycle = hashes.iter().position(|&h| h == hash).unwrap();
            println!("Found it! {} {}", cycle, initial_cycle);
            let initial_height = heights.get(initial_cycle).unwrap();
            let initial_rocks = cycle_len * (initial_cycle + 1);
            let final_height = cave.max_y + 1;
            let final_rocks = cycle_len * (cycle + 1);
            let cycle_height = final_height - initial_height;
            let cycle_rocks = final_rocks - initial_rocks;
            println!("{} {}", cycle_rocks, cycle_height);
            return;
        }
        hashes.push(hash);
        heights.push(cave.max_y + 1);
    }
    panic!("Couldn't find a cycle :(")
}


pub fn solve() {
    println!("{}", solve_1(2022));
    solve_2();
}
