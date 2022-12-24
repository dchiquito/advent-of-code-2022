use std::collections::HashSet;

use crate::advent;

type Pos = (usize, usize);

#[derive(Clone, Debug)]
enum Dir {
    North,
    South,
    West,
    East,
}
impl Dir {
    fn from(c: char) -> Option<Dir> {
        match c {
            '^' => Some(Dir::North),
            'v' => Some(Dir::South),
            '<' => Some(Dir::West),
            '>' => Some(Dir::East),
            _ => None,
        }
    }
}

struct Valley {
    width: usize,
    height: usize,
    blizzards: Vec<(Pos, Dir)>,
    blizzard_grid: Vec<Vec<u32>>,
}
impl Valley {
    fn new(lines: Vec<String>) -> Valley {
        let mut valley = Valley {
            width: lines[0].len() - 2,
            height: lines.len() - 2,
            blizzards: vec![],
            blizzard_grid: vec![],
        };
        // Skip the first and last rows, they are just walls
        for (y, row) in lines.iter().skip(1).take(valley.height).enumerate() {
            valley.blizzard_grid.push(vec![]);
            // Skip the first and last columns, they are just walls
            for (x, c) in row.chars().skip(1).take(valley.width).enumerate() {
                valley.blizzard_grid[y].push(0);
                if let Some(dir) = Dir::from(c) {
                    valley.blizzards.push(((x, y), dir));
                    valley.blizzard_grid[y][x] += 1;
                }
            }
        }
        valley
    }
    /// Move a position in a direction, wrapping around if it is out of bounds.
    fn incr_blizzard((pos, dir): &(Pos, Dir), width: usize, height: usize) -> Pos {
        let (x, y) = *pos;
        match dir {
            Dir::North => (x, (y + height - 1) % height),
            Dir::South => (x, (y + 1) % height),
            Dir::West => ((x + width - 1) % width, y),
            Dir::East => ((x + 1) % width, y),
        }
    }
    fn tick(&mut self) {
        for blizzard in self.blizzards.iter_mut() {
            let (x, y) = blizzard.0;
            self.blizzard_grid[y][x] -= 1;
            blizzard.0 = Valley::incr_blizzard(blizzard, self.width, self.height);
            let (x, y) = blizzard.0;
            self.blizzard_grid[y][x] += 1;
        }
    }
    #[allow(dead_code)]
    fn print(&self) {
        for _ in 0..self.width + 2 {
            print!("#");
        }
        println!();
        for row in self.blizzard_grid.iter() {
            print!("#");
            for c in row.iter() {
                print!("{}", c);
            }
            println!("#");
        }
        for _ in 0..self.width + 2 {
            print!("#");
        }
        println!();
    }
}

fn traverse(valley: &mut Valley, start: &Pos, end: &Pos) -> u32 {
    let mut minutes = 0;
    let mut superpositions: HashSet<Pos> = HashSet::new();
    superpositions.insert(*start);
    while !superpositions.contains(end) {
        valley.tick();
        minutes += 1;
        let mut new_superpositions = HashSet::new();
        if valley.blizzard_grid[start.1][start.0] == 0 {
            new_superpositions.insert(*start);
        }
        for (x, y) in superpositions.iter() {
            let (x, y) = (*x, *y);
            if valley.blizzard_grid[y][x] == 0 {
                new_superpositions.insert((x, y));
            }
            if x > 0 && valley.blizzard_grid[y][x - 1] == 0 {
                new_superpositions.insert((x - 1, y));
            }
            if x < valley.width - 1 && valley.blizzard_grid[y][x + 1] == 0 {
                new_superpositions.insert((x + 1, y));
            }
            if y > 0 && valley.blizzard_grid[y - 1][x] == 0 {
                new_superpositions.insert((x, y - 1));
            }
            if y < valley.height - 1 && valley.blizzard_grid[y + 1][x] == 0 {
                new_superpositions.insert((x, y + 1));
            }
        }
        superpositions = new_superpositions;
    }
    // one extra minute to move to the exit
    minutes += 1;
    valley.tick();
    minutes
}

fn solve_1() -> u32 {
    let lines = advent::read_input(24);
    let mut valley = Valley::new(lines);
    let width = valley.width - 1;
    let height = valley.height - 1;
    traverse(&mut valley, &(0, 0), &(width, height))
}
fn solve_2() -> u32 {
    let lines = advent::read_input(24);
    let mut valley = Valley::new(lines);
    let width = valley.width - 1;
    let height = valley.height - 1;
    traverse(&mut valley, &(0, 0), &(width, height))
        + traverse(&mut valley, &(width, height), &(0, 0))
        + traverse(&mut valley, &(0, 0), &(width, height))
}
pub fn solve() {
    println!("{}", solve_1());
    println!("{}", solve_2());
}
