#[macro_use]
extern crate prettytable;

mod instance_details;
mod opts;
mod tablegen;

use anyhow::Result;
use clap::Parser;
use instance_details::InstanceSet;
use opts::Opts;
use tablegen::TableGenerator;

const USE_CACHE: bool = true;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Opts::parse();
    match cli.operation {
        opts::Operations::Connect(opts) => {
            println!("Connecting to {}", opts.search);
        }
        opts::Operations::List(opts) => {
            let instance_set = InstanceSet::fetch(USE_CACHE).await?;
            TableGenerator::generate(&instance_set).print();
        }
    }

    Ok(())
}
