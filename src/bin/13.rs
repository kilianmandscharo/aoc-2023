use aoc::readlines_with_empty;

fn main() {
    a();
    b();
}

type Grid = Vec<Vec<char>>;

#[derive(PartialEq, Eq)]
enum LineType {
    Horizontal,
    Vertical,
}

struct Line {
    line_type: LineType,
    i: usize,
}

impl std::cmp::PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.line_type == other.line_type && self.i == other.i
    }
}

impl std::cmp::Eq for Line {}

fn check_grid(grid: &Grid) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();

    // Check for horizontal line
    for i in 1..grid.len() {
        let mut is_symmetrical = true;
        let n = if i <= grid.len() / 2 {
            i
        } else {
            grid.len() - i
        };
        for j in 0..n {
            let lower = i - 1 - j;
            let upper = i + j;
            if grid[lower] != grid[upper] {
                is_symmetrical = false;
                break;
            }
        }
        if is_symmetrical {
            lines.push(Line {
                line_type: LineType::Horizontal,
                i,
            });
        }
    }

    // Check for vertical line
    for i in 1..grid[0].len() {
        let mut is_symmetrical = true;
        let n = if i <= grid[0].len() / 2 {
            i
        } else {
            grid[0].len() - i
        };
        for j in 0..n {
            let lower = i - 1 - j;
            let upper = i + j;
            if grid.iter().map(|l| l[lower]).collect::<Vec<char>>()
                != grid.iter().map(|l| l[upper]).collect::<Vec<char>>()
            {
                is_symmetrical = false;
                break;
            }
        }
        if is_symmetrical {
            lines.push(Line {
                line_type: LineType::Vertical,
                i,
            })
        }
    }

    lines
}

fn get_line_sum(l: &Line) -> usize {
    match l.line_type {
        LineType::Vertical => l.i,
        LineType::Horizontal => l.i * 100,
    }
}

fn a() {
    let mut grid: Grid = Vec::new();
    let mut sum = 0;
    for line in readlines_with_empty("13") {
        if line.is_empty() {
            sum += get_line_sum(&check_grid(&grid)[0]);
            grid = Vec::new();
        } else {
            grid.push(line.chars().collect());
        }
    }
    println!("{sum}");
}

fn b() {
    let mut grid: Grid = Vec::new();
    let mut sum = 0;
    for line in readlines_with_empty("13") {
        if line.is_empty() {
            let initial = check_grid(&grid);
            let mut done = false;
            for y in 0..grid.len() {
                if done {
                    break;
                }
                for x in 0..grid[0].len() {
                    grid[y][x] = if grid[y][x] == '#' { '.' } else { '#' };
                    let new = check_grid(&grid);
                    if let Some(l) = new.iter().find(|l| **l != initial[0]) {
                        sum += get_line_sum(&l);
                        done = true;
                        break;
                    }
                    grid[y][x] = if grid[y][x] == '#' { '.' } else { '#' };
                }
            }
            grid = Vec::new();
        } else {
            grid.push(line.chars().collect());
        }
    }
    println!("{sum}");
}
