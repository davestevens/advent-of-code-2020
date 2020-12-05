use std::io::Read;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Seat {
    row: usize,
    col: usize,
    id: usize,
    input: String
}

fn binary_search(mut min: usize, mut max: usize, instructions: &Vec<bool>) -> usize {
    for instruction in instructions {
        let step = ((max + 1) - min) / 2;

        if *instruction {
            min += step;
        } else {
            max -= step;
        }
    }

    assert_eq!(min, max);
    min
}

impl FromStr for Seat {
    type Err = std::num::ParseIntError;

    fn from_str(seat_str: &str) -> Result<Self, Self::Err> {
        let seat_str_chars: Vec<char> = seat_str.chars().collect();
        let row_info: Vec<bool> = seat_str_chars[0 .. 7].to_vec().iter().map(|&c| c == 'B').collect();
        let col_info: Vec<bool> = seat_str_chars[7 .. seat_str_chars.len()].to_vec().iter().map(|&c| c == 'R').collect();


        let row: usize = binary_search(0, 127, &row_info);
        let col: usize = binary_search(0, 7, &col_info);
        let id: usize = (row * 8) + col;

        Ok(
            Seat { row, col, id, input: seat_str.to_string() }
        )
    }
}

fn load_data(filename: &str) -> Vec<Seat> {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|s| Seat::from_str(&s).unwrap()).collect()
}

fn part_1(values: &Vec<Seat>) -> usize {
    let max = values.iter().max_by_key(|s| s.id).unwrap();
    max.id
}

fn part_2(values: &Vec<Seat>) -> usize {
    let mut ids: Vec<usize> = values.iter().map(|v| v.id).collect();
    ids.sort();

    let mut prev: usize = ids[0];
    for id in ids.iter() {
        if (id - prev) > 1 {
            return *id - 1;
        }
        prev = *id;
    }
    0
}

fn main() {
    let inputs: Vec<Seat> = load_data("day5/input.txt");

    let part_1_answer = part_1(&inputs);
    println!("Part 1: {}", part_1_answer);

    let part_2_answer = part_2(&inputs);
    println!("Part 2: {}", part_2_answer);
}
