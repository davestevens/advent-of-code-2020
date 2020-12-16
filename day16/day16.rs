use std::io::Read;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

type Ticket = Vec<usize>;
type Rules = HashMap<String, (usize, usize, usize, usize)>;

#[derive(Debug)]
struct Input {
    rules: Rules,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>
}

fn load_data(filename: &str) -> Input {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let sections: Vec<&str> = contents.split("\r\n\r\n").collect();

    let mut rules: HashMap<String, (usize, usize, usize, usize)> = HashMap::new();
    sections[0].lines().for_each(|line| {
        let key_value: Vec<&str> = line.split(": ").collect();
        let ranges: Vec<&str> = key_value[1].split(" or ").collect();
        let a: Vec<usize> = ranges[0].split("-").map(|r| r.parse::<usize>().unwrap()).collect();
        let b: Vec<usize> = ranges[1].split("-").map(|r| r.parse::<usize>().unwrap()).collect();
        rules.insert(key_value[0].into(), (a[0], a[1], b[0], b[1]));
    });

    let my_ticket: Ticket = sections[1].lines().nth(1).unwrap().split(",").map(|r| r.parse::<usize>().unwrap()).collect();

    let nearby_tickets: Vec<Ticket> = sections[2].lines().skip(1).map(|line| {
        line.split(",").map(|r| r.parse::<usize>().unwrap()).collect()
    }).collect();

    Input { rules, my_ticket, nearby_tickets }
}

fn check_errors(ticket: &Ticket, rules: &Rules) -> usize {
    let mut invalid_values: Vec<usize> = Vec::new();

    for &value in ticket {
        if !rules.into_iter().any(|(_key, rule)| {
            (value >= rule.0 && value <= rule.1) || (value >= rule.2 && value <= rule.3)
        }) {
            invalid_values.push(value);
        }
    }

    invalid_values.iter().sum()
}

fn part_1(values: &Input) -> usize {
    let mut total: usize = 0;

    total += check_errors(&values.my_ticket, &values.rules);

    values.nearby_tickets.iter().for_each(|ticket| {
        total += check_errors(ticket, &values.rules);
    });

    total
}

fn valid_keys_for_value(value: usize, rules: &Rules) -> Vec<String> {
    let mut valid_keys: Vec<String> = Vec::new();

    rules.into_iter().for_each(|(key, rule)| {
        if (value >= rule.0 && value <= rule.1) || (value >= rule.2 && value <= rule.3) {
            valid_keys.push(key.into());
        }
    });

    valid_keys
}

fn valid_keys_for_slot(slot: usize, tickets: &Vec<&Ticket>, rules: &Rules) -> HashSet<String> {
    let valid_keys: Vec<Vec<String>> = tickets.iter().map(|ticket| {
        let value: usize = *ticket.iter().nth(slot).unwrap();
        valid_keys_for_value(value, &rules)
    }).collect();

    HashSet::from_iter(
        rules.into_iter()
        .filter(|(key, _value)| valid_keys.iter().all(|a| a.contains(key)))
        .map(|(key, _value)| key.into())
    )
}

fn build_lookup(mut options: Vec<HashSet<String>>) -> HashMap<String, usize> {
    let mut to_remove: Vec<String> = Vec::new();
    options.iter_mut()
    .filter(|option| option.len() == 1)
    .for_each(|option| {
        let a = option.iter().nth(0).unwrap();
        to_remove.push(a.to_string());
    });

    while to_remove.len() > 0 {
        let a = to_remove.pop().unwrap();
        options.iter_mut().for_each(|option| {
            if option.len() > 1 && option.contains(&a) {
                option.remove(&a);

                if option.len() == 1 {
                    let a = option.iter().nth(0).unwrap();
                    to_remove.push(a.to_string());
                }
            }
        });
    }

    let mut lookup: HashMap<String, usize> = HashMap::new();

    for (index, option) in options.iter().enumerate() {
        let key = option.iter().nth(0).unwrap();
        lookup.insert(key.to_string(), index);
    }

    lookup
}

fn part_2(values: &Input) -> usize {
    let valid_tickets: Vec<&Ticket> = values.nearby_tickets.iter()
    .filter(|t| check_errors(t, &values.rules) == 0)
    .collect();

    let mut possible_options: Vec<HashSet<String>> = vec![HashSet::new(); values.my_ticket.len()];

    for i in 0..values.my_ticket.len() {
        possible_options[i] = valid_keys_for_slot(i, &valid_tickets, &values.rules);
    }

    let lookup: HashMap<String, usize> = build_lookup(possible_options);

    let mut total: usize = 1;
    for (key, value) in lookup.iter() {
        if key.starts_with("departure") {
            total *= *values.my_ticket.get(*value).unwrap();
        }
    }
    total
}

fn main() {
    let mut input: Input = load_data("day16/input.txt");

    let part_1_answer = part_1(&input);
    println!("Part 1: {}", part_1_answer);

    let part_2_answer = part_2(&mut input);
    println!("Part 2: {}", part_2_answer);
}
