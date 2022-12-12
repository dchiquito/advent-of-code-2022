use crate::advent;
use regex::{Captures, Regex};

#[derive(Debug, Clone)]
enum Operation {
    ADD,
    MULTIPLY,
}
impl Operation {
    fn from(c: &str) -> Operation {
        match c {
            "*" => Operation::MULTIPLY,
            "+" => Operation::ADD,
            _ => panic!("Unknown operator {}", c),
        }
    }
    fn apply(&self, left: u32, right: Option<u32>) -> u32 {
        let right = right.unwrap_or(left);
        match self {
            Operation::ADD => left + right,
            Operation::MULTIPLY => left * right,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    operand: Option<u32>,
    divisor: u32,
    true_target: usize,
    false_target: usize,
    inspections: u32,
}

impl Monkey {
    fn new(capture: Captures) -> Monkey {
        Monkey {
            items: (&capture[1])
                .split(", ")
                .map(str::parse::<u32>)
                .map(Result::unwrap)
                .collect(),
            operation: Operation::from(&capture[2]),
            operand: (&capture[3]).parse().ok(),
            divisor: (&capture[4]).parse().unwrap(),
            true_target: (&capture[5]).parse().unwrap(),
            false_target: (&capture[6]).parse().unwrap(),
            inspections: 0,
        }
    }
    fn round_1(monkeys: &mut Vec<Monkey>) {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let monkey: Monkey;
                let item: u32;
                {
                    let mut_monkey = &mut monkeys[i];
                    item = mut_monkey.items.remove(0);
                    mut_monkey.inspections += 1;
                    monkey = mut_monkey.clone();
                }
                let item = monkey.operation.apply(item, monkey.operand);
                let item = item / 3;
                if item % monkey.divisor == 0 {
                    monkeys[monkey.true_target].items.push(item);
                } else {
                    monkeys[monkey.false_target].items.push(item);
                }
            }
        }
    }
    fn round_2(monkeys: &mut Vec<Monkey>) {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let monkey: Monkey;
                let item: u32;
                {
                    let mut_monkey = &mut monkeys[i];
                    item = mut_monkey.items.remove(0);
                    mut_monkey.inspections += 1;
                    monkey = mut_monkey.clone();
                }
                let item = monkey.operation.apply(item, monkey.operand);
                // no reductions now
                // let item = item / 3;
                if item % monkey.divisor == 0 {
                    monkeys[monkey.true_target].items.push(item);
                } else {
                    monkeys[monkey.false_target].items.push(item);
                }
            }
        }
    }
}

fn get_monkeys() -> Vec<Monkey> {
    let file = advent::read_input(11)
        .map(Result::unwrap)
        .reduce(|a, b| a + "\n" + &b)
        .unwrap();
    let monkey_re = Regex::new(r"Monkey [0-9]:\n  Starting items: ([0-9, ]+)\n  Operation: new = old ([\*\+]) ([0-9old]+)\n  Test: divisible by ([0-9]+)\n    If true: throw to monkey ([0-9])\n    If false: throw to monkey ([0-9])").unwrap();
    monkey_re.captures_iter(&file).map(Monkey::new).collect()
}

fn monkey_business(monkeys: &Vec<Monkey>) -> u32 {
    let mut max_inspections_1 = 0;
    for monkey in monkeys.iter() {
        if monkey.inspections > max_inspections_1 {
            max_inspections_1 = monkey.inspections;
        }
    }
    let mut max_inspections_2 = 0;
    for monkey in monkeys.iter() {
        if monkey.inspections > max_inspections_2 && monkey.inspections < max_inspections_1 {
            max_inspections_2 = monkey.inspections;
        }
    }
    max_inspections_1 * max_inspections_2
}

pub fn solve() {
    // part 1
    let mut monkeys = get_monkeys();
    for _ in 0..20 {
        Monkey::round_1(&mut monkeys);
    }
    println!("{}", monkey_business(&monkeys));

    // part 2
    let mut monkeys = get_monkeys();
    for _ in 0..20 {
        Monkey::round_2(&mut monkeys);
    }
    println!("{}", monkey_business(&monkeys));
}
