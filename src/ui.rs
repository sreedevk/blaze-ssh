use crate::instance_details::{InstanceDetails, InstanceSet};
use anyhow::Result;
use std::collections::HashMap;
use cursive::views::{TextView, SelectView};
use cursive::{CursiveExt, Cursive};

pub struct Ui {
    pub instance_set: InstanceSet,
}

impl Ui {
    pub fn new(instance_set: InstanceSet) -> Result<Self> {
        Ok(Self { instance_set })
    }

    pub fn run(&self) -> Result<()> {
        let mut display_group: HashMap<String, InstanceDetails> = HashMap::new();
        self.instance_set
            .instances
            .clone()
            .into_iter()
            .for_each(|instance| {
                display_group.insert(instance.display_name().unwrap_or_default(), instance);
            });
        
        let mut app = Cursive::new();
        let mut instance_select = SelectView::<InstanceDetails>::new();

        self.instance_set.instances.clone().into_iter().for_each(|instance| {
            instance_select.add_item(instance.display_name().unwrap_or_default(), instance);
        });

        app.add_global_callback('q', |s| s.quit());
        app.add_layer(instance_select);
        app.run();

        Ok(())
    }
}
