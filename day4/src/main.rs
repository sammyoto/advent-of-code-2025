use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let puzzle_input: Vec<Vec<char>> = load_puzzle_input("src/test.txt");
    println!("{:?}", puzzle_input[0]);
}

fn load_puzzle_input(filename: &str) -> Vec<Vec<char>>{
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut puzzle_inputs: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let mut puzzle_input: Vec<char> = Vec::new();
        for ch in line.unwrap().chars() {
            puzzle_input.push(ch);
        }

        puzzle_inputs.push(puzzle_input);
    }

    puzzle_inputs
}

fn count_rolls_accessable(paper_roll_grid: &Vec<Vec<char>>, adjacent_roll_threshold: usize) {
    
}
