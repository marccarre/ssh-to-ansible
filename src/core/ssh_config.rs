use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use std::io::BufRead;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use tracing::warn;

/// Field list all the possible keys for a SSH configuration.
/// See also: http://man.openbsd.org/OpenBSD-current/man5/ssh_config.5
#[derive(Clone, Copy, Debug, Display, EnumIter, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Field {
    Host,
    Match,
    AddKeysToAgent,
    AddressFamily,
    BatchMode,
    BindAddress,
    BindInterface,
    CanonicalDomains,
    CanonicalizeFallbackLocal,
    CanonicalizeFallbackLock,
    CanonicalizeHostname,
    CanonicalizeMaxDots,
    CanonicalizePermittedCNAMEs,
    CASignatureAlgorithms,
    CertificateFile,
    ChannelTimeout,
    CheckHostIP,
    Ciphers,
    ClearAllForwardings,
    Compression,
    ConnectionAttempts,
    ConnectTimeout,
    ControlMaster,
    ControlPath,
    ControlPersist,
    DynamicForward,
    EnableEscapeCommandline,
    EnableSSHKeysign,
    EscapeChar,
    ExitOnForwardFailure,
    FingerprintHash,
    ForkAfterAuthentication,
    ForwardAgent,
    ForwardX11,
    ForwardX11Timeout,
    ForwardX11Trusted,
    GatewayPorts,
    GlobalKnownHostsFile,
    GSSAPIAuthentication,
    GSSAPIDelegateCredentials,
    HashKnownHosts,
    HostbasedAcceptedAlgorithms,
    HostbasedAuthentication,
    HostbasedKeyTypes,
    HostKeyAlgorithms,
    HostKeyAlias,
    HostName,
    IdentitiesOnly,
    IdentityAgent,
    IdentityFile,
    IgnoreUnknown,
    Include,
    IPQoS,
    KbdInteractiveAuthentication,
    KbdInteractiveDevices,
    KexAlgorithms,
    KnownHostsCommand,
    LocalCommand,
    LocalForward,
    LogLevel,
    LogVerbose,
    Mac,
    MACs,
    NoHostAuthenticationForLocalhost,
    NumberOfPasswordPrompts,
    ObscureKeystrokeTiming,
    PasswordAuthentication,
    PermitLocalCommand,
    PermitRemoteOpen,
    PKCS11Provider,
    Port,
    PreferredAuthentications,
    ProxyCommand,
    ProxyJump,
    ProxyUseFdpass,
    PubkeyAcceptedAlgorithms,
    PubkeyAcceptedKeyTypes,
    PubkeyAuthentication,
    RekeyLimit,
    RemoteCommand,
    RemoteForward,
    RequestTTY,
    RequiredRSASize,
    RevokedHostKeys,
    SecurityKeyProvider,
    SendEnv,
    ServerAliveCountMax,
    ServerAliveInterval,
    SessionType,
    SetEnv,
    StdinNull,
    StreamLocalBindMask,
    StreamLocalBindUnlink,
    StrictHostKeyChecking,
    SyslogFacility,
    Tag,
    TCPKeepAlive,
    Tunnel,
    TunnelDevice,
    UpdateHostKeys,
    UseKeychain,
    User,
    UserKnownHostsFile,
    VerifyHostKeyDNS,
    VisualHostKey,
    XAuthLocation,
}

lazy_static! {
    #[derive(Debug)]
    static ref FIELDS: HashMap<String, Field> = Field::iter()
        .map(|field| (field.to_string().to_lowercase(), field)) // SSH config keys are case-insensitive.
        .collect::<HashMap<String, Field>>();
}

#[derive(Clone, Debug, PartialEq)]
pub struct SshConfig {
    pub host: String,
    pub fields: BTreeMap<Field, String>,
}

