use crate::elite_journal_data::enums::body_data::RawMaterial;
use serde::Deserialize;
use crate::elite_journal_data::enums::misc::MaterialCategory;

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
pub struct MaterialInventory {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
    #[serde(rename = "Count")]
    count: u32,
}

#[derive(Deserialize)]
pub struct MaterialTraded{
    #[serde(rename = "Material")]
    material: String,
    #[serde(rename = "Material_Localised")]
    material_localised: Option<String>,
    #[serde(rename = "Category")]
    category: MaterialCategory,
    #[serde(rename = "Quantity")]
    quantity: u32,
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
    manufactured: Vec<MaterialInventory>,
    #[serde(rename = "Encoded")]
    encoded: Vec<MaterialInventory>,
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