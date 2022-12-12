use crate::advent;

#[derive(Debug, Clone)]
struct Cell {
    height: u32,
    distance: u32,
}

impl Cell {
    fn new(c: &char) -> Cell {
        let cc = match c {
            &'S' => &'a',
            &'E' => &'z',
            ccc => ccc,
        };
        Cell {
            height: *cc as u32,
            distance: std::u32::MAX,
        }
    }
    fn walkable_adjacents(pos: (usize, usize), grid: &Grid) -> Vec<(usize, usize)> {
        let (x, y) = pos;
        let height = grid[y][x].height;
        let mut adjacents = vec![];
        if x > 0 && grid[y][x - 1].height <= height + 1 {
            adjacents.push((x - 1, y));
        }
        if y > 0 && grid[y - 1][x].height <= height + 1 {
            adjacents.push((x, y - 1));
        }
        if x < grid[0].len() - 1 && grid[y][x + 1].height <= height + 1 {
            adjacents.push((x + 1, y));
        }
        if y < grid.len() - 1 && grid[y + 1][x].height <= height + 1 {
            adjacents.push((x, y + 1));
        }
        adjacents
    }
}

type Grid = Vec<Vec<Cell>>;

fn read_grid() -> (Grid, (usize, usize), (usize, usize)) {
    let grid: Vec<Vec<char>> = advent::read_input(12)
        .map(Result::unwrap)
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'S' {
                start = (x, y);
            }
            if grid[y][x] == 'E' {
                end = (x, y);
            }
        }
    }
    let grid = grid
        .iter()
        .map(|row| row.iter().map(Cell::new).collect())
        .collect();
    (grid, start, end)
}

fn walk_grid(grid: &mut Grid, start: (usize, usize), end: (usize, usize)) -> u32 {
    let (x, y) = start;
    grid[y][x].distance = 0;
    walk_cell(grid, start);
    let (x, y) = end;
    grid[y][x].distance
}
fn walk_cell(grid: &mut Grid, pos: (usize, usize)) {
    let (x, y) = pos;
    let initial_distance = grid[y][x].distance;
    for adjacent in Cell::walkable_adjacents(pos, grid) {
        let (x, y) = adjacent;
        let mut cell = &mut grid[y][x];
        if cell.distance > initial_distance + 1 {
            cell.distance = initial_distance + 1;
            walk_cell(grid, adjacent);
        }
    }
}
pub fn solve() {
    let (mut grid, start, end) = read_grid();
    println!("{:?}", walk_grid(&mut grid, start, end));

    let mut shortest_distance = std::u32::MAX;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x].height == 'a' as u32 {
                let distance = walk_grid(&mut grid.clone(), (x, y), end);
                if distance < shortest_distance {
                    shortest_distance = distance;
                }
            }
        }
    }
    println!("{}", shortest_distance);
}
