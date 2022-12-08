use substring::Substring;

use crate::advent;

fn solver(sequential: usize) {
    let mut lines = advent::read_input(6);
    let line = lines.next().unwrap().unwrap();
    for i in sequential..line.len() {
        let slice = line.substring(i - sequential, i);
        let mut duplicate = false;
        for j in 0..sequential {
            if slice
                .substring(j + 1, sequential)
                .contains(slice.chars().nth(j).unwrap())
            {
                duplicate = true;
            }
        }
        if !duplicate {
            println!("{}", i);
            return;
        }
    }
}

pub fn solve() {
    solver(4);
    solver(14);
}
