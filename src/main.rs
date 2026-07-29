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
use std::thread;
use std::time::Duration;

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

    start_background_reader(ui_event_sender);
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

fn start_background_reader(ui_event_sender: async_channel::Sender<UiEvent>) {
    //Detect current logfile
    let current_log_path = "/mnt/gamestorage/SteamLibrary/steamapps/compatdata/359320/pfx/drive_c/users/steamuser/AppData/Local/Frontier Developments/Elite Dangerous/Journal12543831.cache";
    //Open logfile
    let current_log_result = File::open(current_log_path);
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
        loop {
            thread::sleep(Duration::from_millis(100)); // Poll every so often TODO update to notify or something
            let mut stringcontents: String = String::new();
            match current_log_buffer.read_to_string(&mut stringcontents) {
                Ok(num) => {
                    if num == 0 {
                        continue;
                    }
                }
                Err(error) => {
                    println!("Oopsie woopsie didn't read from log buffer properly")
                }
            }
            let result = parser::parse_logstring(stringcontents);
            let ui_event_sender = ui_event_sender.clone();
            message_bus(result.unwrap(), ui_event_sender)
        }
    });
}

fn message_bus(event_vec: Vec<EliteEvent>, ui_event_sender: async_channel::Sender<UiEvent>) {
    gio::spawn_blocking(move || {
        for event in event_vec {
            match event {
                EliteEvent::Music(music) => {
                    println!("Playing music: {}", music.music_track)
                }
                EliteEvent::FSDJump(fsdjump) => {
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


