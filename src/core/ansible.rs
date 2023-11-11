use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Inventory {
    #[serde(flatten)]
    groups: BTreeMap<String, Group>,
}

impl Inventory {
    pub fn new(name: &str, hosts: Hosts) -> Inventory {
        Inventory {
            groups: BTreeMap::from([(name.to_owned(), Group::new(hosts))]),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Group {
    hosts: Hosts,
}

impl Group {
    fn new(hosts: Hosts) -> Group {
        Group { hosts }
    }
}

pub type Hosts = BTreeMap<String, HostParams>;

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
    pub ansible_ssh_private_key_file: Option<PathBuf>,

    /// This setting is always appended to the default command line for sftp, scp, and ssh. Useful to configure a ProxyCommand for a certain host (or group).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ansible_ssh_common_args: Option<String>,

    /// This setting is always appended to the default ssh command line.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ansible_ssh_extra_args: Option<String>,
}

impl HostParams {
    pub fn new(params: &ssh2_config::HostParams) -> HostParams {
        HostParams {
            ansible_host: params.host_name.clone(),
            ansible_port: params.port,
            ansible_user: params.user.clone(),
            ansible_ssh_private_key_file: params.identity_file.clone().map(|paths| {
                paths
                    .first()
                    .cloned()
                    .expect("option of IdentityFile paths to contain at least 1 path")
            }), // Only support 1 SSH key.
            ansible_ssh_common_args: None, // Not supported for now.
            ansible_ssh_extra_args: None,  // Not supported for now.
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{HostParams, Hosts, Inventory};
    use serde_yaml;
    use std::collections::BTreeMap;

    fn sample_host_params() -> HostParams {
        HostParams {
            ansible_host: Some("127.0.0.1".to_string()),
            ansible_port: Some(50022u16),
            ansible_user: Some("vagrant".to_string()),
            ansible_ssh_private_key_file: None,
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
            ansible_user: vagrant\n"
        );
        Ok(())
    }

    #[test]
    fn serialize_inventory_to_yaml() -> Result<(), serde_yaml::Error> {
        // Given:
        let hosts: Hosts = BTreeMap::from([("default".to_string(), sample_host_params())]);
        let inventory = Inventory::new("local", hosts);

        // When:
        let yaml = serde_yaml::to_string(&inventory)?;

        // Then:
        assert_eq!(
            yaml,
            r###"local:
  hosts:
    default:
      ansible_host: 127.0.0.1
      ansible_port: 50022
      ansible_user: vagrant
"###
        );
        Ok(())
    }
}
