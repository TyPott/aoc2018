use std::collections::{HashMap, HashSet};
use std::str::FromStr;

static INPUT: &str = include_str!("input");

pub fn solve() {
    println!("Day 3, Part 1: {}", part1(INPUT));
    println!("Day 3, Part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    let mut pairs = HashMap::new();
    for claim in input.lines().filter_map(|l| Claim::from_str(l).ok()) {
        for i in claim.y .. claim.y + claim.height {
            for j in claim.x .. claim.x + claim.width {
                *pairs.entry((i, j)).or_insert(0) += 1;
            }
        }
    }
    pairs.values().filter(|&&v| v > 1).count()
}

fn part2(input: &str) -> usize {
    let mut pairs = HashMap::new();
    let mut uncovered = input
        .lines()
        .filter_map(|l| Claim::from_str(l).ok())
        .map(|c| c.id)
        .collect::<HashSet<_>>();

    for claim in input.lines().filter_map(|l| Claim::from_str(l).ok()) {
        for i in claim.y .. claim.y + claim.height {
            for j in claim.x .. claim.x + claim.width {
                let found = *pairs.entry((i, j)).or_insert(claim.id);
                if found != claim.id {
                    uncovered.remove(&claim.id);
                    uncovered.remove(&found);
                }
            }
        }
    }

    *uncovered.iter().take(1).collect::<Vec<_>>()[0]
}

#[derive(Debug)]
struct Claim {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

#[derive(Debug)]
enum Error {
    FormatError,
}

impl FromStr for Claim {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let r = s.split(|c| c == ' ' || c == '#' || c == '@' || c == ',' || c == ':' || c == 'x')
            .filter_map(|n| n.parse::<usize>().ok())
            .collect::<Vec<_>>();
        if r.len() == 5 {
            Ok(Claim { id: r[0], x: r[1], y: r[2], width: r[3], height: r[4] })
        } else {
            Err(Error::FormatError)
        }
    }
}
