use std::io::Read;

fn load_data(filename: &str) -> Vec<String> {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|line| line.to_string()).collect()
}

const DIRECTIONS: [char; 4] = ['N', 'E', 'S', 'W'];

fn make_move(action: char, value: isize, mut x: isize, mut y: isize) -> (isize, isize) {
    match action {
        'N' => y -= value,
        'E' => x += value,
        'S' => y += value,
        'W' => x -= value,
        _ => {}
    }
    (x, y)
}

fn rotate(action: char, value: isize, current_direction: char) -> char {
    let current: isize = DIRECTIONS.iter().position(|&d| d == current_direction).unwrap() as isize;
    let mutiplier = if action == 'L' { -1 } else { 1 };
    let steps = value / 90;

    let diff = (current + (steps * mutiplier)) + DIRECTIONS.len() as isize;
    assert!(diff > 0);
    let diff: usize = diff as usize;
    let next = diff % DIRECTIONS.len();
    DIRECTIONS[next]
}

fn part_1(values: &Vec<String>) -> isize {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut direction: char = 'E';

    for value in values {
        let action: char = value.chars().nth(0).unwrap();
        let value: isize = value[1..value.len()].parse::<isize>().unwrap();

        match action {
            'N' | 'E' | 'S' | 'W'  => {
                let (update_x, update_y) = make_move(action, value, x, y);
                x = update_x;
                y = update_y;
            },
            'L' | 'R' => direction = rotate(action, value, direction),
            'F' => {
                let (update_x, update_y) = make_move(direction, value, x, y);
                x = update_x;
                y = update_y;
            },
            _ => {}
        }
    }

    x + y
}

fn move_ship(value: isize, waypoint_x: isize, waypoint_y: isize, x: isize, y: isize) -> (isize, isize) {
    let diff_x = value * waypoint_x;
    let diff_y = value * waypoint_y;
    (x + diff_x, y + diff_y)
}

fn rotate_waypoint(action: char, value: isize, waypoint_x: isize, waypoint_y: isize) -> (isize, isize) {
    let value = value as f64;
    let mutiplier = if action == 'L' { -1.0 } else { 1.0 };
    let angle_rads = (mutiplier * value) * (std::f64::consts::PI / 180.0);
    let waypoint_x = waypoint_x as f64;
    let waypoint_y = waypoint_y as f64;
    let x = (angle_rads.cos() * waypoint_x) - (angle_rads.sin() * waypoint_y);
    let y = (angle_rads.sin() * waypoint_x) + (angle_rads.cos() * waypoint_y);
    (x.round() as isize, y.round() as isize)
}

fn part_2(values: &Vec<String>) -> isize {
    let mut waypoint_x: isize = 10;
    let mut waypoint_y: isize = -1;
    let mut ship_x: isize = 0;
    let mut ship_y: isize = 0;

    for value in values {
        let action: char = value.chars().nth(0).unwrap();
        let value: isize = value[1..value.len()].parse::<isize>().unwrap();

        match action {
            'N' | 'E' | 'S' | 'W'  => {
                let (update_x, update_y) = make_move(action, value, waypoint_x, waypoint_y);
                waypoint_x = update_x;
                waypoint_y = update_y;
            },
            'L' | 'R' => {
                let (update_x, update_y) = rotate_waypoint(action, value, waypoint_x, waypoint_y);
                waypoint_x = update_x;
                waypoint_y = update_y;
            },
            'F' => {
                let (update_x, update_y) = move_ship(value, waypoint_x, waypoint_y, ship_x, ship_y);
                ship_x = update_x;
                ship_y = update_y;
            },
            _ => {}
        }
    }

    (ship_x + ship_y).abs()
}

fn main() {
    let input: Vec<String> = load_data("day12/input.txt");

    let part_1_answer = part_1(&input);
    println!("Part 1: {}", part_1_answer);

    let part_2_answer = part_2(&input);
    println!("Part 2: {}", part_2_answer);
}
