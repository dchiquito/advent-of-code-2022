use std::collections::HashSet;

use crate::advent;
use substring::Substring;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    U,
    D,
    L,
    R,
}

impl Direction {
    fn from(c: &str) -> Direction {
        match c {
            "U" => Direction::U,
            "D" => Direction::D,
            "L" => Direction::L,
            "R" => Direction::R,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Move(Direction, i32);

fn read_input() -> Vec<Move> {
    let lines = advent::read_input(9);
    lines
        .iter()
        .map(|line| {
            Move(
                Direction::from(line.substring(0, 1)),
                line.substring(2, 4).parse().unwrap(),
            )
        })
        .collect()
}

#[derive(Clone)]
struct Snake {
    x: i32,
    y: i32,
    tail_dx: i32,
    tail_dy: i32,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            x: 0,
            y: 0,
            tail_dx: 0,
            tail_dy: 0,
        }
    }
    fn incr_head(&self, dx: i32, dy: i32) -> Snake {
        Snake {
            x: self.x + dx,
            y: self.y + dy,
            tail_dx: self.tail_dx - dx,
            tail_dy: self.tail_dy - dy,
        }
    }
    fn move_head_to(&self, x: i32, y: i32) -> Snake {
        self.incr_head(x - self.x, y - self.y)
    }
    /**
     * Contract the tail if necessary
     */
    fn scrunch(&self) -> Snake {
        let mut snake = self.clone();
        if (self.tail_dx * self.tail_dy).abs() >= 4 {
            // it's offset in both directions, which means it snaps to a diagonal
            snake.tail_dx = snake.tail_dx.abs() / snake.tail_dx;
            snake.tail_dy = snake.tail_dy.abs() / snake.tail_dy;
            snake
        } else if self.tail_dy < -1 {
            snake.tail_dx = 0;
            snake.tail_dy = -1;
            snake
        } else if snake.tail_dy > 1 {
            snake.tail_dx = 0;
            snake.tail_dy = 1;
            snake
        } else if self.tail_dx < -1 {
            snake.tail_dx = -1;
            snake.tail_dy = 0;
            snake
        } else if snake.tail_dx > 1 {
            snake.tail_dx = 1;
            snake.tail_dy = 0;
            snake
        } else {
            snake
        }
    }
    fn go(&self, direction: &Direction) -> Snake {
        match direction {
            Direction::U => self.incr_head(0, 1),
            Direction::D => self.incr_head(0, -1),
            Direction::L => self.incr_head(-1, 0),
            Direction::R => self.incr_head(1, 0),
        }
        .scrunch()
    }
    fn tail_position(&self) -> (i32, i32) {
        (self.x + self.tail_dx, self.y + self.tail_dy)
    }
}

fn solve_1(moves: &Vec<Move>) {
    let mut snake = Snake::new();
    let mut points = HashSet::new();
    points.insert(snake.tail_position());
    for m in moves.iter() {
        match m {
            Move(direction, distance) => {
                for _ in 0..*distance {
                    snake = snake.go(direction);
                    points.insert(snake.tail_position());
                }
            }
        }
    }
    println!("{}", points.len());
}
#[allow(dead_code)]
fn showit(snakes: &Vec<Snake>, points: &HashSet<(i32, i32)>) {
    let snakepoints: Vec<(i32, i32)> = snakes.iter().map(|snake| (snake.x, snake.y)).collect();
    for y in (-10..10).rev() {
        for x in -15..15 {
            if (x, y) == (0, 0) {
                print!("s");
            } else if snakepoints.contains(&(x, y)) {
                for i in 0..snakepoints.len() {
                    if snakepoints[i] == (x, y) {
                        print!("{}", i);
                        break;
                    }
                }
            } else if points.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("");
}
/*
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
 */
fn solve_2(moves: &Vec<Move>) {
    let mut snakes = vec![];
    for _ in 0..9 {
        snakes.push(Snake::new());
    }
    let mut points = HashSet::new();
    points.insert(snakes.last().unwrap().tail_position());
    for m in moves.iter() {
        match m {
            Move(direction, distance) => {
                for _ in 0..*distance {
                    snakes[0] = snakes[0].go(direction);
                    for i in 1..snakes.len() {
                        let (last_x, last_y) = snakes[i - 1].tail_position();
                        snakes[i] = snakes[i].move_head_to(last_x, last_y).scrunch();
                    }
                    points.insert(snakes.last().unwrap().tail_position());
                }
                // showit(&snakes, &points);
            }
        }
    }
    println!("{}", points.len());
}

pub fn solve() {
    let moves = read_input();
    solve_1(&moves);
    solve_2(&moves);
}

#[cfg(test)]
mod test {
    use super::Snake;

    #[test]
    fn test_incr_head() {
        for dx in -5..5 {
            for dy in -5..5 {
                let snake = Snake::new().incr_head(dx, dy);
                assert_eq!(snake.x, dx);
                assert_eq!(snake.y, dy);
                assert_eq!(snake.tail_position(), (0, 0))
            }
        }
    }

    #[test]
    fn test_scrunchy_touching() {
        // No tail positions that are already touching should be scrunched
        for dx in -1..1 {
            for dy in -1..1 {
                let snake = Snake {
                    x: 0,
                    y: 0,
                    tail_dx: dx,
                    tail_dy: dy,
                }
                .scrunch();
                assert_eq!(snake.tail_position(), (dx, dy));
            }
        }
    }

    #[test]
    fn test_scrunchy_scrunch() {
        fn make_snake(dx: i32, dy: i32) -> Snake {
            Snake {
                x: 0,
                y: 0,
                tail_dx: dx,
                tail_dy: dy,
            }
        }
        // left tail
        let dx = -2;
        for dy in -1..1 {
            assert_eq!(make_snake(dx, dy).scrunch().tail_position(), (-1, 0));
        }
        // right tail
        let dx = 2;
        for dy in -1..1 {
            assert_eq!(make_snake(dx, dy).scrunch().tail_position(), (1, 0));
        }
        // down tail
        let dy = -2;
        for dx in -1..1 {
            assert_eq!(make_snake(dx, dy).scrunch().tail_position(), (0, -1));
        }
        // up tail
        let dy = 2;
        for dx in -1..1 {
            assert_eq!(make_snake(dx, dy).scrunch().tail_position(), (0, 1));
        }
    }

    #[test]
    fn test_scrunchy_corner() {
        fn make_snake(dx: i32, dy: i32) -> Snake {
            Snake {
                x: 0,
                y: 0,
                tail_dx: dx,
                tail_dy: dy,
            }
        }
        assert_eq!(make_snake(-2, -2).scrunch().tail_position(), (-1, -1));
        assert_eq!(make_snake(-2, 2).scrunch().tail_position(), (-1, 1));
        assert_eq!(make_snake(2, -2).scrunch().tail_position(), (1, -1));
        assert_eq!(make_snake(2, 2).scrunch().tail_position(), (1, 1));
    }
}
