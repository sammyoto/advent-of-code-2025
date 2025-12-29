use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let largest_area = find_largest_rectangle();
    println!("Largest Area: {}", largest_area);
    let largest_red_green_area = find_largest_red_green_rectangle();
    println!("Largest Red/Green Area: {}", largest_red_green_area);
}

fn load_puzzle_input(filename: &str) -> Vec<Vec<usize>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut tile_coordinates: Vec<Vec<usize>> = Vec::new();

    for line in reader.lines() {
        let line_split: Vec<usize> = line.unwrap().split(',').map(|x| x.parse::<usize>().unwrap()).collect();
        tile_coordinates.push(line_split);
    }

    tile_coordinates
}

fn find_largest_rectangle() -> usize {
    let puzzle_input = load_puzzle_input("src/puzzle_input.txt");
    let mut largest_area: usize = 0;

    for i in 0..puzzle_input.len() {
        for j in i + 1..puzzle_input.len() {
            let x_len = puzzle_input[i][0].abs_diff(puzzle_input[j][0]) + 1;
            let y_len = puzzle_input[i][1].abs_diff(puzzle_input[j][1]) + 1;
            let area = x_len * y_len;

            if area > largest_area {
                largest_area = area;
            }
        }
    }

    largest_area
}

fn find_largest_red_green_rectangle() -> usize {
    let puzzle_input = load_puzzle_input("src/puzzle_input.txt");
    let mut largest_area: usize = 0;

    largest_area
}