use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = load_puzzle_input("src/puzzle_input.txt");
    let cephalopod_input = load_cephalopod_input("src/puzzle_input.txt");

    let total = sum_all_functions(&input.0, &input.1);
    println!("Normal Total: {}", total);

    let cephalopod_total = cephalopod_sum(&mut load_cephalopod_input("src/puzzle_input.txt"));
    println!("Cephalopod Total: {}", cephalopod_total);
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
fn load_cephalopod_input(filename: &str) -> Vec<Vec<char>> {
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

fn cephalopod_sum(numbers: &mut Vec<Vec<char>>) -> u64 {
    let mut total: u64 = 0;

    // Remove operators from numbers
    let mut operators =  numbers.remove(numbers.len() - 1);
    // Remove spaces from operators
    operators.retain(|x| *x != ' ');

    // Loop through all rows
    // Find columns with only ' '
    // Record those indices
    let mut separation_indices: Vec<usize> = Vec::new();
    // Col
    for i in 0..numbers[0].len() {
        let mut is_separation_column = true;
        // Row
        for j in 0..numbers.len() {
            if numbers[j][i] != ' ' {
                is_separation_column = false;
                break;
            }
        }

        if is_separation_column {
            separation_indices.push(i);
        }
    }

    // Now we know where it's separated, we can now construct numbers
    for i in 0..operators.len() {
        let operator = operators[i];
        let mut local_sum: u64 = 0;
        // Start at 0 and end at 1st separation indicie
        if i == 0 {
            for j in 0..separation_indices[0] {
                let mut column: Vec<char> = numbers.iter().map(|row| row[j]).collect();
                column.retain(|x| *x != ' ');
                let num = get_number_from_vector(&column);

                if j == 0 {
                    local_sum = num;
                } else if operator == '*' {
                    local_sum *= num;
                } else {
                    local_sum += num;
                }
            }
        // Start at last separation indicie and end at end of numbers
        } else if i == operators.len() - 1 {
            for j in separation_indices[separation_indices.len() - 1] + 1..numbers[0].len() {
                let mut column: Vec<char> = numbers.iter().map(|row| row[j]).collect();
                column.retain(|x| *x != ' ');
                let num = get_number_from_vector(&column);


                if j == separation_indices[separation_indices.len() - 1] + 1 {
                    local_sum = num;
                } else if operator == '*' {
                    local_sum *= num;
                } else {
                    local_sum += num;
                }
            }

        // Start at i - 1 separation indicie and end at i separation indicie
        } else {
            for j in separation_indices[i - 1] + 1..separation_indices[i] {
                let mut column: Vec<char> = numbers.iter().map(|row| row[j]).collect();
                column.retain(|x| *x != ' ');
                let num: u64 = get_number_from_vector(&column);

                if j == separation_indices[i - 1] + 1 {
                    local_sum = num;
                } else if operator == '*' {
                    local_sum *= num;
                } else {
                    local_sum += num;
                }
            }
        }

        total += local_sum;
    }

    total
}

fn get_number_from_vector(number_vec: &Vec<char>) -> u64{
    let mut number: u64 = 0;

    for i in 0..number_vec.len() {
        let digit: u64 = number_vec[i].to_digit(10).unwrap() as u64;
        let digit = digit * (10_u64.pow((number_vec.len() - i - 1) as u32));
        number += digit;
        
    }

    number
}