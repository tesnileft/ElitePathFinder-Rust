use crate::custom_structs::exobiospecies::ExoBiologySpecies;
use crate::custom_structs::materials::PlanetRawMaterial;
use crate::custom_structs::system_info::Star;
use crate::elite_journal_data::enums::body_data::{AtmosphereType, StarClass};
use crate::elite_journal_data::enums::body_data::RawMaterial::{Antimony, Cadmium, Mercury, Molybdenum, Niobium, Polonium, Ruthenium, Tellurium, Tin, Tungsten};
use crate::elite_journal_data::enums::exobiology::Genus::Concha;
use crate::elite_journal_data::enums::exobiology::{ExoBiologyVariant, Species};
use crate::exobiology_analysis::determine_species::check_material;

fn concha_stellar_variant(class: &StarClass) -> ExoBiologyVariant {
    match class {
        StarClass::B => ExoBiologyVariant::Indigo,
        StarClass::A => ExoBiologyVariant::Teal,
        StarClass::F => ExoBiologyVariant::Grey,
        StarClass::G => ExoBiologyVariant::Turquoise,
        StarClass::K => ExoBiologyVariant::Red,
        StarClass::L => ExoBiologyVariant::Orange,
        StarClass::Y => ExoBiologyVariant::Yellow,
        StarClass::N => ExoBiologyVariant::Emerald,
        StarClass::W | StarClass::WN | StarClass::WNC | StarClass::WC | StarClass::WO => {
            ExoBiologyVariant::Lime
        }
        _ => ExoBiologyVariant::Unknown,
    }
}
pub fn predict_concha(atmosphere_type: &AtmosphereType, materials: & Vec<PlanetRawMaterial>, stars: & Vec<&Star>, mean_temperature: f64) -> Vec<ExoBiologySpecies>{
    let mut exobios = Vec::new();
    match atmosphere_type {
        AtmosphereType::Ammonia => {
            //Aureolas
                exobios.push(ExoBiologySpecies {
                    genus: Concha,
                    species: Species::Aureolas,
                    variants: stars.iter().map(|star| concha_stellar_variant(&star.class)).collect(),
                });
        }
        AtmosphereType::Nitrogen => {
            //Biconcavis
                let variant = match () {
                    _ if check_material(materials, Antimony) => {
                        ExoBiologyVariant::Peach
                    }
                    _ if check_material(materials, Polonium) => ExoBiologyVariant::Red,
                    _ if check_material(materials, Ruthenium) => {
                        ExoBiologyVariant::Orange
                    }
                    _ if check_material(materials, Tellurium) => {
                        ExoBiologyVariant::Yellow
                    }
                    _ => ExoBiologyVariant::Unknown,
                };
                exobios.push(ExoBiologySpecies {
                    genus: Concha,
                    species: Species::Biconcavis,
                    variants: vec![variant],
                });
        }
        AtmosphereType::CarbonDioxide | AtmosphereType::CarbonDioxideRich => {
            //Labiata - mean temperature below 190 K
            if mean_temperature < 190.0 {
                    exobios.push(ExoBiologySpecies {
                        genus: Concha,
                        species: Species::Labiata,
                        variants: stars.iter().map(|star| concha_stellar_variant(&star.class)).collect(),
                    });
            }
            //Renibus - mean temperature between 180 K and 195 K
            if (180.0..=195.0).contains(&mean_temperature)
            {
                    let variant = match () {
                        _ if check_material(materials, Cadmium) => {
                            ExoBiologyVariant::Red
                        }
                        _ if check_material(materials, Mercury) => {
                            ExoBiologyVariant::Mulberry
                        }
                        _ if check_material(materials, Molybdenum) => {
                            ExoBiologyVariant::Peach
                        }
                        _ if check_material(materials, Niobium) => {
                            ExoBiologyVariant::Blue
                        }
                        _ if check_material(materials, Tungsten) => {
                            ExoBiologyVariant::White
                        }
                        _ if check_material(materials, Tin) => {
                            ExoBiologyVariant::Aquamarine
                        }
                        _ => ExoBiologyVariant::Unknown,
                    };
                    exobios.push(ExoBiologySpecies {
                        genus: Concha,
                        species: Species::Renibus,
                        variants: vec![variant],
                    });
            }
        }
        AtmosphereType::Water | AtmosphereType::WaterRich => {
            //Renibus
                let variant = match () {
                    _ if check_material(materials, Cadmium) => ExoBiologyVariant::Red,
                    _ if check_material(materials, Mercury) => {
                        ExoBiologyVariant::Mulberry
                    }
                    _ if check_material(materials, Molybdenum) => {
                        ExoBiologyVariant::Peach
                    }
                    _ if check_material(materials, Niobium) => ExoBiologyVariant::Blue,
                    _ if check_material(materials, Tungsten) => {
                        ExoBiologyVariant::White
                    }
                    _ if check_material(materials, Tin) => {
                        ExoBiologyVariant::Aquamarine
                    }
                    _ => ExoBiologyVariant::Unknown,
                };
                exobios.push(ExoBiologySpecies {
                    genus: Concha,
                    species: Species::Renibus,
                    variants: vec![variant],
                });
        }
        _ => {}
    }
    exobios
}