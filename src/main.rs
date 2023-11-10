mod common;
mod core;

use crate::common::cli;
use crate::common::error::AppError;
use crate::core::ansible::Inventory;
use crate::core::parser;
use clap::Parser;
use std::fs::File;
use std::io;
use tracing::info;

fn main() -> Result<(), AppError> {
    let args = cli::Arguments::parse();
    common::tracing::initialise(&args);
    info!("Argumented received: {:?}", args);
    let stdin = io::stdin();
    let hosts = parser::parse(&mut stdin.lock())?;
    info!("Parsed SSH config: {:?}", hosts);
    let inventory = Inventory::new(args.environment, hosts);
    info!("Created inventory: {:?}", inventory);
    let file = File::create(&args.filepath)?;
    info!("Created output file: {:?}", args.filepath);
    serde_yaml::to_writer(file, &inventory)?;
    info!("Serialised inventory as YAML at: {:?}", args.filepath);
    Ok(())
}
