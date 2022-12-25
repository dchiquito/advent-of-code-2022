use crate::advent;

type SNAFU = String;

fn to_int(snafu: &SNAFU) -> i64 {
    let mut total = 0;
    let mut place = 1;
    for c in snafu.chars().rev() {
        total += match c {
            '2' => place * 2,
            '1' => place,
            '0' => 0,
            '-' => -place,
            '=' => -(place * 2),
            _ => panic!(),
        };
        place *= 5;
    }
    // let p = (i as f64).log(5) as i64;
    total
}

fn to_snafu(mut i: i64) -> SNAFU {
    if i == 0 {
        return "0".to_string();
    }
    let exp = (i as f64).log(5.0) as u32;
    let mut place = (5 as i64).pow(exp);
    let mut snafu = "".to_string();
    while place > 1 {
        let c;
        if i > place + (place / 2) {
            c = '2';
            i -= place * 2;
        } else if i > (place / 2) {
            c = '1';
            i -= place;
        } else if i > -(place / 2) {
            c = '0';
        } else if i > -place - (place / 2) {
            c = '-';
            i += place;
        } else {
            c = '=';
            i += place * 2;
        }
        snafu.push(c);
        place /= 5;
    }
    if i == 2 {
        snafu.push('2');
    } else if i == 1 {
        snafu.push('1');
    } else if i == 0 {
        snafu.push('0');
    } else if i == -1 {
        snafu.push('-');
    } else {
        snafu.push('=');
    }

    snafu
}

fn solve_1() -> SNAFU {
    let lines = advent::read_input(25);
    let sum = lines.iter().map(to_int).fold(0, |a, b| a + b);
    to_snafu(sum)
}
pub fn solve() {
    println!("{}", solve_1());
}
