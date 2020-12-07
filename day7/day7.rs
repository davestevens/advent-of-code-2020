use std::io::Read;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq)]
struct BagInfo {
    name: String,
    count: usize
}

#[derive(Debug, PartialEq)]
struct Bag {
    parents: Vec<String>,
    children: Vec<BagInfo>
}

fn get_bag_info(input: &str) -> BagInfo {
    let input_split: Vec<&str> = input.split(" ").collect();
    let count = input_split[0].parse::<usize>();

    match count {
        Ok(value) => BagInfo { name: input_split[1..3].join(" "), count: value },
        _ => BagInfo { name: input_split[0..2].join(" "), count: 0 }
    }
}


fn load_data(filename: &str) -> HashMap<String, Bag> {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut map: HashMap<String, Bag> = HashMap::new();

    for line in contents.lines() {
        let line_sections: Vec<&str> = line.split(" contain ").collect();
        let parent_info = get_bag_info(line_sections[0]);

        let children: Vec<BagInfo> = line_sections[1].split(", ").map(|s| get_bag_info(s)).filter(|info| info.name != "no other").collect();
        for child_info in children {
            let parent: &mut Bag = map.entry(parent_info.name.clone()).or_insert(Bag { parents: vec![], children: vec![] });
            parent.children.push(child_info.clone());

            let child: &mut Bag = map.entry(child_info.name.clone()).or_insert(Bag { parents: vec![], children: vec![] });
            child.parents.push(parent_info.name.clone());
        }
    }

    map
}

fn part_1(values: &HashMap<String, Bag>, start: String) -> usize {
    let mut to_visit: Vec<String> = vec![start];
    let mut parents: HashSet<String> = HashSet::new();

    while to_visit.len() > 0 {
        let current = to_visit.pop().unwrap();
        let current_bag = values.get(&current).unwrap();

        for parent in current_bag.parents.iter() {
            if parents.contains(parent) || to_visit.contains(&parent) {
                continue;
            }
            to_visit.push(parent.clone());
            parents.insert(parent.clone());
        }
    }

    parents.len()
}

fn bag_count(values: &HashMap<String, Bag>, bag: String) -> usize {
    let current_bag = values.get(&bag).unwrap();
    let total_children: usize = current_bag.children
        .iter()
        .map(|child| bag_count(&values, child.name.to_string()) * child.count)
        .sum();
    
    total_children + 1
}

fn part_2(values: &HashMap<String, Bag>, start: String) -> usize {
    bag_count(&values, start) - 1
}

fn main() {
    let inputs: HashMap<String, Bag> = load_data("day7/input.txt");

    let part_1_answer = part_1(&inputs, String::from("shiny gold"));
    println!("Part 1: {}", part_1_answer);

    let part_2_answer = part_2(&inputs, String::from("shiny gold"));
    println!("Part 2: {}", part_2_answer);
}
