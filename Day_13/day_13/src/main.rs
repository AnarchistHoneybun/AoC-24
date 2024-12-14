use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Default)]
struct Machine {
    x1: f64,  // Button A X movement
    y1: f64,  // Button A Y movement
    x2: f64,  // Button B X movement
    y2: f64,  // Button B Y movement
    prize_x: f64,
    prize_y: f64,
}

fn read_input(filename: &str) -> Vec<Machine> {
    let path = Path::new(filename);
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    let mut machines = Vec::new();
    let mut current_machine = Machine::default();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }

        if line.starts_with("Button") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let button = parts[1].trim_end_matches(':');
            let x = parts[2][2..].trim_end_matches(',').parse::<f64>().unwrap();
            let y = parts[3][2..].parse::<f64>().unwrap();

            if button == "A" {
                current_machine.x1 = x;
                current_machine.y1 = y;
            } else {
                current_machine.x2 = x;
                current_machine.y2 = y;
            }
        } else if line.starts_with("Prize") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            current_machine.prize_x = parts[1][2..].trim_end_matches(',').parse::<f64>().unwrap();
            current_machine.prize_y = parts[2][2..].parse::<f64>().unwrap();
            machines.push(current_machine);
            current_machine = Machine::default();
        }
    }

    machines
}

fn solve_puzzle(filename: &str, part: i32) {
    let machines = read_input(filename);
    let mut total_tokens = 0;
    let offset = if part == 2 { 10_000_000_000_000_f64 } else { 0_f64 };

    for machine in machines {
        let prize_x = machine.prize_x + offset;
        let prize_y = machine.prize_y + offset;

        // Solve system of equations using cross multiplication
        // x1*a + x2*b = prize_x
        // y1*a + y2*b = prize_y

        let denominator = machine.x1 * machine.y2 - machine.y1 * machine.x2;

        let a = (prize_x * machine.y2 - prize_y * machine.x2) / denominator;
        let b = (prize_y * machine.x1 - prize_x * machine.y1) / denominator;

        // Check if we have integer solutions
        if a.fract() == 0.0 && b.fract() == 0.0 && a >= 0.0 && b >= 0.0 {
            total_tokens += (3.0 * a + b) as i64;
        }
    }

    println!("Part {}: {}", part, total_tokens);
}

fn main() {
    let input_file = "../../input/day13/full.txt";
    solve_puzzle(input_file, 1);
    solve_puzzle(input_file, 2);
}