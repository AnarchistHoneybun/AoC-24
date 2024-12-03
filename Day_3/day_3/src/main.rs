use regex::Regex;
use std::error::Error;
use std::fs;

#[derive(Debug)]
enum Instruction {
    Multiply(i32, i32),
    Enable,
    Disable,
}

// Helper struct to store matches and their positions
struct PatternMatch {
    instruction: Instruction,
    start: usize,
    length: usize,
}

fn find_next_instruction(
    content: &str,
    patterns: &[(Regex, Box<dyn Fn(&regex::Captures) -> Instruction>)],
) -> Option<PatternMatch> {
    let mut earliest_match: Option<PatternMatch> = None;

    // Check all patterns at current position
    for (pattern, constructor) in patterns {
        if let Some(cap) = pattern.captures(content) {
            let full_match = cap.get(0).unwrap();
            let match_start = full_match.start();

            // If this is the earliest match we've found, or if it's the first match
            if earliest_match.is_none() || match_start < earliest_match.as_ref().unwrap().start {
                earliest_match = Some(PatternMatch {
                    instruction: constructor(&cap),
                    start: match_start,
                    length: full_match.len(),
                });
            }
        }
    }

    earliest_match
}

fn parse_instructions(content: &str) -> Result<Vec<Instruction>, Box<dyn Error>> {
    // Define patterns and their corresponding instruction constructors
    let patterns: Vec<(Regex, Box<dyn Fn(&regex::Captures) -> Instruction>)> = vec![
        (
            Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?,
            Box::new(|caps: &regex::Captures| -> Instruction {
                Instruction::Multiply(caps[1].parse().unwrap(), caps[2].parse().unwrap())
            }),
        ),
        (Regex::new(r"do\(\)")?, Box::new(|_| Instruction::Enable)),
        (
            Regex::new(r"don't\(\)")?,
            Box::new(|_| Instruction::Disable),
        ),
    ];

    let mut instructions = Vec::new();
    let mut pos = 0;

    // Process the string character by character
    while pos < content.len() {
        let remainder = &content[pos..];

        if let Some(next_match) = find_next_instruction(remainder, &patterns) {
            instructions.push(next_match.instruction);
            pos += next_match.start + next_match.length;
        } else {
            pos += 1;
        }
    }

    Ok(instructions)
}

fn part_2(filename: &str) {
    let contents = fs::read_to_string(filename).unwrap();
    let instructions = parse_instructions(&contents).unwrap();

    let mut total = 0;
    let mut enabled = true; // Multiplications are enabled by default

    for instruction in instructions {
        match instruction {
            Instruction::Multiply(num1, num2) => {
                if enabled {
                    let result = num1 * num2;
                    total += result;
                }
            }
            Instruction::Enable => {
                enabled = true;
            }
            Instruction::Disable => {
                enabled = false;
            }
        }
    }

    println!("\npart 2: {}", total);
}

fn part_1(filename: &str) {
    // Read the file contents
    let contents = fs::read_to_string(filename).unwrap();

    // Create regex pattern for valid mul instructions
    // Matches mul(num,num) where num is 1-3 digits
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

fn main() {
    let input_file = "../../input/day3/full.txt";

    part_1(input_file);
    part_2(input_file);
}
