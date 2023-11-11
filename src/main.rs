mod common;
mod core;

use crate::common::cli;
use crate::common::error::AppError;
use crate::core::parser::parse_and_serialise_as_yaml;
use clap::Parser;
use tracing::info;

fn main() -> Result<(), AppError> {
    let args = cli::Arguments::parse();
    common::tracing::initialise(&args);
    info!("Argumented received: {:?}", args);
    args.validate()?;
    let mut input = args.input()?;
    let mut output = args.output()?;
    parse_and_serialise_as_yaml(&args.environment, &mut input, &mut output)?;
    info!("That's all folks! 👋🏻😊");
    Ok(())
}
