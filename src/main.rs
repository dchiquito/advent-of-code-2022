mod advent;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut day = 1;
    if args.len() != 2 {
        println!("Assuming day {}", day);
    } else {
        day = str::parse(args.get(1).unwrap()).unwrap()
    }
    match day {
        1 => day01::solve(),
        2 => day02::solve(),
        3 => day03::solve(),
        4 => day04::solve(),
        5 => day05::solve(),
        6 => day06::solve(),
        7 => day07::solve(),
        8 => day08::solve(),
        9 => day09::solve(),
        10 => day10::solve(),
        11 => day11::solve(),
        _ => println!("That day isn't solved yet"),
    }
}
