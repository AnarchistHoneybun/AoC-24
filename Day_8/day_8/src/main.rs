use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn parse_input(contents: &str) -> Vec<(Point, char)> {
    let mut antennas = Vec::new();
    for (y, line) in contents.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch != '.' {
                antennas.push((
                    Point {
                        x: x as i32,
                        y: y as i32,
                    },
                    ch,
                ));
            }
        }
    }
    antennas
}

fn is_in_bounds(point: &Point, max_x: i32, max_y: i32) -> bool {
    point.x >= 0 && point.x <= max_x && point.y >= 0 && point.y <= max_y
}

fn find_antinodes(a1: &Point, a2: &Point, max_x: i32, max_y: i32, is_part2: bool) -> Vec<Point> {
    let mut antinodes = Vec::new();

    // Calculate the vector between antennas
    let dx = a2.x - a1.x;
    let dy = a2.y - a1.y;

    // For part 2, include the antenna positions themselves
    if is_part2 {
        antinodes.push(*a1);
        antinodes.push(*a2);
    }

    // Start from a1, go in opposite direction of a2
    let mut current = Point {
        x: a1.x - dx,
        y: a1.y - dy,
    };

    while is_in_bounds(&current, max_x, max_y) {
        antinodes.push(current);
        if !is_part2 { break; }
        current.x -= dx;
        current.y -= dy;
    }

    // Start from a2, continue in same direction
    let mut current = Point {
        x: a2.x + dx,
        y: a2.y + dy,
    };

    while is_in_bounds(&current, max_x, max_y) {
        antinodes.push(current);
        if !is_part2 { break; }
        current.x += dx;
        current.y += dy;
    }

    antinodes
}

fn part_1(filename: &str) {
    let contents = fs::read_to_string(filename).expect("Failed to read input file");
    let antennas = parse_input(&contents);
    let max_y = contents.lines().count() as i32 - 1;
    let max_x = contents.lines().next().unwrap().len() as i32 - 1;

    let mut freq_groups: HashMap<char, Vec<Point>> = HashMap::new();
    for (point, freq) in antennas {
        freq_groups.entry(freq).or_default().push(point);
    }

    let mut antinodes: HashSet<Point> = HashSet::new();

    for (_freq, points) in freq_groups {
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                let new_antinodes = find_antinodes(&points[i], &points[j], max_x, max_y, false);
                antinodes.extend(new_antinodes);
            }
        }
    }

    println!("Part 1: {}", antinodes.len());
}

fn part_2(filename: &str) {
    let contents = fs::read_to_string(filename).expect("Failed to read input file");
    let antennas = parse_input(&contents);
    let max_y = contents.lines().count() as i32 - 1;
    let max_x = contents.lines().next().unwrap().len() as i32 - 1;

    let mut freq_groups: HashMap<char, Vec<Point>> = HashMap::new();
    for (point, freq) in antennas {
        freq_groups.entry(freq).or_default().push(point);
    }

    let mut antinodes: HashSet<Point> = HashSet::new();

    for (_freq, points) in freq_groups {
        // Skip frequencies with only one antenna
        if points.len() < 2 {
            continue;
        }

        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                let new_antinodes = find_antinodes(&points[i], &points[j], max_x, max_y, true);
                antinodes.extend(new_antinodes);
            }
        }
    }

    println!("Part 2: {}", antinodes.len());
}

fn main() {
    let input_file = "../../input/day8/full.txt";
    part_1(input_file);
    part_2(input_file);
}