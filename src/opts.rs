use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
pub struct ConnectOptions {
    /// Search String to filter instances by
    pub search: Option<String>,

    /// ssh username
    #[clap(short, long)]
    pub user: Option<String>,

    /// ssh port
    #[clap(short, long)]
    pub port: Option<u16>,

    /// ssh private key
    #[clap(short, long)]
    pub key: Option<PathBuf>,

    #[clap(short, long)]
    pub address_type: Option<String>,

    /// jumphost
    #[clap(short, long)]
    pub jumphost: Option<String>,
}

#[derive(Parser, Debug, Clone)]
pub struct ListOptions {
    pub search: Option<String>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Operations {
    /// connect to an ec2 instances
    #[clap(name = "connect", alias = "c")]
    Connect(ConnectOptions),
    /// list filtered ec2 instances
    #[clap(name = "list", alias = "l")]
    List(ListOptions),
    /// Print SSH Command
    #[clap(name = "print", alias = "p")]
    Print(ConnectOptions),
    /// generate default config (~/.config/blssh/config.toml)
    #[clap(name = "configure", alias = "cfg")]
    Configure,
}

#[derive(Parser, Debug, Clone)]
pub struct Opts {
    /// disable using cached ec2 instances list
    #[clap(long, default_value = "false")]
    pub no_cache: bool,

    /// config
    #[clap(short, long)]
    pub config: Option<PathBuf>,

    /// operation to perform (list / connect)
    #[clap(subcommand)]
    pub operation: Operations,
}
