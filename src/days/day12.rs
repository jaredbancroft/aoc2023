use std::io::BufRead;

use crate::helpers::{self, Args};
use anyhow::{Context, Result};
use log::info;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Spring {
    Operational,
    Damaged,
    Unknown
}

pub fn run(args: &mut Args) -> Result<()> {
    info!(target: "Day 12", "Running...");
    info!(target: "Day 12", "Parsing input from file");

    let reader = helpers::read_input_from_file(args)
        .with_context(|| "Problem with buffered file read")?;

    info!(target: "Day 12", "Solving...");

    let mut total_part_1 = 0;
    let mut total_part_2 = 0;
    let mut condition: Vec<Spring> = Vec::new();
    let mut condition_part_2: Vec<Spring> = Vec::new();
    let mut group_sizes: Vec<usize> = Vec::new();
    let mut group_sizes_part_2: Vec<usize> = Vec::new();


    for line in reader.lines() {
        let l = line.unwrap();
        let inputs = l.split_whitespace().collect::<Vec<&str>>();
        condition = inputs[0].chars().map(|x| parse_springs(x)).collect::<Vec<Spring>>();
        group_sizes = inputs[1].trim().split(',').map(|x| x.parse::<usize>().unwrap()).collect();
        condition_part_2 = condition.iter().copied().chain([Spring::Unknown]).cycle().take(condition.len() * 5 + 4).collect();
        group_sizes_part_2 = group_sizes.iter().copied().cycle().take(group_sizes.len() * 5).collect();
        total_part_1 += find_arrangements(condition, group_sizes);
        total_part_2 += find_arrangements(condition_part_2, group_sizes_part_2);
    }  

    println!("Part 1: {}", total_part_1);
    println!("Part 2: {}", total_part_2);

    Ok(())
}

fn find_arrangements(mut condition: Vec<Spring>, groupings: Vec<usize>) -> u64 {
    condition.push(Spring::Operational);
    let mut cache = vec![vec![None::<u64>; condition.len()]; groupings.len()];
    recurse_arrangements(&condition, &groupings, &mut cache)
}

fn recurse_arrangements(condition: &[Spring], groupings: &[usize], cache: &mut [Vec<Option<u64>>]) -> u64 {
    if groupings.is_empty() {
        if condition.contains(&Spring::Damaged) {
            return 0;
        } else {
            return 1;
        }
    }

    if condition.len() < groupings.iter().sum::<usize>() + groupings.len() {
        return 0;
    }

    if let Some(cached) = cache[groupings.len() - 1][condition.len() - 1] {
        return cached;
    }

    let mut count = 0;

    if condition[0] != Spring::Damaged {
        count += recurse_arrangements(&condition[1..], groupings, cache);
    }

    let group_size = groupings[0];

    if !condition[..group_size].contains(&Spring::Operational) && condition[group_size] != Spring::Damaged {
        count += recurse_arrangements(&condition[group_size + 1..], &groupings[1..], cache)
    }

    cache[groupings.len() - 1][condition.len() - 1] = Some(count);
    count
}

fn parse_springs(c: char) -> Spring {
    match c {
        '.' => Spring::Operational,
        '#' => Spring::Damaged,
        '?' => Spring::Unknown,
        _ => panic!("Unknown input character")
    }
}