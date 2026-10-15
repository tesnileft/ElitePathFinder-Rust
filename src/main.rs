use elite_journal_data::enums::body_data::BodyType;
use gtk::prelude::*;
use gtk::{Application, gio, glib};
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::default::Default;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod parser;
mod custom_structs;
mod db;
mod elite_journal_data;
mod settings_area;
mod topbar;
mod window;
mod helpers;
mod exobio_card;
mod planet_data_object;
mod exobiology_analysis;

use crate::custom_structs::materials::PlanetRawMaterial;
use crate::custom_structs::starsystem::StarSystem;
use crate::custom_structs::system_info::Body;
use crate::custom_structs::system_info::Body::{Planet, Star};
use crate::elite_journal_data::enums::misc::JumpType;
use crate::elite_journal_data::enums::signals::SAASignalType;
use crate::helpers::extract_latest_journal;
use crate::parser::EliteEvent;
use window::Window;
use crate::custom_structs::exobiospecies::ExoBiologySpecies;

const APP_ID: &str = "tesnileft.ElitePathfinder_rs";

pub enum UiEvent {
    SetCurrentSystem { system_name: String },
    SetLocationName { location_name: Option<String> },
    SetCommanderName { name: String },
    UpdateCurrency { arx: u64, credits: u64 },
    SetJumping { jumping: bool, destination: Option<String> },
    SetPotentialExobioList{ body_name: String, species: Vec<ExoBiologySpecies> },
    ClearExobioList,
}
#[derive(Default)]
pub struct Cache {
    pub current_system: StarSystem,
    pub current_body: Option<String>,
    pub current_body_type: Option<BodyType>,
    pub commander_name: String,
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

    let cache = Arc::new(Mutex::new(Cache::default()));
    let application = Application::builder().application_id(APP_ID).build();


    let (ui_event_sender, ui_event_receiver) = async_channel::unbounded::<UiEvent>();
    application.connect_activate(move |app| {
        build_ui_xml(app, ui_event_receiver.clone(), cache.clone());
    });
    let database_connection = db::open("elitepathfinder.db");

    let shared_cache = Arc::new(Mutex::new(Cache::default()));
    //TODO load cache from database


    start_background_reader(ui_event_sender, shared_cache);
    application.run()
}

