use crate::custom_structs::exobiospecies::ExoBiologySpecies;
use crate::custom_structs::system_info::{AtmosphereInfo, Star};
use crate::elite_journal_data::enums::body_data::{AtmosphereType, PlanetClass, Volcanism};

pub fn predict(planet_class: &PlanetClass, atmosphere: &AtmosphereInfo, volcanism: Volcanism, temperature: f64, startypes: &Vec<&Star>) -> Vec<ExoBiologySpecies> {
    let mut exobios = Vec::new();

    exobios
}