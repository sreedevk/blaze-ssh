use std::cell::RefCell;
use std::rc::Rc;

use crate::instance_details::{InstanceDetails, InstanceSet};
use anyhow::Result;
use cursive::view::{Margins, Nameable};
use cursive::views::{Dialog, SelectView};
use cursive::{Cursive, CursiveExt};

pub struct Ui {
    pub instance_set: InstanceSet,
}

impl Ui {
    pub fn new(instance_set: InstanceSet) -> Result<Self> {
        Ok(Self { instance_set })
    }

    pub fn run(&self) -> Result<InstanceDetails> {
        let mut app = Cursive::new();
        let mut instance_select = SelectView::<InstanceDetails>::new();
        let selected_instance: Rc<RefCell<Option<InstanceDetails>>> = Rc::new(RefCell::new(None));
        let selected_instance_clone = selected_instance.clone();

        self.instance_set
            .instances
            .clone()
            .into_iter()
            .for_each(|instance| {
                instance_select.add_item(instance.display_name().unwrap_or_default(), instance);
            });

        let instance_select_dialog = Dialog::around(instance_select.with_name("instance_select"))
            .title("Select an instance")
            .padding(Margins::lrtb(1, 1, 1, 1));

        app.add_layer(instance_select_dialog);

        /* Key Bindings */
        app.add_global_callback('q', |s| s.quit());
        app.add_global_callback('j', |s| {
            s.call_on_name(
                "instance_select",
                |view: &mut SelectView<InstanceDetails>| {
                    view.select_down(1);
                },
            );
        });

        app.add_global_callback('k', |s| {
            s.call_on_name(
                "instance_select",
                |view: &mut SelectView<InstanceDetails>| {
                    view.select_up(1);
                },
            );
        });

        app.call_on_name(
            "instance_select",
            |view: &mut SelectView<InstanceDetails>| {
                view.set_on_submit(move |s, selected| {
                    *selected_instance_clone.borrow_mut() = Some(selected.clone());
                    s.quit();
                });
            },
        );

        /* Ui Assembly */
        app.run();

        let selected_instance_result = selected_instance.borrow().as_ref().unwrap().clone();
        Ok(selected_instance_result)
    }
}
