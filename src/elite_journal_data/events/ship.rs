use crate::elite_journal_data::enums::misc::{LegalStatus, PilotRank, ScanType};
use crate::elite_journal_data::enums::system_data::PowerplayPower;
use crate::elite_journal_data::enums::vessels::{SRVType, ShipType};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ShipTargeted {
    timestamp: DateTime<Utc>,
    #[serde(rename = "TargetLocked")]
    target_locked: bool,
    #[serde(rename = "Ship")]
    ship: ShipType,
    #[serde(rename = "Ship_Localised")]
    ship_localised: String,
    #[serde(rename = "ScanStage")]
    scan_stage: u64,
    #[serde(rename = "PilotName")]
    pilot_name: String,
    #[serde(rename = "PilotName_Localised")]
    pilot_localised: String,
    #[serde(rename = "PilotRank")]
    pilot_rank: PilotRank,
    #[serde(rename = "SquadronID")]
    squadron_id: String,
    #[serde(rename = "ShieldHealth")]
    shield_health: f64,
    #[serde(rename = "HullHealth")]
    hull_health: f64,
    #[serde(rename = "LegalStatus")]
    legal_status: LegalStatus,
    #[serde(rename = "Power")]
    power: PowerplayPower,
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