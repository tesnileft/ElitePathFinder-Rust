use crate::custom_structs::exobiospecies::ExoBiologySpecies;
use crate::custom_structs::starsystem::StarSystem;
use crate::custom_structs::system_info::{AtmosphereInfo, Star};
use crate::elite_journal_data::enums::body_data::{PlanetClass, Volcanism};

pub fn predict_shards(star_distance: f64, system: StarSystem, temperature: f64, parent_stars: &Vec<&Star>) -> Vec<ExoBiologySpecies> {
    let mut exobios = Vec::new();

    exobios
}