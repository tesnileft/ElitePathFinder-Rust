use crate::elite_journal_data::enums::body_data::{BodyType, StarClass};
use crate::elite_journal_data::enums::misc::{Allegiance, JumpType};
use crate::elite_journal_data::enums::system_data::{Economy, Government, PowerplayPower, PowerplayState, SystemSecurity};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use crate::elite_journal_data::substructs::factions::{Faction, SystemFaction};

//region - FSD -
#[derive(Deserialize)]
pub struct FSDTarget {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "SystemAddress")]
    system_address: u64,
    #[serde(rename = "StarClass")]
    star_class: String,
    #[serde(rename = "RemainingJumpsInRoute")]
    remaining_jumps_in_route: Option<u64>,
}

#[derive(Deserialize)]
pub struct StartJump {
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "JumpType")]
    pub jump_type: JumpType,
    #[serde(rename = "Taxi")]
    pub taxi: bool,
    ///Only available if the JumpType is `Hyperspace`
    #[serde(rename = "StarSystem")]
    pub star_system: Option<String>,
    ///Only available if the JumpType is `Hyperspace`
    #[serde(rename = "SystemAddress")]
    pub system_address: Option<u64>,
    ///Only available if the JumpType is `Hyperspace`
    #[serde(rename = "StarClass")]
    pub star_class: Option<StarClass>,
}

///Very large struct that covers almost everything about the system that you are currently jumping to
#[derive(Deserialize)]
pub struct FSDJump{
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "Taxi")]
    pub taxi: bool,
    #[serde(rename = "Multicrew")]
    pub multicrew: bool,
    #[serde(rename = "StarSystem")]
    pub star_system: String,
    #[serde(rename = "SystemAddress")]
    pub system_address: u64,
    #[serde(rename = "StarPos")]
    pub star_pos: (f64, f64, f64),
    #[serde(rename = "SystemAllegiance")]
    pub system_allegiance: Allegiance,
    #[serde(rename = "SystemEconomy")]
    pub system_economy: Economy,
    #[serde(rename = "SystemEconomy_Localised")]
    pub system_economy_localised: String,
    #[serde(rename = "SystemSecondEconomy")]
    pub system_second_economy: Economy,
    #[serde(rename = "SystemSecondEconomy_Localised")]
    pub system_second_economy_localised: String,
    #[serde(rename = "SystemGovernment")]
    pub system_government: Government,
    #[serde(rename = "SystemGovernment_Localised")]
    pub system_government_localised: String,
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
    #[serde(rename = "ControllingPower")]
    #[serde(default)]
    pub controlling_power: PowerplayPower,
    #[serde(rename = "Powers")]
    pub powers: Option<Vec<PowerplayPower>>,
    #[serde(rename = "PowerplayState")]
    #[serde(default)]
    pub powerplay_state: PowerplayState,
    #[serde(rename = "PowerplayStateControlProgress")]
    #[serde(default)]
    pub powerplay_state_control_progress: f64,
    #[serde(rename = "PowerplayStateReinforcement")]
    #[serde(default)]
    pub powerplay_state_reinforcement: u64,
    #[serde(rename = "PowerplayStateUndermining")]
    #[serde(default)]
    pub powerplay_state_undermining: u64,
    #[serde(rename = "JumpDist")]
    pub jump_dist: f64,
    #[serde(rename = "FuelUsed")]
    pub fuel_used: f64,
    #[serde(rename = "FuelLevel")]
    pub fuel_level: f64,
    #[serde(rename = "Factions")]
    #[serde(default)]
    pub factions: Vec<Faction>,
    #[serde(rename = "SystemFaction")]
    pub system_faction: Option<SystemFaction>,
}

#[derive(Deserialize)]
pub struct SupercruiseEntry {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Taxi")]
    taxi: bool,
    #[serde(rename = "Multicrew")]
    multicrew: bool,
    #[serde(rename = "StarSystem")]
    star_system: String,
    #[serde(rename = "SystemAddress")]
    system_address: u64,
}

#[derive(Deserialize)]
pub struct SupercruiseExit {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
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
    #[serde(rename = "BodyType")]
    body_type: BodyType,
}

#[derive(Deserialize)]
pub struct SupercruiseDestinationDrop {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Type")]
    type_: String,
    #[serde(rename = "Threat")]
    threat: u64,
    #[serde(rename = "MarketID")]
    market_id: Option<u64>,
}

#[derive(Deserialize)]
pub struct JetConeBoost {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "BoostValue")]
    boost_value: f64,
}