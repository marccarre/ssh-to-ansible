use crate::common::error::AppError;
use crate::core::ansible::Inventory;
use crate::core::ssh_config::SshConfig;
use crate::core::variables::ValueType;
use std::io::{BufRead, Write};
use tracing::info;

/// Parse the provided input SSH config, convert it to an Ansible YAML
/// inventory, named after the provided environment, and write this YAML
/// inventory to the provided output.
pub fn parse_and_serialise_as_yaml(
    environment: &str,
    vars: &Option<Vec<(String, ValueType)>>,
    input: &mut impl BufRead,
    output: &mut impl Write,
) -> Result<(), AppError> {
    let ssh_configs = SshConfig::parse(input)?;
    info!("Successfully parsed SSH config: {:?}", ssh_configs);
    let inventory = Inventory::new(environment, &ssh_configs, vars);
    info!("Successfully generated inventory: {:?}", inventory);
    serde_yaml::to_writer(output, &inventory)?;
    info!("Successfully serialised inventory as YAML",);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::parse_and_serialise_as_yaml;
    use crate::common::error::AppError;
    use crate::common::testing::utilities::{
        sample_ansible_inventory, sample_ansible_inventory_with_vars, SAMPLE_SSH_CONFIG,
    };
    use crate::core::variables::ValueType;
    use std::io::BufWriter;

    #[test]
    fn parse_ssh_config_with_no_vars_and_serialise_as_yaml() -> Result<(), AppError> {
        // Given:
        let mut input = SAMPLE_SSH_CONFIG.as_bytes();
        let mut output = BufWriter::new(Vec::new());
        let environment = "unit-test";

        // When:
        parse_and_serialise_as_yaml(environment, &Option::None, &mut input, &mut output)?;

        // Then:
        let bytes = output.buffer();
        let yaml = String::from_utf8(bytes.to_vec())?;

        assert_eq!(yaml, sample_ansible_inventory(environment));
        Ok(())
    }

    #[test]
    fn parse_ssh_config_with_vars_and_serialise_as_yaml() -> Result<(), AppError> {
        // Given:
        let mut input = SAMPLE_SSH_CONFIG.as_bytes();
        let mut output = BufWriter::new(Vec::new());
        let environment = "unit-test";
        let vars = Some(Vec::from([
            ("become".to_string(), ValueType::Bool(true)),
            (
                "http_port".to_string(),
                ValueType::String("8080".to_string()),
            ),
            ("num_workers".to_string(), ValueType::Int64(4)),
            ("swap_size".to_string(), ValueType::String("3G".to_string())),
        ]));

        // When:
        parse_and_serialise_as_yaml(environment, &vars, &mut input, &mut output)?;

        // Then:
        let bytes = output.buffer();
        let yaml = String::from_utf8(bytes.to_vec())?;

        assert_eq!(yaml, sample_ansible_inventory_with_vars(environment));
        Ok(())
    }
}
