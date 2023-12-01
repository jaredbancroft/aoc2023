use std::{
    path::PathBuf, 
    fs::File, 
    io::BufReader
};

use anyhow::{Context, Result};
use clap::Parser;

/// Program to select Advent of Code 2023 day to run
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Advent Day (1-25)
    #[arg(short, long, value_parser=clap::value_parser!(u8).range(1..26))]
    pub day: u8,
    /// Optional path to puzzle input file
    #[arg(short, long, value_name = "FILE", default_value = ".")]
    pub path: std::path::PathBuf,
    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}

impl Args {
    pub fn resolve_path(&mut self) {
        if self.path == PathBuf::from(".") {
            self.path.push("inputs");
            self.path.push(format!("day{}.txt", self.day))
        }
    }
}

pub fn read_input_from_file(args: &mut Args) -> Result<BufReader<File>> {
    let path = args.path.clone();
    let path_string = format!("{}", path.display());

    let f: File = File::open(path)
        .with_context(|| format!("could not read file '{}'", path_string))?;
    let reader: BufReader<File> = BufReader::new(f);

    Ok(reader)
}