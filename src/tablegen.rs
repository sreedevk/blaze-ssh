use crate::instance_details::InstanceSet;
use prettytable::{format, Cell, Row, Table};

pub struct TableGenerator {
    table: Table,
}

impl TableGenerator {
    pub fn generate(instance_set: &InstanceSet) -> Self {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

        /* Header Row */
        table.set_titles(row!["Name", "Public IP", "Private IP", "Instance ID"]);

        /* Data */
        instance_set
            .instances
            .clone()
            .into_iter()
            .for_each(|instance| {
                let instance_clone = instance.clone();
                table.add_row(Row::new(vec![
                    Cell::new(&instance_clone.instance_name.unwrap_or("".to_string())),
                    Cell::new(&instance_clone.public_ip.unwrap_or("".to_string())),
                    Cell::new(&instance_clone.private_ip.unwrap_or("".to_string())),
                    Cell::new(&instance_clone.instance_id.unwrap_or("".to_string())),
                ]));
            });

        TableGenerator { table }
    }

    pub fn print(&self) {
        self.table.printstd();
    }
}
