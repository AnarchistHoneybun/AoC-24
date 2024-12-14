use std::fs::read_to_string;
use std::io::{self, Write, Read};
use std::process::Command;

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Robot {
    pos: Position,
    vel: Position,
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/c", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

fn parse_input(filename: &str) -> Vec<Robot> {
    let contents = read_to_string(filename).expect("Failed to read input file");
    contents
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split(' ').collect();
            let pos_parts: Vec<&str> = parts[0][2..].split(',').collect();
            let vel_parts: Vec<&str> = parts[1][2..].split(',').collect();

            Robot {
                pos: Position {
                    x: pos_parts[0].parse().unwrap(),
                    y: pos_parts[1].parse().unwrap(),
                },
                vel: Position {
                    x: vel_parts[0].parse().unwrap(),
                    y: vel_parts[1].parse().unwrap(),
                },
            }
        })
        .collect()
}

fn simulate_robots(robots: &[Robot], width: i32, height: i32, seconds: i32) -> Vec<Robot> {
    robots.iter().map(|robot| {
        let new_x = ((robot.pos.x + seconds * robot.vel.x) % width + width) % width;
        let new_y = ((robot.pos.y + seconds * robot.vel.y) % height + height) % height;
        Robot {
            pos: Position { x: new_x, y: new_y },
            vel: robot.vel,
        }
    }).collect()
}

fn calculate_safety_factor(robots: &[Robot], width: i32, height: i32) -> i32 {
    let mut quadrants = vec![0; 4];

    for robot in robots {
        if robot.pos.x == width / 2 || robot.pos.y == height / 2 {
            continue;
        }

        let quadrant = match (robot.pos.x > width / 2, robot.pos.y > height / 2) {
            (false, false) => 0,
            (true, false) => 1,
            (false, true) => 2,
            (true, true) => 3,
        };

        quadrants[quadrant] += 1;
    }

    quadrants.iter().product()
}

fn calculate_grid(robots: &[Robot], seconds: i32, width: i32, height: i32) -> (Vec<Vec<char>>, bool) {
    let mut grid = vec![vec!['.'; width as usize]; height as usize];
    let mut has_overlaps = false;

    for robot in robots {
        let new_x = ((robot.pos.x + seconds * robot.vel.x) % width + width) % width;
        let new_y = ((robot.pos.y + seconds * robot.vel.y) % height + height) % height;

        match grid[new_y as usize][new_x as usize] {
            '.' => grid[new_y as usize][new_x as usize] = '█',
            _ => {
                grid[new_y as usize][new_x as usize] = '▒';
                has_overlaps = true;
            }
        }
    }

    (grid, has_overlaps)
}

fn print_state(grid: &[Vec<char>], seconds: i32) {
    println!("Time: {} seconds", seconds);
    println!("{}", "-".repeat(grid[0].len()));
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
    println!("{}", "-".repeat(grid[0].len()));

    let overlaps = grid.iter()
        .flat_map(|row| row.iter())
        .filter(|&&c| c == '▒')
        .count();

    if overlaps == 0 {
        println!("NO OVERLAPS IN THIS STATE!");
    } else {
        println!("Number of positions with overlaps: {}", overlaps);
    }
    println!("\nPress 'w' for next step, Enter for next non-overlapping state, 'q' to quit");
}

fn find_next_no_overlap(robots: &[Robot], current_time: i32, width: i32, height: i32, max_search: i32) -> Option<i32> {
    for time in (current_time + 1)..=(current_time + max_search) {
        let (_, has_overlaps) = calculate_grid(robots, time, width, height);
        if !has_overlaps {
            return Some(time);
        }
    }
    None
}

fn wait_for_key() -> char {
    let mut buffer = [0; 1];
    io::stdin().read_exact(&mut buffer).unwrap();
    buffer[0] as char
}

fn solve(filename: &str, width: i32, height: i32) {
    let robots = parse_input(filename);

    // Part 1
    let final_positions = simulate_robots(&robots, width, height, 100);
    let safety_factor = calculate_safety_factor(&final_positions, width, height);
    println!("Part 1: {}", safety_factor);
    println!("\nPress Enter to start interactive visualization...");
    wait_for_key();

    // Part 2 - Interactive visualization
    let mut seconds = 0;
    loop {
        clear_screen();
        let (grid, _) = calculate_grid(&robots, seconds, width, height);
        print_state(&grid, seconds);

        match wait_for_key() {
            'q' => break,
            'w' => seconds += 1,
            '\n' | '\r' => {
                if let Some(next_time) = find_next_no_overlap(&robots, seconds, width, height, 10000) {
                    seconds = next_time;
                } else {
                    println!("\nNo non-overlapping state found within next 10000 steps!");
                    println!("Press Enter to continue...");
                    wait_for_key();
                }
            }
            _ => {}
        }
    }
}
fn main() {
    let input_file = "../../input/day14/full.txt";
    let width = 101;
    let height = 103;
    solve(input_file, width, height);
}