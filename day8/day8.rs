use std::io::Read;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
enum Op {
    Acc, Jmp, Nop
}

#[derive(Debug, PartialEq)]
struct Instruction {
    operation: Op,
    argument: isize
}

fn get_operation(input: &str) -> Result<Op, ()> {
    match input {
        "acc" => Ok(Op::Acc),
        "jmp" => Ok(Op::Jmp),
        "nop" => Ok(Op::Nop),
        _  => Err(()),
    }
}

fn get_argument(input: &str) -> isize {
    let sign = if &input[0..1] == "-" { -1 } else { 1 };
    input[1..input.len()].parse::<isize>().unwrap() * sign
}

fn load_data(filename: &str) -> Vec<Instruction> {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(|line| {
        let line_sections: Vec<&str> = line.split(" ").collect();
        Instruction {
            operation: get_operation(line_sections[0]).unwrap(),
            argument: get_argument(line_sections[1])
        }
    }).collect()
}

fn run(values: &Vec<Instruction>) -> (isize, bool) {
    let halt_pc: usize = values.len();
    let mut halted: bool = false;
    let mut pc: usize = 0;
    let mut accumulator: isize = 0;
    let mut seen_operations: HashSet<usize> = HashSet::new();

    loop {
        if pc == halt_pc {
            halted = true;
            break;
        }
        if seen_operations.contains(&pc) {
            break;
        }
        seen_operations.insert(pc);

        let current_operation = &values[pc];

        match current_operation.operation {
            Op::Acc => accumulator += current_operation.argument,
            Op::Jmp => pc = (pc as isize + current_operation.argument) as usize,
            _ => {}
        }

        if current_operation.operation != Op::Jmp {
            pc += 1;
        }
    }

    (accumulator, halted)
}

fn part_1(values: &Vec<Instruction>) -> isize {
    let (acc, _) = run(&values);
    acc
}

fn part_2(values: &mut Vec<Instruction>) -> isize {
    let mut accumulator: isize = 0;

    for index in 0..values.len() {
        match values[index].operation {
            Op::Nop => values[index].operation = Op::Jmp,
            Op::Jmp => values[index].operation = Op::Nop,
            _ => {}
        }
    
        let (acc, halted) = run(&values);
        if halted {
            accumulator = acc;
            break;
        }

        match values[index].operation {
            Op::Nop => values[index].operation = Op::Jmp,
            Op::Jmp => values[index].operation = Op::Nop,
            _ => {}
        }
    }

    accumulator
}

fn main() {
    let mut inputs: Vec<Instruction> = load_data("day8/input.txt");

    let part_1_answer = part_1(&inputs);
    println!("Part 1: {}", part_1_answer);

    let part_2_answer = part_2(&mut inputs);
    println!("Part 2: {}", part_2_answer);
}
