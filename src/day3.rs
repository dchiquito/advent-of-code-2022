use substring::Substring;

use crate::advent;

fn priority(c: char) -> u32 {
    if c.is_uppercase() {
        (c as u32) - 38
    } else {
        (c as u32) - 96
    }
}

pub fn solve() {
    // part 1
    let reader = advent::read_input(3);
    let mut sum = 0;
    let mut lines = 0;
    for line in reader {
        let line = line.unwrap();
        let left = line.substring(0, line.len()/2);
        let right = line.substring(line.len()/2, line.len());
        for c in left.chars() {
            if right.contains(c) {
                sum += priority(c);
                break;
            }
        }
        lines += 1;
    }
    println!("{}", sum);
    
    // part 2
    let mut reader = advent::read_input(3);
    let mut sum = 0;
    for _i in (0..lines).step_by(3) {
        let a = reader.next().unwrap().unwrap();
        let b = reader.next().unwrap().unwrap();
        let c = reader.next().unwrap().unwrap();
        for chr in a.chars() {
            if b.contains(chr) && c.contains(chr) {
                sum += priority(chr);
                break;
            }
        }
    }
    println!("{}", sum);
}
