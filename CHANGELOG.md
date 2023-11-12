# Changelog

## [0.3.0](https://github.com/marccarre/ssh-to-ansible/releases/tag/0.3.0) - 2023-11-12

- Added support for `ansible_ssh_extra_args` and `ansible_ssh_common_args` ([#5](https://github.com/marccarre/ssh-to-ansible/pull/5)).
- Removed the `--debug` CLI flag and the logic behind this flag. ([#6](https://github.com/marccarre/ssh-to-ansible/pull/6)).

## [0.2.0](https://github.com/marccarre/ssh-to-ansible/releases/tag/0.2.0) - 2023-11-11

- Added the ability to read SSH configuration from either `stdin` (default) or
  an input file.
- Added the ability to write the Ansible YAML inventor to either `stdout`
  (default) or an output file.
- Improved tests.
- Added code coverage reports as part of CI.

## [0.1.0](https://github.com/marccarre/ssh-to-ansible/releases/tag/0.1.0) - 2023-11-11

- Added basic logic to parse SSH configuration from standard input and serialise
  it as an Ansible YAML inventory.
- Added basic user documentation in [README.md](./README.md).
