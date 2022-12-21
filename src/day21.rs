use std::collections::HashMap;

use crate::advent;

#[derive(Debug)]
enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}
impl Op {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Op::Add => a + b,
            Op::Subtract => a - b,
            Op::Multiply => a * b,
            Op::Divide => a / b,
        }
    }
}
impl From<&str> for Op {
    fn from(op: &str) -> Self {
        match op {
            "+" => Op::Add,
            "-" => Op::Subtract,
            "*" => Op::Multiply,
            "/" => Op::Divide,
            _ => panic!("not an op: {}", op),
        }
    }
}

#[derive(Debug)]
enum Monkey {
    Number(i64),
    Operation(Op, String, String),
}

fn read_monkeys() -> HashMap<String, Monkey> {
    let lines = advent::read_input(21);
    let mut monkeys = HashMap::new();
    lines.iter().for_each(|line| {
        let mut s = line.split(": ");
        let name = s.next().unwrap().to_string();
        let remainder = s.next().unwrap();
        let monkey: Monkey;
        if let Ok(x) = remainder.parse::<i64>() {
            monkey = Monkey::Number(x)
        } else {
            let mut ss = remainder.split(" ");
            let left = ss.next().unwrap();
            let op = ss.next().unwrap();
            let right = ss.next().unwrap();
            monkey = Monkey::Operation(op.into(), left.to_string(), right.to_string())
        }
        monkeys.insert(name, monkey);
    });
    monkeys
}

fn eval(monkeys: &HashMap<String, Monkey>, monkey: &str) -> i64 {
    let monkey = monkeys.get(monkey).unwrap();
    match monkey {
        Monkey::Number(i) => *i,
        Monkey::Operation(op, left, right) => op.apply(eval(monkeys, left), eval(monkeys, right)),
    }
}

fn solve_1() -> i64 {
    let monkeys = read_monkeys();
    eval(&monkeys, "root")
}
// Evaluate as normal, but error out if we encounter humn
fn eval_2(monkeys: &HashMap<String, Monkey>, monkey: &str) -> Result<i64, ()> {
    if monkey == "humn" {
        return Err(());
    }
    let monkey = monkeys.get(monkey).unwrap();
    Ok(match monkey {
        Monkey::Number(i) => *i,
        Monkey::Operation(op, left, right) => {
            op.apply(eval_2(monkeys, left)?, eval_2(monkeys, right)?)
        }
    })
}
fn derive(monkeys: &HashMap<String, Monkey>, monkey_name: &str, target: i64) -> i64 {
    if monkey_name == "humn" {
        return target;
    }
    let monkey = monkeys.get(monkey_name).unwrap();
    if let Monkey::Operation(op, left, right) = monkey {
        let left_val = eval_2(&monkeys, left);
        let right_val = eval_2(&monkeys, right);
        if left_val.is_ok() {
            // left_val ?? X = target
            let c = left_val.unwrap();
            let new_target = match op {
                Op::Add => target - c,
                Op::Subtract => c - target,
                Op::Multiply => target / c,
                Op::Divide => c / target,
            };
            derive(monkeys, right, new_target)
        } else {
            // X ?? right_val = target
            let c = right_val.unwrap();
            let new_target = match op {
                Op::Add => target - c,
                Op::Subtract => target + c,
                Op::Multiply => target / c,
                Op::Divide => target * c,
            };
            derive(monkeys, left, new_target)
        }
    } else {
        panic!("Trying to derive a constant, non-humn monkey")
    }
}
fn solve_2() -> i64 {
    let monkeys = read_monkeys();
    let root = monkeys.get("root").unwrap();
    if let Monkey::Operation(_, left, right) = root {
        let left_val = eval_2(&monkeys, left);
        let right_val = eval_2(&monkeys, right);
        let target;
        let human_branch;
        if left_val.is_ok() {
            target = left_val.unwrap();
            human_branch = right;
        } else {
            target = right_val.unwrap();
            human_branch = left;
        }
        derive(&monkeys, human_branch, target)
    } else {
        panic!()
    }
}
pub fn solve() {
    println!("{}", solve_1());
    println!("{}", solve_2());
}
