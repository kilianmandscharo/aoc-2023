use aoc::readlines;
use regex::{Match, Regex};

fn main() {
    a();
    b();
}

fn a() {
    let num_re = Regex::new("[0-9]+").unwrap(); // match all groups of numbers
    let symbol_re = Regex::new("[^\\d.]").unwrap(); // match all symbols that are not dots
    let mut sum = 0;
    let lines = readlines("03");
    let w = lines[0].len();
    let h = lines.len();
    for (i, line) in lines.iter().enumerate() {
        let matches = num_re.find_iter(line);
        for m in matches {
            let left = if m.start() == 0 { 0 } else { m.start() - 1 };
            let right = if m.end() >= w - 1 { w - 1 } else { m.end() + 1 };
            if i > 0 && symbol_re.find(&lines[i - 1][left..right]).is_some()
                || symbol_re.find(&line[left..right]).is_some()
                || i < h - 1 && symbol_re.find(&lines[i + 1][left..right]).is_some()
            {
                sum += m.as_str().parse::<i32>().unwrap();
            }
        }
    }
    println!("{sum}");
}

fn b() {
    let star_re = Regex::new("\\*").unwrap();
    let num_re = Regex::new("[0-9]+").unwrap();
    let mut sum = 0;
    let lines = readlines("03");
    let mut i = 1;
    for line in &lines[i..lines.len() - 1] {
        let matches = star_re.find_iter(line);
        for m in matches {
            let left = if m.start() == 0 { 0 } else { m.start() - 1 };
            let right = m.end();
            let mut num_matches: Vec<Match> = Vec::new();
            for i in i - 1..=i + 1 {
                num_matches.extend(num_re.find_iter(&lines[i]));
            }
            let adjacent: Vec<i32> = num_matches
                .iter()
                .filter(|nm| nm.start() <= right && nm.end() - 1 >= left)
                .map(|nm| nm.as_str().parse::<i32>().unwrap())
                .collect();
            if adjacent.len() == 2 {
                sum += adjacent[0] * adjacent[1];
            }
        }
        i += 1;
    }
    println!("{sum}");
}
