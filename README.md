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
cat <<EOF | ./target/debug/s2a
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
```

##### Custom options

```console
cat <<EOF | ./target/debug/s2a -e dev -f dev.yaml
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
```
