use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Coordinates {
    x: i64,
    y: i64,
    z: i64
}

fn main() {
    let puzzle_input: HashMap<usize, Coordinates> = load_puzzle_input("src/test.txt");
    println!("{:?}", puzzle_input.get(&0).unwrap());
}


fn load_puzzle_input(filename: &str) -> HashMap<usize, Coordinates> {
    let mut coordinates: HashMap<usize, Coordinates> = HashMap::new();

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut i: usize = 0;

    for line in reader.lines() {
        let line_split: Vec<i64> = line.unwrap().split(',').map(|x| x.parse().unwrap()).collect();
        let coords: Coordinates = Coordinates {x: line_split[0], y: line_split[1], z: line_split[2]}; 
        coordinates.insert(i, coords);

        i +=1;
    }


    coordinates
}

fn get_circuits(proximity_map: HashMap<usize, usize>) {
    let mut circuits: Vec<Vec<usize>> = Vec::new();
    
}

// Proximity map maps the key of an electric box to the key of the electric box closest to it
fn get_proximity_map(filename: &str) -> HashMap<usize, usize>{
    let electric_boxes: HashMap<usize, Coordinates> = load_puzzle_input(filename);
    let mut proximity_map: HashMap<usize, usize> = HashMap::new();

    // Find out which box is closest to each box
    for (key, coord) in &electric_boxes {
        // Init closest box index and calculate distance first time
        let mut closest_box: usize = 0;
        if *key == 0 {
            closest_box = 1;
        }
        let mut closest_distance: f32 = calculate_euclidian_distance(coord, electric_boxes.get(&closest_box).unwrap());
        for (ref_key, ref_coord) in &electric_boxes {
            // Make sure we're not checking the same box
            if key != ref_key {
                let distance = calculate_euclidian_distance(coord, ref_coord);
                if distance < closest_distance {
                    closest_box = *ref_key;
                    closest_distance = distance;
                }
            }
        }

        proximity_map.insert(*key, closest_box);
    }

    proximity_map
}

fn calculate_euclidian_distance(p1: &Coordinates, p2: &Coordinates) -> f32 {
    (((p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2) + (p1.z - p2.z).pow(2)) as f32).sqrt()
}