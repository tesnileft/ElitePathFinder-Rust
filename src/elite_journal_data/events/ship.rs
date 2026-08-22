use crate::elite_journal_data::enums::misc::{LegalStatus, PilotRank, ScanType};
use crate::elite_journal_data::enums::system_data::PowerplayPower;
use crate::elite_journal_data::enums::vessels::{LimpetType, SRVType, ShipType};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ShipTargeted {
    timestamp: DateTime<Utc>,
    #[serde(rename = "TargetLocked")]
    target_locked: bool,
    #[serde(rename = "Ship")]
    ship: Option<ShipType>,
    #[serde(rename = "Ship_Localised")]
    ship_localised: Option<String>,
    ///Indicates available information in scan
    #[serde(rename = "ScanStage")]
    scan_stage: Option<u64>,
    #[serde(rename = "PilotName")]
    pilot_name: Option<String>,
    #[serde(rename = "PilotName_Localised")]
    pilot_localised: Option<String>,
    #[serde(rename = "PilotRank")]
    pilot_rank: Option<PilotRank>,
    #[serde(rename = "SquadronID")]
    squadron_id: Option<String>,
    #[serde(rename = "ShieldHealth")]
    shield_health: Option<f64>,
    #[serde(rename = "HullHealth")]
    hull_health: Option<f64>,
    #[serde(rename = "LegalStatus")]
    legal_status: Option<LegalStatus>,
    #[serde(rename = "Power")]
    power: Option<PowerplayPower>,
}

#[derive(Deserialize)]
pub struct DockSRV {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "SRVType")]
    srv_type: SRVType,
    #[serde(rename = "SRVType_Localised")]
    srv_type_localised: String,
    #[serde(rename = "ID")]
    id: u64,
}

#[derive(Deserialize)]
pub struct LaunchFighter {
    timestamp: DateTime<Utc>,
    #[serde(rename = "Loadout")]
    loadout: String,
    #[serde(rename = "ID")]
    id: u64,
    #[serde(rename = "PlayerControlled")]
    player_controlled: bool,
}

#[derive(Deserialize)]
pub struct ModuleInfo {
    timestamp: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct ShieldState {
    timestamp: DateTime<Utc>,
    #[serde(rename="ShieldsUp")]
    shields_up: bool
}

#[derive(Deserialize)]
pub struct UnderAttack {
    timestamp: DateTime<Utc>,
    #[serde(rename = "Target")]
    target: String,
}
#[derive(Deserialize)]
pub struct Scanned {
    timestamp: DateTime<Utc>,
    #[serde(rename= "ScanType")]
    scan_type: ScanType,
}

#[derive(Deserialize)]
pub struct EjectCargo{
    timestamp: DateTime<Utc>,
    #[serde(rename= "Type")]
    cargo_type: String,
    #[serde(rename = "Type_Localised")]
    type_localised: String,
    #[serde(rename = "Count")]
    count: u64,
    #[serde(rename = "Abandoned")]
    abandoned: bool,
}

#[derive(Deserialize)]
pub struct LaunchSRV{
    timestamp: DateTime<Utc>,
    #[serde(rename = "SRVType")]
    srv_type: SRVType,
    #[serde(rename = "SRVType_Localised")]
    srv_type_localised: String,
    #[serde(rename = "Loadout")]
    loadout: String,
    #[serde(rename = "ID")]
    id: u64,
    #[serde(rename = "PlayerControlled")]
    player_controlled: bool,
}

#[derive(Deserialize)]
pub struct LaunchDrone{
    timestamp: DateTime<Utc>,
    #[serde(rename = "Type")]
    limpet_type: LimpetType
}
#[derive(Deserialize)]
pub struct RepairDrone{
    timestamp: DateTime<Utc>,
    #[serde(rename = "HullRepaired")]
    hull_repaired: f64,
    #[serde(rename = "CockpitRepaired")]
    cockpit_repaired: f64
}