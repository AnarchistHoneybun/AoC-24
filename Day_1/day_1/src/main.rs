use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn part_2(filename: &str) {
    let file = File::open(filename).unwrap();
    let reader = io::BufReader::new(file);

    // left_map: stores numbers from left column and their frequency in right column
    // right_map: stores numbers seen in right column but not yet in left column
    let mut left_map: HashMap<i64, i64> = HashMap::new();
    let mut right_map: HashMap<i64, i64> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let nums: Vec<i64> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        if nums.len() != 2 {
            continue;
        }

        let (left_num, right_num) = (nums[0], nums[1]);

        *left_map.entry(left_num).or_insert(0) += 1;
        *right_map.entry(right_num).or_insert(0) += 1;
    }

    let mut net_sim_score = 0;

    for (key, value) in left_map.into_iter() {
        let r_value = right_map.get(&key).unwrap_or(&0);
        let sim_score = (key * r_value) * value;

        net_sim_score += sim_score
    }

    println!("Part 2: {}", net_sim_score);
}

fn part_1(filename: &str) {
    let file = File::open(filename).unwrap();
    let reader = io::BufReader::new(file);

    // Create two min-heaps using Reverse for min-heap behavior
    let mut heap1: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut heap2: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    // Read and parse the input file
    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        if let (Some(num1), Some(num2)) = (iter.next(), iter.next()) {
            let num1: i32 = num1.parse().unwrap();
            let num2: i32 = num2.parse().unwrap();
            heap1.push(Reverse(num1));
            heap2.push(Reverse(num2));
        }
    }

    let mut net_distance = 0;

    // Process all numbers
    while !heap1.is_empty() && !heap2.is_empty() {
        if let (Some(Reverse(num1)), Some(Reverse(num2))) = (heap1.pop(), heap2.pop()) {
            net_distance += (num1 - num2).abs();
        }
    }
    println!("Part 1: {}", net_distance);
}

fn main() -> io::Result<()> {
    let input_file = "input_day_1.txt";

    part_1(input_file);
    part_2(input_file);
    Ok(())
}
