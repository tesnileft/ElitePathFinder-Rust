use glib::Object;
use gtk::glib;
use gtk::prelude::WidgetExt;
use gtk::subclass::prelude::ObjectSubclassIsExt;

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
    pub fn set_commandername(&self, name: &str){
        let imp = self.imp();
        imp.commandername_label.set_text(name);
    }
    pub fn set_systemname(&self, name: &str){
        let imp = self.imp();
        imp.systemname_label.set_text(&name);
    }
    pub fn set_locationname(&self, name: &str){
        let imp = self.imp();
        imp.locationname_label.set_text(name);
    }
    pub fn set_credits(&self, credits: u64){
        let imp = self.imp();
        imp.credits_label.set_text(&format!("Credits: {credits}"));
    }
    pub fn set_arx(&self, arx: u64){
        let imp = self.imp();
        imp.arx_label.set_text(&arx.to_string());
    }
    pub fn set_jumping(&self, jumping: bool, destination: Option<&str>){
        let imp = self.imp();
        if jumping {
            let status = match destination {
                Some(dest) => format!("Jumping to {dest}…"),
                None => "Charging FSD…".to_string(),
            };
            imp.jump_status_label.set_text(&status);
            imp.jump_spinner.set_spinning(true);
            imp.jump_overlay_box.set_visible(true);
        } else {
            imp.jump_spinner.set_spinning(false);
            imp.jump_overlay_box.set_visible(false);
        }
    }
}

impl Default for EliteHeaderBar {
    fn default() -> Self {
        Self::new()
    }
}