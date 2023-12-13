#[macro_use]
extern crate prettytable;

mod cmdgen;
mod config;
mod instance_details;
mod opts;
mod tablegen;
mod ui;

use std::process::Command;

use anyhow::Result;
use clap::Parser;
use instance_details::InstanceSet;
use opts::Opts;
use tablegen::TableGenerator;
use ui::Ui;

// TODO: Make this configurable
const USE_CACHE: bool = true;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Opts::parse();
    match cli.operation {
        opts::Operations::Connect(opts) => {
            let config = config::Config::load(opts.config.clone())?;
            let instance_set = InstanceSet::fetch(USE_CACHE).await?;
            let filtered_instance_set = instance_set.filter(&opts.search)?;
            let ui = Ui::new(filtered_instance_set)?;
            let instance = ui.run()?;
            let cmd = cmdgen::CommandGenerator::new(&opts, config, instance)?;
            let ssh_command = cmd.generate()?;

            dbg!(ssh_command);
        }
        opts::Operations::List(opts) => {
            let instance_set = InstanceSet::fetch(USE_CACHE).await?;
            let filtered_instance_set = instance_set.filter(&opts.search)?;
            TableGenerator::generate(&filtered_instance_set).print();
        }
    }

    Ok(())
}
