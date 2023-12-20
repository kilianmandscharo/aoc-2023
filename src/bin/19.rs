use std::collections::{HashMap, VecDeque};

use aoc::readlines_with_empty;
use regex::Regex;

fn main() {
    a();
    b();
}

#[derive(Debug)]
enum Rule {
    Simple(String),
    Comparison(Comparison),
}

#[derive(Debug)]
struct Comparison {
    key: char,
    cmp: fn(i32, i32) -> bool,
    value: i32,
    target: String,
}

#[derive(Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

fn parse_workflow(line: &str) -> Vec<Rule> {
    let mut rules: Vec<Rule> = Vec::new();
    for rule in line.split(",") {
        if let Some((cmp, target)) = rule.split_once(":") {
            let operator =
                cmp.chars().collect::<Vec<char>>()[cmp.find(|c| c == '<' || c == '>').unwrap()];
            let (key, val) = cmp.split_once(operator).unwrap();
            let cmp = if operator == '<' {
                |a, b| a < b
            } else {
                |a, b| a > b
            };
            rules.push(Rule::Comparison(Comparison {
                key: key.chars().next().unwrap(),
                cmp,
                value: val.parse().unwrap(),
                target: target.to_string(),
            }));
        } else {
            rules.push(Rule::Simple(rule.to_string()));
        }
    }
    rules
}

fn compare_part(part: &Part, cmp: &Comparison) -> Option<String> {
    let val = match cmp.key {
        'x' => part.x,
        'm' => part.m,
        'a' => part.a,
        's' => part.s,
        _ => panic!(),
    };
    if (cmp.cmp)(val, cmp.value) {
        Some(cmp.target.to_owned())
    } else {
        None
    }
}

fn a() {
    let mut reached_blank = false;
    let mut workflows: HashMap<String, Vec<Rule>> = HashMap::new();
    let mut parts: Vec<Part> = Vec::new();
    for line in readlines_with_empty("19") {
        if line.is_empty() {
            reached_blank = true;
            continue;
        }
        if !reached_blank {
            let (key, rest) = line.split_once("{").unwrap();
            let rules = parse_workflow(&rest[..rest.len() - 1]);
            workflows.insert(key.to_string(), rules);
        } else {
            let re = Regex::new("[0-9]+").unwrap();
            let matches: Vec<i32> = re
                .find_iter(&line)
                .map(|m| m.as_str().parse::<i32>().unwrap())
                .collect();
            parts.push(Part {
                x: matches[0],
                m: matches[1],
                a: matches[2],
                s: matches[3],
            })
        }
    }
    let mut sum = 0;
    for part in parts {
        let mut current_target = "in".to_string();
        while current_target != "R" {
            if current_target == "A" {
                sum += part.x + part.m + part.a + part.s;
                break;
            }
            let rules = workflows.get(&current_target).unwrap();
            for rule in rules {
                match rule {
                    Rule::Simple(target) => {
                        current_target = target.to_owned();
                        break;
                    }
                    Rule::Comparison(cmp) => {
                        if let Some(target) = compare_part(&part, cmp) {
                            current_target = target;
                            break;
                        }
                    }
                }
            }
        }
    }
    println!("{sum}");
}

#[derive(Debug, Clone, Copy)]
struct Range(i32, i32);

impl Range {
    fn overlap(self, other: Range) -> Option<Range> {
        let Range(self_lower, self_upper) = self;
        let Range(other_lower, other_upper) = other;
        if self_lower > other_upper || self_upper < other_lower {
            return None;
        }
        Some(Range(
            std::cmp::max(self_lower, other_lower),
            std::cmp::min(self_upper, other_upper),
        ))
    }
}

#[derive(Debug, Clone)]
struct RangePart {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
    target: String,
}

impl RangePart {
    fn get_range(&self, c: char) -> Range {
        match c {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!(),
        }
    }
    fn set_range(&mut self, c: char, r: Range) {
        match c {
            'x' => self.x = r,
            'm' => self.m = r,
            'a' => self.a = r,
            's' => self.s = r,
            _ => panic!(),
        }
    }
}

fn cmp_to_ranges(cmp: &Comparison) -> (Range, Range) {
    match (cmp.cmp)(1, 2) {
        true => (Range(1, cmp.value - 1), Range(cmp.value, 4000)),
        false => (Range(cmp.value + 1, 4000), Range(1, cmp.value)),
    }
}

fn b() {
    let mut workflows: HashMap<String, Vec<Rule>> = HashMap::new();
    for line in readlines_with_empty("19") {
        if line.is_empty() {
            break;
        }
        let (key, rest) = line.split_once("{").unwrap();
        let rules = parse_workflow(&rest[..rest.len() - 1]);
        workflows.insert(key.to_string(), rules);
    }

    let mut parts: VecDeque<RangePart> = VecDeque::new();
    parts.push_back(RangePart {
        x: Range(1, 4000),
        m: Range(1, 4000),
        a: Range(1, 4000),
        s: Range(1, 4000),
        target: "in".to_string(),
    });

    let mut sum: u64 = 0;

    while let Some(part) = parts.pop_front() {
        if part.target == "R" {
            continue;
        }
        if part.target == "A" {
            sum += (part.x.1 as u64 - part.x.0 as u64 + 1)
                * (part.m.1 as u64 - part.m.0 as u64 + 1)
                * (part.a.1 as u64 - part.a.0 as u64 + 1)
                * (part.s.1 as u64 - part.s.0 as u64 + 1);
            continue;
        }
        let workflow = workflows.get(&part.target).unwrap();
        let mut current_part = part.clone();
        for rule in workflow {
            match rule {
                Rule::Simple(target) => {
                    current_part.target = target.to_string();
                    parts.push_back(current_part);
                    break;
                }
                Rule::Comparison(cmp) => {
                    let (inner, outer) = cmp_to_ranges(cmp);
                    if let Some(range) = current_part.get_range(cmp.key).overlap(inner) {
                        let mut new_part = current_part.clone();
                        new_part.set_range(cmp.key, range);
                        new_part.target = cmp.target.clone();
                        parts.push_back(new_part);
                        if let Some(range) = current_part.get_range(cmp.key).overlap(outer) {
                            current_part.set_range(cmp.key, range);
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    println!("{sum}");
}
