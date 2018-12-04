use std::collections::HashMap;

static INPUT: &str = include_str!("input");

pub fn solve() {
    println!("Day 2, Part 1: {}", part1(INPUT));
    println!("Day 2, Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let (num_two_dupes, num_three_dupes) = input
        .lines()
        .fold((0, 0), |(twos, threes), word| {
            (twos + has_exact_num_of_dupes(word, 2) as usize,
            threes + has_exact_num_of_dupes(word, 3) as usize)
        });

    num_two_dupes * num_three_dupes
}

fn part2(input: &str) -> String {
    let input_clone = input.clone();
    for a in input.lines() {
        for b in input_clone.lines() {
            if words_differ_by_one(a, b) {
                return common_letters(a, b);
            }
        }
    }
    String::from("")
}

fn has_exact_num_of_dupes(word: &str, num: usize) -> bool {
    let mut counts = HashMap::with_capacity(26);
    for c in word.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    counts.values().any(|&c| c == num)
}

fn words_differ_by_one(a: &str, b: &str) -> bool {
    let num_differing = a.chars()
        .zip(b.chars())
        .filter(|(a, b)| a != b)
        .count();

    num_differing == 1
}

fn common_letters(a: &str, b: &str) -> String {
    a.chars()
        .zip(b.chars())
        .filter(|(a, b)| a == b)
        .map(|(a, _)| a)
        .collect()
}
