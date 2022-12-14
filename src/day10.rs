use crate::advent;
use substring::Substring;

fn run_cpu() -> Vec<i32> {
    let lines = advent::read_input(10);
    let mut x = 1;
    let mut values = vec![];
    for line in lines.iter() {
        if line == "noop" {
            values.push(x);
        } else {
            values.push(x);
            values.push(x);
            x += line.substring(5, 100).parse::<i32>().unwrap();
        }
    }
    values
}

fn print_crt(values: Vec<i32>) {
    let mut x = 0;
    for v in values.iter() {
        if *v == x - 1 || *v == x || *v == x + 1 {
            print!("#");
        } else {
            print!(".");
        }
        x += 1;
        if x >= 40 {
            x = 0;
            println!();
        }
    }
}

pub fn solve() {
    let values = run_cpu();
    println!(
        "{}",
        (20 * values[20 - 1])
            + (60 * values[60 - 1])
            + (100 * values[100 - 1])
            + (140 * values[140 - 1])
            + (180 * values[180 - 1])
            + (220 * values[220 - 1])
    );

    print_crt(values);
}
