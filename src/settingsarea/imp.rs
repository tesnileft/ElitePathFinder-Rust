use gdk::gio::spawn_blocking;
use gdk::glib::{clone, IsA};
use gtk::{gio, glib, CompositeTemplate};
use gtk::glib::subclass::InitializingObject;
use gtk::prelude::{ButtonExt, EditableExt, ObjectExt, SettingsExt, SettingsExtManual};
use gtk::subclass::prelude::*;
use gtk::prelude::*;
use crate::{get_logfilelocation, read_all_journals};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/elite_pathfinder/settingsarea.ui")]
pub struct SettingsArea
{
    #[template_child]
    pub journal_path_entry: TemplateChild<gtk::Entry>,
    #[template_child]
    pub set_path_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub readall_journals_button: TemplateChild<gtk::Button>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for SettingsArea {
    const NAME: &'static str = "SettingsArea";
    type Type = super::SettingsArea;
    type ParentType = gtk::Box;
    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }

}

// Trait shared by all GObjects
impl ObjectImpl for SettingsArea {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();
        let settings = gio::Settings::new("tesnileft.ElitePathFinder_rs");
        let journalpath = settings.get::<String>("elite-journal-logs-path");
        self.journal_path_entry.set_text(&journalpath);
        let entry = self.journal_path_entry.clone();

        let button = self.set_path_button.get();
        button.connect_clicked(move |button| {
            let new_journal_path = entry.text();
            settings.set_string("elite-journal-logs-path", &new_journal_path).expect("Unable to set new journal path in settings");
        });
        let readallbutton = self.readall_journals_button.get();
        readallbutton.connect_clicked(move |_| {
            spawn_blocking(move || {    // Blocking because it will be doing IO
                read_all_journals();
            });
        });

    }
}

// Trait shared by all widgets
impl WidgetImpl for SettingsArea {}

// Trait shared by all buttons
impl BoxImpl for SettingsArea {}