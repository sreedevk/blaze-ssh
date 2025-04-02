#[macro_use]
extern crate prettytable;

mod cmdgen;
mod config;
mod instance_details;
mod opts;
mod tablegen;
mod ui;

use std::process::Command;

use anyhow::{anyhow, Result};
use clap::Parser;
use instance_details::InstanceSet;
use opts::Opts;
use tablegen::TableGenerator;
use ui::Ui;

fn gencmd(
    opts: opts::ConnectOptions,
    cli: Opts,
    instance_set: InstanceSet,
    cmd: &mut Command,
) -> Result<()> {
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
        return Err(anyhow!("No Instance Found"));
    }

    /* run ssh */
    let config = config::Config::load(cli.clone().config)?;
    cmdgen::CommandGenerator::new(&opts, config, instance)?.generate(cmd)?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Opts::parse();
    let instance_set = InstanceSet::fetch(&cli).await?;
    let operation = cli.operation.clone();
    match operation.clone() {
        opts::Operations::Connect(opts) => {
            let mut command = Command::new("sh");
            if gencmd(opts, cli, instance_set, &mut command).is_ok() {
                command.status()?;
            }
        }
        opts::Operations::Print(opts) => {
            let mut command = Command::new("sh");
            if gencmd(opts, cli, instance_set, &mut command).is_ok() {
                println!(
                    "{}",
                    command
                        .get_args()
                        .last()
                        .unwrap_or_default()
                        .to_str()
                        .unwrap_or_default()
                );
            }
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
