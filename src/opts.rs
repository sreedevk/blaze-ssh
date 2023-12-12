use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
pub struct ConnectOptions {
    /// Search String to filter instances by
    pub search: String,

    /// ssh username
    #[clap(short, long, default_value = "ec2-user")]
    pub user: String,

    /// ssh port
    #[clap(short, long, default_value = "22")]
    pub port: u16,

    /// ssh private key
    #[clap(short, long, default_value = "~/.ssh/id_rsa")]
    pub key: PathBuf,

    /// usecache
    #[clap(long, default_value = "true")]
    pub use_cache: bool,

    /// jumphost
    #[clap(short, long)]
    pub jumphost: Option<String>,

    /// config
    #[clap(short, long)]
    pub config: Option<String>,
}

#[derive(Parser, Debug, Clone)]
pub struct ListOptions {
    pub search: String,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Operations {
    Connect(ConnectOptions),
    List(ListOptions),
}

#[derive(Parser, Debug)]
pub struct Opts {
    #[clap(subcommand)]
    pub operation: Operations,
}
