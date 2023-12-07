use anyhow::{Context, Result};

use crate::helpers::Args;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

pub fn run(args: &mut Args) -> Result<()> {
    match args.day {
        1 => day1::run(args).with_context(|| "Error with day 1")?,
        2 => day2::run(args).with_context(|| "Error with day 2")?,
        3 => day3::run(args).with_context(|| "Error with day 3")?,
        4 => day4::run(args).with_context(|| "Error with day 4")?,
        5 => day5::run(args).with_context(|| "Error with day 5")?,
        6 => day6::run(args).with_context(|| "Error with day 6")?,
        _ => panic!(),
    }

    Ok(())
}
