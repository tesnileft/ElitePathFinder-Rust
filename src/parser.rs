use serde::Deserialize;
use serde_json::{Result, Value};
use std::fs;
use std::io::BufReader;
use std::io::prelude::*;
use std::time;
use std::vec;
struct LogFile {
    timestamp: time::SystemTime,
    event: String,
}
#[derive(Deserialize)]
enum SignalType {
    FleetCarrier,
}
#[derive(Deserialize)]
enum GameMode {
    Solo,
    Group,
    Open,
}
#[derive(Deserialize)]
#[allow(non_snake_case)]
struct FSSSignal {
    SignalName: String,
    SignalType: SignalType,
    IsStation: bool,
}
#[derive(Deserialize)]
#[allow(non_snake_case)]
struct FSDTarget {
    Name: String,
    SystemAddress: u32,
    StarClass: String,
    RemainingJumpsInRoute: u32,
}
#[derive(Deserialize)]
#[allow(non_snake_case)]
struct Shiplocker {
    Items: Vec<Item>,
}
#[derive(Deserialize)]
#[allow(non_snake_case)]
struct Item {
    Name: String,
    Name_Localised: String,
    OwnerId: u32,
    Count: u32,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct RawMaterial {
    Name: String,
    Count: u32,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct LocalisedMaterial {
    Name: String,
    Name_Localised: String,
    Count: u32,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct Materials {
    Raw: Vec<RawMaterial>,
    Manufactured: Vec<LocalisedMaterial>,
    Encoded: Vec<LocalisedMaterial>,
}
#[derive(Deserialize)]
#[allow(non_snake_case)]
struct LoadGame {
    FID: String,
    Commander: String,
    Horizons: bool,
    Odyssey: bool,
    Ship: String,
    ShipLocalised: String,
    ShipId: u32,
    ShipName: String,
    ShipIdent: String,
    FuelLevel: f32,
    FielCapacity: f32,
    GameMode: GameMode,
    Group: String,
    Credits: u32,
    Loan: u32,
    language: String,
    gameversion: String,
    build: String,
}

fn parselogfile(path: String) -> Result<()> {
    let log = fs::read_to_string(path).expect("Log read succesfully");
    let loglines = log.lines();
    for line in loglines {
        parselogline(line);
    }
    Ok(())
}

fn parselogline(line: &str) -> Result<()> {
    let v: Value = serde_json::from_str(line)?;
    let event_type = v["event"].as_str().unwrap();
    println!("{}", event_type);
    match event_type {
        "LoadGame" => {
            let journal_entry: LoadGame = serde_json::from_str(line)?;
        }
        "Cargo" => {}
        "Friends" => {}
        "Commander" => {}
        "Materials" => {}
        "Rank" => {}
        "Progress" => {}
        "Reputation" => {}
        "EngineerProgress" => {}
        "SquadronStartup" => {}
        "Statistics" => {}
        "Location" => {}
        "Powerplay" => {}
        "Music" => {}
        "Shiplocker" => {}
        "Missions" => {}
        "Loadout" => {}
        "MissionAbandoned" => {}
        "NavRoute" => {}
        "FSDTarget" => {}
        "CommunityGoalJoin" => {}
        "CommunityGoal" => {}
        "Undocked" => {}
        "StartJump" => {}
        "ReceiveText" => {}
        "NavRouteClear" => {}
        "FSSSignalDiscovered" => {
            let journal_entry: FSSSignal = serde_json::from_str(line)?;
        }
        "FSDJump" => {}
        "SupercruiseDestinationDrop" => {}
        "SupercruiseExit" => {}
        "Scan" => {}
        other => {
            println!("Undocumented event type: {other}")
        }
    }
    Ok(())
}
