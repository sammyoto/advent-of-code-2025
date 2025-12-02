use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/safe_rotation_sequence.txt";
    let lines = load_safe_rotation_sequence(filename);
    let num_zeros = decode_safe_rotation_sequence(&lines);
    println!("Num Zeros: {}", num_zeros);

    let curr_num = 0;
    let turn_size = 1;
    let direction = 'L';
    let new_nums = move_dial(&curr_num, &turn_size, direction);
    println!("Num Zeros: {}, New Num: {}", new_nums.0, new_nums.1);
}

fn load_safe_rotation_sequence(filename: &str) -> Vec<String>{
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .filter_map(Result::ok) // remove .lines() errors
        .collect();

    lines
}

fn decode_safe_rotation_sequence(sequence: &Vec<String>) -> i16 {
    let mut curr_num: i16 = 50;
    let mut zero_count: i16 = 0;

    for input in sequence {
        let direction: char = input.chars().nth(0).unwrap();
        let turn_size: i16 = input[1..].parse::<i16>().unwrap();

        let new_nums: (i16, i16) = move_dial(&curr_num, &turn_size, direction);
        zero_count += new_nums.0;
        curr_num = new_nums.1;
    }

    zero_count
}

fn move_dial(curr_num: &i16, turn_size: &i16, direction: char) -> (i16, i16){
    let mut zero_start: bool = false;
    let mut double_count: bool = false;
    let mut new_num: i16;
    let mut num_zeros: i16 = turn_size/100;

    if *curr_num == 0 {
        zero_start = true;
    }

    if direction == 'L' {
        new_num = curr_num - (turn_size % 100);
        if new_num < 0 {
            new_num = 100 + new_num;

            if !zero_start {
                num_zeros += 1;
            }

            if new_num == 0 {
                double_count = true;
            }
        }
    } else {
        new_num = curr_num + (turn_size % 100);
        if new_num > 99 {
            new_num = new_num - 100;
            num_zeros += 1;

            if new_num == 0 {
                double_count = true;
            }
        }
    }

    if new_num == 0 && !double_count {
        num_zeros += 1;
    }

    (num_zeros, new_num)
}