use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = load_puzzle_input("src/test.txt");
    let cephalopod_input = load_cephalopod_input("src/test.txt");
    println!("Normal Input: {:?}", input.0);
    println!("Cephalopod Input: {:?}", cephalopod_input.0);

    let total = sum_all_functions(&input.0, &input.1);
    println!("{}", total);
}

fn load_puzzle_input(filename: &str) -> (Vec<Vec<u64>>, Vec<char>) {
    let mut numbers: Vec<Vec<u64>> = Vec::new();
    let mut chars: Vec<char> = Vec::new();

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().peekable();

    while let Some(line) = lines.next() {
        let is_last = lines.peek().is_none();

        if is_last {
            for ch in line.unwrap().split_whitespace() {
                let value: char = ch.parse().unwrap();
                chars.push(value);
            }
        } else {
            let mut row = Vec::new();
            for num in line.unwrap().split_whitespace() {
                let value: u64 = num.parse().unwrap();
                row.push(value);
            }
            numbers.push(row);
        }
    }
    (numbers, chars)
}

// TODO
fn load_cephalopod_input(filename: &str) -> (Vec<Vec<String>>, Vec<char>) {
    let mut numbers: Vec<Vec<String>> = Vec::new();
    let mut chars: Vec<char> = Vec::new();

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().peekable();

    while let Some(line) = lines.next() {
        let is_last = lines.peek().is_none();

        if is_last {
            for ch in line.unwrap().split_whitespace() {
                let value: char = ch.parse().unwrap();
                chars.push(value);
            }
        } else {
            let mut row = Vec::new();
            for str in line.unwrap().split_whitespace() {
                let value: String = str.parse().unwrap();
                row.push(value);
            }
            numbers.push(row);
        }
    }
    (numbers, chars)
}

fn sum_all_functions(numbers: &Vec<Vec<u64>>, operators: &Vec<char>) -> u64 {
    let mut total: u64 = 0;

    // Col
    for i in 0..numbers[0].len() {
        let mut local_total = numbers[0][i];
        // Row
        for j in 1..numbers.len() {
            if operators[i] == '*' {
                local_total *= numbers[j][i];
            } else {
                local_total += numbers[j][i];
            }
        }
        total += local_total;
    }

    total
}

fn cephalopod_sum(numbers: &Vec<Vec<String>>, operators: &Vec<char>) -> u64 {
    let mut total: u64 = 0;

    total
}