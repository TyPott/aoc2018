extern crate chrono;
extern crate lazy_static;
extern crate regex;

mod error;
mod record;

use chrono::Timelike;
use record::Event;
use std::collections::HashMap;

static INPUT: &str = include_str!("input");

fn main() {
    println!("Day 4, Part 1: {}", part1(INPUT).unwrap());
    println!("Day 4, Part 2: {}", part2(INPUT).unwrap());
}

fn part1(input: &str) -> Option<usize> {
    let schedule = build_schedule(input);
    let sleepiest_guard = *schedule
        .iter()
        .map(|(id, mins)| (id, mins.iter().sum::<usize>()))
        .max_by_key(|(_, mins)| *mins)?
        .0;
    let sleepiest_minute = schedule[&sleepiest_guard]
        .iter()
        .enumerate()
        .max_by_key(|(_, times)| *times)?
        .0;
    Some(sleepiest_guard * sleepiest_minute)
}

fn part2(input: &str) -> Option<usize> {
    let schedule = build_schedule(input);
    let (guard, minute) = schedule
        .into_iter()
        .filter_map(|(id, mins)| {
            Some((
                id,
                mins.iter()
                    .cloned()
                    .enumerate()
                    .max_by_key(|(_, times)| *times)?,
            ))
        }).max_by_key(|(_, (_, times))| *times)
        .map(|(id, (min, _))| (id, min))?;
    Some(guard * minute)
}

fn build_schedule(input: &str) -> HashMap<usize, [usize; 60]> {
    let mut records = input
        .lines()
        .filter_map(|r| r.parse().ok())
        .collect::<Vec<record::Record>>();
    records.sort_unstable();

    let mut schedule = HashMap::<usize, [usize; 60]>::new();
    let mut current_guard = 0;
    let mut current_minute = 0;

    for record in records {
        match record.event {
            Event::StartShift(id) => current_guard = id,
            Event::FallAsleep => current_minute = record.timestamp.minute(),
            Event::WakeUp => {
                let mut minutes = schedule.entry(current_guard)
                    .or_insert([0; 60]);
                for i in current_minute..record.timestamp.minute() {
                    minutes[i as usize] += 1;
                }
            }
        }
    }

    schedule
}
