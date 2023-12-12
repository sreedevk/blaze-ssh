#[macro_use]
extern crate prettytable;

mod instance_details;
mod opts;
mod tablegen;
mod config;

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
            let instance_set = InstanceSet::fetch(USE_CACHE).await?;
            let filtered_instance_set = instance_set.filter(&opts.search)?;
            println!("Connecting to {}", opts.search);
            dbg!(config::Config::load(None)?);
        }
        opts::Operations::List(opts) => {
            let instance_set = InstanceSet::fetch(USE_CACHE).await?;
            let filtered_instance_set = instance_set.filter(&opts.search)?;
            TableGenerator::generate(&filtered_instance_set).print();
        }
    }


    Ok(())
}
