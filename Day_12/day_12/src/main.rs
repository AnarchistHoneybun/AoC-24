use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: i32,
    col: i32,
}

const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)]; // up, right, down, left

fn read_input(filename: &str) -> Vec<Vec<char>> {
    fs::read_to_string(filename)
        .expect("Failed to read input file")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_prices(grid: &Vec<Vec<char>>) -> (i32, i32) {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut seen = HashSet::new();
    let mut total_price1 = 0;
    let mut total_price2 = 0;

    // Helper function to check if a point is valid
    let is_valid = |p: Point| {
        p.row >= 0 && p.row < rows && p.col >= 0 && p.col < cols
    };

    for r in 0..rows {
        for c in 0..cols {
            let start = Point { row: r, col: c };
            if seen.contains(&start) {
                continue;
            }

            let mut queue = VecDeque::new();
            queue.push_back(start);
            let mut area = 0;
            let mut perimeter = 0;
            let mut perim_points: HashMap<(i32, i32), HashSet<Point>> = HashMap::new();

            // Find all points in this region and count perimeter
            while let Some(current) = queue.pop_front() {
                if seen.contains(&current) {
                    continue;
                }

                seen.insert(current);
                area += 1;

                // Check all neighbors
                for &(dr, dc) in &DIRS {
                    let next = Point {
                        row: current.row + dr,
                        col: current.col + dc,
                    };

                    if is_valid(next) && grid[next.row as usize][next.col as usize] == grid[r as usize][c as usize] {
                        queue.push_back(next);
                    } else {
                        perimeter += 1;
                        // Group perimeter points by direction
                        perim_points
                            .entry((dr, dc))
                            .or_insert_with(HashSet::new)
                            .insert(current);
                    }
                }
            }

            // Count distinct sides
            let mut sides = 0;
            for points in perim_points.values() {
                let mut seen_perim = HashSet::new();

                for &start_point in points {
                    if seen_perim.contains(&start_point) {
                        continue;
                    }

                    // Found a new side
                    sides += 1;

                    // Find all connected perimeter points in this direction
                    let mut perim_queue = VecDeque::new();
                    perim_queue.push_back(start_point);

                    while let Some(current) = perim_queue.pop_front() {
                        if seen_perim.contains(&current) {
                            continue;
                        }
                        seen_perim.insert(current);

                        // Check all neighbors
                        for &(dr, dc) in &DIRS {
                            let next = Point {
                                row: current.row + dr,
                                col: current.col + dc,
                            };
                            if points.contains(&next) {
                                perim_queue.push_back(next);
                            }
                        }
                    }
                }
            }

            total_price1 += area * perimeter;
            total_price2 += area * sides;
        }
    }

    (total_price1, total_price2)
}

pub fn solve(filename: &str) {
    let grid = read_input(filename);
    let (price1, price2) = find_prices(&grid);
    println!("Part 1: {}", price1);
    println!("Part 2: {}", price2);
}

fn main() {
    let input_file = "../../input/day12/full.txt";
    solve(input_file);
}