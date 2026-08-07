use glib::Object;
use gtk::glib;
use gtk::prelude::ObjectExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use rusqlite::fallible_iterator::FallibleIterator;
use crate::planet_data_object::PlanetDataObject;

mod imp;

glib::wrapper! {
    pub struct ExobioCard(ObjectSubclass<imp::ExobioCard>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable,
                    gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl ExobioCard {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn bind(&self, planet_data_object: &PlanetDataObject){
        let body_name_label = self.imp().planet_name.get();
        let value_field = self.imp().total_value.get();
        let mut bindings = self.imp().bindings.borrow_mut();

        let body_name_binding = planet_data_object
            .bind_property("body_name", &body_name_label, "label")
            .sync_create()
            .build();
        bindings.push(body_name_binding);
    }
    pub fn unbind(&self) {
        // Unbind all stored bindings
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }


}

impl Default for ExobioCard {
    fn default() -> Self {
        Self::new()
    }
}