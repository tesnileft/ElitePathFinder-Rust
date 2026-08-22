use crate::elite_journal_data::enums::body_data::BodyType;
use crate::elite_journal_data::enums::misc::Allegiance;
use crate::elite_journal_data::enums::station_data::{StationService, StationType};
use crate::elite_journal_data::enums::system_data::{Economy, Government, PowerplayPower, PowerplayState, SystemSecurity};
use crate::elite_journal_data::substructs::factions::{Faction, SystemFaction};
use crate::elite_journal_data::substructs::station_data::{Engineer, StationEconomy};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Rank {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Combat")]
    combat: u64,
    #[serde(rename = "Trade")]
    trade: u64,
    #[serde(rename = "Explore")]
    explorer: u64,
    #[serde(rename = "Soldier")]
    mercenary: u64,
    #[serde(rename = "Exobiologist")]
    exobiologist: u64,
    #[serde(rename = "CQC")]
    cqc: u64,
    #[serde(rename = "Empire")]
    empire: u64,
    #[serde(rename = "Federation")]
    federation: u64,
}

#[derive(Deserialize)]
pub struct Progress {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Combat")]
    combat: u64,
    #[serde(rename = "Trade")]
    trade: u64,
    #[serde(rename = "Explore")]
    explorer: u64,
    #[serde(rename = "Soldier")]
    mercenary: u64,
    #[serde(rename = "Exobiologist")]
    exobiologist: u64,
    #[serde(rename = "CQC")]
    cqc: u64,
    #[serde(rename = "Empire")]
    empire: u64,
    #[serde(rename = "Federation")]
    federation: u64,
}

#[derive(Deserialize)]
pub struct Reputation {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Empire")]
    empire: f64,
    #[serde(rename = "Federation")]
    federation: f64,
    #[serde(rename = "Independent")]
    independent: f64,
    #[serde(rename = "Alliance")]
    alliance: f64,
}

#[derive(Deserialize)]
pub struct EngineerProgress {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Engineers")]
    engineers: Vec<Engineer>,
}

#[derive(Deserialize)]
pub struct Location {
    //
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "DistFromStarLS")]
    distance_from_star_ls: Option<f64>,
    #[serde(rename = "Docked")]
    docked: bool,
    #[serde(rename = "StationName")]
    station_name: Option<String>,
    #[serde(rename = "StationType")]
    #[serde(default)]
    station_type: StationType,
    #[serde(rename = "MarketID")]
    market_id: Option<u64>,
    #[serde(rename = "StationFaction")]
    station_faction: Option<SystemFaction>,
    #[serde(rename = "StationGovernment")]
    #[serde(default)]
    station_government: Government,
    #[serde(rename = "StationGovernment_Localised")]
    station_government_localized: Option<String>,
    #[serde(rename = "StationServices")]
    station_services: Option<Vec<StationService>>,
    #[serde(rename = "StationEconomy")]
    station_economy: Option<Economy>,
    #[serde(rename = "StationEconomy_Localised")]
    station_economy_localised: Option<String>,
    #[serde(rename = "StationAllegiance")]
    station_allegiance: Option<Allegiance>,
    #[serde(rename = "StationEconomies")]
    station_economies: Option<Vec<StationEconomy>>,
    #[serde(rename = "Taxi")]
    taxi: Option<bool>,
    #[serde(rename = "Multicrew")]
    multicrew: Option<bool>,
    #[serde(rename = "StarSystem")]
    star_system: String,
    #[serde(rename = "SystemAddress")]
    system_address: u64,
    #[serde(rename = "StarPos")]
    pub star_pos: (f64, f64, f64),
    #[serde(rename = "SystemAllegiance")]
    system_allegiance: Option<Allegiance>,
    #[serde(rename = "SystemEconomy")]
    system_economy: Economy,
    #[serde(rename = "SystemEconomy_Localised")]
    system_economy_localised: String,
    #[serde(rename = "SystemSecondEconomy")]
    pub system_second_economy: Economy,
    #[serde(rename = "SystemSecondEconomy_Localised")]
    pub system_second_economy_localised: String,
    #[serde(rename = "SystemGovernment")]
    pub system_government: Government,
    #[serde(rename = "SystemGovernment_Localised")]
    pub system_government_localised: Option<String>,
    #[serde(rename = "SystemSecurity")]
    pub system_security: SystemSecurity,
    #[serde(rename = "SystemSecurity_Localised")]
    pub system_security_localised: String,
    #[serde(rename = "Population")]
    pub population: u64,
    #[serde(rename = "Body")]
    pub body: String,
    #[serde(rename = "BodyID")]
    pub body_id: u64,
    #[serde(rename = "BodyType")]
    pub body_type: BodyType,
    #[serde(rename = "Powers")]
    pub powers: Option<Vec<PowerplayPower>>,
    #[serde(rename = "PowerplayState")]
    pub powerplay_state: Option<PowerplayState>,
    #[serde(rename = "PowerplayStateControlProgress")]
    pub powerplay_control_progress: Option<f64>,
    #[serde(rename = "PowerplayStateReinforcement")]
    pub powerplay_reinforcement: Option<f64>,
    #[serde(rename = "PowerplayStateUndermining")]
    pub powerplay_undermining: Option<f64>,
    #[serde(rename = "Factions")]
    pub factions: Option<Vec<Faction>>,
    #[serde(rename = "SystemFaction")]
    pub system_faction: Option<SystemFaction>,
}

#[derive(Deserialize)]
pub struct Powerplay {
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "Power")]
    pub power: PowerplayPower,
    #[serde(rename = "Rank")]
    pub rank: u64,
    #[serde(rename = "Merits")]
    pub merits: u64,
    #[serde(rename = "TimePledged")]
    pub time_pledged: u64,
}
#[derive(Deserialize)]
pub struct Commander {
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "FID")]
    pub fid: String,
    #[serde(rename = "Name")]
    pub name: String,
}
#[derive(Deserialize)]
pub struct PayFines{
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "Amount")]
    amount: u64,
    #[serde(rename = "AllFines")]
    all_fines: bool,
    #[serde(rename = "Faction")]
    faction: String,
    #[serde(rename = "ShipID")]
    ship_id: u64,
    #[serde(rename = "BrokerPercentage")]
    broker_percentage: f64,
}