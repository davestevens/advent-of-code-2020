use std::io::Read;

const TREE: char = '#';

#[derive(Debug, PartialEq)]
struct DataSet {
    width: usize,
    height: usize,
    data: Vec<bool>
}

fn load_data(filename: &str) -> DataSet {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut width: usize = 0;
    let mut height: usize = 0;
    let mut data: Vec<bool> = Vec::new();
    for s in contents.lines() {
        let line: Vec<bool> = s.chars().map(|c| c == TREE).collect();
        if height == 0 {
            width = line.len();
        }
        height += 1;
        data.extend_from_slice(&line);
    }
    DataSet { width: width, height: height, data }
}

fn part_1(dataset: &DataSet, horizontal_movement: usize, vertical_movement: usize) -> usize {
    let mut horizontal: usize = horizontal_movement;
    let mut vertical: usize = vertical_movement;
    let mut count: usize = 0;

    while vertical < dataset.height {
        let index = horizontal + (vertical * dataset.width);
        if dataset.data[index] {
            count += 1;
        }
        horizontal = (horizontal + horizontal_movement) % dataset.width;
        vertical += vertical_movement;
    }

    count
}

fn part_2(dataset: &DataSet) -> usize {
    let mut total: usize = part_1(&dataset, 1, 1);
    total *= part_1(&dataset, 3, 1);
    total *= part_1(&dataset, 5, 1);
    total *= part_1(&dataset, 7, 1);
    total *= part_1(&dataset, 1, 2);
    total
}

fn main() {
    let inputs: DataSet = load_data("day3/input.txt");

    let part_1_answer = part_1(&inputs, 3, 1);
    println!("Part 1: {}", part_1_answer);

    let part_2_answer = part_2(&inputs);
    println!("Part 2: {}", part_2_answer);
}
