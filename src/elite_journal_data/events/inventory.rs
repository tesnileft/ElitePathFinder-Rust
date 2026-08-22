use crate::elite_journal_data::substructs::inventory::{Component, Consumable, Data, Item, MaterialInventory, SuitModule};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use crate::elite_journal_data::enums::cargo::CargoType;
use crate::elite_journal_data::enums::misc::MaterialCategory;
use crate::elite_journal_data::substructs::ship::CargoItem;

//endregion
//region - Inventory -
#[derive(Deserialize)]
pub struct ShipLocker {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Items")]
    items: Option<Vec<Item>>,
}

#[derive(Deserialize)]
pub struct Cargo {
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "Vessel")]
    pub vessel: String,
    #[serde(rename = "Count")]
    pub count: u64,
    #[serde(rename = "Inventory")]
    pub inventory: Option<Vec<CargoItem>>,
}

#[derive(Deserialize)]
pub struct Backpack {
    timestamp: DateTime<Utc>,
    #[serde(rename = "Items")]
    items: Vec<Item>,
    #[serde(rename = "Components")]
    components: Vec<Component>,
    #[serde(rename = "Consumables")]
    consumables: Vec<Consumable>,
    #[serde(rename = "Data")]
    data: Vec<Data>,
}

#[derive(Deserialize)]
pub struct Materials {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Raw")]
    raw: Vec<MaterialInventory>,
    #[serde(rename = "Manufactured")]
    manufactured: Vec<MaterialInventory>,
    #[serde(rename = "Encoded")]
    encoded: Vec<MaterialInventory>,
}
#[derive(Deserialize)]
pub struct MaterialCollected{
    timestamp: DateTime<Utc>,
    #[serde(rename = "Category")]
    category: MaterialCategory,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
    #[serde(rename = "Count")]
    count: u64,
}


#[derive(Deserialize)]
pub struct ReservoirReplenished {
    timestamp: DateTime<Utc>,
    #[serde(rename = "FuelMain")]
    fuel_main: f64,
    #[serde(rename = "FuelReservoir")]
    fuel_reservoir: f64,
}

#[derive(Deserialize)]
pub struct FuelScoop {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Scooped")]
    scooped: f64,
    #[serde(rename = "Total")]
    total: f64,
}

#[derive(Deserialize)]
pub struct SuitLoadout {
    timestamp: DateTime<Utc>,
    #[serde(rename = "SuitID")]
    suit_id: u64,
    #[serde(rename = "SuitName")]
    suit_name: String,
    #[serde(rename = "SuitName_Localised")]
    suit_name_localised: String,
    #[serde(rename = "SuitMods")]
    suit_mods: Vec<String>,
    #[serde(rename = "LoadoutID")]
    loadout_id: u64,
    #[serde(rename = "LoadoutName")]
    loadout_name: String,
    #[serde(rename = "Modules")]
    modules: Vec<SuitModule>, // Actually weapons
}


#[derive(Deserialize)]
pub struct CollectCargo{
    timestamp: DateTime<Utc>,
    #[serde(rename = "Type")]
    cargo_type: CargoType,
    #[serde(rename = "Type_Localised")]
    cargo_localised: String,
    #[serde(rename = "Stolen")]
    stolen: bool
}