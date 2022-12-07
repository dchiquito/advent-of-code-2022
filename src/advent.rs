use reqwest::{blocking::Client};
use std::{fs::File, io::{Read, Write, BufReader, BufRead, Lines}};

fn input_file_path(day:i32) -> String {
    format!("inputs/{}.txt", day)
}

fn get_secret() -> String {
    let mut f = File::open(".cookie").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    buffer
}

fn write_input_file(day: i32, buffer: &str) {
    let mut f = File::create(input_file_path(day)).unwrap();
    f.write_all(buffer.as_bytes()).unwrap();
}

fn download_input(day: i32) {
    let secret = get_secret();
    let client = Client::new();
    let text = client
        .get(format!("https://adventofcode.com/2022/day/{}/input", day))
        .header("Cookie", format!("session={}", secret)).send().unwrap()
        .text().unwrap();
    write_input_file(day, &text);
}

pub fn read_input(day: i32) -> Lines<BufReader<File>> {
    if !std::path::Path::new(&input_file_path(day)).exists() {
        download_input(day);
    }
    let f = File::open(input_file_path(day)).unwrap();
    BufReader::new(f).lines()
}