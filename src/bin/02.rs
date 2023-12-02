use aoc::readlines;

fn main() {
    a();
    b();
}

fn a() {
    let mut sum = 0;
    for (i, line) in readlines("02").iter().enumerate() {
        let (_, rest) = line.split_once(": ").unwrap();
        let mut possible = true;
        for round in rest.split("; ") {
            for draw in round.split(", ") {
                let (n, color) = draw.split_once(" ").unwrap();
                let limit = match color {
                    "red" => 12,
                    "green" => 13,
                    "blue" => 14,
                    _ => panic!("unknown color"),
                };
                if n.parse::<i32>().unwrap() > limit {
                    possible = false;
                }
            }
        }
        if possible {
            sum += i + 1;
        }
    }
    println!("{sum}");
}

fn b() {
    let mut sum = 0;
    for line in readlines("02") {
        let (_, rest) = line.split_once(": ").unwrap();
        let mut blue = 1;
        let mut red = 1;
        let mut green = 1;
        for round in rest.split("; ") {
            for draw in round.split(", ") {
                let (n, color) = draw.split_once(" ").unwrap();
                let val: i32 = n.parse().unwrap();
                match color {
                    "red" => {
                        red = std::cmp::max(red, val);
                    }
                    "blue" => {
                        blue = std::cmp::max(blue, val);
                    }
                    "green" => {
                        green = std::cmp::max(green, val);
                    }
                    _ => panic!("unknown color"),
                }
            }
        }
        sum += blue * red * green;
    }
    println!("{sum}");
}
