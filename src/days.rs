use anyhow::{Context, Result};

use crate::helpers::Args;

mod day1;
mod day2;


pub fn run(args: &mut Args) -> Result<()> {
    match args.day {
        1 => day1::run(args).with_context(|| "Error with day 1")?,
        2 => day2::run(args).with_context(|| "Error with day 1")?,
        _ => panic!(),
    }

    Ok(())
}
