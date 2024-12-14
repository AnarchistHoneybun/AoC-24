use std::fs::read_to_string;
use std::collections::{HashSet, VecDeque};

fn main() {
    let input_file = "../../input/day10/full.txt";
    part_1(input_file);
    part_2(input_file);
}

fn part_1(filename: &str) {
    let contents = read_to_string(filename).expect("Failed to read file");
    let grid: Vec<Vec<u32>> = contents
        .lines()
        .map(|line|
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        )
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut total_score = 0;

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 0 {
                let score = calculate_trailhead_score(&grid, i, j);
                total_score += score;
            }
        }
    }

    println!("Part 1: {}", total_score);
}

fn part_2(filename: &str) {
    let contents = read_to_string(filename).expect("Failed to read file");
    let grid: Vec<Vec<u32>> = contents
        .lines()
        .map(|line|
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        )
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut total_rating = 0;

    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j] == 0 {
                let rating = calculate_trailhead_rating(&grid, i, j);
                total_rating += rating;
            }
        }
    }

    println!("Part 2: {}", total_rating);
}

fn calculate_trailhead_score(grid: &Vec<Vec<u32>>, start_row: usize, start_col: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut reachable_nines = HashSet::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((start_row, start_col, vec![(start_row, start_col)]));
    visited.insert((start_row, start_col));

    while let Some((row, col, path)) = queue.pop_front() {
        let current_height = grid[row][col];

        if current_height == 9 {
            let mut valid_path = true;
            for i in 1..path.len() {
                let prev_height = grid[path[i-1].0][path[i-1].1];
                let curr_height = grid[path[i].0][path[i].1];
                if curr_height != prev_height + 1 {
                    valid_path = false;
                    break;
                }
            }
            if valid_path {
                reachable_nines.insert((row, col));
            }
            continue;
        }

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dx, dy) in directions {
            let new_row = row as i32 + dx;
            let new_col = col as i32 + dy;

            if new_row >= 0 && new_row < rows as i32 &&
                new_col >= 0 && new_col < cols as i32 {
                let new_row = new_row as usize;
                let new_col = new_col as usize;
                let new_height = grid[new_row][new_col];

                if new_height <= current_height + 1 {
                    let pos = (new_row, new_col);
                    if !visited.contains(&pos) {
                        visited.insert(pos);
                        let mut new_path = path.clone();
                        new_path.push(pos);
                        queue.push_back((new_row, new_col, new_path));
                    }
                }
            }
        }
    }

    reachable_nines.len()
}

fn calculate_trailhead_rating(grid: &Vec<Vec<u32>>, start_row: usize, start_col: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut distinct_paths = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((start_row, start_col, vec![(start_row, start_col)]));

    while let Some((row, col, path)) = queue.pop_front() {
        let current_height = grid[row][col];

        if current_height == 9 {
            let mut valid_path = true;
            for i in 1..path.len() {
                let prev_height = grid[path[i-1].0][path[i-1].1];
                let curr_height = grid[path[i].0][path[i].1];
                if curr_height != prev_height + 1 {
                    valid_path = false;
                    break;
                }
            }
            if valid_path {
                distinct_paths.insert(path);
            }
            continue;
        }

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dx, dy) in directions {
            let new_row = row as i32 + dx;
            let new_col = col as i32 + dy;

            if new_row >= 0 && new_row < rows as i32 &&
                new_col >= 0 && new_col < cols as i32 {
                let new_row = new_row as usize;
                let new_col = new_col as usize;
                let new_height = grid[new_row][new_col];

                if new_height == current_height + 1 {  // Must increase by exactly 1
                    let mut new_path = path.clone();
                    new_path.push((new_row, new_col));
                    queue.push_back((new_row, new_col, new_path));
                }
            }
        }
    }

    distinct_paths.len()
}