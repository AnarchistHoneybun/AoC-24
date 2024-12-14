use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Equation {
    test_value: i64,
    numbers: Vec<i64>,
}

fn evaluate(numbers: &[i64], operators: &[char]) -> i64 {
    let mut result = numbers[0];
    for i in 0..operators.len() {
        match operators[i] {
            '+' => result += numbers[i + 1],
            '*' => result *= numbers[i + 1],
            '|' => {
                // Convert both numbers to strings, concatenate, then parse back to i64
                let combined = format!("{}{}", result, numbers[i + 1]);
                result = combined.parse().unwrap();
            }
            _ => panic!("Invalid operator"),
        }
    }
    result
}

fn try_combinations(eq: &Equation, include_concat: bool) -> bool {
    let operator_positions = eq.numbers.len() - 1;
    let num_operators: i32 = if include_concat { 3 } else { 2 };
    let total_combinations = num_operators.pow(operator_positions as u32);

    for i in 0..total_combinations {
        let mut operators = Vec::new();
        for j in 0..operator_positions {
            // Convert number to base-3 (or base-2 for part 1) for different operators
            let operator = match (i / num_operators.pow(j as u32)) % num_operators {
                0 => '+',
                1 => '*',
                2 => '|',  // Only used in part 2
                _ => panic!("Invalid operator index"),
            };
            operators.push(operator);
        }

        if evaluate(&eq.numbers, &operators) == eq.test_value {
            return true;
        }
    }
    false
}

fn parse_line(line: &str) -> Equation {
    let parts: Vec<&str> = line.split(':').collect();
    let test_value = parts[0].trim().parse().unwrap();

    let numbers: Vec<i64> = parts[1]
        .trim()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    Equation {
        test_value,
        numbers,
    }
}

fn read_equations(filename: &str) -> Vec<Equation> {
    let path = Path::new(filename);
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    reader.lines()
        .filter_map(|line| {
            let line = line.ok()?;
            if line.trim().is_empty() {
                None
            } else {
                Some(parse_line(&line))
            }
        })
        .collect()
}

fn part_1(filename: &str) {
    let equations = read_equations(filename);
    let total: i64 = equations.iter()
        .filter(|eq| try_combinations(eq, false))
        .map(|eq| eq.test_value)
        .sum();

    println!("Part 1: {}", total);
}

fn part_2(filename: &str) {
    let equations = read_equations(filename);
    let total: i64 = equations.iter()
        .filter(|eq| try_combinations(eq, true))
        .map(|eq| eq.test_value)
        .sum();

    println!("Part 2: {}", total);
}

fn main() {
    let input_file = "../../input/day7/full.txt";
    part_1(input_file);
    part_2(input_file);
}