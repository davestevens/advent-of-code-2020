use std::io::Read;
use std::collections::HashMap;

fn apply_mask(value: usize, mask: &Vec<char>) -> usize {
    let mut masked_value: usize = value;
    for (index, bit) in mask.iter().enumerate() {
        match bit {
            '0' => {
                masked_value &= !(1 << index);
            },
            '1' => {
                masked_value |= 1 << index;
            },
            _ => {}
        };
    }
    masked_value
}

fn part_1(filename: &str) -> usize {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut mask: Vec<char> = Vec::new();
    let mut memory: HashMap<&str, usize> = HashMap::new();

    for line in contents.lines() {
        if line.contains("mask =") {
            mask = line.split(" = ").nth(1).unwrap().chars().rev().collect();
        } else {
            let address: &str = line.split("[").nth(1).unwrap().split("]").nth(0).unwrap();
            let value: usize = line.split(" = ").nth(1).unwrap().parse::<usize>().unwrap();
            let masked_value: usize = apply_mask(value, &mask);
            memory.insert(address, masked_value);
        }
    }

    memory.values().into_iter().sum()
}

fn build_masks(original: &Vec<char>) -> Vec<Vec<char>> {
    let mask: Vec<char> = Vec::new();
    let mut masks: Vec<Vec<char>> = vec![mask];
    for bit in original {
        match bit {
            'X' => {
                let current_masks = masks.clone();
                masks.clear();
                current_masks.iter().for_each(|mask| {
                    masks.push(
                        [mask.clone(), ['1'].to_vec()].concat()
                    );
                    masks.push(
                        [mask.clone(), ['Y'].to_vec()].concat()
                    );
                });
            },
            _ => {
                masks.iter_mut().for_each(|mask: &mut Vec<char>| mask.push(*bit))
            }
        };
    }
    masks
}

fn apply_mask_part_2(value: usize, mask: &Vec<char>) -> usize {
    let mut masked_value: usize = value;
    for (index, bit) in mask.iter().enumerate() {
        match bit {
            '0' => {},
            '1' => {
                masked_value |= 1 << index;
            },
            'Y' => {
                masked_value &= !(1 << index);
            },
            _ => {}
        };
    }
    masked_value
}

fn part_2(filename: &str) -> usize {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut masks: Vec<Vec<char>> = Vec::new();
    let mut memory: HashMap<usize, usize> = HashMap::new();

    for line in contents.lines() {
        if line.contains("mask =") {
            let mask: Vec<char> = line.split(" = ").nth(1).unwrap().chars().rev().collect();
            masks = build_masks(&mask);
        }
        else {
            let address: usize = line.split("[").nth(1).unwrap().split("]").nth(0).unwrap().parse::<usize>().unwrap();
            let value: usize = line.split(" = ").nth(1).unwrap().parse::<usize>().unwrap();
            for mask in masks.iter() {
                let masked_address = apply_mask_part_2(address, &mask);
                memory.insert(masked_address, value);
            }
        }
    }

    memory.values().into_iter().sum()
}

fn main() {
    let part_1_answer = part_1("day14/input.txt");
    println!("Part 1: {}", part_1_answer);

    let part_2_answer = part_2("day14/input.txt");
    println!("Part 2: {}", part_2_answer);
}
