use std::default::Default;
use gtk::{Application, Label, gio, glib, SignalListItemFactory};
use gtk::{Entry, prelude::*};
use rusqlite::Connection;
use serde::Deserialize;
use std::collections::HashMap;
use std::env::home_dir;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::{fs, thread};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::hash_map::OccupiedEntry;
use std::ffi::OsStr;
use std::ops::Deref;
use std::time::Duration;
use gdk::Event;
use gtk::glib::property::PropertyGet;
use gtk::graphene::Plane;
use log::error;
use regex::Regex;
use rusqlite::fallible_iterator::FallibleIterator;

mod parser;
mod custom_structs;
mod elite_events;
mod settings_area;
mod topbar;
mod window;
mod helpers;
mod exobio_card;
mod planet_data_object;
mod exobio_analysis;

use crate::UiEvent::SetCurrentSystem;
use crate::elite_events::enums::*;
use crate::parser::EliteEvent;
use window::Window;
use crate::custom_structs::materials::PlanetRawMaterial;
use crate::custom_structs::system_info::Body;
use crate::custom_structs::system_info::*;
use crate::custom_structs::system_info::Body::{Planet, Star};
use crate::elite_events::events::{FSSBodySignals, FSSSignalDiscovered, Genus, Materials, RawMaterial, Scan};
use crate::elite_events::events::Genus::Bacterium;
use crate::elite_events::events::RawMaterial::{Cadmium, Mercury, Molybdenum, Niobium, Tin, Tungsten};
use crate::elite_events::substructs::{AtmosphericGas, BodyComposition};
use crate::exobio_analysis::determine_exobio_species;
use crate::helpers::extract_latest_journal;
use crate::Species::Cerbrus;

const APP_ID: &str = "tesnileft.ElitePathfinder_rs";

pub enum UiEvent {
    SetCurrentSystem { system_name: String },
    UpdateCurrency { arx: u64, credits: u64 },
}
#[derive(Default)]
pub struct Cache {
    pub current_system: StarSystem,
    pub current_body: Option<String>,
    pub current_body_type: Option<BodyType>,
    pub credits: u64,
    pub arx: u64,
    pub in_hyperspace: bool,
    pub game_location: String,
    pub log_location: String,
    pub system_bodies: HashMap<String, Body>,
}
pub type SharedCache = Arc<Mutex<Cache>>;
fn load_custom_widgets()
{
    let _ = topbar::EliteHeaderBar::static_type();
    let _ = settings_area::SettingsArea::static_type();
    let _ = exobio_card::ExobioCard::static_type();
}
fn main() -> glib::ExitCode {
    #[cfg(debug_assertions)]
    unsafe {
        std::env::set_var("GSETTINGS_SCHEMA_DIR", "data");
    }
    load_custom_widgets();
    gio::resources_register_include!("elite_pathfinder.gresource")
        .expect("Failed to register resources.");
    let settings = gio::Settings::new("tesnileft.ElitePathFinder_rs");

    let mut cache = Arc::new(Mutex::new(Cache::default()));
    let application = Application::builder().application_id(APP_ID).build();

    let (ui_event_sender, ui_event_receiver) = async_channel::unbounded::<UiEvent>();
    application.connect_activate(move |app| {
        build_ui_xml(app, ui_event_receiver.clone(), cache.clone());
    });
    let database_connection = Connection::open_in_memory().unwrap();

    let shared_cache = Arc::new(Mutex::new(Cache::default()));
    //TODO load cache from database


    start_background_reader(ui_event_sender, shared_cache);
    application.run()
}

fn start_background_reader(ui_event_sender: async_channel::Sender<UiEvent>, shared_cache: SharedCache){
    //Detect current logfile
    let elite_appdata = helpers::get_current_logfile_path().unwrap();
    let elite_journal_folder = helpers::get_journals_location().unwrap();
    let mut current_log_file = extract_latest_journal(elite_journal_folder.clone());
    //Open logfile
    let current_log_result = File::open(current_log_file.clone());
    let current_log = match current_log_result {
        Ok(file) => file,
        Err(error) => {
            //TODO add error handling for when the log file isn't found
            return;
        }
    };
    let mut current_log_buffer = BufReader::new(current_log);
    // Blocking thread that loops reading
    std::thread::spawn(move || {
        println!("Starting background reader...");
        let mut loops: u64 = 0;
        loop {
            thread::sleep(Duration::from_millis(100)); // Poll every so often TODO update to notify or something
            loops += 1;

            let mut stringcontents: String = String::new();
            match current_log_buffer.read_to_string(&mut stringcontents) {
                Ok(num) => {
                    if num != 0 {
                        let result = parser::parse_logstring(stringcontents);
                        message_bus(result.unwrap(), ui_event_sender.clone(), shared_cache.clone());
                    }
                }
                Err(error) => {
                    println!("Oopsie woopsie didn't read from log buffer properly")
                }
            }
            if loops >= 20 {
                loops = 0;
                let new_log_file = extract_latest_journal(elite_journal_folder.clone());
                if new_log_file != current_log_file {
                    current_log_file = new_log_file.clone();
                    let current_log_result = File::open(new_log_file.clone());
                    let new_log = match current_log_result {
                        Ok(file) => file,
                        Err(error) => {
                            //TODO add error handling for when the log file isn't found
                            return;
                        }
                    };
                    println!("Starting background reader on new log file...");
                    current_log_buffer = BufReader::new(new_log);
                }
            }
        }
    });
}

