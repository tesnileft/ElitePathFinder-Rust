use crate::elite_journal_data::enums::body_data::RawMaterial;
use serde::Deserialize;
use crate::elite_journal_data::enums::cargo::CommodityType;
use crate::elite_journal_data::enums::misc::MaterialCategory;
use crate::elite_journal_data::enums::vessels::CarrierCrewRole;

//region - Inventory Items -
#[derive(Deserialize)]
pub struct Item {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
    #[serde(rename = "OwnerID")]
    owner_id: Option<u32>,
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

//Fleet carrier
#[derive(Deserialize)]
pub struct SpaceUsage{
    #[serde(rename = "TotalCapacity")]
    total_capacity: u64,
    #[serde(rename = "Crew")]
    crew: u64,
    #[serde(rename = "Cargo")]
    cargo: u64,
    #[serde(rename = "CargoSpaceReserved")]
    cargo_space_reserved: u64,
    #[serde(rename = "ShipPacks")]
    ship_packs: u64,
    #[serde(rename = "ModulePacks")]
    module_packs: u64,
    #[serde(rename = "FreeSpace")]
    free_space: u64,
}

#[derive(Deserialize)]
pub struct CarrierFinance{
    #[serde(rename = "CarrierBalance")]
    carrier_balance: u64,
    #[serde(rename = "ReserveBalance")]
    reserve_balance: u64,
    #[serde(rename = "AvailableBalance")]
    available_balance: u64,
    #[serde(rename = "ReservePercent")]
    reserve_percent: f64,
    #[serde(rename = "TaxRate")]
    tax_rate: Option<f64>,
    #[serde(rename = "TaxRate_Shipyard")]
    tax_rate_shipyard: Option<f64>,
    #[serde(rename = "TaxRate_Outfitting")]
    tax_rate_outfitting: Option<f64>,
    #[serde(rename = "TaxRate_Refuel")]
    tax_rate_refuel: Option<f64>,
    #[serde(rename = "TaxRate_Repair")]
    tax_rate_repair: Option<f64>,
    #[serde(rename = "TaxRate_Rearm")]
    tax_rate_rearm: Option<f64>,
    #[serde(rename = "TaxRate_pioneersupplies")]
    tax_rate_pioneer_supplies: Option<f64>,
}

#[derive(Deserialize)]
pub struct CarrierCrew{
    #[serde(rename = "CrewRole")]
    crew_role: CarrierCrewRole,
    #[serde(rename = "Activated")]
    activated: bool,
    #[serde(rename = "Enabled")]
    enabled: Option<bool>,
    #[serde(rename = "CrewName")]
    crew_name: Option<String>,
}

#[derive(Deserialize)]
pub struct CarrierPack{
    #[serde(rename = "PackTheme")]
    pack_theme: String,
    #[serde(rename = "PackTier")]
    pack_tier: u64,
}