use std::{io::BufRead, collections::HashSet};

use crate::helpers::{self, Args};
use anyhow::{Context, Result};
use array2d::Array2D;
use log::info;

pub fn run(args: &mut Args) -> Result<()> {
    info!(target: "Day 3", "Running...");
    info!(target: "Day 3", "Parsing input from file");

    let reader = helpers::read_input_from_file(args)
        .with_context(|| "Problem with buffered file read")?;

    info!(target: "Day 3", "Solving...");

    let mut s = Array2D::filled_with('.', Y_MAX, X_MAX);
    let mut p: Vec<PartNumber> = Vec::new();
    let mut r: Vec<Point> = Vec::new();

    for (y_pos, line) in reader.lines().enumerate() {
        let mut l: Vec<Point> = Vec::new();
        let mut n: String = String::new();
        for (x_pos, value) in line.unwrap().chars().enumerate() {
                if value == '*' {
                    r.push(Point::new(x_pos, y_pos));
                }
                if is_number(value) {
                    n.push(value);
                    l.push(Point::new(x_pos, y_pos));
                } else if !l.is_empty() {
                    p.push(PartNumber::new(n.clone(), n.parse::<i32>().unwrap(), l.clone()));
                    n.clear();
                    l.clear();
                }
            s[(x_pos, y_pos)] = value;
        }
        if !l.is_empty() {
            p.push(PartNumber::new(n.clone(), n.parse::<i32>().unwrap(), l.clone()));
            n.clear();
            l.clear();
        }
    }

    let engine = Engine::new(s, p);
    println!("Part 1: Sum of Part Numbers = {}", engine.calculate_sum_of_part_numbers());
    println!("Part 2: Sum of Gear Ratios = {}", engine.calculate_sum_of_gear_ratios(r));

    Ok(())
}

fn is_symbol(c: char) -> bool {
    if is_period(c) || is_number(c) {
        return false;
    }
    true
}

fn is_number(c: char) -> bool {
    "0123456789".contains(c)
}

fn is_period(c: char) -> bool {
    c == '.'
}

const X_MAX: usize = 140;
const Y_MAX: usize = 140;

struct Engine {
    schematic: Array2D<char>,
    part_numbers: Vec<PartNumber>,
}

impl Engine {
    fn new(schematic: Array2D<char>, part_numbers: Vec<PartNumber>) -> Engine {
        Engine {
            schematic,
            part_numbers,
        }
    }

    fn calculate_sum_of_part_numbers(&self) -> i32 {
        let mut total: i32 = 0;
        for part_number in &self.part_numbers {
            if self.is_valid_part_number(part_number) {
                total += part_number.value;
            }
        }
        total
    }

    fn calculate_sum_of_gear_ratios(&self, ratios: Vec<Point>) -> i32 {
        let mut total: i32 = 0;
        let mut part_numbers: HashSet<PartNumber> = HashSet::new();
        for point in ratios {
            let neighbors: Vec<(usize, usize)> = point.find_neighbors();
            for neighbor in neighbors {
                if is_number(self.schematic[neighbor]) {
                    for p in &self.part_numbers {
                        let test_point = Point::from_tuple(neighbor);
                        if p.in_location(&test_point) {
                            part_numbers.insert(p.clone());
                        }
                    }
                }
            }
            if part_numbers.len() == 2 {
                let mut ratio = 1;
                for part in &part_numbers {
                    ratio *= part.value;
                }
                total += ratio
            }
            part_numbers.clear();
        }
        total
    }

    fn is_valid_part_number(&self, part_number: &PartNumber) -> bool {
        for location in &part_number.locations {
            for neighbor in location.find_neighbors() {
                if is_symbol(self.schematic[neighbor]) {
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct PartNumber {
    name: String,
    value: i32,
    locations: Vec<Point>, 
}

impl PartNumber {
    fn new(name: String, value:i32, locations:Vec<Point> ) -> PartNumber {
        PartNumber {
            name,
            value, 
            locations,
        }
    }

    fn in_location(&self, point: &Point) -> bool {
        if self.locations.contains(point) {
            return true;
        }
        false
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point {x, y}
    }

    fn from_tuple(t: (usize, usize)) -> Point {
        Point {x: t.0, y: t.1 }
    }

    fn find_neighbors(&self) -> Vec<(usize, usize)> {
        let mut neighbors: Vec<(usize, usize)> = Vec::new();
        for y in -1..=1 {
            if (self.y as i32) + y >= 0 && (self.y as i32) + y < Y_MAX as i32 {
                for x in -1..=1 {
                    if (self.x as i32) + x >= 0 && (self.x as i32) + x < X_MAX as i32 {
                        neighbors.push(((self.x as i32 + x) as usize, (self.y as i32 + y) as usize));
                    }
                }
            }
        }
        neighbors
    }
}