impl SshConfig {
    pub fn new() -> SshConfig {
        SshConfig {
            host: "*".to_string(),
            fields: BTreeMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }
}

impl SshConfig {
    pub fn parse(reader: &mut impl BufRead) -> Result<Vec<SshConfig>, std::io::Error> {
        let mut ssh_configs = Vec::new(); // There can me more than one SSH config in a SSH config file.
        let mut ssh_config = SshConfig::new();

        let mut line = String::new(); // Reuse the same memory for each line to reduce allocations.
        loop {
            line.clear();
            if reader.read_line(&mut line)? == 0 {
                if !ssh_config.is_empty() {
                    ssh_configs.push(ssh_config);
                }
                break;
            }
            let line = line.trim(); // Remove leading and trailing whitespaces.
            if line.is_empty() {
                continue; // Skip empty lines.
            }
            if line.starts_with('#') {
                continue; // Skip comments.
            }
            if let Some((key, value)) = line.split_once(' ') {
                let case_insensitive_key = key.to_lowercase(); // SSH config keys are case-insensitive.
                let value = value.trim();
                if let Some(&field) = FIELDS.get(&case_insensitive_key) {
                    // Known SSH config field:
                    if field == Field::Host {
                        if !ssh_config.is_empty() {
                            // Assume the beginning of a new SSH config, and add the SSH config being processed so far to our list of SSH configs:
                            ssh_configs.push(ssh_config);
                            // And re-initialise the current SSH config:
                            ssh_config = SshConfig::new();
                        }
                        ssh_config.host = value.to_owned();
                    } else if let Some(old_value) =
                        ssh_config.fields.insert(field, value.to_owned())
                    {
                        warn!(
                            key,
                            old_value,
                            new_value = value,
                            "Overwrote previous value in SSH config",
                        );
                    }
                } else {
                    warn!(line, "Invalid SSH config: unknown field: {}", key);
                }
            } else {
                warn!(line, "Invalid SSH config: line is not well-formed");
            }
        }
        Ok(ssh_configs)
    }
}

#[cfg(test)]
mod tests {
    use super::{Field, SshConfig};
    use crate::common::testing::utilities::SAMPLE_SSH_CONFIG;
    use std::collections::BTreeMap;

    #[test]
    fn parse_ssh_config() -> Result<(), std::io::Error> {
        // Given:
        let mut input = SAMPLE_SSH_CONFIG.as_bytes();

        // When:
        let ssh_configs = SshConfig::parse(&mut input)?;

        // Then:
        assert_eq!(1, ssh_configs.len());
        assert_eq!(
            ssh_configs[0],
            SshConfig {
                host: "default".to_string(),
                fields: BTreeMap::from([
                    (Field::HostName, "127.0.0.1".to_string()),
                    (Field::User, "vagrant".to_string()),
                    (Field::Port, "50022".to_string()),
                    (Field::UserKnownHostsFile, "/dev/null".to_string()),
                    (Field::StrictHostKeyChecking, "no".to_string()),
                    (Field::PasswordAuthentication, "no".to_string()),
                    (Field::IdentityFile, "/path/to/private_key".to_string()),
                    (Field::IdentitiesOnly, "yes".to_string()),
                    (Field::LogLevel, "FATAL".to_string()),
                    (Field::PubkeyAcceptedKeyTypes, "+ssh-rsa".to_string()),
                    (Field::HostKeyAlgorithms, "+ssh-rsa".to_string()),
                ]),
            }
        );
        Ok(())
    }

    #[test]
    fn parse_empty_string() -> Result<(), std::io::Error> {
        // Given:
        let mut input = "".as_bytes();

        // When:
        let ssh_configs = SshConfig::parse(&mut input)?;

        // Then:
        assert_eq!(0, ssh_configs.len());
        Ok(())
    }

    #[test]
    fn parse_ssh_config_with_empty_lines() -> Result<(), std::io::Error> {
        // Given:
        let mut input = r#"Host default

  HostName 127.0.0.1

"#
        .as_bytes();

        // When:
        let ssh_configs = SshConfig::parse(&mut input)?;

        // Then:
        assert_eq!(1, ssh_configs.len());
        assert_eq!(
            ssh_configs[0],
            SshConfig {
                host: "default".to_string(),
                fields: BTreeMap::from([(Field::HostName, "127.0.0.1".to_string()),]),
            }
        );
        Ok(())
    }

