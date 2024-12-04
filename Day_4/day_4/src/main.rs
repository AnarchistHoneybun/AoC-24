use std::fs::read_to_string;

fn part_1(filename: &str) {
    // Read the file
    let contents = match read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => {
            println!("Error reading file: {}", e);
            return;
        }
    };

    // Convert input to 2D grid
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let directions = [
        (-1, -1), (-1, 0), (-1, 1),  // up-left, up, up-right
        (0, -1),           (0, 1),    // left, right
        (1, -1),  (1, 0),  (1, 1),    // down-left, down, down-right
    ];

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for i in 0..rows {
        for j in 0..cols {
            for &(dx, dy) in &directions {
                if is_xmas_at_position(&grid, i, j, dx, dy) {
                    count += 1;
                }
            }
        }
    }

    println!("part 1: {}", count);
}

fn is_xmas_at_position(grid: &[Vec<char>], start_x: usize, start_y: usize, dx: i32, dy: i32) -> bool {
    let target = ['X', 'M', 'A', 'S'];
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    // Check each character position
    for step in 0..4 {
        // Calculate new position
        let x = start_x as i32 + dx * step;
        let y = start_y as i32 + dy * step;

        // Check bounds
        if x < 0 || x >= rows || y < 0 || y >= cols {
            return false;
        }

        // Check character match
        if grid[x as usize][y as usize] != target[step as usize] {
            return false;
        }
    }

    true
}

fn part_2(filename: &str) {
    // Read the file
    let contents = match read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => {
            println!("Error reading file: {}", e);
            return;
        }
    };

    // Convert input to 2D grid
    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for i in 1..rows-1 {  // Skip first and last rows since we need diagonal space
        for j in 1..cols-1 {  // Skip first and last columns
            if grid[i][j] == 'A' {
                // Check all possible X-MAS patterns centered at this 'A'
                if is_x_mas_at_position(&grid, i, j) {
                    count += 1;
                }
            }
        }
    }

    println!("part 2: {}", count);
}

fn is_x_mas_at_position(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    // Get the four diagonal positions around 'A'
    let top_left = grid[x-1][y-1];
    let top_right = grid[x-1][y+1];
    let bottom_left = grid[x+1][y-1];
    let bottom_right = grid[x+1][y+1];

    let is_valid_mas = |start: char, end: char| {
        (start == 'M' && end == 'S') || (start == 'S' && end == 'M')
    };

    let diagonal1 = is_valid_mas(top_left, bottom_right);
    let diagonal2 = is_valid_mas(top_right, bottom_left);

    diagonal1 && diagonal2
}

fn main() {
    let input_file = "../../input/day4/full.txt";
    part_1(input_file);
    part_2(input_file);
}
