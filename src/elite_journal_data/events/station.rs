use crate::elite_journal_data::enums::station_data::{StationService, StationType};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use crate::elite_journal_data::enums::misc::MaterialCategory;
use crate::elite_journal_data::enums::system_data::Government;
use crate::elite_journal_data::substructs::inventory::{MaterialInventory, MaterialTraded};
use crate::elite_journal_data::substructs::station_data::{LandingPads, StationEconomy, StationFaction};

//endregion
//region - Docking -
#[derive(Deserialize)]
pub struct DockingRequested {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "MarketID")]
    market_id: u64,
    #[serde(rename = "StationName")]
    station_name: String,
    #[serde(rename = "StationType")]
    station_type: StationType,
    #[serde(rename = "LandingPads")]
    landing_pads: LandingPads,
}

#[derive(Deserialize)]
pub struct DockingGranted {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "LandingPad")]
    landing_pad: u64,
    #[serde(rename = "MarketID")]
    market_id: u64,
    #[serde(rename = "StationName")]
    station_name: String,
    #[serde(rename = "StationType")]
    station_type: StationType,
}

#[derive(Deserialize)]
pub struct Docked {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "StationName")]
    station_name: String,
    #[serde(rename = "StationType")]
    station_type: StationType,
    #[serde(rename = "Taxi")]
    taxi: bool,
    #[serde(rename = "Multicrew")]
    multicrew: bool,
    #[serde(rename = "StarSystem")]
    star_system: String,
    #[serde(rename = "SystemAddress")]
    system_address: u64,
    #[serde(rename = "MarketID")]
    market_id: u64,
    #[serde(rename = "StationFaction")]
    station_faction: StationFaction,
    #[serde(rename = "StationGovernment")]
    station_government: Government,
    #[serde(rename = "StationGovernment_Localised")]
    station_government_localised: String,
    #[serde(rename = "StationServices")]
    station_services: Vec<StationService>,
    #[serde(rename = "StationEconomy")]
    station_economy: String,
    #[serde(rename = "StationEconomy_Localised")]
    station_economy_localised: String,
    #[serde(rename = "StationEconomies")]
    station_economies: Vec<StationEconomy>,
    #[serde(rename = "DistFromStarLS")]
    dist_star_ls: f32,
    #[serde(rename = "LandingPads")]
    landing_pads: LandingPads,
}

#[derive(Deserialize)]
pub struct Embark {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "SRV")]
    srv: bool,
    #[serde(rename = "Taxi")]
    taxi: bool,
    #[serde(rename = "Multicrew")]
    multicrew: bool,
    #[serde(rename = "ID")]
    id: u64,
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
}

#[derive(Deserialize)]
pub struct Disembark {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "SRV")]
    srv: bool,
    #[serde(rename = "Taxi")]
    taxi: bool,
    #[serde(rename = "Multicrew")]
    multicrew: bool,
    #[serde(rename = "ID")]
    id: u64,
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
}

#[derive(Deserialize)]
pub struct Undocked {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "StationName")]
    station_name: String,
    #[serde(rename = "StationType")]
    station_type: StationType,
    #[serde(rename = "Taxi")]
    taxi: bool,
    #[serde(rename = "Multicrew")]
    multicrew: bool,
}

#[derive(Deserialize)]
pub struct ApproachSettlement{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "MarketID")]
    market_id: u64,
    #[serde(rename = "StationFaction")]
    station_faction: StationFaction,
    #[serde(rename = "StationGovernment")]
    station_government: Government,
    #[serde(rename = "StationGovernment_Localised")]
    station_government_localised: String,
    #[serde(rename = "StationServices")]
    station_services: Vec<StationService>,
    #[serde(rename = "StationEconomy")]
    station_economy: String,
    #[serde(rename = "StationEconomy_Localised")]
    station_economy_localised: String,
    #[serde(rename = "StationEconomies")]
    station_economies: Vec<StationEconomy>,
    #[serde(rename = "SystemAddress")]
    system_address: u64,
    #[serde(rename = "BodyID")]
    body_id: u64,
    #[serde(rename = "BodyName")]
    body_name: String,
    #[serde(rename = "Latitude")]
    latitude: f64,
    #[serde(rename = "Longitude")]
    longitude: f64,
}

#[derive(Deserialize)]
pub struct MaterialTrade{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "MarketID")]
    market_id: u64,
    #[serde(rename = "TraderType")]
    trader_type: MaterialCategory,
    #[serde(rename = "Paid")]
    paid: MaterialTraded,
    #[serde(rename = "Received")]
    received: MaterialTraded,
}

#[derive(Deserialize)]
pub struct Market{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "MarketID")]
    market_id: u64,
    #[serde(rename = "StationName")]
    station_name: String,
    #[serde(rename = "StationType")]
    station_type: StationType,
    #[serde(rename = "StarSystem")]
    star_system: String,
}