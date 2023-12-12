use crate::instance_details::InstanceDetails;
use prettytable::{Cell, Row, Table};


pub struct TableGenerator {
    table: Table,
}

impl TableGenerator {
    pub fn generate(instances: &[InstanceDetails]) -> Self {
        let mut table = Table::new();
        table.add_row(row!["Name", "Public IP", "Private IP", "Instance ID"]);
        for instance in instances {
            table.add_row(Row::new(vec![
                Cell::new(&instance.instance_name.clone().unwrap_or("".to_string())),
                Cell::new(&instance.public_ip.clone().unwrap_or("".to_string())),
                Cell::new(&instance.private_ip.clone().unwrap_or("".to_string())),
                Cell::new(&instance.instance_id.clone().unwrap_or("".to_string())),
            ]));
        }
        TableGenerator { table }
    }

    pub fn print(&self) {
        self.table.printstd();
    }
}
