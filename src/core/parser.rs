use crate::common::error::AppError;
use crate::core::ansible::{HostParams, Hosts};
use ssh2_config::{Host, ParseRule, SshConfig};
use std::io::BufRead;

pub fn parse(reader: &mut impl BufRead) -> Result<Hosts, AppError> {
    let config = SshConfig::default().parse(reader, ParseRule::STRICT)?;
    let ssh_hosts: Vec<&Host> = ssh_hosts_from(&config);
    let hosts = ssh_hosts
        .into_iter()
        .map(|host| (host_nickname(&host.pattern), HostParams::new(&host.params)))
        .collect::<Hosts>();
    Ok(hosts)
}

/// Get actual hosts from the provided SSH config.
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
    use super::parse;
    use crate::common::error::AppError;
    use crate::core::ansible::HostParams;
    use std::path::PathBuf;

    #[test]
    fn parse_vagrant_ssh_config() -> Result<(), AppError> {
        let ssh_config = r###"Host default
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
  HostKeyAlgorithms +ssh-rsa"###;
        let mut buf_read = ssh_config.as_bytes();
        let hosts = parse(&mut buf_read)?;
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
                ansible_ssh_private_key_file: Some(PathBuf::from(
                    "/Users/me/.vagrant/machines/default/qemu/private_key"
                )),
                ansible_ssh_common_args: None, // Not supported for now.
                ansible_ssh_extra_args: None,  // Not supported for now.
            }
        );
        Ok(())
    }
}
