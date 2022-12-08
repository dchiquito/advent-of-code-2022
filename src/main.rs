mod advent;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut day = 1;
    if args.len() != 2 {
        println!("Assuming day {}", day);
    } else {
        day = str::parse(args.get(1).unwrap()).unwrap()
    }
    match day {
        1 => day1::solve(),
        2 => day2::solve(),
        3 => day3::solve(),
        4 => day4::solve(),
        5 => day5::solve(),
        6 => day6::solve(),
        _ => println!("That day isn't solved yet"),
    }
}
