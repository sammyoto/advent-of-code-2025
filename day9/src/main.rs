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
    let puzzle_input = load_puzzle_input("src/test.txt");
    let puzzle_input_len = puzzle_input.len();
    let mut largest_area: usize = 0;

    for i in 0..puzzle_input_len {
        for j in i + 1..puzzle_input_len {
            let tile_1 = &puzzle_input[i];
            let tile_2 = &puzzle_input[j];

            if check_valid_rectangle(tile_1[0], tile_1[1], tile_2[0], tile_2[1], &puzzle_input) {
                let x_len = tile_1[0].abs_diff(tile_2[0]) + 1;
                let y_len = tile_1[1].abs_diff(tile_2[1]) + 1;
                let area = x_len * y_len;

                println!("x1: {}, x2: {}, y1: {}, y2: {}, area: {}", tile_1[0], tile_2[0], tile_1[1], tile_2[1], area);

                if area > largest_area {
                    largest_area = area;
                }
            }
        }
    }

    largest_area
}

fn check_valid_rectangle(x_1: usize, y_1: usize, x_2: usize, y_2: usize, puzzle_input: &Vec<Vec<usize>>) -> bool {
    // Follow puzzle input and create loops 
    for i in 0..puzzle_input.len() {
        let tile: &Vec<usize> = &puzzle_input[i];
        
        // Check if the tile is inside the area of the potential rectangle, if it is its invalid
        if x_1 >= x_2 {
            if y_1 >= y_2 {
                if tile[0] > x_2 && tile[0] < x_1 && tile[1] > y_2 && tile[1] < y_1 {
                    return false
                } else {
                    continue
                }
            } else {
                if tile[0] > x_2 && tile[0] < x_1 && tile[1] > y_1 && tile[1] < y_2 {
                    return false
                } else {
                    continue
                }
            }
        } else {
            if y_1 >= y_2 {
                if tile[0] > x_1 && tile[0] < x_2 && tile[1] > y_2 && tile[1] < y_1 {
                    return false
                } else {
                    continue
                }
            } else {
                if tile[0] > x_1 && tile[0] < x_2 && tile[1] > y_1 && tile[1] < y_2 {
                    return false
                } else {
                    continue
                }
            }
        }
    }

    true
}