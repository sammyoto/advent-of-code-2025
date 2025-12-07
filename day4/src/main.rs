use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let rolls_accessable: usize = count_rolls_with_removal(4);
    println!("{}", rolls_accessable);
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

fn count_rolls_accessable(paper_roll_grid: &Vec<Vec<char>>, new_paper_roll_grid: &mut Vec<Vec<char>>, adjacent_roll_threshold: usize) -> usize{
    // Loop through the grid
    let mut roll_accessable_count: usize = 0;
    let col_max: usize = paper_roll_grid.len();
    let row_max: usize = paper_roll_grid[0].len();
    
    // ROW
    for i in 0..col_max {
        // COL
        for j in 0..row_max {
            let grid_location = paper_roll_grid[i][j];
            // Don't care about locations with no paper roll
            if grid_location == '@' {
                let mut local_paper_roll_count: usize = 0;
                // Check for all directions once, store in a bool
                // Then check up middle down, check left and right in up middle down
                let mut has_left = true;
                let mut has_right = true;
                let mut has_up = true;
                let mut has_down = true;
               
                if j == 0 {
                    has_left = false;
                }
                if j == col_max -1 {
                    has_right = false;
                }
                if i == 0 {
                    has_up = false;
                }
                if i == row_max - 1 {
                    has_down = false;
                }
                
                // Now check all possible locations
                // Up
                if has_up {
                    // Up
                    if paper_roll_grid[i-1][j] == '@' {
                        local_paper_roll_count += 1;
                    }
                    // Upper left
                    if has_left {
                        if paper_roll_grid[i-1][j-1] == '@' {
                            local_paper_roll_count += 1;
                        }
                    }
                    // Upper right
                    if has_right {
                        if paper_roll_grid[i-1][j+1] == '@' {
                            local_paper_roll_count += 1;
                        }
                    }
                }
                // Middle
                if has_left {
                    if paper_roll_grid[i][j-1] == '@' {
                        local_paper_roll_count += 1;
                    }
                }
                if has_right {
                    if paper_roll_grid[i][j+1] == '@' {
                        local_paper_roll_count += 1;
                    }
                }
                // Down
                if has_down {
                    // Down
                    if paper_roll_grid[i+1][j] == '@' {
                        local_paper_roll_count += 1;
                    }
                    // Down left
                    if has_left {
                        if paper_roll_grid[i+1][j-1] == '@' {
                            local_paper_roll_count += 1;
                        }
                    }
                    // Down right
                    if has_right {
                        if paper_roll_grid[i+1][j+1] == '@' {
                            local_paper_roll_count += 1;
                        }
                    }
                }
                if local_paper_roll_count < adjacent_roll_threshold {
                    roll_accessable_count += 1;
                    new_paper_roll_grid[i][j] = '.';
                }
            }
        }
    }

    roll_accessable_count
}

fn count_rolls_with_removal(adjacent_roll_threshold: usize) -> usize{
    // Load paper roll grids
    let mut starting_paper_roll_grid = load_puzzle_input("src/puzzle_input.txt");
    let mut new_paper_roll_grid = starting_paper_roll_grid.clone();

    let mut total_roll_accessable_count: usize = 0;

    loop {
        let roll_accessable_count = count_rolls_accessable(&starting_paper_roll_grid, &mut new_paper_roll_grid, adjacent_roll_threshold);

        // If no rolls accessable, finish
        if roll_accessable_count == 0 {
            break;
        }

        // Add to total roll count
        total_roll_accessable_count += roll_accessable_count;
        // Change starting paper roll grid to new paper roll grid
        starting_paper_roll_grid = new_paper_roll_grid.clone();
    }

    total_roll_accessable_count
}