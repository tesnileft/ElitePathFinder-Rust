use std::collections::HashMap;
use std::env::home_dir;
use std::ffi::OsStr;
use std::fs;
use std::fs::{metadata, DirEntry, File};
use std::io::{Error, ErrorKind, Read};
use std::path::{Path, PathBuf};
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
pub fn get_logfileslocation() -> Result<PathBuf, std::io::Error> {
    let settings = gio::Settings::new("tesnileft.ElitePathFinder_rs");
    let stored_location: String = settings.get::<String>("elite-journal-logs-path");

    if Path::exists(stored_location.as_ref()) {
        println!("Default path exists");
        return Ok(stored_location.into());
    }
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
        let library_path = get_elite_compat_user_folder_linux().expect("Unable to get system steam library directory data");
        let path: PathBuf = library_path.join(PathBuf::from(
            r"steamapps/compatdata/359320/pfx/drive_c/users/steamuser/Saved Games/Frontier Developments/Elite Dangerous",
        ));
        println!("Elite Dangerous linux path: {:?}", path);
        settings.set_string("elite-journal-logs-path", &path.to_string_lossy()).expect("Failed to set elite-journal-logs-path");
        return Ok(path);
    }
    Err(Error::new(ErrorKind::NotFound, "No saved games"))
}

fn get_elite_compat_user_folder_linux() -> Result<PathBuf, std::io::Error> {
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

pub fn read_all_journals() {
    match get_logfileslocation() {
        Ok(location) => {
            println!("looking at {:?}", location);
            let old_journals = get_stored_journal_paths(location).unwrap();
            for journal_path in old_journals {
                let result = parse_logstring(fs::read_to_string(journal_path).unwrap());
            }
        }
        Err(err) => {
            println!("Unable to determine Elite Dangerous logs location!!{:?}", err);
        }
    }
}

