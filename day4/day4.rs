use std::io::Read;
use std::collections::HashMap;

type Passport = HashMap<String, String>;
type Passports = Vec<Passport>;

fn load_data(filename: &str) -> Passports {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut v: Passports = Vec::new();
    let mut passport: Passport = HashMap::new();
    for s in contents.lines() {
        if s.trim().is_empty() {
            v.push(passport);
            passport = HashMap::new();
            continue;
        }

        s.trim().split_whitespace().for_each(|keyvalue| {
            let split: Vec<&str> = keyvalue.split(":").collect();
            passport.insert(split[0].into(), split[1].into());
        });
    }

    if passport.keys().len() > 0 {
        v.push(passport);
    }

    v
}

const EXPECTED_KEYS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn part_1(values: &Passports) -> usize {
    values.iter().filter(|&passport| {
        EXPECTED_KEYS.iter().all(|&key| passport.contains_key(key.into()))
    }).count()
}

fn is_between_values(value: &str, min: usize, max: usize) -> bool {
    let value_num: usize = value.parse::<usize>().unwrap();
    value_num >= min && value_num <= max
}

fn is_valid_measure(value: &str, units: &str, min: usize, max: usize) -> bool {
    let value_strip = value[..value.len() - units.len()].to_string();
    is_between_values(&value_strip, min, max)
}

fn is_valid_height(value: &str) -> bool {
    if value.ends_with("cm") {
        if !(is_valid_measure(&value, &"cm", 150, 193)) {
            return false;
        }
    } else if value.ends_with("in") {
        if !(is_valid_measure(&value, &"in", 59, 76)) {
            return false;
        }
    } else {
        return false;
    }
    return true
}

fn is_valid_hex(value: &str) -> bool {
    if !value.starts_with('#') {
        return false;
    }
    let value_strip = value[1..value.len()].to_string();
    is_length(&value_strip, 6) && value_strip.chars().all(|c| "01234567890abcdef".contains(c))
}

fn is_length(value: &str, length: usize) -> bool {
    value.len() == length
}

fn exists_in_array(value: &str, array: &[&str]) -> bool {
    array.contains(&value)
}

const EXPECTED_EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn part_2(values: &Passports) -> usize {
    values.iter().filter(|&passport| {
        EXPECTED_KEYS.iter().all(|&key| passport.contains_key(key.into())) &&
        is_between_values(&passport.get("byr").unwrap(), 1920, 2002) &&
        is_between_values(&passport.get("iyr").unwrap(), 2010, 2020) &&
        is_between_values(&passport.get("eyr").unwrap(), 2020, 2030) &&
        is_valid_height(&passport.get("hgt").unwrap()) &&
        is_valid_hex(&passport.get("hcl").unwrap()) &&
        exists_in_array(&passport.get("ecl").unwrap(), &EXPECTED_EYE_COLORS) &&
        is_length(&passport.get("pid").unwrap(), 9)
    }).count()
}

fn main() {
    let inputs: Passports = load_data("day4/input.txt");

    let part_1_answer = part_1(&inputs);
    println!("Part 1: {}", part_1_answer);

    let part_2_answer = part_2(&inputs);
    println!("Part 2: {}", part_2_answer);
}
