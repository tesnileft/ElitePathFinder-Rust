use crate::elite_events::enums::{AtmosphereType, PlanetClass, StarClass, Volcanism};
use crate::elite_events::substructs::{AtmosphericGas, BodyComposition};
use crate::{ExoBiologySpecies, GeologicalThings};
use crate::custom_structs::materials::PlanetRawMaterial;

pub enum Body{
    Star(Star),
    Planet(Planet),
}
pub struct Star{
    body_name: String,
    body_id: u64,
    class: StarClass,
}
#[derive(Default)]
pub struct Planet {
    pub body_name: String,
    pub body_id: u64,
    pub system_address: u64,
    pub planet_class: Option<PlanetClass>,
    pub gravity: Option<f64>,
    pub mean_temperature: Option<f64>,
    pub volcanism: Option<Volcanism>,
    pub atmosphere_type: Option<AtmosphereType>,
    pub atmosphere_composition: Option<Vec<AtmosphericGas>>,
    pub biological_signals: Option<u32>,
    pub potential_species: Option<Vec<ExoBiologySpecies>>,
    pub confirmed_species: Vec<ExoBiologySpecies>,
    pub geological_signals: Option<u32>,
    pub confirmed_geology: Option<Vec<GeologicalThings>>,
    pub body_composition: Option<BodyComposition>,
    pub materials: Option<Vec<PlanetRawMaterial>>
}
impl Planet {
    pub fn new(body_name: String, body_id: u64, system_address: u64) -> Planet {
        Planet {
            body_name,
            body_id,
            system_address,
            ..Default::default()
        }
    }

}