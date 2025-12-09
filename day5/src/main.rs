use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let fresh_id_count = sum_fresh_ids();
    println!("Fresh IDs: {}", fresh_id_count);
    let total_valid_fresh_id_count = get_total_fresh_ids();
    println!("Total Valid Fresh IDs: {}", total_valid_fresh_id_count);
}

fn load_puzzle_input(filename: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Vars for reading
    let mut fresh_id_ranges: Vec<(u64, u64)> = Vec::new();
    let mut ingredient_ids: Vec<u64> = Vec::new();

    // Read each line, if it has a - it's a range, else it's an ingredient ID
    for line in reader.lines() {
        let line = line.unwrap();

        if let Some((left, right)) = line.split_once('-') {
            let lower: u64 = left.parse().unwrap();
            let upper: u64 = right.parse().unwrap();
            fresh_id_ranges.push((lower, upper));
        } else if line.trim().is_empty() {
            continue;
        } else {
            let id: u64 = line.parse().unwrap();
            ingredient_ids.push(id);
        }
    }

    (fresh_id_ranges, ingredient_ids)
}

fn is_id_fresh(id: u64, fresh_ranges: &Vec<(u64, u64)>) -> bool{
    for range in fresh_ranges {
        if id >= range.0 && id <= range.1 {
            return true;
        }
    }

    false
}

fn sum_fresh_ids() -> u64 {
    // Get data and init counter
    let mut fresh_id_count: u64 = 0;
    let inputs = load_puzzle_input("src/puzzle_input.txt");

    let fresh_id_ranges = inputs.0;
    let ingredient_ids = inputs.1;

    for id in ingredient_ids {
        if is_id_fresh(id, &fresh_id_ranges) {
            fresh_id_count += 1;
        }
    }

    fresh_id_count
}

fn get_total_fresh_ids() -> u64 {
    // Get data and init counter
    let mut inputs = load_puzzle_input("src/puzzle_input.txt");
    let fresh_id_ranges = consolidate_ranges(&mut inputs.0);
    let mut fresh_id_count = 0;

    for range in fresh_id_ranges {
        fresh_id_count += range.1 - range.0 + 1;
    }

    fresh_id_count
}

fn consolidate_ranges(ranges: &mut Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    ranges.sort_by_key(|r| r.0);

    let mut merged = vec![ranges[0]];

    for &(start, end) in &ranges[1..] {
        let last = merged.last_mut().unwrap();

        if start <= last.1 {
            // overlap: merge
            last.1 = last.1.max(end);
        } else {
            // no overlap: push
            merged.push((start, end));
        }
    }

    merged
}