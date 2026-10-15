use glib::Object;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use crate::planet_data_object::{format_credits, PlanetDataObject};

mod imp;

glib::wrapper! {
    pub struct ExobioCard(ObjectSubclass<imp::ExobioCard>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable,
                    gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

// Grid columns
const COL_SCANNED: i32 = 0;
const COL_NAME: i32 = 1;
const COL_COLOURS: i32 = 2;
const COL_VALUE: i32 = 3;

impl ExobioCard {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn bind(&self, planet_data_object: &PlanetDataObject){
        let imp = self.imp();
        {
            let mut bindings = imp.bindings.borrow_mut();
            for (property, label) in [
                ("body-name", imp.planet_name.get()),
                ("species-text", imp.planet_info.get()),
                ("total-value", imp.total_value.get()),
            ] {
                bindings.push(
                    planet_data_object
                        .bind_property(property, &label, "label")
                        .sync_create()
                        .build(),
                );
            }
        }
        self.build_species_grid(planet_data_object);
    }

    pub fn unbind(&self) {
        // Unbind all stored bindings
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
        // Drop the species rows (and the handlers holding the data object)
        let species_box = self.imp().species_box.get();
        while let Some(child) = species_box.first_child() {
            species_box.remove(&child);
        }
    }

    fn build_species_grid(&self, data: &PlanetDataObject) {
        let grid = gtk::Grid::builder().column_spacing(12).row_spacing(4).build();

        for (index, row) in data.rows().into_iter().enumerate() {
            let r = index as i32;

            let scanned = gtk::CheckButton::builder()
                .active(row.confirmed)
                .tooltip_text("Scanned")
                .build();
            let name = gtk::Label::builder()
                .label(&row.name)
                .xalign(0.0)
                .hexpand(true)
                .build();
            let colours = gtk::Box::builder().spacing(3).valign(gtk::Align::Center).build();
            for variant in &row.variants {
                let swatch = gtk::Box::builder()
                    .tooltip_text(variant)
                    .css_classes(["swatch", &format!("variant-{}", variant.to_lowercase())])
                    .build();
                colours.append(&swatch);
            }
            let value = gtk::Label::builder()
                .label(format!("¢{}", format_credits(row.value)))
                .xalign(1.0)
                .width_chars(14)
                .css_classes(["numeric"])
                .build();

            grid.attach(&scanned, COL_SCANNED, r, 1, 1);
            grid.attach(&name, COL_NAME, r, 1, 1);
            grid.attach(&colours, COL_COLOURS, r, 1, 1);
            grid.attach(&value, COL_VALUE, r, 1, 1);

            // everything except the grey-out button dims, so it stays clickable to undo
            let dimmable: Vec<gtk::Widget> = vec![
                scanned.clone().upcast(),
                name.upcast(),
                colours.upcast(),
                value.upcast(),
            ];
            set_dimmed(&dimmable, row.absent);

            scanned.connect_toggled({
                let data = data.clone();
                move |button| data.set_row_scanned(index, button.is_active())
            });
        }
        self.imp().species_box.append(&grid);
    }
}

fn set_dimmed(widgets: &[gtk::Widget], dimmed: bool) {
    for widget in widgets {
        if dimmed {
            widget.add_css_class("dimmed");
        } else {
            widget.remove_css_class("dimmed");
        }
    }
}

impl Default for ExobioCard {
    fn default() -> Self {
        Self::new()
    }
}