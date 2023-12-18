use std::{io::BufRead, collections::HashMap};

use crate::helpers::{self, Args};
use anyhow::{Context, Result};
use itertools::Itertools;
use log::info;

const EXPANSION_COEFFICIENT_PART_1: i64 = 2;
const EXPANSION_COEFFICIENT_PART_2: i64 = 1000000;

pub fn run(args: &mut Args) -> Result<()> {
    info!(target: "Day 1", "Running...");
    info!(target: "Day 1", "Parsing input from file");

    let reader = helpers::read_input_from_file(args)
        .with_context(|| "Problem with buffered file read")?;

    info!(target: "Day 1", "Solving...");

    let mut total_part_1 = 0;
    let mut total_part_2 = 0;

    let mut galaxy_location: Vec<(usize, usize)> = Vec::new(); //location is (row, column)
    let mut horizontal_null_space: HashMap<usize, bool> = HashMap::new();
    let mut vertical_null_space: HashMap<usize, bool> = HashMap::new();
   
    for (y, line) in reader.lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            if c != '.' {
                galaxy_location.push((x, y));
                vertical_null_space.entry(x).and_modify(|entry| *entry = false).or_insert(false);
                horizontal_null_space.entry(y).and_modify(|entry| *entry = false).or_insert(false);
            }
            vertical_null_space.entry(x).or_insert(true);
        }
        horizontal_null_space.entry(y).or_insert(true);
    }
    
    for combo in galaxy_location.iter().combinations(2) {
        total_part_1 += distance(combo[0], combo[1], &vertical_null_space, &horizontal_null_space, EXPANSION_COEFFICIENT_PART_1);
        total_part_2 += distance(combo[0], combo[1], &vertical_null_space, &horizontal_null_space, EXPANSION_COEFFICIENT_PART_2);
    }

    println!("Part 1: {}", total_part_1);
    println!("Part 2: {}", total_part_2);

    Ok(())
}

fn distance(start: &(usize, usize), end: &(usize, usize), vertical_map: &HashMap<usize, bool>, horizontal_map: &HashMap<usize, bool>, ec: i64) -> i64 {
    let x = cross_null_space(start.0, end.0, vertical_map);
    let y = cross_null_space(start.1, end.1, horizontal_map);
    (start.0 as i64 - end.0 as i64).abs() + (if x > 0 {x * (ec - 1)} else {0}) + (start.1 as i64 - end.1 as i64).abs() + (if y > 0 {y * (ec - 1)} else {0})
}

fn cross_null_space(start: usize, end: usize, space_map: &HashMap<usize, bool>) -> i64 {
    let mut crossing_count = 0;
    let new_start: usize;
    let new_end: usize;

    if start > end {
        new_start = end;
        new_end = start;
    } else {
        new_start = start;
        new_end = end;
    }
    for i in new_start..new_end {
        if *space_map.get(&i).unwrap() {
            crossing_count += 1;
        }
    }
    crossing_count
}