use aoc::readlines;

fn main() {
    a();
    b();
}

fn a() {
    let mut sum = 0;
    for line in readlines("01") {
        let first = line.chars().find(|c| c.is_numeric()).unwrap();
        let last = line.chars().rev().find(|c| c.is_numeric()).unwrap();
        sum += format!("{first}{last}").parse::<i32>().unwrap();
    }
    println!("{sum}");
}

fn b() {
    fn match_to_number(s: &str) -> Option<i32> {
        match s {
            "one" => Some(1),
            "two" => Some(2),
            "three" => Some(3),
            "four" => Some(4),
            "five" => Some(5),
            "six" => Some(6),
            "seven" => Some(7),
            "eight" => Some(8),
            "nine" => Some(9),
            s if s.parse::<i32>().is_ok() => s.parse::<i32>().ok(),
            _ => None,
        }
    }

    let mut sum = 0;
    for line in readlines("01") {
        let mut nums = Vec::new();
        for i in 0..line.len() {
            for j in i..=line.len() {
                if let Some(num) = match_to_number(&line[i..j]) {
                    nums.push(num);
                }
            }
        }
        let first = nums.first().unwrap();
        let last = nums.last().unwrap();
        sum += format!("{first}{last}").parse::<i32>().unwrap();
    }
    println!("{sum}");
}
