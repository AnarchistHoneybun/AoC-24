use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn check_report_safety(levels: &Vec<i32>) -> bool {
    if levels.len() < 2 {
        return false;
    }

    // Determine if sequence should be increasing or decreasing
    let is_increasing = levels[1] > levels[0];

    // Check each adjacent pair
    for window in levels.windows(2) {
        let diff = (window[1] - window[0]).abs();

        // Check if difference is within valid range (1-3)
        if diff < 1 || diff > 3 {
            return false;
        }

        // Check if sequence maintains its direction
        if is_increasing && window[1] <= window[0] {
            return false;
        }
        if !is_increasing && window[1] >= window[0] {
            return false;
        }
    }

    true
}

fn check_report_safety_with_dampener(levels: &Vec<i32>) -> bool {
    if levels.len() < 2 {
        return false;
    }

    for i in 0..levels.len() -1 {
        if !(1..=3).contains(&(levels[i] - levels[i + 1])) {
            return [i, i+1].iter().any(|&j| {
                let modified: Vec<i32> = levels
                    .iter()
                    .enumerate()
                    .filter(|&(idx, _)| idx != j)
                    .map(|(_, &x)| x)
                    .collect();
                check_report_safety(&modified)
            })
        }
    }

    true
}

fn process_file<F>(filepath: &str, mut process_line: F)
where
    F: FnMut(Vec<i32>),
{
    let path = Path::new(filepath);
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let levels: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        process_line(levels);
    }
}

fn part_1(filepath: &str) {
    let mut safe_reports = 0;

    process_file(filepath, |levels| {
        if check_report_safety(&levels) {
            safe_reports += 1;
        }
    });

    println!("part 1: {}", safe_reports);
}

fn part_2(filepath: &str) {
    let mut safe_reports = 0;

    process_file(filepath, |mut levels| {
        let og_levels = levels.clone();
        levels.reverse();

        if check_report_safety_with_dampener(&levels) || check_report_safety_with_dampener(&og_levels) {
            safe_reports += 1;
        }
    });

    println!("part 2: {}", safe_reports);
}

fn main() {
    let input_file = "../../input/day2/full.txt";
    part_1(input_file);
    part_2(input_file);
}