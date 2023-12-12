use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
pub struct ConnectOptions {
    /// Search String to filter instances by
    pub search: String,
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
