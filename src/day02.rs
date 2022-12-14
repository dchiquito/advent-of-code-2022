use regex::Regex;

use crate::advent;

#[derive(Debug)]
enum ABC {
    A,
    B,
    C,
}
#[derive(Debug)]
enum XYZ {
    X,
    Y,
    Z,
}
#[derive(Debug)]
struct Row {
    l: ABC,
    r: XYZ,
}
impl Row {
    fn parse(line: &str) -> Row {
        let re: Regex = Regex::new(r"^([A-C]) ([X-Z])$").unwrap();
        let captures = re.captures(&line).unwrap();
        let abc = &captures[1];
        let xyz = &captures[2];
        let l = match abc {
            "A" => ABC::A,
            "B" => ABC::B,
            "C" => ABC::C,
            _ => panic!(),
        };
        let r = match xyz {
            "X" => XYZ::X,
            "Y" => XYZ::Y,
            "Z" => XYZ::Z,
            _ => panic!(),
        };
        Row { l, r }
    }
    fn score_1(&self) -> i32 {
        match &self.r {
            XYZ::X => {
                1 + match &self.l {
                    ABC::A => 3,
                    ABC::B => 0,
                    ABC::C => 6,
                }
            }
            XYZ::Y => {
                2 + match &self.l {
                    ABC::A => 6,
                    ABC::B => 3,
                    ABC::C => 0,
                }
            }
            XYZ::Z => {
                3 + match &self.l {
                    ABC::A => 0,
                    ABC::B => 6,
                    ABC::C => 3,
                }
            }
        }
    }
    fn score_2(&self) -> i32 {
        match &self.l {
            ABC::A => match &self.r {
                XYZ::X => 0 + 3,
                XYZ::Y => 3 + 1,
                XYZ::Z => 6 + 2,
            },
            ABC::B => match &self.r {
                XYZ::X => 0 + 1,
                XYZ::Y => 3 + 2,
                XYZ::Z => 6 + 3,
            },
            ABC::C => match &self.r {
                XYZ::X => 0 + 2,
                XYZ::Y => 3 + 3,
                XYZ::Z => 6 + 1,
            },
        }
    }
}

pub fn solve() {
    let reader = advent::read_input(2);
    let rows: Vec<Row> = reader.iter().map(|line| Row::parse(line)).collect();

    // Part 1
    let sum_1 = rows.iter().fold(0, |acc, row| acc + row.score_1());
    println!("{}", sum_1);

    // Part 2
    let sum_2 = rows.iter().fold(0, |acc, row| acc + row.score_2());
    println!("{}", sum_2);
}
