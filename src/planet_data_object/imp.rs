use std::cell::RefCell;
use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use crate::elite_events::enums::AtmosphereType;
use super::PlanetData;

// Object holding the state
#[derive(Properties, Default)]
#[properties(wrapper_type = super::PlanetDataObject)]
pub struct PlanetDataObject {
    #[property(name = "valuable", get, set, type = bool, member = valuable)]
    #[property(name = "body_name", get, set, type = String, member = body_name)]
    pub data: RefCell<PlanetData>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for PlanetDataObject {
    const NAME: &'static str = "ElitePathfinderPlanetData";
    type Type = super::PlanetDataObject;
}

// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for PlanetDataObject {}
