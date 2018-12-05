use std::collections::HashSet;

// This is not the best way of doing file I/O,
// but it's fine here. The puzzle here isn't about
// proper error handling
static INPUT: &str = include_str!("input");

fn main() {
    println!("Day 1, Part 1: {}", part1(INPUT));
    println!("Day 1, Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> i32 {
    input.lines().filter_map(|l| l.parse::<i32>().ok()).sum()
}

fn part2(input: &str) -> i32 {
    let mut seen = HashSet::new();
    let mut accum = 0;
    for line in input.lines().filter_map(|l| l.parse::<i32>().ok()).cycle() {
        accum += line;
        if !seen.insert(accum) { break; }
    }
    accum
}
