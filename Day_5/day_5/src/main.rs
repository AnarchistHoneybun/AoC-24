use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Rule {
    before: u32,
    after: u32,
}

fn parse_rule(line: &str) -> Rule {
    let parts: Vec<&str> = line.split('|').collect();
    Rule {
        before: parts[0].parse().unwrap(),
        after: parts[1].parse().unwrap(),
    }
}

fn parse_update(line: &str) -> Vec<u32> {
    line.split(',')
        .map(|num| num.trim().parse().unwrap())
        .collect()
}

fn is_valid_order(update: &[u32], rules: &[Rule]) -> bool {
    let update_pages: HashSet<u32> = update.iter().cloned().collect();
    let positions: HashMap<u32, usize> = update
        .iter()
        .enumerate()
        .map(|(i, &num)| (num, i))
        .collect();

    for rule in rules {
        if update_pages.contains(&rule.before) && update_pages.contains(&rule.after) {
            if positions[&rule.before] >= positions[&rule.after] {
                return false;
            }
        }
    }
    true
}

fn get_middle_number(update: &[u32]) -> u32 {
    update[update.len() / 2]
}

fn topological_sort(pages: &[u32], rules: &[Rule]) -> Vec<u32> {
    let pages_set: HashSet<u32> = pages.iter().cloned().collect();

    // Build adjacency list and in-degree counts
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut in_degree: HashMap<u32, usize> = HashMap::new();

    // Initialize
    for &page in pages {
        graph.entry(page).or_default();
        in_degree.insert(page, 0);
    }

    // Build graph from applicable rules
    for rule in rules {
        if pages_set.contains(&rule.before) && pages_set.contains(&rule.after) {
            graph.entry(rule.before).or_default().push(rule.after);
            *in_degree.entry(rule.after).or_default() += 1;
        }
    }

    // Find start nodes (in-degree = 0)
    let mut queue: VecDeque<u32> = in_degree
        .iter()
        .filter(|(_, &degree)| degree == 0)
        .map(|(&page, _)| page)
        .collect();

    let mut result = Vec::new();

    // Process queue
    while let Some(page) = queue.pop_front() {
        result.push(page);

        if let Some(neighbors) = graph.get(&page) {
            for &next in neighbors {
                *in_degree.get_mut(&next).unwrap() -= 1;
                if in_degree[&next] == 0 {
                    queue.push_back(next);
                }
            }
        }
    }

    result
}

fn part_1_and_2(filename: &str) {
    let file = File::open(filename).unwrap();
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let mut rules = Vec::new();
    let mut updates = Vec::new();
    let mut reading_rules = true;

    for line in lines {
        if line.is_empty() {
            reading_rules = false;
            continue;
        }

        if reading_rules {
            rules.push(parse_rule(&line));
        } else {
            updates.push(parse_update(&line));
        }
    }

    // Part 1: Sum of middle numbers from valid updates
    let valid_sum: u32 = updates
        .iter()
        .filter(|update| is_valid_order(update, &rules))
        .map(|update| get_middle_number(update))
        .sum();
    println!(
        "Part 1: {}",
        valid_sum
    );

    // Part 2: Fix invalid updates and sum their middle numbers
    let invalid_sum: u32 = updates
        .iter()
        .filter(|update| !is_valid_order(update, &rules))
        .map(|update| {
            let sorted = topological_sort(update, &rules);
            get_middle_number(&sorted)
        })
        .sum();
    println!(
        "Part 2: {}",
        invalid_sum
    );
}

fn main() {
    let input_file = "../../input/day5/full.txt";
    part_1_and_2(input_file);
}
