use std::io::Read;

fn load_data(filename: &str) -> Vec<String> {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|line| line.to_string()).collect()
}

fn perform_operation(a: usize, b: usize, operator: char) -> usize {
    match operator {
        '+' => a + b,
        '*' => a * b,
        _ => 0
    }
}

fn get_next_value(input: &Vec<char>, mut index: usize) -> (usize, usize) {
    let mut value: usize = 0;
    let mut operator: char = '+';

    while index < input.len() {
        let current = input[index];
        index += 1;

        match current {
            '(' => {
                let (next_value, index1) = get_next_value(input, index);
                index = index1;
                value = perform_operation(value, next_value, operator);
            },
            ')' => {
                return (value, index);
            },
            '+' | '*' => {
                operator = current;
            },
            ' ' => {},
            _ => {
                let number = current.to_digit(10).unwrap() as usize;
                value = perform_operation(value, number, operator);
            }
        };
    }

    (value, index)
}

fn part_1(values: &Vec<String>) -> usize {
    values.iter().fold(0, |acc, line| {
        acc + get_next_value(&line.chars().collect(), 0).0
    })
}

fn set_add_precedence(input: &Vec<char>) -> Vec<char> {
    let mut updated: Vec<char> = input.iter().map(|character| {
        if *character == '+' { 'P' } else { *character }
    }).collect();

    while updated.contains(&'P') {
        let index = updated.iter().position(|&c| c == 'P').unwrap();
        updated[index] = '+';

        let mut bracket_count: usize = 0;

        for i in index+1..updated.len() {
            if updated[i] == '(' {
                bracket_count += 1;
            }

            if updated[i] == ')' {
                bracket_count -= 1;
                if bracket_count == 0 {
                    updated.insert(i + 1, ')');
                    break;
                }
            }
    
            if updated[i].is_digit(10) && bracket_count == 0 {
                updated.insert(i + 1, ')');
                break;
            }
        }

        for i in (0..index-1).rev() {
            if updated[i] == ')' {
                bracket_count += 1;
            }

            if updated[i] == '(' {
                bracket_count -= 1;
                if bracket_count == 0 {
                    updated.insert(i, '(');
                    break;
                }
            }

            if updated[i].is_digit(10) && bracket_count == 0 {
                updated.insert(i, '(');
                break;
            }
        }
    }

    updated
}

fn part_2(values: &Vec<String>) -> usize {
    values.iter().fold(0, |acc, line| {
        let input = set_add_precedence(&line.chars().collect());
        acc + get_next_value(&input, 0).0
    })
}

fn main() {
    let inputs: Vec<String> = load_data("day18/input.txt");

    let part_1_answer = part_1(&inputs);
    println!("Part 1: {}", part_1_answer);

    let part_2_answer = part_2(&inputs);
    println!("Part 2: {}", part_2_answer);
}
