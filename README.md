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

## Usage

Pipe any SSH configuration to `s2a`'s standard input, specifying as CLI
arguments:

- the name of this environment (`-e`/`--environment`), and
- the path of the YAML file to generate (`-f`/`--filepath`).

This works with any well-formed SSH configuration, e.g.:

- `cat ~/.ssh/config | s2a`
- `vagrant ssh-config | s2a`

### Example

```console
$ cat <<EOF | s2a -e dev -f dev.yaml
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

### Help

<!-- markdownlint-disable MD013 -->
```console
$ s2a --help
A tool to convert a SSH configuration to an Ansible YAML inventory.

Usage: s2a [OPTIONS]

Options:
  -v, --verbose...                 More output per occurrence
  -q, --quiet...                   Less output per occurrence
      --debug
  -e, --environment <ENVIRONMENT>  Name of the environment to generate [default: local]
  -f, --filepath <FILEPATH>        Path of the Ansible inventory file to generate [default: local.yaml]
  -h, --help                       Print help
  -V, --version                    Print version
```
<!-- markdownlint-enable MD013 -->

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

### Release

```console
export VERSION="X.Y.Z"  # N.B.: no "v" prefix!
git tag -a "${VERSION}" -m "${VERSION}"
git push origin --tags
```

N.B.: in case of release job failure, and a re-release, the tag can be deleted
this way (warning: bad practice to delete tags):

```console
git tag -d "${VERSION}"
git push origin --delete "${VERSION}"
```
