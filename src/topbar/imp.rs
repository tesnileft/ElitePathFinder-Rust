use gdk::glib::IsA;
use gtk::{glib, CompositeTemplate};
use gtk::glib::subclass::InitializingObject;
use gtk::subclass::prelude::*;

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/elite_pathfinder/topbar.ui")]
pub struct EliteHeaderBar;

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for EliteHeaderBar {
    const NAME: &'static str = "EliteHeaderBar";
    type Type = super::EliteHeaderBar;
    type ParentType = gtk::Box;
    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for EliteHeaderBar {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();
        
    }

}

// Trait shared by all widgets
impl WidgetImpl for EliteHeaderBar {}

// Trait shared by all buttons
impl BoxImpl for EliteHeaderBar {}