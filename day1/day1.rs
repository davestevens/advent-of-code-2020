use std::io::Read;

const TARGET: i32 = 2020;

fn load_data(filename: &str) -> Vec<i32> {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut v: Vec<i32> = Vec::new();
    for s in contents.lines() {
        v.push(s.parse::<i32>().unwrap());
    }
    return v;
}

fn part_1(values: &Vec<i32>, target: i32) -> i32 {
    for value in values.iter() {
        let remainder: i32 = target - value;
        let index = values.iter().position(|&r| r == remainder);
        match index {
            Some(_) => {
                return value * remainder;
            },
            _ => {}
        }
    }
    return -1;
}

fn part_2(values: &Vec<i32>, target: i32) -> i32 {
    for (index, value) in values.iter().enumerate() {
        for value_2 in values[index + 1..].iter() {
            let remainder: i32 = target - (value + value_2);
            let index = values.iter().position(|&r| r == remainder);
            match index {
                Some(_) => {
                    return value * value_2 * remainder;
                },
                _ => {}
            }
        }
    }
    return -1;
}

fn main() {
    let mut inputs: Vec<i32> = load_data("day1/input.txt");
    inputs.sort();

    let part_1_answer = part_1(&inputs, TARGET);
    println!("Part 1: {}", part_1_answer);

    let part_2_answer = part_2(&inputs, TARGET);
    println!("Part 2: {}", part_2_answer);
}
