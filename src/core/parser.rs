use crate::common::error::AppError;
use crate::core::ansible::Inventory;
use crate::core::ssh_config::SshConfig;
use std::io::{BufRead, Write};
use tracing::info;

/// Parse the provided input SSH config, convert it to an Ansible YAML
/// inventory, named after the provided environment, and write this YAML
/// inventory to the provided output.
pub fn parse_and_serialise_as_yaml(
    environment: &str,
    input: &mut impl BufRead,
    output: &mut impl Write,
) -> Result<(), AppError> {
    let ssh_configs = SshConfig::parse(input)?;
    info!("Successfully parsed SSH config: {:?}", ssh_configs);
    let inventory = Inventory::new(environment, &ssh_configs);
    info!("Successfully generated inventory: {:?}", inventory);
    serde_yaml::to_writer(output, &inventory)?;
    info!("Successfully serialised inventory as YAML",);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::parse_and_serialise_as_yaml;
    use crate::common::error::AppError;
    use crate::common::testing::utilities::{sample_ansible_inventory, SAMPLE_SSH_CONFIG};
    use std::io::BufWriter;

    #[test]
    fn parse_ssh_config_and_serialise_as_yaml() -> Result<(), AppError> {
        // Given:
        let mut input = SAMPLE_SSH_CONFIG.as_bytes();
        let mut output = BufWriter::new(Vec::new());
        let environment = "unit-test";

        // When:
        parse_and_serialise_as_yaml(environment, &mut input, &mut output)?;

        // Then:
        let bytes = output.buffer();
        let yaml = String::from_utf8(bytes.to_vec())?;

        assert_eq!(yaml, sample_ansible_inventory(environment));
        Ok(())
    }
}
