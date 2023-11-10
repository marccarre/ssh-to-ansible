mod common;
mod core;

use crate::common::cli;
use crate::common::error::AppError;
use crate::core::ansible::Inventory;
use crate::core::parser;
use clap::Parser;
use common::cli::Arguments;
use std::fs::File;
use std::io::{self, BufRead};
use tracing::info;

fn main() -> Result<(), AppError> {
    let args = cli::Arguments::parse();
    let mut stdin = io::stdin().lock();
    parse_and_serialise_to_yaml(args, &mut stdin)
}

fn parse_and_serialise_to_yaml(args: Arguments, input: &mut impl BufRead) -> Result<(), AppError> {
    info!("Argumented received: {:?}", args);
    common::tracing::initialise(&args);
    let hosts = parser::parse(input)?;
    info!("Parsed SSH config: {:?}", hosts);
    let inventory = Inventory::new(args.environment, hosts);
    info!("Created inventory: {:?}", inventory);
    let file = File::create(&args.filepath)?;
    info!("Created output file: {:?}", args.filepath);
    serde_yaml::to_writer(file, &inventory)?;
    info!("Serialised inventory as YAML at: {:?}", args.filepath);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::parse_and_serialise_to_yaml;
    use crate::common::cli::Arguments;
    use crate::common::error::AppError;
    use clap::Parser;
    use std::fs::{self, File};
    use std::io::{BufReader, Read};

    #[test]
    fn parse_and_serialise_to_yaml_with_short_options() -> Result<(), AppError> {
        // Given:
        let filepath = "target/test/parse_and_serialise_to_yaml_with_short_options.yaml";
        fs::create_dir_all("target/test")?;
        let args = Arguments::parse_from(["", "-e", "dev", "-f", filepath]);
        let ssh_config = sample_ssh_config();
        let mut input = ssh_config.as_bytes();

        // When:
        parse_and_serialise_to_yaml(args, &mut input)?;

        // Then:
        let contents = read_file(filepath)?;
        assert_eq!(
            contents,
            r###"dev:
  hosts:
    default:
      ansible_host: 127.0.0.1
      ansible_port: 50022
      ansible_user: vagrant
      ansible_ssh_private_key_file: /Users/me/.vagrant/machines/default/qemu/private_key
"###
        );
        Ok(())
    }

    fn sample_ssh_config() -> String {
        r###"Host default
  HostName 127.0.0.1
  User vagrant
  Port 50022
  UserKnownHostsFile /dev/null
  StrictHostKeyChecking no
  PasswordAuthentication no
  IdentityFile /Users/me/.vagrant/machines/default/qemu/private_key
  IdentitiesOnly yes
  LogLevel FATAL
  PubkeyAcceptedKeyTypes +ssh-rsa
  HostKeyAlgorithms +ssh-rsa"###
            .to_string()
    }

    fn read_file(filepath: &str) -> Result<String, AppError> {
        let file = File::open(filepath)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        Ok(contents)
    }
}
