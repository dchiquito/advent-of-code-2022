use crate::advent;

struct Map {
    rows: Vec<Vec<char>>,
    width: i32,
    height: i32,
}

impl Map {
    fn new(lines: Vec<String>) -> Map {
        let width = lines.iter().map(String::len).max().unwrap();
        let rows: Vec<Vec<char>> = lines
            .iter()
            .map(|line| {
                let padding = width - line.len();
                let padding_iter = " ".chars().cycle().take(padding);
                line.chars().chain(padding_iter).collect()
            })
            .collect();
        let height = rows.len();
        Map {
            rows,
            width: width as i32,
            height: height as i32,
        }
    }
    fn step(&self, x: i32, y: i32, dir: &Direction) -> Option<(i32, i32)> {
        let (dx, dy) = dir.delta();
        let (nx, ny) = (x + dx, y + dy);
        let (mut nx, mut ny) = (
            ((nx + self.width) % self.width),
            ((ny + self.height) % self.height),
        );
        // handle the wraparound case
        if self.rows[ny as usize][nx as usize] == ' ' {
            match dir {
                Direction::Up => (nx, ny) = (nx, self.height - 1),
                Direction::Right => (nx, ny) = (0, ny),
                Direction::Down => (nx, ny) = (nx, 0),
                Direction::Left => (nx, ny) = (self.width - 1, ny),
            }
            while self.rows[ny as usize][nx as usize] == ' ' {
                (nx, ny) = (nx + dx, ny + dy);
            }
        }
        if self.rows[ny as usize][nx as usize] == '#' {
            None
        } else {
            Some((nx, ny))
        }
    }
    fn walk(&self, x: i32, y: i32, dir: &Direction, steps: i32) -> (i32, i32) {
        let (mut x, mut y) = (x, y);
        for _ in 0..steps {
            if let Some((nx, ny)) = self.step(x, y, dir) {
                (x, y) = (nx, ny);
            } else {
                break;
            }
        }
        (x, y)
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(&self, c: char) -> Direction {
        match c {
            'L' => match self {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
            },
            'R' => match self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            },
            _ => panic!("{} is not a direction", c),
        }
    }
    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
    fn num(&self) -> i32 {
        match self {
            Direction::Up => 3,
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
        }
    }
}

fn solve_1() -> i32 {
    let mut lines = advent::read_input(22);
    let direction_str = lines.last().unwrap();
    let direction_str = direction_str.replace("L", " L ");
    let direction_str = direction_str.replace("R", " R ");
    let directions = direction_str.split(" ");
    lines.pop();
    lines.pop();
    let map = Map::new(lines);
    let mut x = map.rows[0]
        .iter()
        .enumerate()
        .find(|(_, c)| c != &&' ')
        .map(|(i, _)| i)
        .unwrap() as i32;
    let mut y = 0;
    let mut dir = Direction::Right;
    for instruction in directions {
        match instruction {
            "L" => dir = dir.turn('L'),
            "R" => dir = dir.turn('R'),
            num_str => {
                let steps = num_str.parse().unwrap();
                (x, y) = map.walk(x, y, &dir, steps);
            }
        }
    }
    (1000 * (y + 1)) + (4 * (x + 1)) + dir.num()
}

struct Cube {
    map: Map,
    size: i32,
}
impl Cube {
    fn new(lines: Vec<String>) -> Cube {
        let map = Map::new(lines);
        let total_tiles = map
            .rows
            .iter()
            .map(|row| row.iter().filter(|&c| c != &' ').count())
            .fold(0, |a, b| a + b);
        let size = ((total_tiles / 6) as f64).sqrt() as i32;
        Cube { map, size }
    }
    fn get(&self, x: i32, y: i32) -> char {
        if x < 0 || x >= self.map.width || y < 0 || y >= self.map.height {
            ' '
        } else {
            self.map.rows[y as usize][x as usize]
        }
    }
    fn walk(&self, x: i32, y: i32, dir: &Direction, steps: i32) -> (i32, i32, Direction) {
        let (mut x, mut y, mut dir) = (x, y, dir.clone());

        for _ in 0..steps {
            let (nx, ny, ndir, collision) = self.step(x, y, &dir);
            if !collision {
                (x, y, dir) = (nx, ny, ndir);
            } else {
                break;
            }
        }
        (x, y, dir.clone())
    }
    fn ghost_walk(&self, x: i32, y: i32, dir: &Direction, steps: i32) -> (i32, i32, Direction) {
        let (mut x, mut y, mut dir) = (x, y, dir.clone());

        for _ in 0..steps {
            (x, y, dir, _) = self.step(x, y, &dir);
        }
        (x, y, dir.clone())
    }
    fn step(&self, x: i32, y: i32, dir: &Direction) -> (i32, i32, Direction, bool) {
        let (dx, dy) = dir.delta();
        let (nx, ny) = (x + dx, y + dy);
        // if the step would hit a ' ', that is, a non-joined cube edge, then
        // * turn left
        // * walk just past the next edge of the cube
        // * turn right
        // * walk back the same number of steps
        // * turn left
        // If either edge is also non-joined, then this will recurse around the cube until a
        // path is found.
        match self.get(nx, ny) {
            ' ' => {
                let dir = dir.turn('L');
                let distance = match dir {
                    Direction::Up => (y % self.size) + 1,
                    Direction::Right => self.size - (x % self.size),
                    Direction::Down => self.size - (y % self.size),
                    Direction::Left => (x % self.size) + 1,
                };
                let (x, y, dir) = self.ghost_walk(x, y, &dir, distance);
                let dir = dir.turn('R');
                let (x, y, dir) = self.ghost_walk(x, y, &dir, distance);
                let dir = dir.turn('L');
                let collision = self.get(x, y) == '#';
                (x, y, dir, collision)
            }
            '.' => (nx, ny, dir.clone(), false),
            '#' => (nx, ny, dir.clone(), true),
            c => panic!("what is {} doin in the map", c),
        }
    }
}
fn solve_2() -> i32 {
    let mut lines = advent::read_input(22);
    let direction_str = lines.last().unwrap();
    let direction_str = direction_str.replace("L", " L ");
    let direction_str = direction_str.replace("R", " R ");
    let directions = direction_str.split(" ");
    lines.pop();
    lines.pop();
    let cube = Cube::new(lines);
    let mut x = cube.map.rows[0]
        .iter()
        .enumerate()
        .find(|(_, c)| c != &&' ')
        .map(|(i, _)| i)
        .unwrap() as i32;
    let mut y = 0;
    let mut dir = Direction::Right;
    for instruction in directions {
        match instruction {
            "L" => dir = dir.turn('L'),
            "R" => dir = dir.turn('R'),
            num_str => {
                let steps = num_str.parse().unwrap();
                (x, y, dir) = cube.walk(x, y, &dir, steps);
            }
        }
    }
    (1000 * (y + 1)) + (4 * (x + 1)) + dir.num()
}
pub fn solve() {
    println!("{}", solve_1());
    println!("{}", solve_2());
}
