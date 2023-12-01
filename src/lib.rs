use std::fs;

pub fn readlines(filename: &str) -> Vec<String> {
    let file = fs::read_to_string(format!(
        "/home/dominik/projects/aoc-2023/input/{filename}.in"
    ))
    .expect("could not read file");
    return file
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
}
