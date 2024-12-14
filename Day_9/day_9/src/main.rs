use std::fs;
use std::array::from_fn;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let input_file = "../../input/day9/sample.txt";
    solve_puzzle(input_file);
}

fn solve_puzzle(filename: &str) {
    let contents = fs::read_to_string(filename).expect("Failed to read file");
    let disk_map = parse(&contents);

    let part1_result = part_1(&disk_map);
    let part2_result = part_2(&disk_map);

    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", part2_result);
}

fn parse(input: &str) -> Vec<usize> {
    input.trim().bytes().map(|b| (b - b'0') as usize).collect()
}

fn part_1(disk: &[usize]) -> usize {
    let mut free_ptr = 0;  // Points to current free block
    let mut file_ptr = disk.len() + disk.len() % 2;  // Points to last file

    let mut space_available = 0;  // Current free space
    let mut space_needed = 0;     // Space needed for current file

    let mut current_pos = 0;
    let mut checksum = 0;

    while free_ptr < file_ptr {
        // Fill as much of current free block as possible
        let space_used = space_needed.min(space_available);
        let (new_sum, new_pos) = update_checksum(checksum, current_pos, file_ptr, space_used);
        checksum = new_sum;
        current_pos = new_pos;

        space_available -= space_used;
        space_needed -= space_used;

        // Move to next file if current one is placed
        if space_needed == 0 {
            file_ptr -= 2;
            space_needed = disk[file_ptr];
        }

        // Move to next free block if current one is filled
        if space_available == 0 {
            let block_size = disk[free_ptr];
            let (new_sum, new_pos) = update_checksum(checksum, current_pos, free_ptr, block_size);
            checksum = new_sum;
            current_pos = new_pos;

            space_available = disk[free_ptr + 1];
            free_ptr += 2;
        }
    }

    let (final_sum, _) = update_checksum(checksum, current_pos, file_ptr, space_needed);
    final_sum
}

fn part_2(disk: &[usize]) -> usize {
    // Array of min-heaps, index is block size
    let mut free_blocks: [BinaryHeap<Reverse<usize>>; 10] = from_fn(|_| BinaryHeap::new());
    let mut current_pos = 0;

    // Build heaps of free blocks by size
    for (i, &size) in disk.iter().enumerate() {
        if i % 2 == 1 && size > 0 {
            free_blocks[size].push(Reverse(current_pos));
        }
        current_pos += size;
    }

    let mut checksum = 0;

    // Process files from right to left
    for (i, &size) in disk.iter().enumerate().rev() {
        current_pos -= size;

        if i % 2 == 1 { continue; }

        // Find leftmost free block that fits
        let mut best_pos = current_pos;
        let mut best_size = usize::MAX;

        for block_size in size..10 {
            if let Some(&Reverse(pos)) = free_blocks[block_size].peek() {
                if pos < best_pos {
                    best_pos = pos;
                    best_size = block_size;
                }
            }
        }

        // Update checksum
        let file_id = i / 2;
        checksum += file_id * (best_pos * size + get_triangle_number(size));

        // Update free blocks if moved
        if best_size != usize::MAX {
            free_blocks[best_size].pop();
            if size < best_size {
                free_blocks[best_size - size].push(Reverse(best_pos + size));
            }
        }
    }

    checksum
}

fn update_checksum(sum: usize, pos: usize, file_index: usize, size: usize) -> (usize, usize) {
    let file_id = file_index / 2;
    let pos_sum = pos * size + get_triangle_number(size);
    (sum + file_id * pos_sum, pos + size)
}

fn get_triangle_number(n: usize) -> usize {
    const TRIANGLES: [usize; 10] = [0, 0, 1, 3, 6, 10, 15, 21, 28, 36];
    TRIANGLES[n]
}