use crate::advent;
use regex::Regex;
use std::fmt::Debug;
use std::ops::Add;

#[derive(Debug, PartialEq, Eq)]
struct Sensor {
    sx: i64,
    sy: i64,
    bx: i64,
    by: i64,
}

impl Sensor {
    fn distance(&self) -> i64 {
        return (self.sx - self.bx).abs() + (self.sy - self.by).abs();
    }
    fn range(&self, row: i64) -> Option<(i64, i64)> {
        let sd = self.distance();
        let dy = (self.sy - row).abs();
        if sd < dy {
            None
        } else {
            let dx = sd - dy;
            Some((self.sx - dx, self.sx + dx))
        }
    }
}

fn parse_input() -> Vec<Sensor> {
    let re = Regex::new(
        r"Sensor at x=([-\d]+), y=([-\d]+): closest beacon is at x=([-\d]+), y=([-\d]+)",
    )
    .unwrap();
    advent::read_input(15)
        .iter()
        .map(|line| {
            let capture = re.captures(line).unwrap();
            Sensor {
                sx: capture[1].parse().unwrap(),
                sy: capture[2].parse().unwrap(),
                bx: capture[3].parse().unwrap(),
                by: capture[4].parse().unwrap(),
            }
        })
        .collect()
}

fn merge_ranges(ranges: &Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut ranges = ranges.clone();
    let mut new = vec![];
    while !ranges.is_empty() {
        let (mut a, mut b) = ranges.pop().unwrap();
        let mut i = 0;
        while i < ranges.len() {
            let (aa, bb) = ranges[i];
            // there is no overlap iff (a < b < aa < bb) or (aa < bb < a < b)
            if !(b < aa || bb < a) {
                // merge the lists
                a = a.min(aa);
                b = b.max(bb);
                ranges.remove(i);
                // now we may be overlapping things we weren't before, so reset i :(
                i = 0;
            } else {
                // no overlap, try the next one
                i += 1;
            }
        }
        new.push((a, b));
    }
    new
}

fn solve_1(row: i64) {
    let sensors = parse_input();
    let ranges: Vec<(i64, i64)> = sensors
        .iter()
        .map(|sensor| sensor.range(row))
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect();
    let ranges = merge_ranges(&ranges);
    let range_sum = ranges.iter().map(|(a, b)| b - a).reduce(i64::add).unwrap();
    println!("{}", range_sum);
}

fn solve_2(limit: i64) {
    let sensors = parse_input();
    for y in 0..limit {
        let mut x = 0;
        let mut ranges: Vec<(i64, i64)> = sensors
            .iter()
            .map(|s| s.range(y))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect();
        ranges.sort_by(|(a, _), (b, _)| a.cmp(b));
        for (a, b) in ranges.iter() {
            if *a <= x && x <= *b {
                x = b + 1;
            }
        }
        if x < limit {
            println!("{}", (x * 4000000) + y);
            return ();
        }
    }
}
pub fn solve() {
    // testing
    // solve_1(10);
    solve_1(2000000);

    // testing
    // solve_2(20);
    solve_2(4000000);
}
