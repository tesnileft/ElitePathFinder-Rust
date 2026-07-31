use std::collections::HashMap;
use std::env::home_dir;
use std::ffi::OsStr;
use std::fs;
use std::fs::{metadata, DirEntry, File};
use std::io::{Error, ErrorKind, Read};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use gtk::gio;
use gtk::prelude::{SettingsExt, SettingsExtManual};
use serde::Deserialize;
use crate::{helpers};
use crate::parser::parse_logstring;

#[derive(Deserialize, Debug)]
struct LibraryFolders(HashMap<String, Library>);
#[derive(Deserialize, Debug)]
#[serde(rename = "libraryfolders")]
struct Library {
    pub path: String,
    pub label: String,
    pub contentid: String,
    pub totalsize: u64,
    pub update_clean_bytes_tally: u64,
    pub time_last_update_verified: u64, //Assuming this is a unix timestamp
    #[serde(default)]
    pub apps: HashMap<String, String>,
}
///Tries to detect location of log files.
pub fn get_journals_location() -> Result<PathBuf, std::io::Error> {
    //TODO check this ig, I don't have windows installed
    let default_windows_path = PathBuf::from(r"\Saved Games\Frontier Developments\Elite Dangerous");
    if (std::env::consts::OS == "windows") {
        let full_windows_path = home_dir()
            .expect("Failed to get home directory")
            .join(default_windows_path);
        if Path::exists(full_windows_path.as_path()) {
            return Ok(full_windows_path);
        }
        return Err(Error::new(
            ErrorKind::NotFound,
            "EliteDangerous path not default path",
        ));
    }
    else if std::env::consts::OS == "linux" {
        let library_path = get_elite_steam_library_folder_linux()?;
        let linux_path: PathBuf = library_path.join(PathBuf::from(
            r"steamapps/compatdata/359320/pfx/drive_c/users/steamuser/Saved Games/Frontier Developments/Elite Dangerous"));
        println!("Elite Dangerous linux path: {:?}", linux_path);
        return Ok(linux_path);
    }
    Err(Error::new(ErrorKind::NotFound, "No saved games"))
}

pub fn get_current_logfile_path() -> Result<PathBuf, std::io::Error> {
    let library_path = get_elite_steam_library_folder_linux().expect("Unable to get system steam library directory data");
    let path: PathBuf = library_path.join(PathBuf::from(
        r"steamapps/compatdata/359320/pfx/drive_c/users/steamuser/AppData/Local/Frontier Developments/Elite Dangerous",
    ));
    Ok(path)
}
///Automatically tries to determine where the steam library with Elite is installed
/// Relevant for linux because it will have to store appdata/documents pertaining to elite in there (logfiles)
fn get_elite_steam_library_folder_linux() -> Result<PathBuf, std::io::Error> {
    // Look at steam library folder
    let libraryfolders_path =
        home_dir()
            .expect("Failed to get home directory")
            .join(PathBuf::from(
                r".local/share/Steam/steamapps/libraryfolders.vdf",
            )); //Stores where what is installed with steam
    let mut file = File::open(libraryfolders_path)?;
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    let vfd_content: LibraryFolders =
        keyvalues_serde::from_str(&contents).expect("VDF Decoding Failed");

    for (e, l) in vfd_content.0 { // Iterate over all steam locations to look for the library elite is in
        match l.apps.get("359320"){ //ED appid
            Some(v) => {
                if *v != *"0"
                {
                    println!("Found a library with Elite installed!!");
                    return Ok(PathBuf::from(l.path));
                }
            }
            None => {continue;}
        };
    }
    Err(Error::new(ErrorKind::NotFound, "Unable to automatically determine steam library location"))
}
///Checks supplied path for .log files
fn get_stored_journal_paths(path_buf: PathBuf) -> Result<Vec<PathBuf>, std::io::Error>
{
    let journals: Vec<_> = fs::read_dir(&path_buf)?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|path| {
            path.extension() == Some(OsStr::new("log"))
        })
        .collect();
    Ok(journals)
}

///Returns path to most recently modified log
pub fn extract_latest_journal(journal_folder: PathBuf) -> PathBuf {
    let journals = get_stored_journal_paths(journal_folder).unwrap();
    let mut most_recent_journal = PathBuf::new();
    let mut most_recent_modification: SystemTime = SystemTime::UNIX_EPOCH;
    for log in journals {
        let log_metadata = metadata(log.as_path()).unwrap();
        let log_modified = log_metadata.modified().unwrap();
        if most_recent_modification < log_modified {
            most_recent_modification = log_modified;
            most_recent_journal = log;
        }
    };
    PathBuf::from(most_recent_journal)
}


pub fn read_all_journals() {
    match get_journals_location() {
        Ok(location) => {
            println!("looking at {:?}", location);
            let all_journals = get_stored_journal_paths(location).unwrap();
            for journal_path in all_journals {
                let result = parse_logstring(fs::read_to_string(journal_path).unwrap());
            }
        }
        Err(err) => {
            println!("Unable to determine Elite Dangerous logs location!!{:?}", err);
        }
    }
}

