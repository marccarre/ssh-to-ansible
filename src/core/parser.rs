use crate::common::error::AppError;
use crate::core::ansible::{HostParams, Hosts, Inventory};
use ssh2_config::{Host, ParseRule, SshConfig};
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
    let hosts = parse(input)?;
    info!("Successfully parsed SSH config: {:?}", hosts);
    let inventory = Inventory::new(environment, hosts);
    info!("Successfully generated inventory: {:?}", inventory);
    serde_yaml::to_writer(output, &inventory)?;
    info!("Successfully serialised inventory as YAML",);
    Ok(())
}

/// Parse the provided SSH config into a collection of `Hosts`.
pub fn parse(reader: &mut impl BufRead) -> Result<Hosts, AppError> {
    let config = SshConfig::default().parse(reader, ParseRule::STRICT)?;
    let ssh_hosts: Vec<&Host> = ssh_hosts_from(&config);
    let hosts = ssh_hosts
        .into_iter()
        .map(|host| (host_nickname(&host.pattern), HostParams::new(&host.params)))
        .collect::<Hosts>();
    Ok(hosts)
}

/// Get actual hosts from the provided SSH config,
/// i.e. remove wildcard ('*') host.
fn ssh_hosts_from(config: &SshConfig) -> Vec<&Host> {
    config
        .get_hosts()
        .iter()
        .filter(|host| {
            host.pattern
                .iter()
                .any(|host_clause| host_clause.pattern != "*")
        })
        .collect::<Vec<&Host>>()
}

fn host_nickname(pattern: &[ssh2_config::HostClause]) -> String {
    pattern
        .first()
        .expect("host pattern to contains at a least 1 host nickname")
        .pattern
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::{parse, parse_and_serialise_as_yaml};
    use crate::common::error::AppError;
    use crate::common::testing::utilities::{sample_ansible_inventory, SAMPLE_SSH_CONFIG};
    use crate::core::ansible::HostParams;
    use std::io::BufWriter;
    use std::path::PathBuf;

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

    #[test]
    fn parse_ssh_config() -> Result<(), AppError> {
        // Given:
        let mut input = SAMPLE_SSH_CONFIG.as_bytes();

        // When:
        let hosts = parse(&mut input)?;

        // Then:
        assert_eq!(hosts.len(), 1);
        assert!(hosts.contains_key("default"));
        let host_params = hosts
            .get("default")
            .cloned()
            .expect("value to exist since key exists");
        assert_eq!(
            host_params,
            HostParams {
                ansible_host: Some("127.0.0.1".to_string()),
                ansible_port: Some(50022u16),
                ansible_user: Some("vagrant".to_string()),
                ansible_ssh_private_key_file: Some(PathBuf::from("/path/to/private_key")),
                ansible_ssh_common_args: None, // Not supported for now.
                ansible_ssh_extra_args: None,  // Not supported for now.
            }
        );
        Ok(())
    }
}
