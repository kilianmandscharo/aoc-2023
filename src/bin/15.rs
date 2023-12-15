use aoc::readlines;

fn main() {
    a();
    b();
}

fn hash(s: &str) -> usize {
    s.chars()
        .map(|c| c as u8)
        .fold(0, |acc, val| ((acc + val as usize) * 17) % 256)
}

#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: usize,
}

fn a() {
    let lines = readlines("15");
    let sum = lines[0]
        .split(",")
        .map(|s| hash(s))
        .fold(0, |acc, val| acc + val);
    println!("{sum}");
}

fn b() {
    let lines = readlines("15");
    let mut boxes: Vec<Vec<Lens>> = Vec::with_capacity(256);
    for _ in 0..256 {
        boxes.push(Vec::new());
    }
    for step in lines[0].split(",") {
        if step.find("=").is_some() {
            let (label, focal_length) = step.split_once("=").unwrap();
            let focal_length = focal_length.parse::<usize>().unwrap();
            let lenses = &mut boxes[hash(label)];
            if let Some(index) = lenses.iter().position(|lens| lens.label == label) {
                lenses[index].focal_length = focal_length;
            } else {
                lenses.push(Lens {
                    label: label.to_string(),
                    focal_length,
                });
            }
        } else {
            let (label, _) = step.split_once("-").unwrap();
            let lenses = &mut boxes[hash(label)];
            if let Some(index) = lenses.iter().position(|lens| lens.label == label) {
                lenses.remove(index);
            }
        }
    }
    let mut sum = 0;
    for (i, b) in boxes.into_iter().enumerate() {
        for (pos, lens) in b.into_iter().enumerate() {
            sum += (i + 1) * (pos + 1) * lens.focal_length;
        }
    }
    println!("{sum}");
}
