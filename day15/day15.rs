use std::io::Read;
use std::collections::HashMap;
use std::time::Instant;

fn load_data(filename: &str) -> Vec<usize> {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.split(",").map(|line| line.parse::<usize>().unwrap()).collect()
}

fn run(values: &Vec<usize>, iteration: usize) -> usize {
    let mut numbers: HashMap<usize, (usize, usize)> = HashMap::new();
    let mut prev_number: usize = 0;
    for (index, value) in values.iter().enumerate() {
        numbers.insert(*value, (index + 1, index + 1));
        prev_number = *value;
    }

    for index in values.len()+1..=iteration {
        let (prev_0, prev_1) = numbers.get(&prev_number).unwrap();
        let current = prev_1 - prev_0;
        let prev_indexes: &mut (usize, usize) = numbers.entry(current).or_insert((0, index));
        prev_indexes.0 = prev_indexes.1;
        prev_indexes.1 = index;
        prev_number = current;
    }

    prev_number
}

fn main() {
    let inputs: Vec<usize> = load_data("day15/input.txt");

    let part_1_answer = run(&inputs, 2020);
    println!("Part 1: {}", part_1_answer);

    let part_2_time = Instant::now();
    let part_2_answer = run(&inputs, 30000000);
    let part_2_elapsed = part_2_time.elapsed();
    println!("Part 2: {} ({:.2?})", part_2_answer, part_2_elapsed);
}
