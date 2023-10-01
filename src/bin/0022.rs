use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("input/0022");
    let mut file = match File::open(&path) {
        Err(e) => panic!("couldn't open {}: {}", path.display(), e),
        Ok(file) => file,
    };
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let mut names_only: Vec<&str> = s
        .split(",")
        .map(|line| &line[1..line.len() - 1])
        .collect::<Vec<&str>>();
    names_only.sort();
    let result = names_only.iter().enumerate().fold(0u32, |acc, (i, name)| {
        name.chars().fold(0u32, |acc, c| (c as u32 - 64) + acc) * (i as u32 + 1) + acc
    });

    println!("{}", result);
}
