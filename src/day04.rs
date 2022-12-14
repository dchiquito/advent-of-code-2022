use regex::{Captures, Regex};

use crate::advent;

#[derive(Debug)]
struct Row {
    a: i32,
    b: i32,
    x: i32,
    y: i32,
}

impl Row {
    fn new(captures: Captures) -> Row {
        Row {
            a: str::parse(&captures[1]).unwrap(),
            b: str::parse(&captures[2]).unwrap(),
            x: str::parse(&captures[3]).unwrap(),
            y: str::parse(&captures[4]).unwrap(),
        }
    }
    fn is_nested(&self) -> bool {
        return (self.a <= self.x && self.y <= self.b) || (self.x <= self.a && self.b <= self.y);
    }
    fn is_overlapping(&self) -> bool {
        return (self.a <= self.x && self.x <= self.b) || (self.x <= self.a && self.a <= self.y);
    }
}

pub fn solve() {
    let reader = advent::read_input(4);
    let re: Regex = Regex::new(r"^([0-9]+)-([0-9]+),([0-9]+)-([0-9]+)$").unwrap();
    let rows: Vec<Row> = reader
        .iter()
        .map(|line| Row::new(re.captures(&line).unwrap()))
        .collect();
    let nested = rows
        .iter()
        .fold(0, |acc, row| acc + (row.is_nested() as i32));
    println!("{}", nested);
    let overlapping = rows
        .iter()
        .fold(0, |acc, row| acc + (row.is_overlapping() as i32));
    println!("{}", overlapping);
}
