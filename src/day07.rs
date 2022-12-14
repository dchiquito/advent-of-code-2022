use crate::advent;
use regex::Regex;
use substring::Substring;

#[derive(Debug)]
struct Dir {
    name: String,
    files: Vec<File>,
    dirs: Vec<Dir>,
    file_size: u32,
}

impl Dir {
    fn new(name: &str) -> Dir {
        Dir {
            name: name.to_string(),
            files: vec![],
            dirs: vec![],
            file_size: 0,
        }
    }
    fn lookup(&mut self, path: &Vec<String>) -> &mut Dir {
        if path.is_empty() {
            return self;
        }
        let mut path = path.clone();
        let dirname = path.remove(0);
        for dir in self.dirs.iter_mut() {
            if dir.name == dirname {
                return dir.lookup(&path);
            }
        }
        panic!("File not found")
    }
    fn add_file(&mut self, file: File) {
        self.file_size += file.size;
        self.files.push(file);
    }
    fn add_dir(&mut self, dir: Dir) {
        self.dirs.push(dir);
    }
    fn size(&self) -> u32 {
        let mut size = self.file_size;
        for dir in self.dirs.iter() {
            size += dir.size();
        }
        size
    }
}

#[derive(Debug)]
struct File {
    #[allow(dead_code)]
    name: String,
    size: u32,
}

impl File {
    fn new(name: &str, size: u32) -> File {
        File {
            name: name.to_string(),
            size,
        }
    }
}

fn build_fs() -> Dir {
    let file_re = Regex::new(r"^([0-9]+) ([a-z\.]+)$").unwrap();
    let dir_re = Regex::new(r"^dir ([a-z]+)$").unwrap();
    let lines = advent::read_input(7);
    let mut iter = lines.iter();
    let mut root = Dir::new("/");
    let mut path: Vec<String> = vec![];
    while let Some(line) = iter.next() {
        match line.as_str() {
            "$ cd .." => {
                path.pop();
            }
            "$ cd /" => {
                path = vec![];
            }
            "$ ls" => {
                // we don't actually need to do anything here lol
            }
            _ => {
                if line.starts_with("$ cd ") {
                    let dir = line.substring(5, line.len());
                    path.push(dir.to_string());
                } else {
                    let dir = root.lookup(&path);
                    if let Some(captures) = file_re.captures(&line) {
                        let size = str::parse(&captures[1]).unwrap();
                        let name = &captures[2];
                        dir.add_file(File::new(name, size));
                    } else if let Some(captures) = dir_re.captures(&line) {
                        let name = &captures[1];
                        dir.add_dir(Dir::new(name));
                    }
                    // println!("NEEW {:?}", dir);
                }
            }
        };
    }
    root
}

fn solve_1(dir: &Dir) -> u32 {
    let mut total_return = 0;
    if dir.size() < 100000 {
        total_return += dir.size();
    }
    for subdir in dir.dirs.iter() {
        total_return += solve_1(subdir);
    }
    total_return
}

fn _walk_2(dir: &Dir, min_dir_size: u32) -> Option<u32> {
    let mut smallest_dir_size = dir.size();
    if smallest_dir_size < min_dir_size {
        return None;
    }
    for subdir in dir.dirs.iter() {
        if let Some(subdir_size) = _walk_2(subdir, min_dir_size) {
            if subdir_size > min_dir_size && subdir_size < smallest_dir_size {
                smallest_dir_size = subdir_size;
            }
        }
    }
    Some(smallest_dir_size)
}

fn solve_2(root: &Dir) {
    const TOTAL_SIZE: u32 = 70000000;
    const REQUIRED_FREE_SPACE: u32 = 30000000;
    let min_dir_size = root.size() - (TOTAL_SIZE - REQUIRED_FREE_SPACE);
    println!("{}", _walk_2(root, min_dir_size).unwrap());
}

pub fn solve() {
    let root = build_fs();
    println!("{}", solve_1(&root));
    solve_2(&root);
}
