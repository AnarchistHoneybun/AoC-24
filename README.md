# Advent of Code, 2024
Solutions to [Advent of Code](https://adventofcode.com/2024/about) puzzles for the year 2024

## Brief descriptions

### Day 1
- Part 1: Read the numbers, add them to binary min-heap while reading. Pop the heaps, take the difference of the resulting numbers, and add to running total of distance between the lists.
- Part 2: Read the numbers, add to a hashmap counting instances while they're being read. Iterate over left hashmap, using the same key to access the right hashmap. use (key_i*value_r_i)*value_l_i to get similarity score for current element, and add to running total.
