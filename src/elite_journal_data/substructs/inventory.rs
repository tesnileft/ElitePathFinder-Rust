use crate::elite_journal_data::enums::body_data::RawMaterial;
use serde::Deserialize;

//region - Inventory Items -
#[derive(Deserialize)]
pub struct Item {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
    #[serde(rename = "OwnerID")]
    owner_id: u32,
    #[serde(rename = "Count")]
    count: u32,
}

#[derive(Deserialize)]
pub struct RawMaterialInventory {
    #[serde(rename = "Name")]
    name: RawMaterial,
    #[serde(rename = "Count")]
    count: u32,
}

#[derive(Deserialize)]
pub struct LocalisedMaterialInventory {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: String,
    #[serde(rename = "Count")]
    count: u32,
}

#[derive(Deserialize)]
pub struct Consumable{
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: String,
    #[serde(rename = "Count")]
    count: u64
}

#[derive(Deserialize)]
pub struct Data{
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: String,
    #[serde(rename = "Count")]
    count: u64
}

#[derive(Deserialize)]
pub struct Materials {
    #[serde(rename = "Raw")]
    raw: Vec<RawMaterial>,
    #[serde(rename = "Manufactured")]
    manufactured: Vec<LocalisedMaterialInventory>,
    #[serde(rename = "Encoded")]
    encoded: Vec<LocalisedMaterialInventory>,
}

#[derive(Deserialize)]
pub struct Component{
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Count")]
    count: u64
}

#[derive(Deserialize)]
pub struct SuitModule{
    #[serde(rename = "SlotName")]
    slot_name: String,
    #[serde(rename = "SuitModuleID")]
    suit_module_id: u64,
    #[serde(rename = "ModuleName")]
    module_name: String,
    #[serde(rename = "ModuleName_Localised")]
    module_name_localised: String,
    #[serde(rename = "Class")]
    class: u64,
    #[serde(rename = "WeaponMods")]
    weapon_mods: Vec<String>
}