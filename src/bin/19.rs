use std::collections::HashMap;

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

fn b() {}
