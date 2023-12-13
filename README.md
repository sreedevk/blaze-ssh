# Blaze SSH

## Description
Blaze SSH is a configurable CLI tool that helps you ssh into amazon aws ec2 instances without leaving your terminal.
It is build with Rust and uses the official AWS SDK for Rust.
configuration is done using a `.toml` file (`"~/.config/blaze/config.toml"` by default).

## Motivation
I have been using [aws-ssh](https://github.com/sreedevk/aws-ssh) for a while now. Its a great tool but with all of the dependencies like fzf, jq, aws-cli etc., It has been difficult to come up with an effective way to package the application into an easily installable format. Also as someone that enjoys experimenting with with new shells, its inconvenient to have to switch back to zsh just to be able to use aws-ssh.

This tool attempts to solve the problem by keeping the dependencies at a minimum and using Rust to build a single executable binary.
A few conveniences of aws-ssh are missing, but will be added soon along with some additional features like aws-ecs support.

## Installation
Currently, the only way to install blaze is using cargo:

```bash
$ cargo install blaze-ssh
```

## Usage
```bash
Usage: blaze-ssh <COMMAND>

Commands:
  connect
  list
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## Examples

### Listing Instances

```bash
# Lists all instances that contain the term "staging" in their "Name" tag
$ blaze-ssh list staging
```

### Connecting to an Instance
```bash
# Interactively select an instance to connect to from a list of instances that contain the term "production" in their "Name" tag
$ blaze-ssh connect production
```
