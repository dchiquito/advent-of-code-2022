use crate::advent;
use std::ops::Add;

#[derive(Debug)]
struct Tree {
    height: u32,
    visible: bool,
}
impl Tree {
    fn new(height: u32) -> Tree {
        Tree {
            height,
            visible: false,
        }
    }
}

type Croft = Vec<Vec<Tree>>;

fn load_data() -> Croft {
    let lines = advent::read_input(8);
    lines
        .map(Result::unwrap)
        .map(|line| {
            line.chars()
                .map(|c| Tree::new(c.to_digit(10).unwrap()))
                .collect()
        })
        .collect()
}

fn solve_1(croft: &mut Croft) -> u32 {
    macro_rules! check_tree {
        ($tree:ident, $occluder:ident) => {
            if $tree.height as i32 > $occluder {
                $tree.visible = true;
                $occluder = $tree.height as i32;
                if $occluder >= 9 {
                    // optimization, nothing is higher than 9
                    break;
                }
            }
        };
    }
    for y in 0..croft.len() {
        let row = croft.get_mut(y).unwrap();
        let mut occluder: i32 = -1;
        for x in 0..row.len() {
            let tree = row.get_mut(x).unwrap();
            check_tree!(tree, occluder);
        }
        let mut occluder: i32 = -1;
        for x in (0..row.len()).rev() {
            let tree = row.get_mut(x).unwrap();
            check_tree!(tree, occluder);
        }
    }

    for x in 0..croft.get(0).unwrap().len() {
        let mut occluder: i32 = -1;
        for y in 0..croft.len() {
            let tree = croft.get_mut(y).unwrap().get_mut(x).unwrap();
            check_tree!(tree, occluder);
        }
        let mut occluder: i32 = -1;
        for y in (0..croft.len()).rev() {
            let tree = croft.get_mut(y).unwrap().get_mut(x).unwrap();
            check_tree!(tree, occluder);
        }
    }

    croft
        .iter()
        .map(|row| {
            row.iter()
                .map(|tree| tree.visible as u32)
                .reduce(u32::add)
                .unwrap()
        })
        .reduce(u32::add)
        .unwrap()
}

fn solve_2(croft: Croft) -> u32 {
    let mut top_score = 0;
    for y in 1..(croft.len() - 1) {
        let row = croft.get(y).unwrap();
        for x in 1..(row.len() - 1) {
            let tree = row.get(x).unwrap();

            let mut score = 1;

            // up
            let mut dy = 1;
            while y - dy > 0 && croft.get(y - dy).unwrap().get(x).unwrap().height < tree.height {
                dy += 1;
            }
            score *= dy;

            // down
            let mut dy = 1;
            while y + dy < croft.len() - 1
                && croft.get(y + dy).unwrap().get(x).unwrap().height < tree.height
            {
                dy += 1;
            }
            score *= dy;

            // left
            let mut dx = 1;
            while x - dx > 0 && row.get(x - dx).unwrap().height < tree.height {
                dx += 1;
            }
            score *= dx;

            // right
            let mut dx = 1;
            while x + dx < row.len() - 1 && row.get(x + dx).unwrap().height < tree.height {
                dx += 1;
            }
            score *= dx;

            if score > top_score {
                top_score = score;
            }
        }
    }

    top_score as u32
}

pub fn solve() {
    let mut croft = load_data();

    println!("{}", solve_1(&mut croft));
    println!("{}", solve_2(croft))
}
