use std::fs;

fn main() {
    load_puzzle_input();
}

fn load_puzzle_input() -> Vec<String>{
    let data = fs::read_to_string("src/puzzle-input.csv")
        .expect("could not read file");

    let values: Vec<String> = data
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    values
}