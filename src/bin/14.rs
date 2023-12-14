#![allow(dead_code)]
use aoc::readlines;

fn main() {
    a();
    b();
}

struct Grid {
    data: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn get(&self, y: usize, x: usize) -> char {
        self.data[y][x]
    }
    fn swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let tmp = self.data[y1][x1];
        self.data[y1][x1] = self.data[y2][x2];
        self.data[y2][x2] = tmp;
    }
    fn tilt_right(&mut self) {
        for y in 0..self.height {
            let mut last_rock = self.width - 1;
            loop {
                if last_rock == 0 {
                    break;
                }
                // find the next rounded rock
                let mut x = last_rock - 1;
                while x > 0 {
                    if self.get(y, x) == 'O' {
                        break;
                    }
                    x -= 1;
                }
                // no rounded rock found
                if self.get(y, x) != 'O' {
                    break;
                }
                // push it to the bottom
                while x < last_rock && self.get(y, x + 1) == '.' {
                    self.swap(x, y, x + 1, y);
                    x += 1;
                }
                last_rock = x;
            }
        }
    }
    fn tilt_left(&mut self) {
        for y in 0..self.height {
            let mut last_rock = 0;
            loop {
                if last_rock == self.width - 1 {
                    break;
                }
                // find the next rounded rock
                let mut x = last_rock + 1;
                while x < self.width - 1 {
                    if self.get(y, x) == 'O' {
                        break;
                    }
                    x += 1;
                }
                // no rounded rock found
                if self.get(y, x) != 'O' {
                    break;
                }
                // push it to the bottom
                while x > last_rock && self.get(y, x - 1) == '.' {
                    self.swap(x, y, x - 1, y);
                    x -= 1;
                }
                last_rock = x;
            }
        }
    }
    fn tilt_down(&mut self) {
        for x in 0..self.width {
            let mut last_rock = self.height - 1;
            loop {
                if last_rock == 0 {
                    break;
                }
                // find the next rounded rock
                let mut y = last_rock - 1;
                while y > 0 {
                    if self.get(y, x) == 'O' {
                        break;
                    }
                    y -= 1;
                }
                // no rounded rock found
                if self.get(y, x) != 'O' {
                    break;
                }
                // push it to the bottom
                while y < last_rock && self.get(y + 1, x) == '.' {
                    self.swap(x, y, x, y + 1);
                    y += 1;
                }
                last_rock = y;
            }
        }
    }
    fn tilt_up(&mut self) {
        for x in 0..self.width {
            let mut last_rock = 0;
            loop {
                if last_rock == self.height - 1 {
                    break;
                }
                // find the next rounded rock
                let mut y = last_rock + 1;
                while y < self.height - 1 {
                    if self.get(y, x) == 'O' {
                        break;
                    }
                    y += 1;
                }
                // no rounded rock found
                if self.get(y, x) != 'O' {
                    break;
                }
                // push it to the top
                while y > last_rock && self.get(y - 1, x) == '.' {
                    self.swap(x, y, x, y - 1);
                    y -= 1;
                }
                last_rock = y;
            }
        }
    }
    fn calculate_load(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .map(|(i, row)| (self.height - i) * row.into_iter().filter(|c| **c == 'O').count())
            .fold(0, |acc, val| acc + val)
    }
    fn cycle(&mut self) {
        self.tilt_up();
        self.tilt_left();
        self.tilt_down();
        self.tilt_right();
    }
}

fn parse_grid() -> Grid {
    let data: Vec<Vec<char>> = readlines("14")
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();
    let width = data[0].len();
    let height = data.len();
    Grid {
        data,
        width,
        height,
    }
}

fn a() {
    let mut grid = parse_grid();
    grid.tilt_up();
    let total_load = grid.calculate_load();
    println!("{total_load}");
}

fn b() {
    // Uncomment below for investigating the cycle
    /*let mut grid = parse_grid();
     for i in 0..1000 {
        grid.cycle();
        println!("{} -> {}", i + 1, grid.calculate_load())
    }*/

    // The investigation reveals that after 121 cycles the process falls into
    // this repeating loop, which means that calculating the value at
    // 1_000_000_000 is trivial
    let looped_values = vec![
        79743, 79742, 79734, 79723, 79716, 79708, 79714, 79711, 79708, 79718, 79724, 79727, 79731,
        79718, 79723, 79730, 79736, 79733,
    ];
    let loop_reached_at = 122;
    let final_loop_value = looped_values[(1_000_000_000 - loop_reached_at) % looped_values.len()];
    println!("{final_loop_value}");
}
