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
    let answer = calculate_answer();
    println!("Answer 1: {}", answer);
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

fn get_circuits(proximity_map: HashMap<usize, usize>) -> Vec<Vec<usize>>{
    let mut circuits: Vec<Vec<usize>> = Vec::new();
    let mut circuit_map: HashMap<usize, usize> = HashMap::new();
    let mut circuit_counter: usize = 0;

    // Loop over each electric box and its closest box
    for (electric_box, closest_box) in proximity_map {
        // 1. Electric box and closest box are both not in a circuit
        if !circuit_map.contains_key(&electric_box) && !circuit_map.contains_key(&closest_box) {
            // Add boxes to a circuit and push it to circuits
            let mut circuit: Vec<usize> = Vec::new();
            circuit.push(electric_box);
            circuit.push(closest_box);
            circuits.push(circuit);

            // Reflect this in circuit map
            circuit_map.insert(electric_box, circuit_counter);
            circuit_map.insert(closest_box, circuit_counter);

            // Increment circuit counter
            circuit_counter += 1;
        }
        // 2. Electric box and closest box are both in a circuit
        else if circuit_map.contains_key(&electric_box) && circuit_map.contains_key(&closest_box) {
            // We want to merge the two circuits and remap the circuit map
            if circuit_map.get(&electric_box).unwrap() != circuit_map.get(&closest_box).unwrap() {
                // Get a clone of the closest box's circuit
                let mut closest_circuit = circuits[*circuit_map.get(&closest_box).unwrap()].clone();
                // Append this to the electric box's circuit
                circuits[*circuit_map.get(&electric_box).unwrap()].append(&mut closest_circuit);
                // Delete the circuit at closest box
                circuits.remove(*circuit_map.get(&closest_box).unwrap());
                // Remake the circuit map, likely an easier way to do this but for now remake the whole map
                circuit_map.clear();
                for i in 0..circuits.len() {
                    for j in 0..circuits[i].len() {
                        circuit_map.insert(circuits[i][j], i);
                    }
                }
                circuit_counter = circuits.len() - 1;
            }
        }
        // 3. Electric box is in a circuit
        else if circuit_map.contains_key(&electric_box) {
            // Push closest box to the circuit containing electric box
            let circuit_index: usize = *circuit_map.get(&electric_box).unwrap();
            circuits[circuit_index].push(closest_box);
            // Reflect this in circuit map
            circuit_map.insert(closest_box, circuit_index);
        }
        // 4. Closest box is in a circuit
        else if circuit_map.contains_key(&closest_box) {
            // Push electric box to the circuit containing closest box
            let circuit_index: usize = *circuit_map.get(&closest_box).unwrap();
            circuits[circuit_index].push(electric_box);
            // Reflect this in circuit map
            circuit_map.insert(electric_box, circuit_index);
        }
        
    }

    println!("{:?}", circuits);
    circuits
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

fn calculate_answer() -> usize {
    let circuits = get_circuits(get_proximity_map("src/puzzle_input.txt"));
    let mut circuit_sizes: Vec<usize> = circuits.iter().map(|x| x.len()).collect();
    circuit_sizes.sort_by(|a, b| b.cmp(a));
    circuit_sizes[0] * circuit_sizes[1] * circuit_sizes[2]
}