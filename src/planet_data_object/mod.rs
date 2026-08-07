mod imp;

use glib::Object;
use gtk::glib;
use gtk::gio;

glib::wrapper! {
    pub struct PlanetDataObject(ObjectSubclass<imp::PlanetDataObject>);
}

impl PlanetDataObject {
    pub fn new(valuable: bool, body_name: String) -> Self {
        
        Object::builder()
            .property("valuable", valuable)
            .property("body_name", body_name)
            .build()
    }
}
#[derive(Default)]
pub struct PlanetData {
    pub valuable: bool,
    pub body_name: String,
}