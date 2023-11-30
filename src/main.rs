use anyhow::{Result, Context};
use clap::Parser;
use log::info;

mod days;
use crate::days::run;
mod helpers;
use crate::helpers::Args;


fn main() -> Result<()> {
    let mut args: Args = Args::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    info!(target: "Init", "Starting...");

    info!(target: "Init", "Resolving paths");
    info!(target: "Init", "Initial path: {}", args.path.display());
    args.resolve_path();
    info!(target: "Init", "Final path: {}", args.path.display());
    info!(target: "Init", "Startup complete");
    
    info!(target: "Main", "Starting run of day{}", args.day);
    run(&mut args).with_context(|| "Main")?;
    
    Ok(())
}
