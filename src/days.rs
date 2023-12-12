use anyhow::{Context, Result};

use crate::helpers::Args;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub fn run(args: &mut Args) -> Result<()> {
    match args.day {
        1 => day1::run(args).with_context(|| "Error with day 1")?,
        2 => day2::run(args).with_context(|| "Error with day 2")?,
        3 => day3::run(args).with_context(|| "Error with day 3")?,
        4 => day4::run(args).with_context(|| "Error with day 4")?,
        5 => day5::run(args).with_context(|| "Error with day 5")?,
        6 => day6::run(args).with_context(|| "Error with day 6")?,
        7 => day7::run(args).with_context(|| "Error with day 7")?,
        8 => day8::run(args).with_context(|| "Error with day 8")?,
        9 => day9::run(args).with_context(|| "Error with day 9")?,
        _ => panic!(),
    }

    Ok(())
}
