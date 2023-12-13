use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
pub struct ConnectOptions {
    /// Search String to filter instances by
    pub search: String,

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

    /// usecache
    #[clap(long, default_value = "true")]
    pub use_cache: bool,

    /// jumphost
    #[clap(short, long)]
    pub jumphost: Option<String>,

    /// config
    #[clap(short, long)]
    pub config: Option<PathBuf>,
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
