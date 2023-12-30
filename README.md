<!-- markdownlint-disable MD013 MD033 MD041 -->
<div align="center">
  <a href="https://crates.io/crates/ssh-to-ansible"><img src="https://img.shields.io/crates/v/ssh-to-ansible.svg" alt="crates.io version"></a>
  <a href="https://github.com/marccarre/ssh-to-ansible/actions"><img src="https://github.com/marccarre/ssh-to-ansible/actions/workflows/ci.yaml/badge.svg" alt="build status" /></a>
  <a href="https://github.com/marccarre/ssh-to-ansible/releases"><img src="https://img.shields.io/github/downloads/marccarre/ssh-to-ansible/total.svg" alt="downloads" /></a>
  <a href="https://coveralls.io/github/marccarre/ssh-to-ansible?branch=main"><img src="https://coveralls.io/repos/github/marccarre/ssh-to-ansible/badge.svg?branch=main" alt="Coverage Status" /></a>
</div>
<!-- markdownlint-enable MD013 MD033 MD041 -->

# ssh-to-ansible

A tool to convert a SSH configuration to an Ansible YAML inventory.

## Installation

```console
brew install marccarre/homebrew-ssh-to-ansible/s2a
```

Or

```console
brew tap marccarre/homebrew-ssh-to-ansible
brew install s2a
```

Or download from the [release](https://github.com/marccarre/ssh-to-ansible/releases)
page and install manually at your convenience.

## Usage

Provide any SSH configuration as an input to `s2a`, either via `stdin` or as an
input file, optionally define the name of the environment (`-e`/`--environment`)
for the Ansible inventory, and optionally provide an output YAML file.

`s2a` works with any well-formed SSH configuration, e.g.:

- `cat ~/.ssh/config | s2a`
- `vagrant ssh-config | s2a`

### Examples

#### Default options

By default, `s2a` defaults the environment to be `local`, reads from `stdin` and
writes to `stdout`:

<!-- markdownlint-disable MD013 -->
```console
$ cat <<EOF | s2a
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

local:
  hosts:
    default:
      ansible_host: 127.0.0.1
      ansible_port: 50022
      ansible_user: vagrant
      ansible_ssh_private_key_file: /Users/me/.vagrant/machines/default/qemu/private_key
      ansible_ssh_extra_args: -o HostKeyAlgorithms=+ssh-rsa -o IdentitiesOnly=yes -o LogLevel=FATAL -o PasswordAuthentication=no -o PubkeyAcceptedKeyTypes=+ssh-rsa -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null
```
<!-- markdownlint-enable MD013 -->

#### Configure the Ansible inventory's environment

<!-- markdownlint-disable MD013 -->
```console
$ cat <<EOF | s2a -e dev
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

dev:
  hosts:
    default:
      ansible_host: 127.0.0.1
      ansible_port: 50022
      ansible_user: vagrant
      ansible_ssh_private_key_file: /Users/me/.vagrant/machines/default/qemu/private_key
      ansible_ssh_extra_args: -o HostKeyAlgorithms=+ssh-rsa -o IdentitiesOnly=yes -o LogLevel=FATAL -o PasswordAuthentication=no -o PubkeyAcceptedKeyTypes=+ssh-rsa -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null
```
<!-- markdownlint-enable MD013 -->

#### Read from input file instead of `stdin`

<!-- markdownlint-disable MD013 -->
```console
$ cat <<EOF > ssh_config
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

$ s2a -i ssh_config

local:
  hosts:
    default:
      ansible_host: 127.0.0.1
      ansible_port: 50022
      ansible_user: vagrant
      ansible_ssh_private_key_file: /Users/me/.vagrant/machines/default/qemu/private_key
      ansible_ssh_extra_args: -o HostKeyAlgorithms=+ssh-rsa -o IdentitiesOnly=yes -o LogLevel=FATAL -o PasswordAuthentication=no -o PubkeyAcceptedKeyTypes=+ssh-rsa -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null
```
<!-- markdownlint-enable MD013 -->

#### Write to output file instead of `stdout`

<!-- markdownlint-disable MD013 -->
```console
$ cat <<EOF | s2a -o local.yaml
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
      ansible_ssh_extra_args: -o HostKeyAlgorithms=+ssh-rsa -o IdentitiesOnly=yes -o LogLevel=FATAL -o PasswordAuthentication=no -o PubkeyAcceptedKeyTypes=+ssh-rsa -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null
```
<!-- markdownlint-enable MD013 -->

### Help

<!-- markdownlint-disable MD013 -->
```console
$ s2a --help
A tool to convert a SSH configuration to an Ansible YAML inventory.

Usage: s2a [OPTIONS]

Options:
  -v, --verbose...
          More output per occurrence
  -q, --quiet...
          Less output per occurrence
  -e, --environment <ENVIRONMENT>
          Name of the environment to generate [default: local]
  -i, --input-filepath <INPUT_FILEPATH>
          Path of the input SSH configuration to parse [default: stdin]
  -o, --output-filepath <OUTPUT_FILEPATH>
          Path of the output Ansible inventory file to generate [default: stdout]
  -h, --help
          Print help
  -V, --version
          Print version
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

### Release

```console
export VERSION="X.Y.Z"  # N.B.: no "v" prefix!
git tag -a "${VERSION}" -m "${VERSION}"
git push origin --tags
cargo login
cargo publish --dry-run
cargo publish
```

Then update the Homebrew Tap at:
<https://github.com/marccarre/homebrew-ssh-to-ansible>

N.B.: in case of release job failure, and a re-release, the tag can be deleted
this way (warning: bad practice to delete tags):

```console
git tag -d "${VERSION}"
git push origin --delete "${VERSION}"
```
