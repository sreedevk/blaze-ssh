#[macro_use] extern crate prettytable;

mod opts;
mod tablegen;
mod instance_details;

use anyhow::Result;
use opts::Opts;
use clap::Parser;
use instance_details::InstanceSet;
use tablegen::TableGenerator;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Opts::parse();
    match cli.operation {
        opts::Operations::Connect(opts) => {
            println!("Connecting to {}", opts.search);
        }
        opts::Operations::List(opts) => {
            println!("Listing instances with search string {}", opts.search);
        }
    }

    TableGenerator::generate(&instance_set).print();

    Ok(())
}
