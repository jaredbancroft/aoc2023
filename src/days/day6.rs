use std::io::BufRead;

use crate::helpers::{self, Args};
use anyhow::{Context, Result};
use log::info;

pub fn run(args: &mut Args) -> Result<()> {
    info!(target: "Day 6", "Running...");
    info!(target: "Day 6", "Parsing input from file");

    let reader = helpers::read_input_from_file(args)
        .with_context(|| "Problem with buffered file read")?;

    info!(target: "Day 6", "Solving...");
    let mut total_part_1 = 1;
    let mut total_part_2 = 0;

    let mut line = reader.lines();
    let mut values = line.next().unwrap().unwrap().trim().split(':').collect::<Vec<&str>>()[1].to_string();

    let times: Vec<i64> = values.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let one_time: String = values.split_whitespace().collect();

    values = line.next().unwrap().unwrap().trim().split(':').collect::<Vec<&str>>()[1].to_string();

    let distances: Vec<i64> = values.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let one_distance: String = values.split_whitespace().collect();
    
    for (i, time) in times.iter().enumerate() {
        total_part_1 *= race(distances[i], *time);
    }

    total_part_2 += race(one_distance.parse::<i64>().unwrap(), one_time.parse::<i64>().unwrap());

    fn race(d: i64, time: i64) -> i64 {
        let mut winner = 0;
        for t in 1..time {
            if ((time - t) * t) > d {
                winner += 1;
            }
        }
        winner
    }

    println!("Part 1: {}", total_part_1);
    println!("Part 2: {}", total_part_2);

    Ok(())
}