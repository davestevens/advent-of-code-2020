use std::io::Read;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct PasswordPolicy {
    min: u32,
    max: u32,
    character: char,
    example: String
}

impl FromStr for PasswordPolicy {
    type Err = std::num::ParseIntError;

    fn from_str(password_policy_str: &str) -> Result<Self, Self::Err> {
        // TODO: Find a better way of handling this parsing...
        let a: Vec<&str> = password_policy_str.split(":").collect();
        let b: Vec<&str> = a[0].split(" ").collect();
        let c: Vec<u32> = b[0].split("-").map(|s| s.parse::<u32>().unwrap()).collect();

        let min: u32 = c[0];
        let max: u32 = c[1];
        let character: char = b[1].chars().nth(0).unwrap();
        let example: String = String::from(a[1].trim());

        Ok(
            PasswordPolicy { min, max, character, example }
        )
    }
}

fn load_data(filename: &str) -> Vec<PasswordPolicy> {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|s| PasswordPolicy::from_str(&s).unwrap()).collect()
}

fn part_1(values: &Vec<PasswordPolicy>) -> u32 {
    let mut total_valid: u32 = 0;
    for value in values {
        let mut counts: HashMap<char, u32> = HashMap::new();
        for c in value.example.chars() {
            *counts.entry(c).or_default() += 1;
        }
        let count: u32 = *counts.get(&value.character).unwrap_or(&0);
        if count >= value.min && count <= value.max {
            total_valid += 1;
        }
    }
    total_valid
}

fn part_2(values: &Vec<PasswordPolicy>) -> u32 {
    let mut total_valid: u32 = 0;
    for value in values {
        let a: char = value.example.chars().nth(value.min as usize - 1).unwrap();
        let b: char = value.example.chars().nth(value.max as usize - 1).unwrap();
        if (a == value.character) ^ (b == value.character) {
            total_valid += 1;
        }
    }
    total_valid
}

fn main() {
    let inputs: Vec<PasswordPolicy> = load_data("day2/input.txt");

    let part_1_answer = part_1(&inputs);
    println!("Part 1: {}", part_1_answer);

    let part_2_answer = part_2(&inputs);
    println!("Part 2: {}", part_2_answer);
}
