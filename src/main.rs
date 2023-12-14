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

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Opts::parse();
    match cli.clone().operation {
        opts::Operations::Connect(opts) => {
            let config = config::Config::load(opts.clone().config)?;
            let instance_set = InstanceSet::fetch(&cli).await?;
            let filtered_instance_set = instance_set.filter(&opts.search)?;
            let ui = Ui::new(filtered_instance_set)?;
            let instance = ui.run()?;

            /* run ssh */
            let command_generator = cmdgen::CommandGenerator::new(&opts, config, instance)?;
            let mut command = Command::new("sh");
            let ssh_command = command_generator.generate(&mut command)?;

            ssh_command.status()?;
        }
        opts::Operations::List(opts) => {
            let instance_set = InstanceSet::fetch(&cli).await?;
            let filtered_instance_set = instance_set.filter(&opts.search)?;
            TableGenerator::generate(&filtered_instance_set).print();
        }
    }

    Ok(())
}
