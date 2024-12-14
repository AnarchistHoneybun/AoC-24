use std::fs;
use std::collections::HashMap;

fn main() {
    let input_file = "../../input/day11/full.txt";
    solve_puzzle(input_file);
}

fn solve_puzzle(filename: &str) {
    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    let stones: Vec<u64> = contents
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Should be a valid number"))
        .collect();

    let mut cache: HashMap<(u64, u32), u64> = HashMap::new();
    let total_stones: u64 = stones.iter()
        .map(|&stone| how_many_eventually(stone, 75, &mut cache))
        .sum();

    println!("After 75 blinks, there are {} stones.", total_stones);
}

fn how_many_eventually(x: u64, iters: u32, cache: &mut HashMap<(u64, u32), u64>) -> u64 {
    // Check cache first
    if let Some(&result) = cache.get(&(x, iters)) {
        return result;
    }

    // Base case
    if iters == 0 {
        return 1;
    }

    let result = if x == 0 {
        how_many_eventually(1, iters - 1, cache)
    } else {
        let digits = x.to_string();
        let n = digits.len();

        if n % 2 == 0 {
            let (left_str, right_str) = digits.split_at(n / 2);
            let left = left_str.parse::<u64>().unwrap();
            let right = right_str.parse::<u64>().unwrap();

            how_many_eventually(left, iters - 1, cache) +
                how_many_eventually(right, iters - 1, cache)
        } else {
            how_many_eventually(x * 2024, iters - 1, cache)
        }
    };

    // Store in cache before returning
    cache.insert((x, iters), result);
    result
}