fn start_background_reader(ui_event_sender: async_channel::Sender<UiEvent>, shared_cache: SharedCache){
    //Detect current logfile
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
                    let mut cache = shared_cache.lock().unwrap();
                    cache.commander_name = load.commander.clone();
                    cache.credits = load.credits;
                    let arx = cache.arx;
                    drop(cache);
                    let _ = ui_event_sender.send_blocking(UiEvent::SetCommanderName { name: load.commander });
                    let _ = ui_event_sender.send_blocking(UiEvent::UpdateCurrency { arx, credits: load.credits });
                }
                EliteEvent::Commander(commander) => {
                    let mut cache = shared_cache.lock().unwrap();
                    cache.commander_name = commander.name.clone();
                    drop(cache);
                    let _ = ui_event_sender.send_blocking(UiEvent::SetCommanderName { name: commander.name });
                }
                EliteEvent::Location(location) => {
                    let mut cache = shared_cache.lock().unwrap();
                    let current_system = StarSystem {
                        name: location.star_system.clone(),
                        address: location.system_address,
                        star_position: location.star_pos,
                        security: location.system_security,
                        allegiance: location.system_allegiance.unwrap_or_default(),
                        economy: location.system_economy,
                        second_economy: location.system_second_economy,
                        government: location.system_government,
                        ..Default::default()
                    };
                    cache.current_system = current_system;
                    cache.game_location = location.station_name.clone().unwrap_or_default();
                    drop(cache);
                    let _ = ui_event_sender.send_blocking(UiEvent::SetCurrentSystem { system_name: location.star_system });
                    let _ = ui_event_sender.send_blocking(UiEvent::SetLocationName { location_name: location.station_name });
                }
                EliteEvent::Docked(docked) => {
                    let mut cache = shared_cache.lock().unwrap();
                    cache.game_location = docked.station_name.clone();
                    drop(cache);
                    let _ = ui_event_sender.send_blocking(UiEvent::SetLocationName { location_name: Some(docked.station_name) });
                }
                EliteEvent::Undocked(_) => {
                    let mut cache = shared_cache.lock().unwrap();
                    cache.game_location.clear();
                    drop(cache);
                    let _ = ui_event_sender.send_blocking(UiEvent::SetLocationName { location_name: None });
                }
                EliteEvent::StartJump(start_jump) => {
                    if start_jump.jump_type == JumpType::Hyperspace
                    {
                        let mut cache = shared_cache.lock().unwrap();
                        cache.in_hyperspace = true;
                        drop(cache);
                        let _ = ui_event_sender.send_blocking(UiEvent::SetJumping { jumping: true, destination: start_jump.star_system });
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
                    unlocked_cache.current_system = jumpingto;
                    unlocked_cache.game_location.clear();
                    unlocked_cache.in_hyperspace = false;
                    let system_name = unlocked_cache.current_system.name.clone();
                    drop(unlocked_cache);
                    let _ = ui_event_sender.send_blocking(UiEvent::ClearExobioList);
                    let _ = ui_event_sender.send_blocking(UiEvent::SetCurrentSystem { system_name });
                    let _ = ui_event_sender.send_blocking(UiEvent::SetLocationName { location_name: None });
                    let _ = ui_event_sender.send_blocking(UiEvent::SetJumping { jumping: false, destination: None });
                }
                EliteEvent::FSSBodySignals(signals) => {
                    let mut cache = shared_cache.lock().unwrap();
                    let entry = cache.current_system.bodies.entry(signals.body_id.clone());
                    let biological_count = signals
                        .signals
                        .iter()
                        .find(|s| matches!(s.type_, SAASignalType::Biological))
                        .map(|s| s.count)
                        .unwrap_or(0);
                    let geological_count = signals
                        .signals
                        .iter()
                        .find(|s| matches!(s.type_, SAASignalType::Geological))
                        .map(|s| s.count)
                        .unwrap_or(0);
                    match entry {
                        Occupied(mut entry) => {
                            if let Planet(planet) = entry.get_mut() {
                                planet.biological_signals = Some(biological_count);
                                planet.geological_signals = Some(geological_count);
                            }
                        }
                        Vacant(entry) => {
                            let newplanet = custom_structs::system_info::Planet{
                                body_name: signals.body_name.clone(),
                                body_id: signals.body_id,
                                system_address: signals.system_address,
                                biological_signals: Some(biological_count),
                                geological_signals: Some(geological_count),
                                ..Default::default()
                            };
                            entry.insert(Planet(newplanet));
                        }
                    }
                    if cache.current_system.all_bodies_found {
                        if let Some((body_name, species)) = cache.current_system.update_potential_exobio(signals.body_id) {
                            drop(cache);
                            let _ = ui_event_sender.send_blocking(UiEvent::SetPotentialExobioList { body_name, species });
                        }
                    }
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
                                Star(star) => {
                                    //TODO
                                }
                                Planet(planet) => {
                                    planet.parents = Some(scan.parents.unwrap_or_default());
                                    planet.planet_class = scan.planet_class;
                                    planet.gravity = scan.surface_gravity;
                                    planet.mean_temperature = scan.surface_temperature;
                                    planet.volcanism = Some(scan.volcanism);
                                    planet.distance_from_arrival_ls = Some(scan.distance_from_arrival_ls);
                                    planet.atmosphere = Some(custom_structs::system_info::AtmosphereInfo::new(scan.atmosphere_type, scan.atmosphere, scan.atmosphere_composition.unwrap_or_default()));
                                    planet.body_composition = scan.composition;
                                    planet.materials = Some(mats);
                                }
                            }
                        }
                        Vacant(entry) => {
                            if let (Some(star_type), Some(luminosity)) = (scan.star_class, scan.luminosity){
                                let new_star = custom_structs::system_info::Star{
                                    body_name: scan.body_name.clone(),
                                    body_id: scan.body_id,
                                    luminosity,
                                    class: star_type,
                                    subclass: scan.star_subclass.unwrap(),
                                    stellar_mass: scan.stellar_mass.unwrap(),
                                };
                                entry.insert(Star(new_star));
                            }
                            else if let Some(plan_clas) = scan.planet_class.as_ref() {
                                let newplanet = custom_structs::system_info::Planet{
                                    body_name: scan.body_name.clone(),
                                    body_id: scan.body_id,
                                    system_address: scan.system_address,
                                    parents: Some(scan.parents.unwrap_or_default()),
                                    planet_class: scan.planet_class,
                                    gravity: scan.surface_gravity,
                                    mean_temperature: scan.surface_temperature,
                                    volcanism: Some(scan.volcanism),
                                    distance_from_arrival_ls: Some(scan.distance_from_arrival_ls),
                                    atmosphere: Some(custom_structs::system_info::AtmosphereInfo::new(scan.atmosphere_type, scan.atmosphere, scan.atmosphere_composition.unwrap_or_default())),
                                    body_composition: scan.composition,
                                    materials: Some(mats),
                                    ..Default::default()
                                };
                                entry.insert(Planet(newplanet));
                            }

                        }
                    }
                    //Only predict once the whole system is scanned, otherwise results are incomplete
                    if cache.current_system.all_bodies_found {
                        if let Some((body_name, species)) = cache.current_system.update_potential_exobio(scan.body_id) {
                            drop(cache);
                            let _ = ui_event_sender.send_blocking(UiEvent::SetPotentialExobioList { body_name, species });
                        }
                    }
                }
                EliteEvent::FSSAllBodiesFound(found) => {
                    let mut cache = shared_cache.lock().unwrap();
                    if found.system_address != cache.current_system.address {
                        continue;
                    }
                    cache.current_system.all_bodies_found = true;
                    let ids: Vec<u64> = cache.current_system.bodies.keys().copied().collect();
                    let results: Vec<_> = ids
                        .into_iter()
                        .filter_map(|id| cache.current_system.update_potential_exobio(id))
                        .collect();
                    drop(cache);
                    for (body_name, species) in results {
                        let _ = ui_event_sender.send_blocking(UiEvent::SetPotentialExobioList { body_name, species });
                    }
                }
                _ => {}
            }
        }
    });
}




fn build_ui_xml(
    app: &Application,
    ui_event_receiver: async_channel::Receiver<UiEvent>,
    cache: SharedCache,
) {
    let provider = gtk::CssProvider::new();
    provider.load_from_resource("/elite_pathfinder/style.css");
    if let Some(display) = gtk::gdk::Display::default() {
        gtk::style_context_add_provider_for_display(&display, &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
    }
    let window = Window::new(app, ui_event_receiver, cache);
    window.present();
}