fn message_bus(event_vec: Vec<EliteEvent>, ui_event_sender: async_channel::Sender<UiEvent>, shared_cache: SharedCache) {
    gio::spawn_blocking(move || {
        for event in event_vec {
            
            match event {
                EliteEvent::LoadGame(load) => {
                    println!("Game Loaded:");
                    println!("CMDR: {}", load.commander);
                    println!("Credits: {}", load.credits);
                }
                EliteEvent::Music(music) => {
                }
                EliteEvent::StartJump(start_jump) => {
                    if start_jump.jump_type == JumpType::Hyperspace
                    {

                    }
                }
                EliteEvent::FSDJump(fsdjump) => {
                    let mut unlocked_cache = shared_cache.lock().unwrap();
                    let jumpingto = StarSystem{
                        name: fsdjump.star_system,
                        address: fsdjump.system_address,
                        star_position: fsdjump.star_pos,
                        security: fsdjump.system_security,
                        allegiance: fsdjump.system_allegiance,
                        economy: fsdjump.system_economy,
                        second_economy: fsdjump.system_second_economy,
                        government: fsdjump.system_government,
                        ..Default::default()
                    };
                    println!("Parsed Jump to: {}", jumpingto.name);
                    unlocked_cache.current_system = jumpingto;
                }
                EliteEvent::FSSBodySignals(signals) => {

                }
                EliteEvent::Scan(scan) => {
                    let mut cache = shared_cache.lock().unwrap();
                    let entry = cache.current_system.bodies.entry(scan.body_id.clone());
                    let mats = scan.materials.iter()
                        .map(|scan_mat| PlanetRawMaterial{material: scan_mat.name.clone(), percentage: scan_mat.percent} )
                        .collect();
                    match entry {
                        Occupied(mut entry) => {
                            match entry.get_mut() {
                                Star(star) => {}
                                Planet(planet) => {
                                    planet.planet_class = scan.planet_class;
                                    planet.gravity = Some(scan.surface_gravity);
                                    planet.mean_temperature = Some(scan.surface_temperature);
                                    planet.volcanism = Some(scan.volcanism);
                                    planet.atmosphere_type= Some(scan.atmosphere_type);
                                    planet.atmosphere_composition = Some(scan.atmosphere_composition);
                                    planet.body_composition = Some(scan.composition);
                                    planet.materials = Some(mats);
                                }
                            }
                        }
                        Vacant(entry) => {

                            let newplanet = custom_structs::system_info::Planet{
                                body_name: scan.body_name.clone(),
                                body_id: scan.body_id,
                                system_address: scan.system_address,
                                planet_class: scan.planet_class,
                                gravity: Some(scan.surface_gravity),
                                mean_temperature: Some(scan.surface_temperature),
                                volcanism: Some(scan.volcanism),
                                atmosphere_type: Some(scan.atmosphere_type),
                                atmosphere_composition: Some(scan.atmosphere_composition),
                                body_composition: Some(scan.composition),
                                materials: Some(mats),
                                ..Default::default()
                            };
                            entry.insert(Planet(newplanet));
                        }
                    }
                }
                EliteEvent::StartJump(startjump) => {}
                other => {}
            }
        }
    });
}

#[derive(Default)]
struct StarSystem{
    name: String,
    address: u64,
    bodies: HashMap<u64, Body>,
    star_position: (f64, f64, f64), //Galactic Coordinates
    security: SystemSecurity,
    allegiance: Allegiance,
    economy: Economy,
    second_economy: Economy,
    government: Government
}
impl StarSystem{
    ///Returns full list of potential species
    pub fn get_potential_exobio(&self, body_id: u64) -> Option<Vec<ExoBiologySpecies>>{
        determine_exobio_species(self, body_id)
    }
}
fn check_material(materials: &Vec<PlanetRawMaterial>, checkmaterial: RawMaterial) -> bool {
    materials.iter().any(|m| m.material == checkmaterial)
}
enum GeologicalThings{
    Fumarole,
    IceFumarole,
    Geyser,
    IceGeyser,
    LavaSpout,
    GasVent,
}
struct ExoBiologySpecies{
    genus: Genus,
    species: Species,
    variants: Vec<ExoBiologyVariant>,
}
pub fn get_species_value(species: Species) -> u64
{
    match species {
        Species::Acies | Species::Aurasus | Species::Vesicula => 1_000_000,
        Species::Alcyoneum => 1_658_500,
        Species::Bullaris => 1_152_500,
        Species::Cerbrus => 1_689_800,
        Species::Informem => 8_418_000,
        Species::Nebulus => 5_289_900,
        Species::Omentum => 4_638_900,
        Species::Scopulum => 4_934_500,
        Species::Tela => 1_949_000,
        Species::Verrata => 3_897_000,
        Species::Volu => 7_774_700,
    }
}

fn build_ui_xml(
    app: &Application,
    ui_event_receiver: async_channel::Receiver<UiEvent>,
    cache: SharedCache,
) {
    let window = Window::new(app, ui_event_receiver, cache);


    window.present();
}


