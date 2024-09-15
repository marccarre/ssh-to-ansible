use crate::core::ssh_config::{Field, SshConfig};
use crate::core::variables::ValueType;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use tracing::debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct Inventory {
    #[serde(flatten)]
    groups: BTreeMap<String, Hosts>,
}

impl Inventory {
    pub fn new(
        name: &str,
        ssh_configs: &[SshConfig],
        vars: &Option<Vec<(String, ValueType)>>,
    ) -> Inventory {
        Inventory {
            groups: BTreeMap::from([(name.to_owned(), Hosts::new(ssh_configs, vars))]),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hosts {
    hosts: BTreeMap<String, HostParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vars: Option<BTreeMap<String, ValueType>>,
}

impl Hosts {
    pub fn new(ssh_configs: &[SshConfig], vars: &Option<Vec<(String, ValueType)>>) -> Hosts {
        Hosts {
            hosts: ssh_configs
                .iter()
                .map(|ssh_config| (ssh_config.host.to_owned(), HostParams::new(ssh_config)))
                .collect::<BTreeMap<String, HostParams>>(),
            vars: vars
                .clone()
                .map(|vec| vec.into_iter().collect::<BTreeMap<String, ValueType>>()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
/// HostParams groups all the Ansible inventory parameters for Ansible to connect to this host.
/// See also: https://docs.ansible.com/ansible/latest/inventory_guide/intro_inventory.html#connecting-to-hosts-behavioral-inventory-parameters
pub struct HostParams {
    /// The name of the host to connect to, if different from the alias you wish to give to it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ansible_host: Option<String>,

    /// The connection port number, if not the default (22 for ssh)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ansible_port: Option<u16>,

    /// The user name to use when connecting to the host
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ansible_user: Option<String>,

    /// Private key file used by SSH. Useful if using multiple keys and you do not want to use SSH agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ansible_ssh_private_key_file: Option<String>,

    /// This setting is always appended to the default command line for sftp, scp, and ssh. Useful to configure a ProxyCommand for a certain host (or group).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ansible_ssh_common_args: Option<String>,

    /// This setting is always appended to the default ssh command line.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ansible_ssh_extra_args: Option<String>,
}

impl HostParams {
    pub fn new(ssh_config: &SshConfig) -> HostParams {
        debug!("Provided SSH config: {:?}", ssh_config);
        let mut fields = ssh_config.fields.clone();
        let ansible_host = fields.remove(&Field::HostName);
        let ansible_port = fields
            .remove(&Field::Port)
            .map(|s| s.parse::<u16>().expect("an integer between 0 and 65535"));
        let ansible_user = fields.remove(&Field::User);
        let ansible_ssh_private_key_file = fields.remove(&Field::IdentityFile);
        let ansible_ssh_common_args = fields.remove(&Field::ProxyCommand);
        let ansible_ssh_extra_args = if fields.is_empty() {
            None
        } else {
            Some(
                fields
                    .into_iter()
                    .map(|(k, v)| format!("-o {k}={v}"))
                    .collect::<Vec<String>>()
                    .join(" "),
            )
        };
        HostParams {
            ansible_host,
            ansible_port,
            ansible_user,
            ansible_ssh_private_key_file,
            ansible_ssh_common_args,
            ansible_ssh_extra_args,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{HostParams, Inventory};
    use crate::core::{
        ssh_config::{Field, SshConfig},
        variables::ValueType,
    };
    use serde_yaml;
    use std::collections::BTreeMap;

    fn sample_host_params() -> HostParams {
        HostParams {
            ansible_host: Some("127.0.0.1".to_string()),
            ansible_port: Some(50022u16),
            ansible_user: Some("vagrant".to_string()),
            ansible_ssh_private_key_file: Some("/path/to/private_key".to_string()),
            ansible_ssh_common_args: None,
            ansible_ssh_extra_args: None,
        }
    }

    #[test]
    fn serialize_host_params_to_yaml() -> Result<(), serde_yaml::Error> {
        // Given:
        let host_params = sample_host_params();

        // When:
        let yaml = serde_yaml::to_string(&host_params)?;

        // Then:
        assert_eq!(
            yaml,
            "ansible_host: 127.0.0.1\n\
            ansible_port: 50022\n\
            ansible_user: vagrant\n\
            ansible_ssh_private_key_file: /path/to/private_key\n"
        );
        Ok(())
    }

    #[test]
    fn serialize_inventory_to_yaml() -> Result<(), serde_yaml::Error> {
        // Given:
        let ssh_config = SshConfig {
            host: "default".to_string(),
            fields: BTreeMap::from([
                (Field::HostName, "127.0.0.1".to_string()),
                (Field::User, "vagrant".to_string()),
                (Field::Port, "50022".to_string()),
                (Field::IdentityFile, "/path/to/private_key".to_string()),
                (Field::StrictHostKeyChecking, "no".to_string()),
                (Field::PasswordAuthentication, "no".to_string()),
            ]),
        };
        let ssh_configs = Vec::from([ssh_config]);
        let vars = Some(Vec::from([
            ("become".to_string(), ValueType::Bool(true)),
            (
                "http_port".to_string(),
                ValueType::String("8080".to_string()),
            ),
            ("num_workers".to_string(), ValueType::Int64(4)),
            ("swap_size".to_string(), ValueType::String("3G".to_string())),
        ]));
        let inventory = Inventory::new("local", &ssh_configs, &vars);

        // When:
        let yaml = serde_yaml::to_string(&inventory)?;

        // Then:
        assert_eq!(
            yaml,
            r#"local:
  hosts:
    default:
      ansible_host: 127.0.0.1
      ansible_port: 50022
      ansible_user: vagrant
      ansible_ssh_private_key_file: /path/to/private_key
      ansible_ssh_extra_args: -o PasswordAuthentication=no -o StrictHostKeyChecking=no
  vars:
    become: true
    http_port: '8080'
    num_workers: 4
    swap_size: 3G
"#
        );
        Ok(())
    }
}
