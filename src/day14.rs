use crate::advent;
use regex::Regex;
use std::fmt::Debug;

type Rocks = Vec<Vec<(i32, i32)>>;

#[derive(Debug)]
struct Dimensions {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Dimensions {
    fn new(rocks: &Rocks) -> Dimensions {
        let mut min_x = std::i32::MAX;
        let mut max_x = std::i32::MIN;
        let mut max_y = std::i32::MIN;
        for rock in rocks {
            for (x, y) in rock {
                if *x < min_x {
                    min_x = *x;
                }
                if *x > max_x {
                    max_x = *x;
                }
                if *y > max_y {
                    max_y = *y;
                }
            }
        }
        Dimensions {
            min_x,
            max_x: max_x + 5,
            min_y: 0,
            max_y: max_y + 5, // just some buffer, shouldn't impact performance
        }
    }
}

fn parse_lines() -> Rocks {
    let re = Regex::new(r"(\d+),(\d+)").unwrap();
    advent::read_input(14)
        .iter()
        .map(|line| {
            re.captures_iter(&line)
                .map(|capture| (capture[1].parse().unwrap(), capture[2].parse().unwrap()))
                .collect()
        })
        .collect()
}

struct Cave {
    grid: Vec<Vec<bool>>,
    dimensions: Dimensions,
}

impl Cave {
    fn new(rocks: &Rocks) -> Cave {
        let dimensions = Dimensions::new(rocks);
        let mut cave = Cave {
            grid: (dimensions.min_y..dimensions.max_y)
                .map(|_| {
                    (dimensions.min_x..dimensions.max_x)
                        .map(|_| false)
                        .collect()
                })
                .collect(),
            dimensions,
        };
        for rock in rocks {
            // lol that's not what reduce is for
            rock.iter().reduce(|left, right| {
                cave.fill_rock_row(*left, *right);
                right
            });
        }
        cave
    }
    fn fill_rock_row(&mut self, a: (i32, i32), b: (i32, i32)) {
        let (x1, y1) = a;
        let (x2, y2) = b;
        if x1 == x2 {
            for y in y1.min(y2)..(y1.max(y2) + 1) {
                self.fill(x1, y);
            }
        }
        if y1 == y2 {
            for x in x1.min(x2)..(x1.max(x2) + 1) {
                self.fill(x, y1);
            }
        }
    }
    fn fill(&mut self, x: i32, y: i32) {
        self.grid[(y - self.dimensions.min_y) as usize][(x - self.dimensions.min_x) as usize] =
            true;
    }
    fn get(&self, x: i32, y: i32) -> bool {
        if x < self.dimensions.min_x || x > self.dimensions.max_x {
            return false;
        }
        self.grid[(y - self.dimensions.min_y) as usize][(x - self.dimensions.min_x) as usize]
    }
    fn drop_sand(&mut self, path: &mut Vec<(i32, i32)>) -> bool {
        if path.is_empty() {
            return false;
        }
        let (mut x, mut y) = path.last().unwrap();
        loop {
            if y + 1 >= self.dimensions.max_y {
                return false;
            }
            if !self.get(x, y + 1) {
                // The sand can fall straight down
                path.push((x, y + 1));
                (x, y) = (x, y + 1);
                continue;
            }
            if !self.get(x - 1, y + 1) {
                // The sand can fall to the left
                path.push((x - 1, y + 1));
                (x, y) = (x - 1, y + 1);
                continue;
            }
            if !self.get(x + 1, y + 1) {
                // The sand can fall to the right
                path.push((x + 1, y + 1));
                (x, y) = (x + 1, y + 1);
                continue;
            }
            // The sand can't fall anymore
            // Fill in the settled grid square
            self.fill(x, y);
            // Pop the current square off of the path so the next sand starts one unit higher
            path.pop();
            return true;
        }
    }
}

impl Debug for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_struct("Cave").field("grid", &self.grid).field("dimensions", &self.dimensions).finish()
        for y in self.dimensions.min_y..self.dimensions.max_y {
            for x in self.dimensions.min_x..self.dimensions.max_x {
                if self.get(x, y) {
                    f.write_str("#")?;
                } else {
                    f.write_str(".")?;
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

fn solve_1() {
    let mut cave = Cave::new(&parse_lines());
    let mut i = 0;
    let mut path = vec![(500, 0)];
    while cave.drop_sand(&mut path) {
        i += 1;
    }
    println!("{}", i);
}

fn solve_2() {
    let mut rocks = parse_lines();
    // Add the floor
    let d = Dimensions::new(&rocks);
    rocks.push(vec![
        (d.min_x - d.max_y, d.max_y - 3),
        (d.max_x + d.max_y, d.max_y - 3),
    ]);
    let mut cave = Cave::new(&rocks);
    let mut i = 0;
    let mut path = vec![(500, 0)];
    while cave.drop_sand(&mut path) {
        i += 1;
    }
    println!("{}", i);
}

pub fn solve() {
    solve_1();
    solve_2();
}
