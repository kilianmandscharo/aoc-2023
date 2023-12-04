use std::collections::HashMap;

use aoc::readlines;

fn main() {
    a();
    b();
}

fn a() {
    let mut points = 0;
    for line in readlines("04") {
        let (left, right) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
        let mut map = HashMap::new();
        for val in left.split(" ") {
            if !val.is_empty() {
                map.insert(val, true);
            }
        }
        let count = right
            .split(" ")
            .filter(|val| map.get(val).is_some())
            .count();
        if count > 0 {
            points += (2 as i32).pow(count as u32 - 1);
        }
    }
    println!("{points}");
}

fn b() {
    let lines = readlines("04");
    let mut counts = vec![1; lines.len()];
    for (i, line) in lines.iter().enumerate() {
        let (left, right) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
        let mut map = HashMap::new();
        for val in left.split(" ") {
            if !val.is_empty() {
                map.insert(val, true);
            }
        }
        let count = right
            .split(" ")
            .filter(|val| map.get(val).is_some())
            .count();
        for card_idx in i + 1..=i + count {
            counts[card_idx] += counts[i];
        }
    }
    println!("{}", counts.iter().fold(0, |acc, val| acc + val));
}
