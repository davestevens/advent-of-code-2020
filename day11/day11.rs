use std::io::Read;
use std::cmp;

const FLOOR: char = '.';
const SEAT: char = 'L';
const OCCUPIED: char = '#';

fn load_data(filename: &str) -> Vec<Vec<char>> {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|line| {
        line.chars().collect()
    }).collect()
}

fn clone(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    input.iter().map(|line| line.to_vec()).collect()
}

type CheckFunction = fn(usize, usize, &Vec<Vec<char>>) -> usize;

fn check_immediate_surrounding(seat_x: usize, seat_y: usize, layout: &Vec<Vec<char>>) -> usize {
    let height: usize = layout.len() - 1;
    let width: usize = layout[0].len() - 1;

    let min_x: usize = if seat_x < 1 { 0 } else { seat_x - 1 };
    let max_x: usize = cmp::min(seat_x + 1, width);

    let min_y: usize = if seat_y < 1 { 0 } else { seat_y - 1 };
    let max_y: usize = cmp::min(seat_y + 1, height);

    let mut occupied_count: usize = 0;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if x == seat_x && y == seat_y {
                continue;
            }
            if layout[y][x] == OCCUPIED {
                occupied_count += 1;
            }
        }
    }

    occupied_count
}

fn check_visible_surrounding(seat_x: usize, seat_y: usize, layout: &Vec<Vec<char>>) -> usize {
    let height: usize = layout.len() - 1;
    let width: usize = layout[0].len() - 1;
    let mut occupied_count: usize = 0;

    // ABOVE
    {
        for y in (0..seat_y).rev() {
            if layout[y][seat_x] != FLOOR {
                occupied_count += if layout[y][seat_x] == OCCUPIED { 1 } else { 0 };
                break;
            }
        }
    }
    //ABOVERIGHT
    {
        let mut x: usize = seat_x;
        for y in (0..seat_y).rev() {
            x += 1;
            if x > width { break; }
            if layout[y][x] != FLOOR {
                occupied_count += if layout[y][x] == OCCUPIED { 1 } else { 0 };
                break;
            }
        }
    }
    // RIGHT
    for x in seat_x+1..=width {
        if layout[seat_y][x] != FLOOR {
            occupied_count += if layout[seat_y][x] == OCCUPIED { 1 } else { 0 };
            break;
        }
    }
    // DOWNRIGHT
    {
        let mut x: usize = seat_x;
        for y in seat_y+1..=height {
            x += 1;
            if x > width { break; }
            if layout[y][x] != FLOOR {
                occupied_count += if layout[y][x] == OCCUPIED { 1 } else { 0 };
                break;
            }
        }
    }
    // DOWN
    for y in seat_y+1..=height {
        if layout[y][seat_x] != FLOOR {
            occupied_count += if layout[y][seat_x] == OCCUPIED { 1 } else { 0 };
            break;
        }
    }
    // DOWNLEFT
    {
        let mut x: usize = seat_x;
        for y in seat_y+1..=height {
            if x == 0 { break; }
            x -= 1;
            if layout[y][x] != FLOOR {
                occupied_count += if layout[y][x] == OCCUPIED { 1 } else { 0 };
                break;
            }
        }
    }
    // LEFT
    {
        for x in (0..seat_x).rev() {
            if layout[seat_y][x] != FLOOR {
                occupied_count += if layout[seat_y][x] == OCCUPIED { 1 } else { 0 };
                break;
            }
        }
    }
    // ABOVELEFT
    {
        let mut x: usize = seat_x;
        for y in (0..seat_y).rev() {
            if x == 0 { break; }
            x -= 1;
            if layout[y][x] != FLOOR {
                occupied_count += if layout[y][x] == OCCUPIED { 1 } else { 0 };
                break;
            }
        }
    }

    occupied_count
}

fn iterate(values: &Vec<Vec<char>>, check_function: CheckFunction, occupied_count: usize) -> Vec<Vec<char>> {
    let mut cloned = clone(values);
    let height = values.len();
    let width = values[0].len();

    for y in 0..height {
        for x in 0..width {
            match values[y][x] {
                SEAT => {
                    let count = check_function(x, y, values);
                    if count == 0 {
                        cloned[y][x] = OCCUPIED;
                    }
                },
                OCCUPIED => {
                    let count = check_function(x, y, values);
                    if count >= occupied_count {
                        cloned[y][x] = SEAT;
                    }
                },
                _ => {}
            }
        }
    }

    cloned
}

fn cmp(a: &Vec<Vec<char>>, b: &Vec<Vec<char>>) -> bool {
    let height = a.len();
    let width = a[0].len();

    for y in 0..height {
        for x in 0..width {
            if a[y][x] != b[y][x] {
                return false;
            }
        }
    }

    true
}

fn count_occupied(layout: &Vec<Vec<char>>) -> usize {
    let height = layout.len();
    let width = layout[0].len();
    let mut count: usize = 0;

    for y in 0..height {
        for x in 0..width {
            if layout[y][x] == OCCUPIED {
                count += 1;
            }
        }
    }
    return count;
}

fn part_1(values: &Vec<Vec<char>>) -> usize {
    let mut current: Vec<Vec<char>> = clone(values);

    loop {
        let next = iterate(&current, check_immediate_surrounding, 4);
        if cmp(&current, &next) {
            break;
        }
        current = next;
    }

    count_occupied(&current)
}

fn part_2(values: &Vec<Vec<char>>) -> usize {
    let mut current: Vec<Vec<char>> = clone(values);

    loop {
        let next = iterate(&current, check_visible_surrounding, 5);
        if cmp(&current, &next) {
            break;
        }
        current = next;
    }

    count_occupied(&current)
}

fn main() {
    let input: Vec<Vec<char>> = load_data("day11/input.txt");

    let part_1_answer = part_1(&input);
    println!("Part 1: {}", part_1_answer);

    let part_2_answer = part_2(&input);
    println!("Part 2: {}", part_2_answer);
}
