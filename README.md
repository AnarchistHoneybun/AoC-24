# Advent of Code, 2024
Solutions to [Advent of Code](https://adventofcode.com/2024/about) puzzles for the year 2024

## Brief descriptions

### Day 1
- Part 1: Read the numbers, add them to binary min-heap while reading. Pop the heaps, take the difference of the resulting numbers, and add to running total of distance between the lists.
- Part 2: Read the numbers, add to a hashmap counting instances while they're being read. Iterate over left hashmap, using the same key to access the right hashmap. use (key_i*value_r_i)*value_l_i to get similarity score for current element, and add to running total.

### Day 2
- Part 1: Check first and second level of a report to determine the direction, then slide a window of size 2 over it. If direction and difference constraints hold for each pair, return true else false.
- Part 2: Slide a window of size 2 over the report, and check difference constraints. If violated, make two modified reports by removing current window elements one at a time. Check these using the process for part 1. Check each report in both direction, and return true if either of those checks return true; need to do this since checking report direction when any element could be removed is a headache.

### Day 3
- Part 1: single regex to find all valid mul's and then extract number from them to multiply and add to running total
- Part 2: larger regex that matches mul, do and don't. Have a flag (set by default) to tell if muls are enabled or not. On any match, if do or don't, flip flag accordingly, and if mul, extract the numbers, the compute and add to total or not based on the flag
- 
