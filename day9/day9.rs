use std::io::Read;

fn load_data(filename: &str) -> Vec<usize> {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|line| line.parse::<usize>().unwrap()).collect()
}

fn is_valid(target: usize, values: Vec<usize>) -> bool {
    for value in values.iter() {
        let remainder: isize = (target as isize) - (*value as isize);
        let index = values.iter().position(|&r| (r as isize) == remainder);
        match index {
            Some(_) => return true,
            _ => {}
        }
    }

    false
}

fn part_1(values: &Vec<usize>, preamble_size: usize) -> usize {
    for (index, value) in values.iter().enumerate() {
        if index < preamble_size { continue; }
        let previous = values[index - preamble_size..index].to_vec();
        if !is_valid(*value, previous) {
            return *value;
        }
    }

    0
}

fn part_2(values: &Vec<usize>, target: usize) -> usize {
    for (start, _) in values.iter().enumerate() {
        let mut end = start + 1;
        loop {
            let to_sum = values[start..end].to_vec();
            let sum: usize = to_sum.iter().sum();
            if sum == target {
                return to_sum.iter().min().unwrap() + to_sum.iter().max().unwrap();
            } else if sum > target {
                break;
            }

            end += 1;
        }
    }

    0
}

fn main() {
    let inputs: Vec<usize> = load_data("day9/input.txt");

    let part_1_answer = part_1(&inputs, 25);
    println!("Part 1: {}", part_1_answer);

    let part_2_answer = part_2(&inputs, part_1_answer);
    println!("Part 2: {}", part_2_answer);
}
