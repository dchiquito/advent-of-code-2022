use std::collections::HashMap;

use crate::advent;

type Pos = (i64, i64);

#[derive(Clone, Debug)]
enum Dir {
    North,
    South,
    West,
    East,
}
impl Dir {
    fn incr(&self, pos: &Pos) -> Pos {
        let (x, y) = *pos;
        match self {
            Dir::North => (x, y - 1),
            Dir::South => (x, y + 1),
            Dir::West => (x - 1, y),
            Dir::East => (x + 1, y),
        }
    }
}

type Elf = u32;

struct Proposal {
    origin: Pos,
    proposal: Option<(Dir, Pos)>,
}

impl Proposal {
    fn new(pos: &Pos) -> Proposal {
        Proposal {
            origin: pos.clone(),
            proposal: None,
        }
    }
    fn propose(&mut self, pos: &Pos, dir: &Dir) {
        self.origin = pos.clone();
        self.proposal = Some((dir.clone(), dir.incr(pos)));
    }
    fn idle(&mut self, pos: &Pos) {
        self.origin = pos.clone();
        self.proposal = None;
    }
}

struct Field {
    elves: HashMap<Pos, Elf>,
    proposals: HashMap<Elf, Proposal>,
}

impl Field {
    fn new(lines: Vec<String>) -> Field {
        let mut elves: HashMap<Pos, Elf> = HashMap::new();
        let mut i = 0;
        for (y, row) in lines.iter().enumerate() {
            for (x, c) in row.chars().enumerate() {
                if c == '#' {
                    elves.insert((x as i64, y as i64), i);
                    i += 1;
                }
            }
        }
        let mut proposals = HashMap::new();
        for (pos, elf) in elves.iter() {
            proposals.insert(*elf, Proposal::new(pos));
        }
        Field { elves, proposals }
    }
    fn has_adjacents(&self, pos: &Pos) -> bool {
        let (x, y) = *pos;
        for dx in -1..2 {
            for dy in -1..2 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if self.elves.contains_key(&(x + dx, y + dy)) {
                    return true;
                }
            }
        }
        return false;
    }
    fn can_elf_propose(&self, pos: &Pos, dir: &Dir) -> bool {
        let (x, y) = *pos;
        match dir {
            Dir::North => {
                !(self.elves.contains_key(&(x - 1, y - 1))
                    || self.elves.contains_key(&(x, y - 1))
                    || self.elves.contains_key(&(x + 1, y - 1)))
            }
            Dir::South => {
                !(self.elves.contains_key(&(x - 1, y + 1))
                    || self.elves.contains_key(&(x, y + 1))
                    || self.elves.contains_key(&(x + 1, y + 1)))
            }
            Dir::West => {
                !(self.elves.contains_key(&(x - 1, y - 1))
                    || self.elves.contains_key(&(x - 1, y))
                    || self.elves.contains_key(&(x - 1, y + 1)))
            }
            Dir::East => {
                !(self.elves.contains_key(&(x + 1, y - 1))
                    || self.elves.contains_key(&(x + 1, y))
                    || self.elves.contains_key(&(x + 1, y + 1)))
            }
        }
    }
    fn round(&mut self, dir_index: usize) -> bool {
        let dirs: Vec<Dir> = vec![Dir::North, Dir::South, Dir::West, Dir::East];
        for (pos, elf) in self.elves.iter() {
            let dirs = dirs.iter().cycle().skip(dir_index).take(4);
            // TODO there's a lot of duplicate hashmap checks, can be cleaned up with some
            // temporary data structures
            self.proposals.get_mut(elf).unwrap().idle(pos);
            if self.has_adjacents(pos) {
                for dir in dirs {
                    if self.can_elf_propose(pos, dir) {
                        self.proposals.get_mut(elf).unwrap().propose(pos, dir);
                        break;
                    }
                }
            }
        }
        let mut did_anyone_move = false;
        for (elf, proposal) in self.proposals.iter() {
            if let Some((dir, new_pos)) = &proposal.proposal {
                // Check for an opposing elf with an opposing proposal
                let opposing_pos = dir.incr(new_pos);
                if let Some(opposing_elf_id) = self.elves.get(&opposing_pos) {
                    if let Some((_, opposing_proposal_pos)) =
                        self.proposals.get(opposing_elf_id).unwrap().proposal
                    {
                        if *new_pos == opposing_proposal_pos {
                            // there is an opposing elf, with a proposal, which would place it into
                            // the same place as us.
                            // Skip applying this proposal.
                            continue;
                        }
                    }
                }
                // if there was a conflict, the loop would have continued already.
                // apply the proposal.
                self.elves.insert(*new_pos, *elf);
                self.elves.remove(&proposal.origin);
                did_anyone_move = true;
            }
        }
        did_anyone_move
    }
    fn bounds(&self) -> (i64, i64, i64, i64) {
        (
            *self.elves.iter().map(|((x, _), _)| x).min().unwrap(),
            *self.elves.iter().map(|((x, _), _)| x).max().unwrap(),
            *self.elves.iter().map(|((_, y), _)| y).min().unwrap(),
            *self.elves.iter().map(|((_, y), _)| y).max().unwrap(),
        )
    }
    #[allow(dead_code)]
    fn print(&self) {
        let (minx, maxx, miny, maxy) = self.bounds();
        for y in miny..maxy + 1 {
            for x in minx..maxx + 1 {
                if self.elves.contains_key(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn solve_1() -> i64 {
    let lines = advent::read_input(23);
    let mut field = Field::new(lines);
    //field.print();
    for i in 0..10 {
        field.round(i % 4);
    }
    // field.print();
    let (minx, maxx, miny, maxy) = field.bounds();
    ((maxx + 1 - minx) * (maxy + 1 - miny)) - (field.elves.len() as i64)
}
fn solve_2() -> usize {
    let lines = advent::read_input(23);
    let mut field = Field::new(lines);
    //field.print();
    let mut i = 0;
    while field.round(i % 4) {
        i += 1;
    }
    // field.print();
    i + 1
}

pub fn solve() {
    println!("{}", solve_1());
    println!("{}", solve_2());
}
