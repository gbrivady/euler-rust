use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("input/0018");
    let mut file = match File::open(&path) {
        Err(e) => panic!("couldn't open {}: {}", path.display(), e),
        Ok(file) => file,
    };
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    let pyramid: Vec<Vec<u32>> = s
        .split("\n")
        .map(|line| line.split(" ").map(|x| x.parse::<u32>().unwrap()).collect())
        .collect();

    // Start at the bottom and work my way up

    let mut dist_to_bottom: Vec<u32> = vec![0; 100];

    for level in pyramid.iter().rev() {
        for i in 0..level.len() {
            dist_to_bottom[i] = level[i] + std::cmp::max(dist_to_bottom[i], dist_to_bottom[i + 1]);
        }
    }
    print!("{}", dist_to_bottom[0]);
}
