use aoc::readlines;

fn main() {
    a();
    b();
}

#[derive(Clone, Copy)]
enum Side {
    Front,
    Back,
}

fn get_side(nums: &[i32], side: Side) -> i32 {
    if nums.iter().all(|num| *num == 0) {
        return 0;
    }
    let next_row: Vec<i32> = nums.windows(2).map(|w| w[1] - w[0]).collect();
    let num = get_side(&next_row, side);
    return match side {
        Side::Front => nums.first().unwrap() - num,
        Side::Back => nums.last().unwrap() + num,
    };
}

fn get_sum(side: Side) -> i32 {
    return readlines("09")
        .iter()
        .map(|line| {
            line.split(" ")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|nums| get_side(&nums, side))
        .fold(0, |acc, val| acc + val);
}

fn a() {
    println!("{}", get_sum(Side::Back));
}

fn b() {
    println!("{}", get_sum(Side::Front));
}
