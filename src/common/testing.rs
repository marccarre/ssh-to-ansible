#[cfg(test)]
pub mod utilities {
    use std::io::{Read, Write};
    use std::path::PathBuf;
    use std::{fs::File, io::BufReader};
    use tempfile::{tempdir, TempDir};

    pub const SAMPLE_SSH_CONFIG: &str = r###"Host default
  HostName 127.0.0.1
  User vagrant
  Port 50022
  UserKnownHostsFile /dev/null
  StrictHostKeyChecking no
  PasswordAuthentication no
  IdentityFile /path/to/private_key
  IdentitiesOnly yes
  LogLevel FATAL
  PubkeyAcceptedKeyTypes +ssh-rsa
  HostKeyAlgorithms +ssh-rsa"###;

    pub fn sample_ansible_inventory(environment: &str) -> String {
        format!(
            r###"{environment}:
  hosts:
    default:
      ansible_host: 127.0.0.1
      ansible_port: 50022
      ansible_user: vagrant
      ansible_ssh_private_key_file: /path/to/private_key
"###
        )
    }

    pub fn temp_file(filename: &str, content: &str) -> Result<(TempDir, PathBuf), std::io::Error> {
        let (dir, input_filepath) = temp_filepath(filename)?;
        let mut input_file = File::create(&input_filepath)?;
        input_file.write_all(content.as_bytes())?;
        input_file.flush()?;
        Ok((dir, input_filepath))
    }

    pub fn temp_filepath(filename: &str) -> Result<(TempDir, PathBuf), std::io::Error> {
        let dir = tempdir()?;
        let filepath = dir.path().join(filename);
        Ok((dir, filepath))
    }

    pub fn read_file(filepath: &PathBuf) -> Result<String, std::io::Error> {
        let file = File::open(filepath)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        Ok(contents)
    }
}
