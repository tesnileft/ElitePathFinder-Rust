use crate::elite_journal_data::enums::exobiology::Species;
use crate::elite_journal_data::substructs::body_data::{BodySurfaceSignal, ExobioGenus};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use crate::elite_journal_data::enums::misc::MaterialCategory;

#[derive(Deserialize)]
pub struct ScanOrganic {
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "ScanType")]
    pub scan_type: String,
    #[serde(rename = "Genus")]
    pub genus: String,
    #[serde(rename = "Genus_Localised")]
    pub genus_localised: String,
    #[serde(rename = "Species")]
    pub species: Species,
    #[serde(rename = "Species_Localised")]
    pub species_localised: String,
    #[serde(rename = "Variant")]
    pub variant: String,
    #[serde(rename = "Variant_Localised")]
    pub variant_localised: String,
    #[serde(rename = "WasLogged")]
    pub was_logged: bool,
    #[serde(rename = "SystemAddress")]
    pub system_address: u64,
    #[serde(rename = "Body")]
    pub body_id: u64,
}

#[derive(Deserialize)]
pub struct CodexEntry {
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "EntryID")]
    pub entry_id: u64,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Name_Localised")]
    pub name_localised: String,
    #[serde(rename = "SubCategory")]
    pub sub_category: String,
    #[serde(rename = "SubCategory_Localised")]
    pub sub_category_localised: String,
    #[serde(rename = "Category")]
    pub category: String,
    #[serde(rename = "Category_Localised")]
    pub category_localised: String,
    #[serde(rename = "Region")]
    pub region: String,
    #[serde(rename = "Region_Localised")]
    pub region_localised: String,
    #[serde(rename = "System")]
    pub star_system: String,
    #[serde(rename = "SystemAddress")]
    pub system_address: u64,
    #[serde(rename = "BodyID")]
    pub body_id: u64,
    #[serde(rename = "NearestDestination")]
    pub nearest_destination: Option<String>,
    #[serde(rename = "Latitude")]
    pub latitude: Option<f64>,
    #[serde(rename = "Longitude")]
    pub longitude: Option<f64>,
    #[serde(rename = "IsNewEntry")]
    pub is_new_entry: Option<bool>,
}

#[derive(Deserialize)]
pub struct SAAScanComplete {
    //Surface Scan
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "BodyName")]
    pub body_name: String,
    #[serde(rename = "SystemAddress")]
    pub system_address: u64,
    #[serde(rename = "BodyID")]
    pub body_id: u64,
    #[serde(rename = "ProbesUsed")]
    pub probes_used: u64,
    #[serde(rename = "EfficiencyTarget")]
    pub efficiency_target: f64,
}
#[derive(Deserialize)]
///Surface scan signal results (Bio, geo)
pub struct SAASignalsFound {
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "BodyName")]
    pub body_name: String,
    #[serde(rename = "SystemAddress")]
    pub system_address: u64,
    #[serde(rename = "BodyID")]
    pub body_id: u64,
    #[serde(rename = "Signals")]
    pub signals: Vec<BodySurfaceSignal>,
    ///Potentially empty
    #[serde(rename = "Genuses")]
    pub genuses: Vec<ExobioGenus>,
}

#[derive(Deserialize)]
pub struct MaterialDiscovered{
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "Category")]
    category: MaterialCategory,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
    #[serde(rename = "DiscoveryNumber")]
    discovery_number: u64,
}

#[derive(Deserialize)]
pub struct DatalinkScan {
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "Message")]
    pub message: DatalinkScanMessage,
    #[serde(rename = "Message_Localised")]
    pub message_localised: String
}

#[derive(Deserialize)]
pub enum DatalinkScanMessage{
    #[serde(rename = "$DATAPOINT_GAMEPLAY_complete;")]
    DataLinkScanComplete,
}
#[derive(Deserialize)]
pub struct NavBeaconScan{
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "SystemAddress")]
    pub system_address: u64,
    #[serde(rename = "NumBodies")]
    pub num_bodies: u64,
}