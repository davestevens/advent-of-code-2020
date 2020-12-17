use std::io::Read;
use std::collections::HashMap;

type Grid = HashMap<String, char>;

#[derive(Debug, Clone)]
struct Input {
    grid: Grid,
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
    w: (isize, isize)
}

const ACTIVE: char = '#';
const INACTIVE: char = '.';

fn load_data(filename: &str) -> Input {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut grid: Grid = HashMap::new();
    let mut max_y: usize = 0;
    let mut max_x: usize = 0;
    for (y, line) in contents.lines().enumerate() {
        max_y = if y > max_y { y } else { max_y };
        for (x, cube) in line.chars().enumerate() {
            max_x = if x > max_x { x } else { max_x };
            let key: String = format!("{}_{}_{}_{}", x, y, 0, 0);
            grid.insert(key, cube);
        }
    }
    Input { grid, x: (0, max_x as isize), y: (0, max_y as isize), z: (0, 0), w: (0, 0) }
}

fn set_cube(grid: &mut Grid, x: isize, y: isize, z: isize, w: isize, state: char) {
    let key: String = format!("{}_{}_{}_{}", x, y, z, w);
    grid.insert(key, state);
}

fn get_cube(grid: &Grid, x: isize, y: isize, z: isize, w: isize) -> char {
    let key: String = format!("{}_{}_{}_{}", x, y, z, w);
    *grid.get(&key).unwrap_or(&INACTIVE)
}

fn count_active_neighbors(grid: &Grid, x: isize, y: isize, z: isize, w: isize) -> usize {
    let mut count: usize = 0;

    for current_w in w-1..=w+1 {
        for current_z in z-1..=z+1 {
            for current_y in y-1..=y+1 {
                for current_x in x-1..=x+1 {
                    if current_x == x && current_y == y && current_z == z && current_w == w { continue; }
                    let value = get_cube(grid, current_x, current_y, current_z, current_w);
                    count += if value == ACTIVE { 1 } else { 0 };
                }
            }
        }
    }

    count
}

fn iterate(input: &mut Input, include_fourth_dimension: bool) {
    let mut min_w: isize = if include_fourth_dimension { input.w.0 } else { 1 };
    let mut max_w: isize = if include_fourth_dimension { input.w.1 } else { -1 };
    let mut min_z: isize = input.z.0;
    let mut max_z: isize = input.z.1;
    let mut min_y: isize = input.y.0;
    let mut max_y: isize = input.y.1;
    let mut min_x: isize = input.x.0;
    let mut max_x: isize = input.x.1;

    let mut current: Grid = input.grid.clone();
    for w in min_w-1..=max_w+1 {
        for z in min_z-1..=max_z+1 {
            for y in min_y-1..=max_y+1 {
                for x in min_x-1..=max_x+1 {
                    let value = get_cube(&input.grid, x, y, z, w);
                    let active_neighbor_count = count_active_neighbors(&input.grid, x, y, z, w);
                    match value {
                        ACTIVE => {
                            if active_neighbor_count < 2 || active_neighbor_count > 3 {
                                set_cube(&mut current, x, y, z, w, INACTIVE);
                            }
                        },
                        INACTIVE => {
                            if active_neighbor_count == 3 {
                                set_cube(&mut current, x, y, z, w, ACTIVE);

                                min_x = if x < min_x { x } else { min_x };
                                min_y = if y < min_y { y } else { min_y };
                                min_z = if z < min_z { z } else { min_z };
                                min_w = if w < min_w { w } else { min_w };

                                max_x = if x > max_x { x } else { max_x };
                                max_y = if y > max_y { y } else { max_y };
                                max_z = if z > max_z { z } else { max_z };
                                max_w = if w > max_w { w } else { max_w };
                            }
                        },
                        _ => {}
                    };
                }
            }
        }
    }
    input.grid = current;
    input.x = (min_x, max_x);
    input.y = (min_y, max_y);
    input.z = (min_z, max_z);
    input.w = (min_w, max_w);
}

fn run(mut values: Input, include_fourth_dimension: bool) -> usize {
    for _ in 0..6 {
        iterate(&mut values, include_fourth_dimension);
    }
    values.grid.values().filter(|&cube| cube == &ACTIVE).count()
}

fn main() {
    let input: Input = load_data("day17/input.txt");

    let part_1_answer = run(input.clone(), false);
    println!("Part 1: {}", part_1_answer);

    let part_2_answer = run(input.clone(), true);
    println!("Part 2: {}", part_2_answer);
}
