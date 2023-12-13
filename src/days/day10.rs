use std::{io::BufRead, collections::{HashMap, VecDeque}};

use crate::helpers::{self, Args};
use anyhow::{Context, Result};
use array2d::Array2D;
use colored::Colorize;
use log::{info, debug};

const MAX: usize = 140;

pub fn run(args: &mut Args) -> Result<()> {
    info!(target: "Day 10", "Running...");
    info!(target: "Day 10", "Parsing input from file");

    let reader = helpers::read_input_from_file(args)
        .with_context(|| "Problem with buffered file read")?;

    info!(target: "Day 10", "Solving...");
    let mut total_part_1 = 0;
    let mut total_part_2 = 0;
    let mut pipe_map = Array2D::filled_with('.', MAX, MAX);
    let mut s_loc = (0,0);
    let mut path: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: Vec<(usize, usize)> = Vec::new();

    for (y, line) in reader.lines().enumerate() {
        for (x,c) in line.unwrap().chars().enumerate() {
            if c == 'S' {
                s_loc = (x,y);
            }
            pipe_map[(x, y)] = c;
        }
    }

    path.push_back(s_loc);
    visited.push(s_loc);

    while !path.is_empty() {
        let current = path.pop_front().unwrap();
        for neighbor in get_neighbor_locations(current) {
            if is_valid_location(neighbor.0, pipe_map[neighbor.1], pipe_map[current]) 
            && !visited.contains(&neighbor.1) {
                debug!("Valid Neighbor {} , {:?}, {}", neighbor.0, neighbor.1, pipe_map[neighbor.1]);
                path.push_back(neighbor.1);
                visited.push(neighbor.1);
            }
        }
         total_part_1 += 1;

        debug!("{:?}", path);
    }

    for y in 0..MAX {
        for x in 0..MAX {
            let symbol = pipe_map[(x, y)];

            if visited.contains(&(x, y)) {
                if symbol == 'S' {
                    print!("{}", format!("{}", symbol).yellow());
                } else {
                    print!("{}", format!("{}", symbol).green());
                }
            } else if is_outside((x, y), visited.clone(), pipe_map.clone()) {
                print!("O");
            } else {
                print!("{}", "I".red());
                total_part_2 += 1;
            }
        }
        println!();
    }

    println!("{}", total_part_1 / 2);
    println!("{}", total_part_2);
    
    Ok(())
}

fn get_neighbor_locations(loc: (usize, usize)) -> HashMap<char, (usize, usize)> {
    
    let mut neighbors: HashMap<char, (usize, usize)> = HashMap::new();

    if loc.1 + 1 < MAX {
        let south = (loc.0, loc.1 + 1);
        neighbors.insert('S', south);
    }

    if loc.1 as i32 > 0 {
        let north = (loc.0, loc.1 - 1);
        neighbors.insert('N', north);
    }

    if loc.0 + 1 < MAX {
        let east = (loc.0 + 1, loc.1);
        neighbors.insert('E', east);
    }

    if loc.0 as i32 > 0 {
        let west = (loc.0 - 1, loc.1);
        neighbors.insert('W', west);
    }

    neighbors
}

fn is_valid_location(direction: char, symbol: char, starting_symbol: char) -> bool{
    match symbol {
        '|' if (direction == 'N' &&  ['|', 'L', 'J', 'S'].contains(&starting_symbol))
            || (direction == 'S' && ['|', '7', 'F', 'S'].contains(&starting_symbol)) => true,
        '-' if (direction == 'E' &&  ['-', 'L', 'F', 'S'].contains(&starting_symbol))
            || (direction == 'W' && ['-', '7', 'J', 'S'].contains(&starting_symbol)) => true,
        'L' if (direction == 'S' &&  ['|', '7', 'F', 'S'].contains(&starting_symbol))
            || (direction == 'W' && ['-', '7', 'J', 'S'].contains(&starting_symbol)) => true,
        'J' if (direction == 'S' &&  ['|', '7', 'F', 'S'].contains(&starting_symbol))
            || (direction == 'E' && ['-', 'L', 'F', 'S'].contains(&starting_symbol)) => true,
        '7' if (direction == 'N' &&  ['|', 'L', 'J', 'S'].contains(&starting_symbol))
            || (direction == 'E' && ['-', 'L', 'F', 'S'].contains(&starting_symbol)) => true,
        'F' if (direction == 'N' &&  ['|', 'L', 'J', 'S'].contains(&starting_symbol))
            || (direction == 'W' && ['-', '7', 'J', 'S'].contains(&starting_symbol)) => true,
        _ => false
    }
}

fn is_outside(loc: (usize, usize), loop_points: Vec<(usize, usize)>, symbols: Array2D<char>) -> bool {
    let mut x = loc.0;
    let y = loc.1;

    let mut count = 0;

    while x < MAX {
        //only works for cases where S is not equivalent to an F or 7
        if loop_points.contains(&(x, y)) && (symbols[(x, y)] == 'F' || symbols[(x, y)] == '7' || symbols[(x, y)] == '|') {
            count += 1;
        }
        x += 1;
    }

    if count % 2 == 0 {
        return true;
    }
    
    false
}