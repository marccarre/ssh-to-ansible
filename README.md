<!-- markdownlint-disable MD013 MD033 MD041 -->
<div>
  <a href="https://github.com/marccarre/ssh-to-ansible/actions">
    <img src="https://github.com/marccarre/ssh-to-ansible/actions/workflows/ci.yaml/badge.svg" alt="build status">
  </a>
  <a href="https://github.com/marccarre/ssh-to-ansible/releases">
    <img src="https://img.shields.io/github/downloads/marccarre/ssh-to-ansible/total.svg" alt="downloads">
  </a>
</div>
<!-- markdownlint-enable MD013 MD033 MD041 -->

# ssh-to-ansible

A tool to convert a SSH configuration to an Ansible YAML inventory.

## Development

### Setup

```console
brew install just
just setup
```

### Build

```console
cargo build
```

### Lint

```console
just lint
```

### Test

#### Unit tests

```console
cargo test
```

#### Coverage

```console
just cover
```

#### End-to-end tests

##### Default options

```console
$ cat <<EOF | ./target/debug/s2a
Host default
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
  HostKeyAlgorithms +ssh-rsa
EOF

$ cat local.yaml
local:
  hosts:
    default:
      ansible_host: 127.0.0.1
      ansible_port: 50022
      ansible_user: vagrant
      ansible_ssh_private_key_file: /Users/me/.vagrant/machines/default/qemu/private_key
```

##### Custom options

```console
$ cat <<EOF | ./target/debug/s2a -e dev -f dev.yaml
Host default
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
  HostKeyAlgorithms +ssh-rsa
EOF

$ cat dev.yaml
dev:
  hosts:
    default:
      ansible_host: 127.0.0.1
      ansible_port: 50022
      ansible_user: vagrant
      ansible_ssh_private_key_file: /Users/me/.vagrant/machines/default/qemu/private_key
```
