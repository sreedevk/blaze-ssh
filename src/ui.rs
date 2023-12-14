use std::cell::RefCell;
use std::rc::Rc;

use crate::instance_details::{InstanceDetails, InstanceSet};
use anyhow::Result;

use cursive::{
    theme::{BaseColor, Color, Palette, PaletteColor, Theme},
    view::{Margins, Nameable},
    views::{Dialog, ScrollView, SelectView},
    {Cursive, CursiveExt},
};

pub struct Ui {
    pub instance_set: InstanceSet,
}

impl Ui {
    pub fn new(instance_set: InstanceSet) -> Result<Self> {
        Ok(Self { instance_set })
    }

    pub fn theme() -> Result<Theme> {
        let mut palette = Palette::default();
        palette[PaletteColor::Background] = Color::Dark(BaseColor::Black);

        Ok(Theme {
            shadow: true,
            borders: cursive::theme::BorderStyle::Simple,
            palette,
        })
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

        instance_select.set_autojump(true);

        let instance_scrollview = ScrollView::new(instance_select.with_name("instance_select"));
        let instance_select_dialog = Dialog::around(instance_scrollview)
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
        app.set_theme(Self::theme()?);
        app.run();

        let default_instance_result = InstanceDetails::default();
        let selected_instance_result = selected_instance
            .borrow()
            .as_ref()
            .unwrap_or(&default_instance_result)
            .clone();

        Ok(selected_instance_result)
    }
}
