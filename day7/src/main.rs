use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let tachyon_manifold = load_puzzle_input("src/test.txt");
    println!("{:?}", tachyon_manifold[0]);
}

fn load_puzzle_input(filename: &str) -> Vec<Vec<char>> {
    let mut rows: Vec<Vec<char>> = Vec::new();

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    for line in lines {
        let mut chars: Vec<char> = Vec::new();
        for ch in line.unwrap().chars() {
            let value: char = ch;
            chars.push(value);
        }
        rows.push(chars);
    }

    rows
}
