use std::fs;

pub fn readlines(filename: &str) -> Vec<String> {
    let file = fs::read_to_string(format!(
        "/home/dominik/programming/aoc-2023/input/{filename}.in"
    ))
    .expect("could not read file");
    return file
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
}

pub fn readlines_with_empty(filename: &str) -> Vec<String> {
    let file = fs::read_to_string(format!(
        "/home/dominik/programming/aoc-2023/input/{filename}.in"
    ))
    .expect("could not read file");
    return file
        .split("\n")
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
}
