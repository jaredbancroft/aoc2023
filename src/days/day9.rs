use std::io::BufRead;

use crate::helpers::{self, Args};
use anyhow::{Context, Result};
use log::info;

pub fn run(args: &mut Args) -> Result<()> {
    info!(target: "Day 9", "Running...");
    info!(target: "Day 9", "Parsing input from file");

    let reader = helpers::read_input_from_file(args)
        .with_context(|| "Problem with buffered file read")?;

    info!(target: "Day 9", "Solving...");
    let mut total_part_1 = 0;
    let mut total_part_2 = 0;
    let lines = reader.lines();
    let mut intermediate_vectors_part1: Vec<Vec<i32>> = Vec::new();
    let mut intermediate_vectors_part2: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        let mut vec: Vec<i32> = line.unwrap().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        while !is_all_zeros(&vec) {
            intermediate_vectors_part1.push(vec.clone());
            intermediate_vectors_part2.push(vec.clone());
            vec = vec_diff(vec);
        }

        let mut previous_element = 0;
        while let Some(last_element) = intermediate_vectors_part1.pop() {
            previous_element += last_element.iter().last().unwrap();
        }
        total_part_1 += previous_element;

        previous_element = 0;
        while let Some(first_element) = intermediate_vectors_part2.pop() {
            previous_element = first_element.first().unwrap() - previous_element;
        }

        total_part_2 += previous_element;

    }

    println!("{}", total_part_1);
    println!("{}", total_part_2);
    
    Ok(())
}

fn vec_diff(input: Vec<i32>) -> Vec<i32> {
    let vals = input.iter();
    let next_vals = input.iter().skip(1);

    vals.zip(next_vals).map(|(cur, next)| next - cur).collect()
}

fn is_all_zeros(arr: &Vec<i32>) -> bool {
    if arr.is_empty() {
        return true;
    }
    let first = 0;
    arr.iter().all(|&item| item == first)
}