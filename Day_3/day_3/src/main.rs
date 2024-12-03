use regex::Regex;
use std::fs;

fn part_1(filename: &str) {
    // Read the file contents
    let contents = fs::read_to_string(filename).unwrap();

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // Initialize sum
    let mut total = 0;

    // Find all matches and process them
    for cap in re.captures_iter(&contents) {
        // Extract numbers from capture groups
        let num1: i32 = cap[1].parse().unwrap();
        let num2: i32 = cap[2].parse().unwrap();

        // Multiply numbers and add to total
        let result = num1 * num2;
        total += result;
    }

    println!("\npart 1: {}", total);
}

fn part_2(filename: &str) {
    // Read the file contents
    let contents = fs::read_to_string(filename).unwrap();

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    let mut total = 0;
    let mut enabled = true;  // Multiplications are enabled by default

    // Find and process all instructions in order
    for cap in re.captures_iter(&contents) {
        let instruction = cap.get(0).unwrap().as_str();

        if instruction == "do()" {
            enabled = true;
        } else if instruction == "don't()" {
            enabled = false;
        } else {
            let num1: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
            let num2: i32 = cap.get(2).unwrap().as_str().parse().unwrap();

            if enabled {
                let result = num1 * num2;
                total += result;
            }
        }
    }

    println!("\npart 2: {}", total);
}

fn main() {
    let input_file = "../../input/day3/full.txt";

    part_1(input_file);
    part_2(input_file);
}
