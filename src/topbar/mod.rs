use glib::Object;
use gtk::glib;

mod imp;

glib::wrapper! {
    pub struct EliteHeaderBar(ObjectSubclass<imp::EliteHeaderBar>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable,
                    gtk::Buildable, gtk::ConstraintTarget;
}

impl EliteHeaderBar {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

impl Default for EliteHeaderBar {
    fn default() -> Self {
        Self::new()
    }
}