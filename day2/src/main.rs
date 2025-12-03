use std::fs;

fn main() {
    let puzzle_input: Vec<String> = load_puzzle_input();
    let invalid_id_sum: u128 = sum_invalid_ids(&puzzle_input);
    println!("Final Invalid Sum Id: {}", invalid_id_sum);
}

fn load_puzzle_input() -> Vec<String> {
    let data = fs::read_to_string("src/puzzle-input.csv")
        .expect("could not read file");

    let values: Vec<String> = data
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    values
}

fn sum_invalid_ids(ids: &Vec<String>) -> u128 {
    let mut invalid_id_sum: u128 = 0;

    for id_range in ids {
        let mut id_iterator= id_range.split("-");
        let lower: u128 = id_iterator.next().unwrap().parse::<u128>().unwrap();
        let upper: u128 = id_iterator.next().unwrap().parse::<u128>().unwrap();

        for i in lower..upper + 1 {
            let id_str = i.to_string();

            for j in 0..id_str.len()/2 {
                let mut pattern_match: bool = true;
                let pattern = &id_str[0..j + 1];

                if id_str.len() % pattern.len() != 0 {
                    continue;
                }

                for k in 0..id_str.len()/pattern.len() {
                    let check_start_index = k * pattern.len();
                    if pattern != &id_str[check_start_index..check_start_index + pattern.len()] {
                        pattern_match = false;
                    }
                    if !pattern_match {
                        break;
                    }
                }

                if pattern_match {
                    invalid_id_sum += i;
                    break;
                }

            }
        }
        println!("Sum for range {} - {} -> {}", lower, upper, invalid_id_sum);
     }

    invalid_id_sum
}