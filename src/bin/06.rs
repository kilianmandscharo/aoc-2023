use std::iter::zip;

use aoc::readlines;
use regex::Regex;

fn main() {
    a();
    b();
}

fn possible_wins(time: i64, n: i64) -> i64 {
    if time % 2 == 0 {
        2 * ((time / 2) - n) + 1
    } else {
        2 * ((time / 2 + 1) - n)
    }
}

fn a() {
    let lines = readlines("06");
    let re = Regex::new("[0-9]+").unwrap();

    let times = re
        .find_iter(&lines[0])
        .map(|m| m.as_str().parse::<i64>().unwrap());

    let distances = re
        .find_iter(&lines[1])
        .map(|m| m.as_str().parse::<i64>().unwrap());

    let mut total = 1;

    for (time, distance) in zip(times, distances) {
        for n in 0..=time {
            if (time - n) * n > distance {
                total *= possible_wins(time, n);
                break;
            }
        }
    }

    println!("{total}");
}

fn b() {
    let lines = readlines("06");

    let time = lines[0]
        .replace(|c: char| !c.is_numeric(), "")
        .parse::<i64>()
        .unwrap();

    let distance = lines[1]
        .replace(|c: char| !c.is_numeric(), "")
        .parse::<i64>()
        .unwrap();

    // Can be optimized with binary search
    for n in 0..=time {
        if (time - n) * n > distance {
            println!("{}", possible_wins(time, n));
            break;
        }
    }
}
