use crate::elite_journal_data::enums::body_data::RawMaterial;
use crate::elite_journal_data::enums::exobiology::Genus;
use crate::elite_journal_data::enums::signals::SAASignalType;
use serde::Deserialize;

//endregion
//region - Planet Data -
#[derive(Deserialize)]
pub struct  AtmosphericGas {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Percent")]
    percent: f64,
}

#[derive(Deserialize)]
pub struct  BodyComposition {
    #[serde(rename = "Ice")]
    pub ice: f64,
    #[serde(rename = "Rock")]
    pub rock: f64,
    #[serde(rename = "Metal")]
    pub metal: f64,
}

#[derive(Deserialize)]
pub struct CelestialRings {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "RingClass")]
    pub ring_class: String,
    #[serde(rename = "MassMT")]
    pub mass_megatons: f64,
    #[serde(rename = "InnerRad")]
    pub inner_radius: f64,
    #[serde(rename = "OuterRad")]
    pub outer_radius: f64,
}

#[derive(Deserialize)]
pub struct RawMaterialInfo {
    #[serde(rename = "Name")]
    pub name: RawMaterial,
    #[serde(rename = "Name_Localised")]
    #[serde(default)]
    pub name_localized: String,
    #[serde(rename = "Percent")]
    pub percent: f64,
}

#[derive(Deserialize)]
pub struct BodySurfaceSignal {
    #[serde(rename = "Type")]
    pub type_: SAASignalType,
    #[serde(rename = "Type_Localised")]
    pub type_localised: String,
    #[serde(rename = "Count")]
    pub count: u64,
}

#[derive(Deserialize)]
pub struct ExobioGenus {
    ///Enum of the genus type
    #[serde(rename = "Genus")]
    pub genus: Genus,
    #[serde(rename = "Genus_Localised")]
    pub genus_localised: String,
}