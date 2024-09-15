use crate::common::error::AppError;
use crate::core::variables::ValueType;
use clap::Parser;
use clap_verbosity_flag::{Verbosity, WarnLevel};
use derive_more::FromStr;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use tracing::{debug, warn};

#[derive(Parser, Debug)]
#[command(author="Marc Carré", version, about="A tool to convert a SSH configuration to an Ansible YAML inventory.", long_about = None)]
pub struct Arguments {
    #[clap(flatten)]
    pub verbose: Verbosity<WarnLevel>,

    /// Name of the environment to generate.
    #[arg(short, long, default_value_t = String::from("local"))]
    pub environment: String,

    /// Ansible variables to add to the hosts, as colon-separated name:value pair, e.g., --var new_ssh_port:22222 --var swap_size:3G
    // #[arg(long = "var", value_parser = parse_key_val::<String, String>)]
    #[arg(long = "var", value_parser = parse_key_value)]
    pub vars: Option<Vec<(String, ValueType)>>,

    /// Path of the input SSH configuration to parse [default: stdin]
    #[arg(short, long)]
    pub input_filepath: Option<PathBuf>,

    /// Path of the output Ansible inventory file to generate [default: stdout]
    #[arg(short, long)]
    pub output_filepath: Option<PathBuf>,
}

/// Parse a single key-value pair into a (`String`, `ValueType`) pair.
fn parse_key_value(s: &str) -> Result<(String, ValueType), Box<dyn Error + Send + Sync + 'static>> {
    let pos = s
        .find(':')
        .ok_or_else(|| format!("invalid key:value pair: no `:` found in `{s}`"))?;
    let key = s[..pos].to_string();
    let value = ValueType::from_str(&s[pos + 1..])?;
    Ok((key, value))
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

#[cfg(test)]
mod tests {
    use super::Arguments;
    use crate::common::error::AppError;
    use crate::common::testing::utilities::{
        read_file, sample_ansible_inventory, temp_file, temp_filepath, SAMPLE_SSH_CONFIG,
    };
    use crate::core::variables::ValueType;
    use clap::Parser;
    use std::fs;
    use std::io::{Read, Write};

    #[test]
    fn validate_defaults() {
        let args = Arguments::parse_from([""]);
        let result = args.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn validate_vars() {
        let args = Arguments::parse_from([
            "",
            "--var",
            "become:true",
            "--var",
            "num_workers:4",
            "--var",
            "pi:1.23",
            "--var",
            "swap_size:3G",
            "--var",
            "port:'22'",
        ]);
        let result = args.validate();
        assert!(result.is_ok());
        let vars = args.vars.unwrap();
        assert_eq!(vars.len(), 5);

        let (key1, value1) = vars.clone().into_iter().next().unwrap();
        assert_eq!(key1, "become");
        assert_eq!(value1, ValueType::Bool(true));

        let (key2, value2) = vars.clone().into_iter().nth(1).unwrap();
        assert_eq!(key2, "num_workers");
        assert_eq!(value2, ValueType::Int64(4));

        let (key3, value3) = vars.clone().into_iter().nth(2).unwrap();
        assert_eq!(key3, "pi");
        assert_eq!(value3, ValueType::Float64(1.23));

        let (key4, value4) = vars.clone().into_iter().nth(3).unwrap();
        assert_eq!(key4, "swap_size");
        assert_eq!(value4, ValueType::String("3G".to_string()));

        let (key5, value5) = vars.into_iter().nth(4).unwrap();
        assert_eq!(key5, "port");
        assert_eq!(value5, ValueType::String("22".to_string()));
    }

    #[test]
    fn validate_non_existing_input_file() {
        let args = Arguments::parse_from(["", "-i", "non-existing-ssh-config-file"]);
        let result = args.validate();
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(err.to_string(), "Invalid user input for arg \"-i/--input-filepath\". Reason: the provided input filepath does not exist or cannot be accessed");
    }

    #[test]
    fn validate_invalid_input_file() {
        let args = Arguments::parse_from(["", "-i", "./target"]);
        let result = args.validate();
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(err.to_string(), "Invalid user input for arg \"-i/--input-filepath\". Reason: the provided input filepath is not a file");
    }

    #[test]
    fn validate_valid_output_file() -> Result<(), AppError> {
        fs::create_dir_all("./target/test")?;
        let args =
            Arguments::parse_from(["", "-o", "./target/test/validate_valid_output_file.yaml"]);
        let result = args.validate();
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn validate_invalid_output_file() {
        let args = Arguments::parse_from(["", "-o", "./target"]);
        let result = args.validate();
        assert!(result.is_err());
        let err = result.err().unwrap();
        assert_eq!(err.to_string(), "Invalid user input for arg \"-o/--output-filepath\". Reason: the provided output filepath is not a file");
    }

    #[test]
    fn input_from_file() -> Result<(), AppError> {
        // Given:
        let (dir, input_filepath) = temp_file("test_input_from_file", SAMPLE_SSH_CONFIG)?;
        let args = Arguments::parse_from(["", "-i", &input_filepath.to_string_lossy()]);
        args.validate()?;

        // When:
        let mut input_file = args.input()?;

        // Then:
        let mut input_string = String::new();
        input_file.read_to_string(&mut input_string)?;
        assert_eq!(input_string, SAMPLE_SSH_CONFIG);
        dir.close()?; // clean-up.
        Ok(())
    }

    #[test]
    fn output_to_file() -> Result<(), AppError> {
        // Given:
        let sample_inventory = sample_ansible_inventory("unit-test");
        let (dir, output_filepath) = temp_filepath("test_output_to_file")?;
        let args = Arguments::parse_from(["", "-o", &output_filepath.to_string_lossy()]);
        args.validate()?;

        // When:
        let mut output_file = args.output()?;
        output_file.write_all(sample_inventory.as_bytes())?;
        output_file.flush()?;

        // Then:
        let output_string = read_file(&output_filepath)?;
        assert_eq!(output_string, sample_inventory);
        dir.close()?; // clean-up.
        Ok(())
    }
}
