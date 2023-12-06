use std::io::BufRead;

use crate::helpers::{self, Args};
use anyhow::{Context, Result};
use log::info;

pub fn run(args: &mut Args) -> Result<()> {
    info!(target: "Day 5", "Running...");
    info!(target: "Day 5", "Parsing input from file");

    let reader = helpers::read_input_from_file(args)
        .with_context(|| "Problem with buffered file read")?;

    info!(target: "Day 5", "Solving...");

    let mut almanac = Almanac::new();
    let mut a = String::new();

    for line in reader.lines() {
        a = [a.clone(), line.with_context(|| "Error reading line")?].join("\n");
    }
    let chunks = a.split("\n\n");
    info!(target: "Day 5", "Reading chunks...");
    for chunk in chunks {
        almanac.read_chunk(chunk);
    }
    info!(target: "Day 5", "Reading chunks...Done");

    for seed in almanac.seeds.clone() {
        almanac.part1(seed);
    }
    
    println!("Part 1 - Lowest Location {}", almanac.lowest);

    almanac.lowest = i64::MAX;

    let a = almanac.seeds.clone();

    for i in (0..a.len()).step_by(2) {
        for j in a[i]..a[i]+a[i+1] {
            almanac.part1(j);
        }
    }

    println!("Part 2 - Lowest Location {}", almanac.lowest);

    Ok(())
}

#[derive(Debug, Clone)]
struct Almanac {
    seeds: Vec<i64>,
    soil: Vec<Vec<i64>>,
    fertilizer: Vec<Vec<i64>>,
    water: Vec<Vec<i64>>,
    light: Vec<Vec<i64>>,
    temperature: Vec<Vec<i64>>,
    humidity: Vec<Vec<i64>>,
    location: Vec<Vec<i64>>,
    lowest: i64,
}

impl Almanac {
    fn new() -> Almanac{
        Almanac {
            seeds: Vec::new(),
            soil: Vec::new(),
            fertilizer: Vec::new(),
            water: Vec::new(),
            light: Vec::new(),
            temperature: Vec::new(),
            humidity: Vec::new(),
            location: Vec::new(),
            lowest: i64::MAX,         
        }
    }

    fn read_chunk(&mut self, chunk: &str) {
        let id = chunk.trim().split(':').collect::<Vec<&str>>()[0];
        let values = chunk.trim().split(':').collect::<Vec<&str>>()[1];
        self.set_by_id(id, values);
        
    }

    fn set_by_id(&mut self, id: &str, values: &str) {
        let mut map: Vec<Vec<i64>> = Vec::new();

        for value in  values.trim().split('\n').collect::<Vec<&str>>() {
            let v = value.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            map.push(v);
        }
        
        match id {
            "seeds"       => self.seeds = map[0].clone(),
            "seed-to-soil map"        => self.soil = map,
            "soil-to-fertilizer map"  => self.fertilizer = map,
            "fertilizer-to-water map"       => self.water = map,
            "water-to-light map"       => self.light = map,
            "light-to-temperature map" => self.temperature = map,
            "temperature-to-humidity map"    => self.humidity = map,
            "humidity-to-location map"    => self.location = map,
            _ => panic!("Unrecoverable error - no matching id")
        }
    }

    fn part1(&mut self, seed: i64) {
        let list = ["soil", "fertilizer", "water", "light", "temperature", "humidity", "location"];
        let mut src = seed;
        for l in list {
            src = match l {
                "soil" => self.next_seed(src, self.soil.as_slice()),
                "fertilizer" => self.next_seed(src, self.fertilizer.as_slice()),
                "water" => self.next_seed(src, self.water.as_slice()),
                "light" => self.next_seed(src, self.light.as_slice()),
                "temperature" => self.next_seed(src, self.temperature.as_slice()),
                "humidity" => self.next_seed(src, self.humidity.as_slice()),
                "location" => self.next_seed(src, self.location.as_slice()),
                _ => panic!()
            };
        }
        if src < self.lowest {
            self.lowest = src;
        }
    }

    fn next_seed(&self, src: i64, param: &[Vec<i64>]) -> i64 {
        for p in param {
            if (p[1]..p[1]+p[2]).contains(&src) {
                return p[0] + (src - p[1]);
            }
        }
        src
    }
}