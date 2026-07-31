use gtk::{Application, Label, gio, glib};
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
use std::ffi::OsStr;
use std::time::Duration;
use gdk::Event;
use log::error;
use regex::Regex;
use rusqlite::fallible_iterator::FallibleIterator;

mod parser;

mod elite_events;
mod settings_area;
mod topbar;
mod window;
mod helpers;

use crate::UiEvent::SetCurrentSystem;
use crate::elite_events::enums::*;
use crate::parser::EliteEvent;
use window::Window;
use crate::helpers::extract_latest_journal;

const APP_ID: &str = "tesnileft.ElitePathfinder_rs";

pub enum UiEvent {
    SetCurrentSystem { system_name: String },
    UpdateCurrency { arx: u64, credits: u64 },
}
#[derive(Default)]
pub struct Cache {
    pub current_system: String,
    pub current_body: Option<String>,
    pub current_body_type: Option<BodyType>,
    pub credits: u64,
    pub arx: u64,
    pub in_hyperspace: bool,
    pub game_location: String,
    pub log_location: String,
}
pub type SharedCache = Arc<Mutex<Cache>>;
fn main() -> glib::ExitCode {
    #[cfg(debug_assertions)]
    unsafe {
        std::env::set_var("GSETTINGS_SCHEMA_DIR", "data");
    }

    gio::resources_register_include!("elite_pathfinder.gresource")
        .expect("Failed to register resources.");
    let settings = gio::Settings::new("tesnileft.ElitePathFinder_rs");

    let cache = Arc::new(Mutex::new(Cache::default()));
    let application = Application::builder().application_id(APP_ID).build();
    let _ = topbar::EliteHeaderBar::static_type();
    let _ = settings_area::SettingsArea::static_type();
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

fn read_newest_log_file() -> Result<String, std::io::Error> {
    let current_log_result = File::open(
        "/mnt/gamestorage/SteamLibrary/steamapps/compatdata/359320/pfx/drive_c/users/steamuser/AppData/Local/Frontier Developments/Elite Dangerous/Journal12543831.cache",
    );
    match current_log_result {
        Ok(file) => {
            let current_log: File = file;
            let mut current_log_buf = BufReader::new(current_log);
            let mut new_contents = String::new();
            current_log_buf.read_to_string(&mut new_contents).unwrap();
            Ok(new_contents)
        }
        Err(error) => Err(error),
    }
}
fn get_current_logfile_name(appdata_elite: PathBuf) -> Result<PathBuf, std::io::Error> {
    for entry in std::fs::read_dir(appdata_elite.clone())? {
        let path = entry?.path();
        let filename = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name,
            None => continue,
        };
        if let Some(number_part) = filename
            .strip_prefix("Journal")
            .and_then(|s| s.strip_suffix(".cache"))
        {
            if number_part.chars().all(|c| c.is_ascii_digit()) {
                println!("Valid journal cache: {}", filename);
                return Ok(filename.into());
            }
        }
    }
    Err(Error::new(ErrorKind::Other, "No journal log file found."))
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
                    println!("Playing music: {}", music.music_track)
                }
                EliteEvent::StartJump(start_jump) => {
                    if start_jump.jump_type == JumpType::Hyperspace
                    {

                    }
                }
                EliteEvent::FSDJump(fsdjump) => {
                    let unlocked_cache = shared_cache.lock().unwrap();
                    let event = SetCurrentSystem {
                        system_name: fsdjump.star_system,
                    };
                    ui_event_sender
                        .send_blocking(event)
                        .expect("UI Event Channel unavailable");
                }
                EliteEvent::StartJump(startjump) => {}
                other => {}
            }
        }
    });
}
fn build_ui_xml(
    app: &Application,
    ui_event_receiver: async_channel::Receiver<UiEvent>,
    cache: SharedCache,
) {
    let window = Window::new(app, ui_event_receiver, cache);
    window.present();
}


