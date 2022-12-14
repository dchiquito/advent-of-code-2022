use crate::advent;
use regex::Regex;
use std::{cmp::Ordering, fmt::Debug};
use substring::{self, Substring};

#[derive(Clone, PartialEq, Eq)]
enum Expr {
    Int(i32),
    List(Vec<Expr>),
}

impl PartialOrd for Expr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Expr::Int(s) => match other {
                Expr::Int(o) => Some(s.cmp(o)),
                Expr::List(_o) => Expr::List(vec![self.clone()]).partial_cmp(&other.clone()),
            },
            Expr::List(s) => match other {
                Expr::Int(_o) => self.partial_cmp(&Expr::List(vec![other.clone()])),
                Expr::List(o) => {
                    for (ss, oo) in s.iter().zip(o.iter()) {
                        let cmp = ss.partial_cmp(oo);
                        if cmp != Some(Ordering::Equal) {
                            return cmp;
                        }
                    }
                    Some(s.len().cmp(&o.len()))
                }
            },
        }
    }
}

impl Ord for Expr {
    fn cmp(&self, other: &Self) -> Ordering {
        if let Some(ord) = self.partial_cmp(other) {
            ord
        } else {
            panic!("its only partial")
        }
    }
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Int(i) => fmt.write_fmt(format_args!("{}", i)),
            Expr::List(v) => fmt.write_fmt(format_args!("{:?}", v)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Int(i32),
    LParen,
    RParen,
}

impl Expr {
    fn tokenize(line: &str) -> (Token, String) {
        // dirty hack
        if line.substring(0, 1) == "," {
            return Expr::tokenize(line.substring(1, line.len()));
        }
        let int_re = Regex::new(r"(\d+),?").unwrap();
        match line.substring(0, 1) {
            "[" => (Token::LParen, line.substring(1, line.len()).to_string()),
            "]" => (Token::RParen, line.substring(1, line.len()).to_string()),
            _ => {
                let captures = int_re.captures(line).unwrap();
                (
                    Token::Int((&captures[1]).parse().unwrap()),
                    line.substring((&captures[0]).len(), line.len()).to_string(),
                )
            }
        }
    }
    fn parse(line: &str) -> (Expr, String) {
        let (token, line) = Expr::tokenize(line);
        match token {
            Token::LParen => Expr::parse_list(&line),
            Token::Int(i) => (Expr::Int(i), line),
            _ => panic!("Invalid syntax"),
        }
    }
    fn parse_list(line: &str) -> (Expr, String) {
        let (mut token, mut line) = Expr::tokenize(line);
        let mut list: Vec<Expr> = vec![];
        while token != Token::RParen {
            list.push(match token {
                Token::LParen => {
                    let expr: Expr;
                    (expr, line) = Expr::parse_list(&line);
                    expr
                }
                Token::RParen => panic!("Loop should already be over"),
                Token::Int(i) => Expr::Int(i),
            });
            (token, line) = Expr::tokenize(&line);
        }
        (Expr::List(list), line)
    }
}

fn solve_1() {
    let mut lines = advent::read_input(13).map(Result::unwrap);
    let mut index = 1;
    let mut sum = 0;
    loop {
        let l1 = lines.next().unwrap();
        let l2 = lines.next().unwrap();
        let e1 = Expr::parse(&l1).0;
        let e2 = Expr::parse(&l2).0;
        if e1.cmp(&e2) == Ordering::Less {
            sum += index;
        }
        index += 1;
        if lines.next().is_none() {
            break;
        }
    }
    println!("{}", sum);
}

fn solve_2() {
    let mut lines = advent::read_input(13).map(Result::unwrap);
    let mut exprs = vec![];
    loop {
        let l1 = lines.next().unwrap();
        let l2 = lines.next().unwrap();
        let e1 = Expr::parse(&l1).0;
        let e2 = Expr::parse(&l2).0;
        exprs.push(e1);
        exprs.push(e2);
        if lines.next().is_none() {
            break;
        }
    }
    let two = Expr::parse("[[2]]").0;
    let six = Expr::parse("[[6]]").0;
    exprs.push(two.clone());
    exprs.push(six.clone());

    exprs.sort();

    let mut a = 0;
    let mut b = 0;
    for i in 0..exprs.len() {
        if exprs[i] == two {
            a = i + 1;
        }
        if exprs[i] == six {
            b = i + 1;
        }
    }
    println!("{}", a * b);
}

pub fn solve() {
    solve_1();
    solve_2();
}
