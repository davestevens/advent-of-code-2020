use std::io::Read;
use std::collections::HashSet;
use std::collections::HashMap;

fn part_1(filename: &str) -> usize {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
    .split("\r\n\r\n")
    .map(|s| {
        let mut answers: HashSet<char> = HashSet::new();
        for line in s.split_whitespace() {
            for character in line.chars() {
                answers.insert(character);
            }
        }
        answers.len()
    })
    .sum()
}

fn part_2(filename: &str) -> usize {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
    .split("\r\n\r\n")
    .map(|s| {
        let mut answers: HashMap<char, usize> = HashMap::new();
        let lines: Vec<&str> = s.split_whitespace().collect();
        let line_count = lines.len();
        for line in lines {
            for character in line.chars() {
                *answers.entry(character).or_default() += 1;
            }
        }
        answers.retain(|_, v| *v == line_count);
        answers.len()
    })
    .sum()
}

fn main() {
    let part_1_answer = part_1("day6/input.txt");
    println!("Part 1: {}", part_1_answer);

    let part_2_answer = part_2("day6/input.txt");
    println!("Part 2: {}", part_2_answer);
}
