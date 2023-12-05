use aoc::readlines;
use regex::Regex;

fn main() {
    a();
    b();
}

struct Range {
    to_start: i64,
    from_start: i64,
    n: i64,
}

fn get_maps(lines: &Vec<String>) -> Vec<Vec<Range>> {
    let re = Regex::new("[0-9]+").unwrap();
    let mut maps: Vec<Vec<Range>> = Vec::new();
    let mut current_map: Vec<Range> = Vec::new();
    for line in lines.iter().skip(2) {
        let matches: Vec<i64> = re
            .find_iter(line)
            .filter(|m| m.as_str().parse::<i64>().is_ok())
            .map(|m| m.as_str().parse::<i64>().unwrap())
            .collect();
        if matches.len() == 0 {
            maps.push(current_map);
            current_map = Vec::new();
        } else {
            current_map.push(Range {
                to_start: matches[0],
                from_start: matches[1],
                n: matches[2],
            })
        }
    }
    maps.push(current_map);
    maps
}

fn a() {
    let lines = readlines("05");
    let maps = get_maps(&lines);
    let re = Regex::new("[0-9]+").unwrap();

    let seeds = re
        .find_iter(&lines[0])
        .map(|s| s.as_str().parse::<i64>().unwrap());

    let mut min = None;
    for seed in seeds {
        let mut val = seed;
        for map in &maps {
            for range in map {
                if val >= range.from_start && val <= range.from_start + range.n {
                    val = val - (range.from_start - range.to_start);
                    break;
                }
            }
        }
        min = if min.is_none() {
            Some(val)
        } else {
            Some(std::cmp::min(min.unwrap(), val))
        }
    }
    println!("{}", min.unwrap());
}

fn b() {
    let re = Regex::new("[0-9]+").unwrap();
    let lines = readlines("05");
    let maps = get_maps(&lines);

    struct SeedRange {
        start: i64,
        end: i64,
    }

    let mut seeds: Vec<SeedRange> = Vec::new();
    let nums = re
        .find_iter(&lines[0])
        .map(|s| s.as_str().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    for i in 1..nums.len() {
        if i % 2 != 0 {
            seeds.push(SeedRange {
                start: nums[i - 1],
                end: nums[i - 1] + nums[i],
            })
        }
    }

    let mut loc: i64 = 0;
    'outer: loop {
        let mut val = loc;
        for map in maps.iter().rev() {
            for range in map.iter().rev() {
                if val >= range.to_start && val <= range.to_start + range.n {
                    val = val - (range.to_start - range.from_start);
                    break;
                }
            }
        }

        for seed in &seeds {
            if val >= seed.start && val < seed.end {
                println!("{loc}");
                break 'outer;
            }
        }

        loc += 1;
    }
}
