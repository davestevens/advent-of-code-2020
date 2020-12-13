use std::io::Read;

#[derive(Debug, PartialEq)]
struct Input {
    target: usize,
    ids: Vec<(usize, usize)>
}

fn load_data(filename: &str) -> Input {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.split("\r\n").collect();
    let target: usize = lines[0].parse::<usize>().unwrap();
    let ids: Vec<(usize, usize)> = lines[1].split(",").enumerate().map(|(index, id)| {
        let parsed_id = id.parse::<usize>();
        match parsed_id {
            Ok(id) => (index, id),
            _ => (index, 0)
        }
    })
    .filter(|(_, id)| id != &0_usize)
    .collect();

    Input { target, ids }
}

fn part_1(input: &Input) -> usize {
    let mut target = input.target;
    loop {
        for (_, id) in input.ids.iter() {
            if (target % id) == 0 {
                return (target - input.target) * id;
            }
        }

        target += 1;
    }
}

fn part_2(input: &Input) -> usize {
    let mut current_index: usize = 2;
    let mut time: usize = 0;

    while current_index <= input.ids.len() {
        let current = input.ids[0..current_index].to_vec();
        let multiple = current[0..current.len() - 1].iter().fold(1, |sum, (_, id)| sum * id);

        loop {
            if current.iter().all(|(index, id)| (time + index) % id == 0) {
                break;
            }
            time += multiple;
        }

        current_index += 1;
    }

    time
}

fn main() {
    let input: Input = load_data("day13/input.txt");

    let part_1_answer = part_1(&input);
    println!("Part 1: {}", part_1_answer);

    let part_2_answer = part_2(&input);
    println!("Part 2: {}", part_2_answer);
}
