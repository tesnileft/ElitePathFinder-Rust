use crate::elite_journal_data::enums::vessels::{ShipType, SlotType};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use crate::elite_journal_data::substructs::inventory::{MaterialInventory};
use crate::elite_journal_data::substructs::ship::{FuelCapacity, Module};
use crate::elite_journal_data::substructs::station_data::StoredShip;

#[derive(Deserialize)]
pub struct Loadout {
    timestamp: DateTime<Utc>,
    #[serde(rename = "Ship")]
    ship: String,
    #[serde(rename = "ShipID")]
    ship_id: u64,
    #[serde(rename = "ShipName")]
    ship_name: String,
    #[serde(rename = "ShipIdent")]
    ship_ident: String,
    #[serde(rename = "HullValue")]
    hull_value: u64,
    #[serde(rename = "HullHealth")]
    hull_health: f64,
    #[serde(rename = "UnladenMass")]
    unladen_mass: f64,
    #[serde(rename = "CargoCapacity")]
    cargo_capacity: u64,
    #[serde(rename = "MaxJumpRange")]
    max_jump_range: f64,
    #[serde(rename = "FuelCapacity")]
    fuel_capacity: FuelCapacity,
    #[serde(rename = "Rebuy")]
    rebuy: u64,
    #[serde(rename = "Modules")]
    modules: Vec<Module>,
}

//endregion
//region - Station Features -
#[derive(Deserialize)]
pub struct RefuelAll {
    timestamp: DateTime<Utc>,
    #[serde(rename = "Cost")]
    cost: u64,
    #[serde(rename = "Amount")]
    amount: f64,
}

#[derive(Deserialize)]
pub struct Shipyard {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "MarketID")]
    market_id: u64,
    #[serde(rename = "StationName")]
    station_name: String,
    #[serde(rename = "StarSystem")]
    star_system: String, //This is actually the body name
}

#[derive(Deserialize)]
pub struct StoredShips {
    timestamp: DateTime<Utc>,
    #[serde(rename = "StationName")]
    station_name: String,
    #[serde(rename = "MarketID")]
    market_id: u64,
    #[serde(rename = "StarSystem")]
    star_system: Option<String>,
    #[serde(rename = "ShipsHere")]
    ships_here: Vec<StoredShip>,
}

#[derive(Deserialize)]
pub struct ShipyardTransfer {
    timestamp: DateTime<Utc>,
    #[serde(rename = "ShipType")]
    ship_type: ShipType,
    #[serde(rename = "ShipID")]
    ship_id: u64,
    #[serde(rename = "System")]
    system_name: String,
    #[serde(rename = "ShipMarketID")]
    ship_market_id: u64,
    #[serde(rename = "Distance")]
    distance: f64,
    #[serde(rename = "TransferPrice")]
    transfer_price: u64,
    #[serde(rename = "TransferTime")]
    transfer_time: u64,
    #[serde(rename = "MarketID")]
    market_id: u64,
}

#[derive(Deserialize)]
pub struct ShipyardSwap {
    timestamp: DateTime<Utc>,
    #[serde(rename = "ShipType")]
    ship_type: ShipType,
    #[serde(rename = "ShipType_Localised")]
    ship_type_localised: String,
    #[serde(rename = "ShipID")]
    ship_id: u64,
    #[serde(rename = "StoreOldShip")]
    store_old_ship: ShipType,
    #[serde(rename = "StoreShipID")]
    store_ship_id: u64,
    #[serde(rename = "MarketID")]
    market_id: u64,
}

#[derive(Deserialize)]
pub struct Repair{
    timestamp: DateTime<Utc>,
    #[serde(rename = "Items")]
    items: Vec<String>,
    #[serde(rename="Cost")]
    cost: u64
}
#[derive(Deserialize)]
pub struct RepairAll{
    timestamp: DateTime<Utc>,
    #[serde(rename="Cost")]
    cost: u64
}
#[derive(Deserialize)]
pub struct StoredModules{
    timestamp: DateTime<Utc>,
    #[serde(rename="MarketID")]
    market_id: u64,
    #[serde(rename="StationName")]
    station_name: String,
    #[serde(rename = "StarSystem")]
    star_system: String,
    #[serde(rename = "Items")]
    items: Vec<StoredModule>,
}
#[derive(Deserialize)]
pub struct StoredModule{
    #[serde(rename="Name")]
    name: String,
    #[serde(rename= "Name_Localised")]
    name_localised: String,
    #[serde(rename= "StorageSlot")]
    storage_slot: u64,
    #[serde(rename= "StarSystem")]
    star_system: String,
    #[serde(rename= "MarketID")]
    market_id: u64,
    #[serde(rename= "TransferCost")]
    transfer_cost: u64,
    #[serde(rename= "TransferTime")]
    transfer_time: u64,
    #[serde(rename= "BuyPrice")]
    buy_price: u64,
    #[serde(rename= "Hot")]
    hot: bool,
    #[serde(rename= "EngineerModifications")]
    engineer_modifications: Option<String>,
    #[serde(rename= "Level")]
    level: Option<u64>,
    #[serde(rename= "Quality")]
    quality: Option<f64>,
}
#[derive(Deserialize)]
pub struct EngineerCraft{
    timestamp: DateTime<Utc>,
    #[serde(rename = "Slot")]
    slot: SlotType,
    #[serde(rename = "Module")]
    module: String,
    #[serde(rename = "Ingredients")]
    ingredients: Vec<MaterialInventory>
}

#[derive(Deserialize)]
pub struct Outfitting {
    timestamp: DateTime<Utc>,
    #[serde(rename = "MarketID")]
    market_id: u64,
    #[serde(rename = "StationName")]
    station_name: String,
    #[serde(rename = "StarSystem")]
    star_system: Option<String>,
}
