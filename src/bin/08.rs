use aoc::readlines;
use core::panic;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    a();
    b();
}

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

fn parse_input() -> (HashMap<String, Node>, Vec<char>) {
    let re = Regex::new("[A-Z]+").unwrap();
    let lines = readlines("08");
    let directions = lines[0].chars().collect::<Vec<_>>();

    let mut map: HashMap<String, Node> = HashMap::new();
    for line in lines.iter().skip(1) {
        let (name, child_names) = line.split_once(" = ").unwrap();
        let names = re
            .find_iter(child_names)
            .map(|m| m.as_str())
            .collect::<Vec<_>>();
        let node = Node {
            left: names[0].to_string(),
            right: names[1].to_string(),
        };
        map.insert(name.to_string(), node);
    }

    (map, directions)
}

fn get_count<F>(start: &str, map: &HashMap<String, Node>, directions: &Vec<char>, is_end: F) -> i64
where
    F: Fn(&str) -> bool,
{
    let mut count = 0;
    let mut direction_index = 0;
    let mut current = start;

    loop {
        if is_end(current) {
            break;
        }

        match directions[direction_index] {
            'L' => current = &map.get(current).unwrap().left,
            'R' => current = &map.get(current).unwrap().right,
            _ => panic!(),
        }

        direction_index = (direction_index + 1) % directions.len();
        count += 1;
    }

    count
}

fn a() {
    let (map, directions) = parse_input();
    println!(
        "{}",
        get_count("AAA", &map, &directions, |s| { s == "ZZZ" })
    )
}

fn b() {
    let (map, directions) = parse_input();
    let starting_points = map.keys().filter(|key| key.ends_with("A"));
    let mut counts = Vec::new();

    for start in starting_points {
        counts.push(get_count(start, &map, &directions, |s| s.ends_with("Z")));
    }

    let mut vals = counts.clone();
    loop {
        let first = vals.first().unwrap();
        if vals.iter().all(|val| val == first) {
            break;
        }
        let min = vals.iter().min().unwrap();
        let i = vals.iter().position(|val| val == min).unwrap();
        vals[i] += counts[i];
    }
    println!("{}", vals[0]);
}
