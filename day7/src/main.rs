use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap};

fn main() {
    let tachyon_manifold = load_puzzle_input("src/puzzle_input.txt");
    let beam_splits = count_tachyon_beam_splits(&tachyon_manifold);
    let quantum_timelines = count_quantum_timelines(&tachyon_manifold);
    println!("Beam Splits: {}",beam_splits);
    println!("Quantum Timelines: {}", quantum_timelines);
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

fn count_tachyon_beam_splits(tachyon_manifold: &Vec<Vec<char>>) -> usize{
    let mut beam_splits: usize = 0;
    let mut beam_indicies: HashSet<usize> = HashSet::new();

    // Push index of beam starting position
    beam_indicies.insert(tachyon_manifold[0].iter().position(|&r| r == 'S').unwrap());

    // Row
    for i in 1..tachyon_manifold.len() {
        // Col
        for j in 0..tachyon_manifold[0].len() {
            let curr_element: char = tachyon_manifold[i][j];

            // If the curr element is a splitter
            if curr_element == '^' {
                // If there is a beam indicie  at this splitter
                if beam_indicies.contains(&j) {
                    // Remove the index, add two indexes for + and - 1 of the index
                    beam_indicies.remove(&j);
                    beam_indicies.insert(j+1);
                    beam_indicies.insert(j-1);

                    // Increment beam split counter
                    beam_splits += 1;
                }
            }
        }
    }

    beam_splits
}

fn count_quantum_timelines(tachyon_manifold: &Vec<Vec<char>>) -> usize {
    let mut timelines: usize = 1;

    let mut timeline_indicies: HashMap<usize, usize> = HashMap::new();

    // Push index of beam starting position
    timeline_indicies.insert(tachyon_manifold[0].iter().position(|&r| r == 'S').unwrap(), 1);

    // Row
    for i in 1..tachyon_manifold.len() {
        // Col
        for j in 0..tachyon_manifold[0].len() {
            let curr_element: char = tachyon_manifold[i][j];

            // If the curr element is a splitter
            if curr_element == '^' {
                // If there is a beam indicie  at this splitter
                if timeline_indicies.contains_key(&j) {
                    // Count how many timelines are at the current index
                    let index_count = *timeline_indicies.get(&j).unwrap();
                    // println!("Row: {}", i);
                    // println!("Col: {}", j);
                    // println!("Index Count: {}", index_count);
                    // println!("Timeline Indicies: {:?}", timeline_indicies);
                    // Remove the index, add two indexes for + and - 1 of the index
                    timeline_indicies.remove(&j);

                    let left = timeline_indicies.entry(j - 1).or_insert(0);
                    *left += index_count;
                    let right = timeline_indicies.entry(j + 1).or_insert(0);
                    *right += index_count;
                    
                    // Increment beam split counter
                    timelines += index_count;
                }
            }
        }


    }

    timelines
}
