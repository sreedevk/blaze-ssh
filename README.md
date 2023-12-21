<h1 align="center">Blaze SSH</h1>
<div align="center">
  <img src="https://github.com/sreedevk/blaze-ssh/assets/36154121/074b4d46-167c-4e66-807f-9cc3c730d1f1" width="200" />
</div>

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

After installation, make sure to create a config file at `~/.config/blaze/config.toml`. See [Configuration](#configuration) for more details.
You may use the `configure` command to generate a default config file.

```bash
$ blssh configure
```

## Usage
```bash
Usage: blssh [OPTIONS] <COMMAND>

Commands:
  connect    connect to an ec2 instances
  list       list filtered ec2 instances
  configure  generate default config (~/.config/blssh/config.toml)
  help       Print this message or the help of the given subcommand(s)

Options:
      --no-cache         disable using cached ec2 instances list
  -c, --config <CONFIG>  config
  -h, --help             Print help

# Connection Opts
connect to an ec2 instances

Usage: blssh connect [OPTIONS] [SEARCH]

Arguments:
  [SEARCH]  Search String to filter instances by

Options:
  -u, --user <USER>                  ssh username
  -p, --port <PORT>                  ssh port
  -k, --key <KEY>                    ssh private key
  -a, --address-type <ADDRESS_TYPE>
  -j, --jumphost <JUMPHOST>          jumphost
  -h, --help                         Print help
```

## Configuration
Currently, blaze-ssh expects to find a config file at `~/.config/blaze/config.toml`. Please make sure to create this file. 

```toml
[config]
private-key = "~/.ssh/id_rsa.pem"
default-user = "ec2-user"
jumphost = ""
port = 22
address-type = "private"
```

## Examples

### Listing Instances

```bash
# Lists all instances that contain the term "staging" in their "Name" tag
$ blssh list staging
```

### Connecting to an Instance
```bash
# Interactively select an instance to connect to from a list of instances that contain the term "production-1" in their "Name" tag
$ blssh connect production-1

# Connecting with a non default (configured in ~/.config/blaze/config.toml) private key
$ blssh connect production-1 --key ~/.ssh/production.pem

# Connecting with a non default (configured in ~/.config/blaze/config.toml) user
$ blssh connect production-1 --user ubuntu

# Connecting with a non default (configured in ~/.config/blaze/config.toml) port
$ blssh connect production-1 --port 2222

# Connecting with a non default jumphost host
$ blssh connect production-1 --jumphost "user@192.168.1.1"

# Connecting with a non default (configured in ~/.config/blaze/config.toml) address type
# Options are "public" & "private"
$ blssh connect production-1 --address-type public

# Connect using a custom config
$ blssh --config ~/custom-config.toml connect production-1

# Disable use of cached instance information (stored in /tmp/blaze_ssh_cache.json)
$ blssh --no-cache connect production-1
```

# Known Issues
1. When navigating using j/k on the connect ui, the list scroll doesn't work. [PR #2]
2. The behavior without a config file is untested.

# Roadmap
1. Fix known Issues
2. Package application for distribution
3. If only one instance is found, connect to it directly without showing the connect ui [PR #1](https://github.com/sreedevk/blaze-ssh/pull/1)
4. Add scp support to copy files to/from instances
