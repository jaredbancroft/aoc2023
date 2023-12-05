use std::{io::BufRead, collections::HashMap};

use crate::helpers::{self, Args};
use anyhow::{Context, Result};
use log::info;

pub fn run(args: &mut Args) -> Result<()> {
    info!(target: "Day 4", "Running...");
    info!(target: "Day 4", "Parsing input from file");

    let reader = helpers::read_input_from_file(args)
        .with_context(|| "Problem with buffered file read")?;

    info!(target: "Day 4", "Solving...");

    let mut total_part1: u32 = 0;
    let mut total_part2: u32 = 0;

    let mut game: HashMap<u32, u32> = HashMap::new();

    for (idx, line) in reader.lines().enumerate() {
        let mut card = Card::new();
        card.read(line.with_context(|| "Error reading line")?);
        let number_of_winners = card.check_number_of_winners();

        total_part1 += card.score_winner(number_of_winners);
        
        let k = idx as u32 + 1;

        game.entry(k).and_modify(|v| *v += 1).or_insert(1);

        for _ in 0..*game.get(&k).unwrap() {
            for j in 1..number_of_winners + 1 {
                game.entry(k + j).and_modify(|v| *v += 1).or_insert(1);
            }
        }
    }
    
    for v in game.values() {
        total_part2 += v;
    }
    println!("Part1: {}", total_part1);
    println!("Part1: {}", total_part2);
   
    Ok(())
}

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<String>,
    my_numbers: Vec<String>
}

impl Card {
    fn new() -> Card {
        Card {
            winning_numbers: Vec::new(), 
            my_numbers: Vec::new()
        }
    }

    fn read(&mut self, input: String) {
        let numbers: Vec<&str> = (input.split(':').collect::<Vec<&str>>()[1]).split('|').collect();
        let winning_numbers: Vec<String> = numbers[0].split_whitespace().map(|x: &str| x.to_string()).collect();
        let my_numbers: Vec<String> = numbers[1].split_whitespace().map(|x: &str| x.to_string()).collect();
        self.winning_numbers = winning_numbers;
        self.my_numbers = my_numbers;
    }

    fn check_number_of_winners(&self) -> u32 {
        let mut num: u32 = 0;

        for my_number in &self.my_numbers {
            if self.winning_numbers.contains(my_number) {
                num += 1;
            }
        }
        num
    }

    fn score_winner(&self, num: u32) -> u32 {
        let base: u32 = 2;
        if num == 0 {
            return 0;
        }
        base.pow(num - 1)
    }
}