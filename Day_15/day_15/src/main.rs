use std::fs;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::io::{self, Write};

type Grid = Vec<Vec<char>>;
type Point = (i32, i32);

struct Warehouse {
    grid: Grid,
    robot: Point,
    height: i32,
    width: i32,
}

impl Warehouse {
    fn from_string(input: &str, scale_up: bool) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len();
        let base_width = lines[0].len();

        if !scale_up {
            let mut grid = vec![vec!['.'; base_width]; height];
            let mut robot = (0, 0);

            for (r, line) in lines.iter().enumerate() {
                for (c, ch) in line.chars().enumerate() {
                    match ch {
                        '@' => {
                            robot = (r as i32, c as i32);
                            grid[r][c] = '.';
                        },
                        ch => grid[r][c] = ch,
                    }
                }
            }

            return Warehouse {
                grid,
                robot,
                height: height as i32,
                width: base_width as i32,
            }
        }

        // Part 2: Double width
        let width = base_width * 2;
        let mut grid = vec![vec!['.'; width]; height];
        let mut robot = (0, 0);

        for (r, line) in lines.iter().enumerate() {
            let mut c = 0;
            for ch in line.chars() {
                match ch {
                    '#' => {
                        grid[r][c*2] = '#';
                        grid[r][c*2 + 1] = '#';
                    },
                    'O' => {
                        grid[r][c*2] = '[';
                        grid[r][c*2 + 1] = ']';
                    },
                    '@' => {
                        robot = (r as i32, (c*2) as i32);
                        grid[r][c*2] = '.';
                        grid[r][c*2 + 1] = '.';
                    },
                    '.' => {
                        grid[r][c*2] = '.';
                        grid[r][c*2 + 1] = '.';
                    },
                    _ => {}
                }
                c += 1;
            }
        }

        Warehouse {
            grid,
            robot,
            height: height as i32,
            width: width as i32,
        }
    }

    fn try_move(&mut self, dr: i32, dc: i32) -> bool {
        let (r, c) = self.robot;
        let (new_r, new_c) = (r + dr, c + dc);

        // Check if moving into wall
        if self.grid[new_r as usize][new_c as usize] == '#' {
            return false;
        }

        // If moving into empty space
        if self.grid[new_r as usize][new_c as usize] == '.' {
            self.robot = (new_r, new_c);
            return true;
        }

        // Moving into a box - do BFS to find all affected boxes
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        queue.push_back((r, c));
        let mut ok = true;

        while let Some((rr, cc)) = queue.pop_front() {
            if seen.contains(&(rr, cc)) {
                continue;
            }
            seen.insert((rr, cc));

            let (rrr, ccc) = (rr + dr, cc + dc);
            let curr_char = self.grid[rrr as usize][ccc as usize];

            if curr_char == '#' {
                ok = false;
                break;
            }

            if curr_char == 'O' {
                queue.push_back((rrr, ccc));
            }
            if curr_char == '[' {
                queue.push_back((rrr, ccc));
                if self.grid[rrr as usize][(ccc+1) as usize] == ']' {
                    queue.push_back((rrr, ccc+1));
                }
            }
            if curr_char == ']' {
                queue.push_back((rrr, ccc));
                if self.grid[rrr as usize][(ccc-1) as usize] == '[' {
                    queue.push_back((rrr, ccc-1));
                }
            }
        }

        if !ok {
            return false;
        }

        // Move boxes in sorted order
        while !seen.is_empty() {
            let mut moved_any = false;
            let mut points: Vec<_> = seen.iter().cloned().collect();
            points.sort_by_key(|&(r, c)| (r + dr, c + dc));  // Sort in direction of movement

            for (rr, cc) in points {
                let (rrr, ccc) = (rr + dr, cc + dc);
                if !seen.contains(&(rrr, ccc)) {
                    let curr_char = self.grid[rr as usize][cc as usize];
                    self.grid[rrr as usize][ccc as usize] = curr_char;
                    self.grid[rr as usize][cc as usize] = '.';
                    seen.remove(&(rr, cc));
                    moved_any = true;
                }
            }

            if !moved_any {
                break;
            }
        }

        self.robot = (new_r, new_c);
        true
    }

    fn calculate_gps_sum(&self) -> i32 {
        let mut sum = 0;
        for r in 0..self.height {
            for c in 0..self.width {
                if self.grid[r as usize][c as usize] == 'O' ||
                    self.grid[r as usize][c as usize] == '[' {
                    sum += 100 * r + c;
                }
            }
        }
        sum
    }

    fn display(&self) {
        for r in 0..self.height {
            for c in 0..self.width {
                let ch = if (r, c) == self.robot {
                    '@'
                } else {
                    self.grid[r as usize][c as usize]
                };
                print!("{}", ch);
            }
            println!();
        }
    }
}

fn solve_visual(filename: &str, part2: bool) {
    let contents = fs::read_to_string(filename)
        .expect("Failed to read input file");

    let parts: Vec<&str> = contents.split("\n\n").collect();
    let map = parts[0];
    let moves = parts[1].chars()
        .filter(|c| ['<', '>', '^', 'v'].contains(c))
        .collect::<String>();

    let mut warehouse = Warehouse::from_string(map, part2);

    println!("Initial warehouse state (Part {}):", if part2 { 2 } else { 1 });
    warehouse.display();
    println!("\nMove sequence: {}", moves);
    println!("\nPress Enter to step through moves...");

    for (i, movement) in moves.chars().enumerate() {
        let mut input = String::new();
        print!("Move {} of {} ({}): ", i + 1, moves.len(), movement);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let (dr, dc) = match movement {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => (0, 0),
        };

        let moved = warehouse.try_move(dr, dc);

        println!("\x1B[2J\x1B[1H");
        println!("After move {} ({}){}:", i + 1, movement,
                 if moved { "" } else { " - BLOCKED" });
        warehouse.display();
    }

    let result = warehouse.calculate_gps_sum();
    println!("\nFinal state reached!");
    println!("Sum of GPS coordinates: {}", result);
}

fn solve(filename: &str, part2: bool) {
    let contents = fs::read_to_string(filename)
        .expect("Failed to read input file");

    let parts: Vec<&str> = contents.split("\n\n").collect();
    let map = parts[0];
    let moves = parts[1].chars()
        .filter(|c| ['<', '>', '^', 'v'].contains(c))
        .collect::<String>();

    let mut warehouse = Warehouse::from_string(map, part2);

    for movement in moves.chars() {
        let (dr, dc) = match movement {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => (0, 0),
        };
        let _ = warehouse.try_move(dr, dc);
    }

    let result = warehouse.calculate_gps_sum();
    println!("Part {}: {}", if part2 { 2 } else { 1 }, result);
}


fn main() {
    let input_file = "../../input/day15/full.txt";
    solve(input_file, false);
    solve(input_file, true);
}

