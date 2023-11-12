mod common;
mod core;

#[macro_use]
extern crate lazy_static;

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

#[cfg(test)]
mod tests {
    use crate::common::testing::utilities::{
        read_file, sample_ansible_inventory, temp_file, temp_filepath, SAMPLE_SSH_CONFIG,
    };
    use assert_cmd::Command;
    use predicates::ord::eq;

    #[test]
    fn s2a_read_stdin_write_stdout_default_environment() {
        // Given:
        let mut cmd = Command::cargo_bin("s2a").unwrap();

        // When:
        let assert = cmd.write_stdin(SAMPLE_SSH_CONFIG).assert();

        // Then:
        assert
            .success()
            .code(eq(0))
            .stdout(eq(sample_ansible_inventory("local")));
    }

    #[test]
    fn s2a_read_stdin_write_stdout_custom_environment() {
        // Given:
        let mut cmd = Command::cargo_bin("s2a").unwrap();
        let environment = "e2e-test";

        // When:
        let assert = cmd
            .arg("-e")
            .arg(environment)
            .write_stdin(SAMPLE_SSH_CONFIG)
            .assert();

        // Then:
        assert
            .success()
            .code(eq(0))
            .stdout(eq(sample_ansible_inventory(environment)));
    }

    #[test]
    fn s2a_read_file_write_stdout_default_environment() -> Result<(), std::io::Error> {
        // Given:
        let (dir, input_filepath) = temp_file("test_input_from_file", SAMPLE_SSH_CONFIG)?;
        let mut cmd = Command::cargo_bin("s2a").unwrap();

        // When:
        let assert = cmd.arg("-i").arg(input_filepath).assert();

        // Then:
        assert
            .success()
            .code(eq(0))
            .stdout(eq(sample_ansible_inventory("local")));

        dir.close()?; // clean-up.
        Ok(())
    }

    #[test]
    fn s2a_read_stdin_write_file_default_environment() -> Result<(), std::io::Error> {
        // Given:
        let (dir, output_filepath) = temp_filepath("test_output_to_file")?;
        let mut cmd = Command::cargo_bin("s2a").unwrap();

        // When:
        let assert = cmd
            .arg("-o")
            .arg(&output_filepath)
            .write_stdin(SAMPLE_SSH_CONFIG)
            .assert();

        // Then:
        assert.success().code(eq(0)).stdout(eq(""));
        let output_string = read_file(&output_filepath)?;
        assert_eq!(output_string, sample_ansible_inventory("local"));

        dir.close()?; // clean-up.
        Ok(())
    }
}