    #[test]
    fn parse_ssh_config_with_comments() -> Result<(), std::io::Error> {
        // Given:
        let mut input = r#"Host default
  # The following line is the hostname:
  HostName 127.0.0.1
  # This is the end of the SSH configuration."#
            .as_bytes();

        // When:
        let ssh_configs = SshConfig::parse(&mut input)?;

        // Then:
        assert_eq!(1, ssh_configs.len());
        assert_eq!(
            ssh_configs[0],
            SshConfig {
                host: "default".to_string(),
                fields: BTreeMap::from([(Field::HostName, "127.0.0.1".to_string()),]),
            }
        );
        Ok(())
    }

    #[test]
    fn parse_ssh_config_with_lowercased_keys() -> Result<(), std::io::Error> {
        // Given:
        let mut input = "host default\nhostname 127.0.0.1".as_bytes();

        // When:
        let ssh_configs = SshConfig::parse(&mut input)?;

        // Then:
        assert_eq!(1, ssh_configs.len());
        assert_eq!(
            ssh_configs[0],
            SshConfig {
                host: "default".to_string(),
                fields: BTreeMap::from([(Field::HostName, "127.0.0.1".to_string()),]),
            }
        );
        Ok(())
    }

    #[test]
    fn parse_ssh_config_with_duplicate_field_keeps_last_value() -> Result<(), std::io::Error> {
        // Given:
        let mut input = r#"Host default
  HostName 127.0.0.1
  HostName 127.0.0.2"#
            .as_bytes();

        // When:
        let ssh_configs = SshConfig::parse(&mut input)?;

        // Then:
        assert_eq!(1, ssh_configs.len());
        assert_eq!(
            ssh_configs[0],
            SshConfig {
                host: "default".to_string(),
                fields: BTreeMap::from([(Field::HostName, "127.0.0.2".to_string()),]),
            }
        );
        Ok(())
    }

    #[test]
    fn parse_ssh_config_with_unknown_field() -> Result<(), std::io::Error> {
        // Given:
        let mut input = r#"Host default
  Unknown foobar
  HostName 127.0.0.1"#
            .as_bytes();

        // When:
        let ssh_configs = SshConfig::parse(&mut input)?;

        // Then:
        assert_eq!(1, ssh_configs.len());
        assert_eq!(
            ssh_configs[0],
            SshConfig {
                host: "default".to_string(),
                fields: BTreeMap::from([(Field::HostName, "127.0.0.1".to_string()),]),
            }
        );
        Ok(())
    }

    #[test]
    fn parse_ssh_config_with_non_well_formed_line() -> Result<(), std::io::Error> {
        // Given:
        let mut input = r#"Host default
  invalid-line
  HostName 127.0.0.1"#
            .as_bytes();

        // When:
        let ssh_configs = SshConfig::parse(&mut input)?;

        // Then:
        assert_eq!(1, ssh_configs.len());
        assert_eq!(
            ssh_configs[0],
            SshConfig {
                host: "default".to_string(),
                fields: BTreeMap::from([(Field::HostName, "127.0.0.1".to_string()),]),
            }
        );
        Ok(())
    }

    #[test]
    fn parse_two_ssh_configs() -> Result<(), std::io::Error> {
        // Given:
        let mut input = r#"Host host1
  HostName 192.168.0.1
Host host2
  HostName 192.168.0.2
"#
        .as_bytes();

        // When:
        let ssh_configs = SshConfig::parse(&mut input)?;

        // Then:
        assert_eq!(2, ssh_configs.len());
        assert_eq!(
            ssh_configs[0],
            SshConfig {
                host: "host1".to_string(),
                fields: BTreeMap::from([(Field::HostName, "192.168.0.1".to_string()),]),
            }
        );
        assert_eq!(
            ssh_configs[1],
            SshConfig {
                host: "host2".to_string(),
                fields: BTreeMap::from([(Field::HostName, "192.168.0.2".to_string()),]),
            }
        );
        Ok(())
    }
}
