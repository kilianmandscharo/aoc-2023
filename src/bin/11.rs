use aoc::readlines;

fn main() {
    a();
    b();
}

#[derive(Debug, Copy, Clone)]
struct Galaxy {
    x: usize,
    y: usize,
}

fn shift_galaxies_vert(galaxies: &mut [Galaxy], y: usize, shift_amount: usize) {
    for galaxy in galaxies {
        if galaxy.y > y {
            galaxy.y += shift_amount;
        }
    }
}

fn shift_galaxies_hor(galaxies: &mut [Galaxy], x: usize, shift_amount: usize) {
    for galaxy in galaxies {
        if galaxy.x > x {
            galaxy.x += shift_amount;
        }
    }
}

fn row_empty(grid: &Vec<Vec<char>>, row: usize) -> bool {
    grid[row].iter().all(|c| *c != '#')
}

fn col_empty(grid: &Vec<Vec<char>>, col: usize) -> bool {
    grid.iter().map(|row| row[col]).all(|c| c != '#')
}

fn distance(from: &Galaxy, to: &Galaxy) -> usize {
    let horizontal_distance = from.x.abs_diff(to.x);
    let vertical_distance = from.y.abs_diff(to.y);
    horizontal_distance + vertical_distance
}

fn shift_galaxies(universe: &Vec<Vec<char>>, galaxies: &mut Vec<Galaxy>, shift_amount: usize) {
    let mut shift_counter = 0;
    for y in 0..universe.len() {
        if row_empty(&universe, y) {
            shift_galaxies_vert(galaxies, y + shift_counter, shift_amount);
            shift_counter += shift_amount;
        }
    }

    let mut shift_counter = 0;
    for x in 0..universe[0].len() {
        if col_empty(&universe, x) {
            shift_galaxies_hor(galaxies, x + shift_counter, shift_amount);
            shift_counter += shift_amount;
        }
    }
}

fn get_universe() -> Vec<Vec<char>> {
    readlines("11")
        .into_iter()
        .map(|line| line.chars().collect())
        .collect()
}

fn get_galaxies(universe: &Vec<Vec<char>>) -> Vec<Galaxy> {
    let mut galaxies: Vec<Galaxy> = Vec::new();
    for y in 0..universe.len() {
        for x in 0..universe[0].len() {
            if universe[y][x] == '#' {
                galaxies.push(Galaxy { x, y });
            }
        }
    }
    galaxies
}

fn get_distances(galaxies: Vec<Galaxy>) -> usize {
    let mut sum = 0;
    for i in 0..galaxies.len() - 1 {
        for j in i + 1..galaxies.len() {
            let d = distance(&galaxies[i], &galaxies[j]);
            sum += d;
        }
    }
    sum
}

fn calculate_distances(shift_amount: usize) -> usize {
    let universe = get_universe();
    let mut galaxies = get_galaxies(&universe);
    shift_galaxies(&universe, &mut galaxies, shift_amount);
    get_distances(galaxies)
}

fn a() {
    println!("{}", calculate_distances(1));
}

fn b() {
    println!("{}", calculate_distances(999999));
}
