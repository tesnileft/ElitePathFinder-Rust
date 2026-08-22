use serde::Deserialize;
use std::fmt::Display;
#[derive(Deserialize, PartialEq, Eq)]
pub enum JumpType {
    Supercruise,
    Hyperspace,
}

#[derive(Deserialize)]
pub enum FriendStatus {
    Online,
    Offline,
}
///Faction Allegiances
#[derive(Deserialize, Default)]
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

#[derive(Deserialize)]
pub enum LegalStatus {
    Clean,
    Lawless,
    Wanted,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub enum ScanType{
    Cargo,
}

#[derive(Deserialize)]
pub enum MaterialCategory {
    #[serde(alias = "raw")]
    Raw,
    #[serde(alias = "manufactured")]
    Manufactured,
    #[serde(alias = "encoded")]
    Encoded,
}