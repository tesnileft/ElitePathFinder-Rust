use crate::custom_structs::exobiospecies::ExoBiologySpecies;
use crate::custom_structs::materials::PlanetRawMaterial;
use crate::custom_structs::system_info::{AtmosphereInfo, Star};
use crate::elite_journal_data::enums::body_data::{AtmosphereType, LuminosityClass, PlanetClass, StarClass, Volcanism};
use crate::elite_journal_data::enums::body_data::LuminosityClass::V;
use crate::elite_journal_data::enums::body_data::PlanetClass::Icy;
use crate::elite_journal_data::enums::body_data::RawMaterial::*;
use crate::elite_journal_data::enums::exobiology::ExoBiologyVariant;
use crate::elite_journal_data::enums::exobiology::Genus::Electricae;
use crate::elite_journal_data::enums::exobiology::Species::{Pluma, Radialem};
use crate::exobiology_analysis::determine_species::check_material;

pub fn predict(planet_class: &PlanetClass, atmosphere: &AtmosphereInfo, materials: &Vec<PlanetRawMaterial>, parent_stars: &Vec<&Star>) -> Vec<ExoBiologySpecies> {
    let mut exobios = Vec::new();
    if matches!(atmosphere.atmosphere_type, AtmosphereType::Helium | AtmosphereType::Neon | AtmosphereType::Argon) && matches!(planet_class, Icy){
        if parent_stars.into_iter().any( |star|  matches!(star.class, StarClass::H | StarClass::SupermassiveBlackHole | StarClass::D | StarClass::DA | StarClass::N))
        || parent_stars.into_iter().any(|star| matches!(star.class, StarClass::A) && star.luminosity >= LuminosityClass::V)
        {
            let variant = match () {
                _ if check_material(materials, Antimony) => ExoBiologyVariant::Cobalt,
                _ if check_material(materials, Polonium) => ExoBiologyVariant::Cyan,
                _ if check_material(materials, Ruthenium) => ExoBiologyVariant::Blue,
                _ if check_material(materials, Technetium) => ExoBiologyVariant::Magenta,
                _ if check_material(materials, Tellurium) => ExoBiologyVariant::Red,
                _ if check_material(materials, Yttrium) => ExoBiologyVariant::Mulberry,
                _ => ExoBiologyVariant::Unknown,
            };
            exobios.push(ExoBiologySpecies{
                genus: Electricae,
                species: Pluma,
                variants: vec![variant]
            })
        }
    }
    if matches!(atmosphere.atmosphere_type, AtmosphereType::Helium | AtmosphereType::Neon | AtmosphereType::Argon | AtmosphereType::Nitrogen) && matches!(planet_class, Icy){
            //Todo determine proximity to a nebula
        let variant = match () {
            _ if check_material(materials, Antimony) => ExoBiologyVariant::Cyan,
            _ if check_material(materials, Polonium) => ExoBiologyVariant::Cobalt,
            _ if check_material(materials, Ruthenium) => ExoBiologyVariant::Blue,
            _ if check_material(materials, Technetium) => ExoBiologyVariant::Aquamarine,
            _ if check_material(materials, Tellurium) => ExoBiologyVariant::Magenta,
            _ if check_material(materials, Yttrium) => ExoBiologyVariant::Green,
            _ => ExoBiologyVariant::Unknown,
        };
        exobios.push(ExoBiologySpecies{
            genus: Electricae,
            species: Radialem,
            variants: vec![variant]
        });
    }
    exobios
}