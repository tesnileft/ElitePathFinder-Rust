use crate::elite_journal_data::enums::game_data::GameMode;
use crate::elite_journal_data::substructs::statistics::*;
use chrono::{DateTime, Utc};
use serde::Deserialize;

//endregion
//region - Misc -
#[derive(Deserialize)]
pub struct Statistics {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Bank_Account")]
    bankaccount: BankAccountStats,
    #[serde(rename = "Combat")]
    combat: CombatStats,
    #[serde(rename = "Crime")]
    crime: CrimeStats,
    #[serde(rename = "Trading")]
    trading: TradingStats,
    #[serde(rename = "Smuggling")]
    smuggling: SmugglingStats,
    #[serde(rename = "Mining")]
    mining: MiningStats,
    #[serde(rename = "Exploration")]
    exploration: ExplorationStats,
    #[serde(rename = "Passengers")]
    passengers: PassengersStats,
    #[serde(rename = "Search_And_Rescue")]
    search_and_recue: SearchAndRecueStats,
    #[serde(rename = "Squadron")]
    squadron: SquadronStats,
    #[serde(rename = "Crafting")]
    crafting: CraftingStats,
    #[serde(rename = "Crew")]
    crew: CrewStats,
    #[serde(rename = "Material_Trader_Stats")]
    material_trader_stats: MaterialTraderStats,
    #[serde(rename = "Exobiology")]
    exobiology: ExobiologyStats,
}

#[derive(Deserialize)]
pub struct Music {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "MusicTrack")]
    pub(crate) music_track: String,
}

#[derive(Deserialize)]
pub struct NavRoute {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct NavRouteClear {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct LoadGame {
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "FID")]
    pub fid: String, //Frontier ID
    #[serde(rename = "Commander")]
    pub commander: String,
    #[serde(rename = "Horizons")]
    pub has_horizons: bool,
    #[serde(rename = "Odyssey")]
    pub has_odyssey: bool,
    #[serde(rename = "Ship")]
    pub ship: String,
    #[serde(rename = "ShipLocalised")]
    pub ship_localised: Option<String>,
    #[serde(rename = "ShipID")]
    pub ship_id: u64,
    #[serde(rename = "ShipName")]
    pub ship_name: String,
    #[serde(rename = "ShipIdent")]
    pub ship_identity: String,
    #[serde(rename = "FuelLevel")]
    pub fuel_level: f32,
    #[serde(rename = "FuelCapacity")]
    pub fuel_capacity: f32,
    #[serde(rename = "GameMode")]
    pub game_mode: GameMode,
    #[serde(rename = "Group")]
    pub group: String,
    #[serde(rename = "Credits")]
    pub credits: u64,
    #[serde(rename = "Loan")]
    pub loan: u64,
    pub language: String,
    pub gameversion: String,
    pub build: String,
}

#[derive(Deserialize)]
pub struct Shutdown {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
}

//region - Logging Specific -
#[derive(Deserialize)]
pub struct FileHeader {
    pub part: u64,
    pub language: String,
    #[serde(rename = "Odyssey")]
    pub odyssey: bool,
    pub gameversion: String,
    pub build: String,
}

