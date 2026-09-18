use serde::{Deserialize, Serialize};
use std::fmt::Display;
#[derive(Deserialize, Serialize, PartialEq, Eq)]
pub enum JumpType {
    Supercruise,
    Hyperspace,
}

#[derive(Deserialize, Serialize)]
pub enum FriendStatus {
    Online,
    Offline,
}
///Faction Allegiances
#[derive(Deserialize, Serialize, Default)]
pub enum Allegiance {
    Empire,
    #[serde(alias = "PilotsFederation")]
    Federation,
    Alliance,
    Independent,
    #[default]
    #[serde(rename = "")]
    None,
}

#[derive(Deserialize, Serialize)]
pub enum LegalStatus {
    Clean,
    Lawless,
    Wanted,
}

#[derive(Deserialize, Serialize)]
pub enum PilotRank {
    Harmless,
    MostlyHarmless,
    Novice,
    Competent,
    Expert,
    Master,
    Dangerous,
    Deadly,
    Elite,
}

#[derive(Deserialize, Serialize)]
pub enum ScanType{
    Cargo,
}

#[derive(Deserialize, Serialize)]
pub enum MaterialCategory {
    #[serde(alias = "raw")]
    Raw,
    #[serde(alias = "manufactured")]
    Manufactured,
    #[serde(alias = "encoded")]
    Encoded,
}