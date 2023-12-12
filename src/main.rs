#[macro_use] extern crate prettytable;

mod opts;
mod tablegen;
mod instance_details;
mod cache;

use anyhow::Result;
use opts::Opts;
use clap::Parser;
use instance_details::InstanceDetails;
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

    let aws_config = aws_config::load_from_env().await;
    let aws_client = aws_sdk_ec2::Client::new(&aws_config);
    let response = aws_client.describe_instances().send().await?;
    let instances = response
        .reservations()
        .into_iter()
        .flat_map(|reservation| reservation.instances())
        .map(InstanceDetails::from_instance)
        .flat_map(Result::ok)
        .collect::<Vec<_>>();

    TableGenerator::generate(&instances).print();

    Ok(())
}
