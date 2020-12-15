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
    let mut numbers: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut prev_number: usize = 0;
    for (index, value) in values.iter().enumerate() {
        numbers.insert(*value, vec![index + 1]);
        prev_number = *value;
    }
    for index in values.len()+1..=iteration {
        let prev_indexes = numbers.get(&prev_number).unwrap();
        let current: usize = if prev_indexes.len() < 2 {
            0
        } else {
            prev_indexes[prev_indexes.len()-1] - prev_indexes[prev_indexes.len()-2]
        };
        let prev_indexes: &mut Vec<usize> = numbers.entry(current).or_insert(vec![]);
        prev_indexes.push(index);
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
