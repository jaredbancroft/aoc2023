use std::{io::BufRead, collections::HashMap};

use crate::helpers::{self, Args};
use anyhow::{Context, Result};
use log::info;

pub fn run(args: &mut Args) -> Result<()> {
    info!(target: "Day 8", "Running...");
    info!(target: "Day 8", "Parsing input from file");

    let reader = helpers::read_input_from_file(args)
        .with_context(|| "Problem with buffered file read")?;

    info!(target: "Day 8", "Solving...");
    let mut total_part_1 = 0;

    let mut lines = reader.lines();
    let mut starting_nodes: Vec<String> = Vec::new();
    let directions = lines.next().unwrap().unwrap();
    lines.next();

    let mut network: HashMap<String, (String, String)> = HashMap::new();

    for line in lines {
        let l = line.unwrap();
        let values = l.split('=').collect::<Vec<&str>>();
        let node = values[0].trim().to_string();
        if is_starting_node(&node) {
            starting_nodes.push(node.to_string());
        }
        let next_nodes = rem_first_and_last(values[1]).split(',').collect::<Vec<&str>>();
        let left_node = next_nodes[0].trim().to_string();
        let right_node = next_nodes[1].trim().to_string();
       
        network.insert(node, (left_node, right_node));
    }

    let mut location = "AAA";

    while location != "ZZZ" {
        for direction in directions.chars() {
            if direction == 'R' {
                location = &network.get(location).unwrap().1;
                total_part_1 += 1;
            } else {
                location = &network.get(location).unwrap().0;
                total_part_1 += 1;
            }
        }
    }
    println!("{}", total_part_1);

    let mut p2_res: HashMap<String, i32> = HashMap::new();

    for node in starting_nodes {
        let mut test_node = node;
        let mut count = 0;
        while !is_ending_node(&test_node) {
            for direction in directions.chars() {
                if direction == 'R' {
                    test_node = network.get(&test_node).unwrap().1.clone();
                } else {
                    test_node = network.get(&test_node).unwrap().0.clone();
                }
                count += 1;
            }
        }
        p2_res.insert(test_node, count);
    }
    
    println!("{:?}", p2_res.values());
    //put values into lowest common multiple calculator and that is the answer.
    Ok(())
}

fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next();
    chars.next_back();
    chars.as_str()
}

fn is_starting_node(value: &str) -> bool {
    let mut chars = value.chars();
    chars.next();
    chars.next();
    if chars.next().unwrap() == 'A' {
        return true;
    }
    false
}

fn is_ending_node(value: &str) -> bool {
    let mut chars = value.chars();
    chars.next();
    chars.next();
    if chars.next().unwrap() == 'Z' {
        return true;
    }
    false
}