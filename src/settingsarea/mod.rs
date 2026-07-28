use glib::Object;
use gtk::glib;
use gtk::subclass::prelude::ObjectSubclassIsExt;

mod imp;

glib::wrapper! {
    pub struct SettingsArea(ObjectSubclass<imp::SettingsArea>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable,
                    gtk::Buildable, gtk::ConstraintTarget;
}

impl SettingsArea {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for SettingsArea {
    fn default() -> Self {
        Self::new()
    }
}