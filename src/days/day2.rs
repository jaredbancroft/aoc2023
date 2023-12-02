use std::io::BufRead;

use crate::helpers::{self, Args};
use anyhow::{Context, Result};
use log::info;

pub fn run(args: &mut Args) -> Result<()> {
    info!(target: "Day 2", "Running...");
    info!(target: "Day 2", "Parsing input from file");

    let reader = helpers::read_input_from_file(args)
        .with_context(|| "Problem with buffered file read")?;

    info!(target: "Day 2", "Solving...");

    let mut game_counter = 0;
    let mut winning_games_part1 = 0;
    let mut winning_games_part2 = 0;

    for line in reader.lines() {
        game_counter += 1;
        let game = CubeGame::from_line(line.with_context(|| "Error reading line from buffer")?);
        
        if game.is_valid() {
            winning_games_part1 += game_counter;
        }

        winning_games_part2 += game.min_cubes();
    }

    println!{"Part 1 Sum: {}", winning_games_part1};
    println!{"Part 2 Sum: {}", winning_games_part2};

    Ok(())
}

#[derive(Debug)]
struct CubeGameRound {
    red: u8,
    green: u8,
    blue: u8,
}

impl CubeGameRound {
    fn new(r: u8, g: u8, b: u8) -> CubeGameRound {
        CubeGameRound{red: r, green: g, blue: b}
    }
}

impl PartialEq for CubeGameRound {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}

#[derive(Debug)]
struct CubeGame{
    rounds: Vec<CubeGameRound>,
}

impl CubeGame {
    fn new(r: Vec<CubeGameRound>) -> CubeGame {
        CubeGame {rounds: r}
    }

    fn from_line(l: String) -> CubeGame{
        CubeGame::new(CubeGame::parse(l))
    }

    fn parse(value: String) -> Vec<CubeGameRound> {
        let v: Vec<&str> = value.split(':').collect();
        let rs: Vec<&str> = v[1].split(';').collect();
        let mut rounds: Vec<CubeGameRound> = Vec::new();
        for r in rs {
            let mut red: u8 = 0;
            let mut green: u8 = 0;
            let mut blue: u8 = 0;
            let colors: Vec<&str> = r.split(',').collect();
            for color in colors {
                let entry: Vec<&str> = color.trim_start().split(' ').collect();
                match entry[1] {
                    "red" => red = entry[0].parse::<u8>().unwrap(),
                    "green" => green = entry[0].parse::<u8>().unwrap(),
                    "blue" => blue = entry[0].parse::<u8>().unwrap(),
                    _ => panic!("Unrecoverable error: unexpected color {}", entry[1]),
                };
            }
            rounds.push(CubeGameRound::new(red, green, blue));
        }
        rounds
    }

    fn is_valid(&self) -> bool {
        const RED_MAX: u8 = 12;
        const GREEN_MAX: u8 = 13;
        const BLUE_MAX: u8 = 14;
        let mut valid = true;

        for round in &self.rounds {
            if round.red > RED_MAX || round.green > GREEN_MAX || round.blue > BLUE_MAX {
                valid = false;
            }
        }
        valid
    }
    fn min_cubes(&self) -> i32 {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for round in &self.rounds {
            if round.red > red {
                red = round.red;
            } 
            if round.green > green {
                green = round.green;
            }
            if round.blue > blue {
                blue = round.blue;
            }
        }
        red as i32 * green as i32 * blue as i32
    }

}

impl PartialEq for CubeGame {
    fn eq(&self, other: &Self) -> bool {
        self.rounds == other.rounds
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    /* cSpell:disable */
    #[test]
    fn can_parse_line_into_game() {
        let input1 = String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let test1 = CubeGame::from_line(input1);
        assert_eq!(test1, CubeGame {
            rounds: vec! [
                CubeGameRound {red: 4, green: 0, blue: 3},
                CubeGameRound {red: 1, green: 2, blue: 6},
                CubeGameRound {red: 0, green: 2, blue: 0},
            ]
        });
    }
    
}