use std::io::Read;
use std::cmp;

fn load_data(filename: &str) -> Vec<usize> {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|line| line.parse::<usize>().unwrap()).collect()
}

fn part_1(values: &Vec<usize>) -> usize {
    let mut differences: Vec<usize> = Vec::new();
    let mut previous: usize = 0;

    for value in values {
        differences.push(value - previous);
        previous = *value;
    }

    let ones = differences.iter().filter(|&d| *d == 1).count();
    let threes = differences.iter().filter(|&d| *d == 3).count() + 1;

    ones * threes
}

fn part_2(values: &Vec<usize>) -> usize {
    let mut adapters = vec![0];
    adapters.append(&mut values.clone());
    let mut arrangements: Vec<usize> = vec![0; adapters.len()];
    arrangements[0] = 1;

    for (index, _) in adapters.iter().enumerate() {
        let start: usize = if index < 3 { 0 } else { index - 3 };
        let end: usize = cmp::max(index, 0);
        for prev in start..end {
            if adapters[index] <= (adapters[prev] + 3) {
                arrangements[index] += arrangements[prev];
            }
        }
    }

    arrangements[adapters.len() - 1]
}

fn main() {
    let mut inputs: Vec<usize> = load_data("day10/input.txt");
    inputs.sort();

    let part_1_answer = part_1(&inputs);
    println!("Part 1: {}", part_1_answer);

    let part_2_answer = part_2(&inputs);
    println!("Part 2: {}", part_2_answer);
}
