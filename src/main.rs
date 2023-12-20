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
    let instance_set = InstanceSet::fetch(&cli).await?;
    match cli.clone().operation {
        opts::Operations::Connect(opts) => {
            let filtered_instance_set = instance_set.filter(&opts.search)?;
            let instance = match filtered_instance_set.is_non_selectable() {
                true => filtered_instance_set.instances.first().unwrap().clone(),
                false => {
                    let mut ui = Ui::new(
                        filtered_instance_set,
                        config::Config::read_raw(cli.clone().config)?,
                    )?;

                    ui.run()?
                }
            };

            if instance.is_empty() {
                eprintln!("No instance found");
                return Ok(());
            }

            /* run ssh */
            let config = config::Config::load(cli.clone().config)?;
            let command_generator = cmdgen::CommandGenerator::new(&opts, config, instance)?;
            let mut command = Command::new("sh");
            let ssh_command = command_generator.generate(&mut command)?;

            ssh_command.status()?;
        }
        opts::Operations::List(opts) => {
            let filtered_instance_set = instance_set.filter(&opts.search)?;
            TableGenerator::generate(&filtered_instance_set).print();
        }
        opts::Operations::Configure => {
            config::Config::write_default_config()?;
        }
    }

    Ok(())
}
