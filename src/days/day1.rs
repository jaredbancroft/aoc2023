use std::io::BufRead;

use crate::helpers::{self, Args};
use anyhow::{Context, Result};
use log::info;

pub fn run(args: &mut Args) -> Result<()> {
    info!(target: "Day 1", "Running...");
    info!(target: "Day 1", "Parsing input from file");

    let reader = helpers::read_input_from_file(args);

    info!(target: "Day 1", "Solving...");
    for line in reader?.lines() {
        println!("{}", line.with_context(|| "Text")?);
    }

    Ok(())
}
