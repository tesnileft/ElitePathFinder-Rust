use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Touchdown {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "PlayerControlled")]
    player_controlled: bool,
    #[serde(rename = "Taxi")]
    taxi: bool,
    #[serde(rename = "Multicrew")]
    multicrew: bool,
    #[serde(rename = "StarSystem")]
    star_system: String,
    #[serde(rename = "SystemAddress")]
    system_address: u64,
    #[serde(rename = "Body")]
    body: String,
    #[serde(rename = "BodyID")]
    body_id: u64,
    #[serde(rename = "OnStation")]
    on_station: bool,
    #[serde(rename = "OnPlanet")]
    on_planet: bool,
    #[serde(rename = "Latitude")]
    latitude: f32,
    #[serde(rename = "Longitude")]
    longitude: f32,
}

#[derive(Deserialize)]
pub struct Liftoff {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "PlayerControlled")]
    player_controlled: bool,
    #[serde(rename = "Taxi")]
    taxi: bool,
    #[serde(rename = "Multicrew")]
    multicrew: bool,
    #[serde(rename = "StarSystem")]
    star_system: String,
    #[serde(rename = "SystemAddress")]
    system_address: u64,
    #[serde(rename = "Body")]
    body: String,
    #[serde(rename = "BodyID")]
    body_id: u64,
    #[serde(rename = "OnStation")]
    on_station: bool,
    #[serde(rename = "OnPlanet")]
    on_planet: bool,
    #[serde(rename = "Latitude")]
    latitude: f32,
    #[serde(rename = "Longitude")]
    longitude: f32,
}

#[derive(Deserialize)]
pub struct ApproachBody {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "StarSystem")]
    star_system: String,
    #[serde(rename = "SystemAddress")]
    system_address: u64,
    #[serde(rename = "Body")]
    body: String,
    #[serde(rename = "BodyID")]
    body_id: u64,
}

#[derive(Deserialize)]
pub struct LeaveBody {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "StarSystem")]
    star_system: String,
    #[serde(rename = "SystemAddress")]
    system_address: u64,
    #[serde(rename = "Body")]
    body: String,
    #[serde(rename = "BodyID")]
    body_id: u64,
}