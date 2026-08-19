use crate::custom_structs::exobiospecies::ExoBiologySpecies;
use crate::custom_structs::materials::PlanetRawMaterial;
use crate::custom_structs::system_info::{AtmosphereInfo, Star};
use crate::elite_journal_data::enums::body_data::{AtmosphereType, PlanetClass, RawMaterial, Volcanism};
use crate::elite_journal_data::enums::body_data::RawMaterial::{Antimony, Cadmium, Mercury, Molybdenum, Niobium, Polonium, Ruthenium, Technetium, Tellurium, Tin, Tungsten, Yttrium};
use crate::elite_journal_data::enums::exobiology::{ExoBiologyVariant, Species};
use crate::elite_journal_data::enums::exobiology::Genus::Fungoida;
use crate::exobiology_analysis::determine_species::check_material;

pub fn predict_fungoida(atmosphere: &AtmosphereInfo, materials: &Vec<PlanetRawMaterial>, temperature: f64) -> Vec<ExoBiologySpecies> {
    let mut exobios = Vec::new();
    match atmosphere.atmosphere_type {
        AtmosphereType::Ammonia
        | AtmosphereType::Methane
        | AtmosphereType::MethaneRich => {
            //Setisis
                let variant = match () {
                    _ if check_material(materials, Antimony) => {
                        ExoBiologyVariant::Peach
                    }
                    _ if check_material(materials, Polonium) => {
                        ExoBiologyVariant::White
                    }
                    _ if check_material(materials, Ruthenium) => {
                        ExoBiologyVariant::Gold
                    }
                    _ if check_material(materials, Technetium) => {
                        ExoBiologyVariant::Lime
                    }
                    _ if check_material(materials, Tellurium) => {
                        ExoBiologyVariant::Yellow
                    }
                    _ if check_material(materials, Yttrium) => {
                        ExoBiologyVariant::Orange
                    }
                    _ => ExoBiologyVariant::Unknown,
                };
                exobios.push(ExoBiologySpecies {
                    genus: Fungoida,
                    species: Species::Setisis,
                    variants: vec![variant],
                });
        }
        AtmosphereType::Argon | AtmosphereType::ArgonRich => {
            //Bullarum
                let variant = match () {
                    _ if check_material(materials, Antimony) => {
                        ExoBiologyVariant::Red
                    }
                    _ if check_material(materials, Polonium) => {
                        ExoBiologyVariant::Mulberry
                    }
                    _ if check_material(materials, Ruthenium) => {
                        ExoBiologyVariant::Magenta
                    }
                    _ if check_material(materials, Technetium) => {
                        ExoBiologyVariant::Peach
                    }
                    _ if check_material(materials, Tellurium) => {
                        ExoBiologyVariant::Gold
                    }
                    _ if check_material(materials, Yttrium) => {
                        ExoBiologyVariant::Orange
                    }
                    _ => ExoBiologyVariant::Unknown,
                };
                exobios.push(ExoBiologySpecies {
                    genus: Fungoida,
                    species: Species::Bullarum,
                    variants: vec![variant],
                });
        }
        AtmosphereType::CarbonDioxide
        | AtmosphereType::CarbonDioxideRich
        | AtmosphereType::Water
        | AtmosphereType::WaterRich => {
            //Gelata and Stabitis
            if !matches!(
                atmosphere.atmosphere_type,
                AtmosphereType::CarbonDioxide | AtmosphereType::CarbonDioxideRich)
                || (180.0..=195.0).contains(&temperature) {
                    //Gelata
                    let gelata_variant = match () {
                        _ if check_material(materials, Cadmium) => {
                            ExoBiologyVariant::Cyan
                        }
                        _ if check_material(materials, Mercury) => {
                            ExoBiologyVariant::Lime
                        }
                        _ if check_material(materials, Molybdenum) => {
                            ExoBiologyVariant::Mulberry
                        }
                        _ if check_material(materials, Niobium) => {
                            ExoBiologyVariant::Green
                        }
                        _ if check_material(materials, Tungsten) => {
                            ExoBiologyVariant::Orange
                        }
                        _ if check_material(materials, Tin) => {
                            ExoBiologyVariant::Red
                        }
                        _ => ExoBiologyVariant::Unknown,
                    };
                    exobios.push(ExoBiologySpecies {
                        genus: Fungoida,
                        species: Species::Gelata,
                        variants: vec![gelata_variant],
                    });

                    //Stabitis
                    let stabitis_variant = match () {
                        _ if check_material(materials, Cadmium) => {
                            ExoBiologyVariant::Blue
                        }
                        _ if check_material(materials, Mercury) => {
                            ExoBiologyVariant::Green
                        }
                        _ if check_material(materials, Molybdenum) => {
                            ExoBiologyVariant::Magenta
                        }
                        _ if check_material(materials, Niobium) => {
                            ExoBiologyVariant::White
                        }
                        _ if check_material(materials, Tungsten) => {
                            ExoBiologyVariant::Peach
                        }
                        _ if check_material(materials, Tin) => {
                            ExoBiologyVariant::Orange
                        }
                        _ => ExoBiologyVariant::Unknown,
                    };
                    exobios.push(ExoBiologySpecies {
                        genus: Fungoida,
                        species: Species::Stabitis,
                        variants: vec![stabitis_variant],
                    });
            }
        }
        _ => {}
    }
    exobios
}