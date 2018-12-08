use std::collections::HashMap;
use std::num;
use std::str;

static INPUT: &str = include_str!("input");

fn main() {
    println!("Day 6, Part 1: {}", part1(&INPUT).unwrap());
    println!("Day 6, Part 2: {}", part2(&INPUT, 10000).unwrap());
}

#[derive(Hash, PartialEq, Eq)]
struct Point(i32, i32);

impl str::FromStr for Point {
    type Err = num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = s.splitn(2, ", ").map(|s| s.parse::<i32>().unwrap());
        let row = v.next().unwrap();
        let col = v.next().unwrap();
        Ok(Point(row, col))
    }
}

fn manhattan(p: &Point, row: i32, col: i32) -> i32 {
    (p.0 - row).abs() + (p.1 - col).abs()
}

fn part1(input: &str) -> Option<u32> {
    let points: Vec<Point> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut areas = HashMap::new();
    for point in points.iter() {
        areas.insert(point, 0u32);
    }

    let min_row = points.iter().map(|p| p.0).min()?;
    let max_row = points.iter().map(|p| p.0).max()?;
    let min_col = points.iter().map(|p| p.1).min()?;
    let max_col = points.iter().map(|p| p.1).max()?;

    for r in min_row..=max_row {
        for c in min_col..=max_col {
            let mut v: Vec<_> = points.iter().map(|p| (p, manhattan(p, r, c))).collect();
            v.sort_by_key(|&(_, d)| d);
            if v[0].1 < v[1].1 {
                if r == min_row || r == max_row || c == min_col || c == max_col {
                    areas.remove(v[0].0);
                } else {
                    areas.entry(&v[0].0).and_modify(|e| *e += 1);
                }
            }
        }
    }

    Some(*areas.values().max()?)
}

fn part2(input: &str, distance: i32) -> Option<u32> {
    let points: Vec<Point> = input.lines().map(|line| line.parse().unwrap()).collect();
    let gap = distance / points.len() as i32 + 1;
    let min_row = points.iter().map(|p| p.0).min()? - gap;
    let max_row = points.iter().map(|p| p.0).max()? + gap;
    let min_col = points.iter().map(|p| p.1).min()? - gap;
    let max_col = points.iter().map(|p| p.1).max()? + gap;

    let mut result = 0u32;
    for r in min_row..=max_row {
        for c in min_col..=max_col {
            let total: i32 = points.iter().map(|p| manhattan(p, r, c)).sum();
            if total < distance {
                result += 1;
            }
        }
    }
    Some(result)
}
