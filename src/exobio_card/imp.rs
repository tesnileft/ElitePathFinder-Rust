use std::cell::RefCell;
use gdk::gio::spawn_blocking;
use gdk::glib::{clone, IsA};
use gtk::{gio, glib, CompositeTemplate};
use gtk::glib::Binding;
use gtk::glib::subclass::InitializingObject;
use gtk::prelude::{ButtonExt, EditableExt, ObjectExt, SettingsExt, SettingsExtManual};
use gtk::subclass::prelude::*;
use gtk::prelude::*;
use crate::{helpers::get_journals_location};
use crate::helpers::read_all_journals;

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/elite_pathfinder/exobio_card.ui")]
pub struct ExobioCard
{
    #[template_child]
    pub planet_name: TemplateChild<gtk::Label>,
    #[template_child]
    pub planet_info: TemplateChild<gtk::Label>,
    #[template_child]
    pub species_box: TemplateChild<gtk::Box>,
    #[template_child]
    pub total_value: TemplateChild<gtk::Label>,
    pub bindings: RefCell<Vec<Binding>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for ExobioCard {
    const NAME: &'static str = "ExobioCard";
    type Type = super::ExobioCard;
    type ParentType = gtk::Box;
    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }

}

// Trait shared by all GObjects
impl ObjectImpl for ExobioCard {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();
    }
}

// Trait shared by all widgets
impl WidgetImpl for ExobioCard {}

// Trait shared by all buttons
impl BoxImpl for ExobioCard {}