use aoc::readlines;
use std::collections::HashSet;

fn main() {
    a();
    b();
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Stop,
}

fn step(grid: &Vec<Vec<char>>, p: Point, d: Direction) -> (Point, Direction) {
    let next_point = match d {
        Direction::Down => Point { x: p.x, y: p.y + 1 },
        Direction::Up => Point { x: p.x, y: p.y - 1 },
        Direction::Left => Point { x: p.x - 1, y: p.y },
        Direction::Right => Point { x: p.x + 1, y: p.y },
        Direction::Stop => Point { x: p.x, y: p.y },
    };
    let new_direction = match grid[next_point.y][next_point.x] {
        '|' => d,
        '-' => d,
        'L' => match d {
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Up,
            _ => panic!(),
        },
        'J' => match d {
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Up,
            _ => panic!(),
        },
        '7' => match d {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Down,
            _ => panic!(),
        },
        'F' => match d {
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Down,
            _ => panic!(),
        },
        'S' => Direction::Stop,
        _ => panic!(),
    };
    (next_point, new_direction)
}

fn make_grid() -> Vec<Vec<char>> {
    readlines("10")
        .iter()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_start(grid: &Vec<Vec<char>>) -> Point {
    let mut start = Point { x: 0, y: 0 };
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'S' {
                start.y = i;
                start.x = j;
            }
        }
    }
    start
}

fn get_raycast_intersections(
    grid: &Vec<Vec<char>>,
    visited: &HashSet<Point>,
    point_to_check: Point,
) -> i32 {
    let mut intersections = 0;
    let mut last_open: Option<char> = None;
    for col in 0..point_to_check.x {
        let p = Point {
            x: col,
            y: point_to_check.y,
        };
        // Ignore any non-loop elements
        if !visited.contains(&p) {
            continue;
        }
        match grid[point_to_check.y][col] {
            '|' => intersections += 1,
            'L' => {
                intersections += 1;
                last_open = Some('L');
            }
            'F' => {
                intersections += 1;
                last_open = Some('F');
            }
            '7' => {
                if let Some(last_open) = last_open {
                    if last_open == 'F' {
                        intersections += 1;
                    }
                }
            }
            'J' => {
                if let Some(last_open) = last_open {
                    if last_open == 'L' {
                        intersections += 1;
                    }
                }
            }
            _ => {}
        }
    }
    intersections
}

fn a() {
    let grid = make_grid();

    let mut current_point = find_start(&grid);
    let mut direction = Direction::Left; // the initial direction is known
    let mut count = 0;

    while direction != Direction::Stop {
        (current_point, direction) = step(&grid, current_point, direction);
        count += 1;
    }

    println!("{}", count / 2);
}

fn b() {
    let mut grid = make_grid();

    let mut current_point = find_start(&grid);
    let mut points: Vec<Point> = Vec::new();
    let mut direction = Direction::Left; // the initial direction is known

    while direction != Direction::Stop {
        points.push(current_point);
        (current_point, direction) = step(&grid, current_point, direction);
    }

    let visited = points.clone().into_iter().collect::<HashSet<Point>>();
    grid[current_point.y][current_point.x] = '7'; // Replace S with correct pipe

    let mut count = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if !visited.contains(&Point { x, y })
                && get_raycast_intersections(&grid, &visited, Point { x, y }) % 2 != 0
            {
                count += 1;
            }
        }
    }

    println!("{count}");
}
