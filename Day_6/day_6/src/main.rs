use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn get_delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, (usize, usize), Direction) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut start_pos = (0, 0);
    let mut start_dir = Direction::Up;

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '^' {
                start_pos = (i, j);
                start_dir = Direction::Up;
            }
        }
    }

    let mut clean_grid = grid.clone();
    clean_grid[start_pos.0][start_pos.1] = '.';
    (clean_grid, start_pos, start_dir)
}

fn walk(grid: &[Vec<char>], start: (usize, usize), dir: Direction) -> Option<Vec<(usize, usize)>> {
    let mut path = vec![start];
    let mut pos = start;
    let mut dir = dir;
    let mut visited = HashSet::new();

    loop {
        let (dy, dx) = dir.get_delta();
        let next_y = pos.0 as i32 + dy;
        let next_x = pos.1 as i32 + dx;

        if next_y < 0 || next_y >= grid.len() as i32 ||
            next_x < 0 || next_x >= grid[0].len() as i32 {
            return Some(path);
        }

        let next = (next_y as usize, next_x as usize);
        if grid[next.0][next.1] == '#' {
            if visited.contains(&(pos, dir)) {
                return None;
            }
            visited.insert((pos, dir));
            dir = dir.turn_right();
            continue;
        }

        pos = next;
        path.push(pos);
    }
}

fn part_1(filename: &str) {
    let input = fs::read_to_string(filename).expect("Failed to read input file");
    let (grid, start, dir) = parse_input(&input.trim());

    if let Some(path) = walk(&grid, start, dir) {
        let unique: HashSet<_> = path.into_iter().collect();
        println!("Part 1: {}", unique.len());
    }
}

fn part_2(filename: &str) {
    let input = fs::read_to_string(filename).expect("Failed to read input file");
    let (mut grid, start, dir) = parse_input(&input.trim());


    let path = walk(&grid, start, Direction::Up).unwrap();

    let mut obstacles = HashSet::new();
    path.iter().skip(1).for_each(|p| {
        grid[p.0][p.1] = '#';
        if walk(&grid, start, Direction::Up).is_none() {
            obstacles.insert(*p);
        }
        grid[p.0][p.1] = '.';
    });

    println!("Part 2: {}", obstacles.len());
}

fn main() {
    let input_file = "../../input/day6/full.txt";
    part_1(input_file);
    part_2(input_file);
}