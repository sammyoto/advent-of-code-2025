use core::num;
use std::fs::File;
use std::io::{BufRead, BufReader};
  
fn main() {
    let battery_banks: Vec<Vec<u64>> = load_battery_banks("src/test.txt");
    println!("{}", sum_largest_joltages(&battery_banks, 12));
}

fn load_battery_banks(filename: &str) -> Vec<Vec<u64>>{
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut battery_banks: Vec<Vec<u64>> = Vec::new();

    for line in reader.lines() {
        let mut battery_bank: Vec<u64> = Vec::new();
        for ch in line.unwrap().chars() {
            if let Some(d) = ch.to_digit(10) {
                battery_bank.push(d as u64);
            }
        }

        battery_banks.push(battery_bank);
    }

    battery_banks
}

fn find_largest_joltage(battery_bank: &Vec<u64>, num_batteries: usize) -> u64{
    let mut voltage_indices:Vec<usize> = Vec::new();

    // Loop over number of batteries
    for i in 0..num_batteries {
        // Stop looking at index where there are still enough batteries left to turn on
        let end_search_index = battery_bank.len() - num_batteries + i + 1;
        let mut curr_largest_voltage_index = 0;

        // If on first battery, start searching at Index 0
        if i == 0 {
            for k in 0..battery_bank.len() - end_search_index {
                if battery_bank[k] > battery_bank[curr_largest_voltage_index] {
                    curr_largest_voltage_index = k;
                }
            }
        // Else start searching at index AFTER index of previous battery
        } else {
            curr_largest_voltage_index = voltage_indices[i - 1] + 1;
            for k in (voltage_indices[i - 1] + 1)..end_search_index {
                if battery_bank[k] > battery_bank[curr_largest_voltage_index] {
                    curr_largest_voltage_index = k;
                }
            }
        }

        // Push largest index to voltage indices
        voltage_indices.push(curr_largest_voltage_index);
    }

    println!("{:?}", voltage_indices);

    // Now construct a number from digits
    let mut largest_possible_voltage: u64 = 0;
    for i in 0..voltage_indices.len() {
        // Construct a multiplier for each index
        let multiplier = (10_u64).pow((num_batteries - i - 1) as u32);
        // Add each multiplied number to the total voltage
        largest_possible_voltage += battery_bank[voltage_indices[i]] * multiplier;
    }

    largest_possible_voltage
}

fn sum_largest_joltages(battery_banks: &Vec<Vec<u64>>, num_batteries: usize) -> u64{
    let mut voltage_sum: u64 = 0;

    for battery_bank in battery_banks {
        voltage_sum += find_largest_joltage(battery_bank, num_batteries);
    }

    voltage_sum
}
