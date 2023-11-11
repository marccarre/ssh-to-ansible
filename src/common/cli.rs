use crate::common::error::AppError;
use clap::Parser;
use clap_verbosity_flag::{Verbosity, WarnLevel};
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use tracing::{debug, warn};

#[derive(Parser, Debug)]
#[command(author="Marc Carré", version, about="A tool to convert a SSH configuration to an Ansible YAML inventory.", long_about = None)]
pub struct Arguments {
    #[clap(flatten)]
    pub verbose: Verbosity<WarnLevel>,

    #[arg(long, default_value_t = false)]
    pub debug: bool,

    /// Name of the environment to generate.
    #[arg(short, long, default_value_t = String::from("local"))]
    pub environment: String,

    /// Path of the input SSH configuration to parse [default: stdin]
    #[arg(short, long)]
    pub input_filepath: Option<PathBuf>,

    /// Path of the output Ansible inventory file to generate [default: stdout]
    #[arg(short, long)]
    pub output_filepath: Option<PathBuf>,
}

impl Arguments {
    pub fn validate(&self) -> Result<(), AppError> {
        if let Some(input_filepath) = &self.input_filepath {
            if !input_filepath.exists() {
                return Err(AppError::InvalidInput {
                    arg: "-i/--input-filepath",
                    reason: "the provided input filepath does not exist or cannot be accessed"
                        .to_string(),
                });
            }
            if !input_filepath.is_file() {
                return Err(AppError::InvalidInput {
                    arg: "-i/--input-filepath",
                    reason: "the provided input filepath is not a file".to_string(),
                });
            }
        }
        if let Some(output_filepath) = &self.output_filepath {
            if output_filepath.exists() {
                if output_filepath.is_file() {
                    warn!(
                        "file will be overwritten: {:?}",
                        output_filepath.as_os_str()
                    );
                } else {
                    return Err(AppError::InvalidInput {
                        arg: "-o/--output-filepath",
                        reason: "the provided output filepath is not a file".to_string(),
                    });
                }
            }
        }
        debug!("Input successfully validated");
        Ok(())
    }

    pub fn input(&self) -> Result<impl BufRead, AppError> {
        let input: Box<dyn BufRead> = if let Some(input_filepath) = &self.input_filepath {
            let input_file = File::open(input_filepath)?;
            debug!("Opened input file: {:?}", input_filepath);
            Box::new(BufReader::new(input_file))
        } else {
            Box::new(io::stdin().lock())
        };
        Ok(input)
    }

    pub fn output(&self) -> Result<impl Write, AppError> {
        let output: Box<dyn Write> = if let Some(output_filepath) = &self.output_filepath {
            let output_file = File::create(output_filepath)?;
            debug!("Created output file: {:?}", output_filepath);
            Box::new(BufWriter::new(output_file))
        } else {
            Box::new(io::stdout().lock())
        };
        Ok(output)
    }
}
