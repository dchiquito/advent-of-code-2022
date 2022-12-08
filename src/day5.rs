use std::fmt::Debug;

use regex::Regex;

use crate::advent;

#[derive(Debug)]
struct Yard {
    stacks: Vec<Stack>,
}

impl Yard {
    fn new() -> Yard {
        Yard { stacks: vec![] }
    }
    fn move_crates(&mut self, num: i32, from: usize, to: usize) {
        for _ in 0..num {
            let carte = self.stacks[from].crates.pop().unwrap();
            self.stacks[to].crates.push(carte);
        }
    }
    fn move_crate_stack(&mut self, num: i32, from: usize, to: usize) {
        let mut tmp = vec![];
        for _ in 0..num {
            let carte = self.stacks[from].crates.pop().unwrap();
            tmp.push(carte);
        }
        for _ in 0..num {
            self.stacks[to].crates.push(tmp.pop().unwrap());
        }
    }
    fn repr(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.crates.last().unwrap().to_string())
            .fold("".to_string(), |acc, c| acc + &c)
    }
}

#[derive(Debug, Default)]
struct Stack {
    crates: Vec<Crate>,
}

impl Stack {
    fn new() -> Stack {
        Stack { crates: vec![] }
    }
    fn push_bottom(&mut self, carte: Crate) {
        self.crates.insert(0, carte);
    }
}

type Crate = String;

fn load_initial_yard() -> Yard {
    let crate_re: Regex = Regex::new(r"(?:\[([A-Z])\] )|(?:    )").unwrap();
    let mut yard = Yard::new();
    for _ in 0..9 {
        // TODO don't hardcode magic numbers
        yard.stacks.push(Stack::new())
    }
    let reader = advent::read_input(5);
    for line in reader {
        let line = line.unwrap() + " ";
        if line == " " {
            break;
        }
        let mut i = 0;
        for captures in crate_re.captures_iter(&line) {
            if let Some(m) = captures.get(1) {
                yard.stacks[i].push_bottom(m.as_str().to_string());
            }
            i += 1;
        }
    }
    yard
}

pub fn solve() {
    let mut yard = load_initial_yard();

    // part 1
    // Borrowing is hard, so just read the damn file twice
    let move_re = Regex::new(r"move ([0-9]+) from ([1-9]) to ([1-9])").unwrap();
    let reader = advent::read_input(5);
    for line in reader {
        let line = line.unwrap();
        if let Some(captures) = move_re.captures(&line) {
            let num = str::parse::<i32>(&captures[1]).unwrap();
            let from = str::parse::<usize>(&captures[2]).unwrap() - 1;
            let to = str::parse::<usize>(&captures[3]).unwrap() - 1;
            yard.move_crates(num, from, to);
        }
    }
    println!("{}", yard.repr());


    // two more reads for part 2 bby
    let mut yard = load_initial_yard();
    let reader = advent::read_input(5);
    for line in reader {
        let line = line.unwrap();
        if let Some(captures) = move_re.captures(&line) {
            let num = str::parse::<i32>(&captures[1]).unwrap();
            let from = str::parse::<usize>(&captures[2]).unwrap() - 1;
            let to = str::parse::<usize>(&captures[3]).unwrap() - 1;
            yard.move_crate_stack(num, from, to);
        }
    }
    println!("{}", yard.repr());
}
