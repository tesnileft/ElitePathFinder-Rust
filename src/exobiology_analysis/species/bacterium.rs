
use crate::custom_structs::exobiospecies::ExoBiologySpecies;
use crate::custom_structs::system_info::{Planet, Star};
use crate::elite_journal_data::enums::body_data::{AtmosphereType, StarClass, Volcanism};
use crate::elite_journal_data::enums::body_data::RawMaterial::*;
use crate::elite_journal_data::enums::exobiology::{ExoBiologyVariant, Species};
use crate::elite_journal_data::enums::exobiology::Genus::Bacterium;
use crate::exobiology_analysis::determine_species::check_material;

fn stellar_variant(class: &StarClass) -> ExoBiologyVariant {
    match class {
        StarClass::O => ExoBiologyVariant::Turquoise,
        StarClass::B => ExoBiologyVariant::Grey,
        StarClass::A => ExoBiologyVariant::Yellow,
        StarClass::F => ExoBiologyVariant::Lime,
        StarClass::G => ExoBiologyVariant::Emerald,
        StarClass::K => ExoBiologyVariant::Green,
        StarClass::M => ExoBiologyVariant::Teal,
        StarClass::L => ExoBiologyVariant::Sage,
        StarClass::T => ExoBiologyVariant::Russet,
        StarClass::Y => ExoBiologyVariant::Mauve,
        StarClass::TTS => ExoBiologyVariant::Maroon,
        StarClass::AeBe => ExoBiologyVariant::Orange,
        StarClass::W | StarClass::WN | StarClass::WNC | StarClass::WC | StarClass::WO => {
            ExoBiologyVariant::Amber
        }
        StarClass::D
        | StarClass::DA
        | StarClass::DB
        | StarClass::DC
        | StarClass::DO
        | StarClass::DQ
        | StarClass::DX
        | StarClass::DAV
        | StarClass::DBV
        | StarClass::DCV => ExoBiologyVariant::Ochre,
        StarClass::N => ExoBiologyVariant::Indigo,
        StarClass::H | StarClass::SupermassiveBlackHole | StarClass::X | StarClass::RoguePlanet => {
            ExoBiologyVariant::Unknown
        }
        _ => {
            println!("Unsupported star type");
            ExoBiologyVariant::Unknown
        }
    }
}
fn bacterium_starvariants(stars: &Vec<&Star>) -> Vec<ExoBiologyVariant>{
    stars.iter().map(|star| stellar_variant(&star.class)).collect()
}

pub fn predict_bacterium(planet: &Planet, star_parents: &Vec<& Star>) -> Vec<ExoBiologySpecies>{
    let mut exobios = Vec::new();
    if let (Some(volcanism), Some(materials), Some(atmosphere)) = (planet.volcanism.clone(), planet.materials.as_ref(), planet.atmosphere.as_ref()){
        if matches!(
            volcanism, //Excludes others
            Volcanism::Helium
                | Volcanism::Iron
                | Volcanism::MajorSilicateVapour
                | Volcanism::MinorSilicateVapour
        ) {
            //Tela
            let variant = match () {
                _ if check_material(materials, Cadmium) => ExoBiologyVariant::Gold,
                _ if check_material(materials, Mercury) => ExoBiologyVariant::Orange,
                _ if check_material(materials, Molybdenum) => ExoBiologyVariant::Yellow,
                _ if check_material(materials, Niobium) => ExoBiologyVariant::Magenta,
                _ if check_material(materials, Tungsten) => ExoBiologyVariant::Green,
                _ if check_material(materials, Tin) => ExoBiologyVariant::Cobalt,
                _ => ExoBiologyVariant::Unknown,
            };
            exobios.push(ExoBiologySpecies {
                genus: Bacterium,
                species: Species::Tela,
                variants: vec![variant],
            });
        }
        match atmosphere.atmosphere_type {
            AtmosphereType::Helium => {
                if let Some(materials) = planet.materials.as_ref() {
                    let variant = match () {
                        _ if check_material(materials, Antimony) => ExoBiologyVariant::Magenta,
                        _ if check_material(materials, Polonium) => ExoBiologyVariant::Gold,
                        _ if check_material(materials, Ruthenium) => ExoBiologyVariant::Orange,
                        _ if check_material(materials, Technetium) => ExoBiologyVariant::Cyan,
                        _ if check_material(materials, Tellurium) => ExoBiologyVariant::Green,
                        _ if check_material(materials, Yttrium) => ExoBiologyVariant::Cobalt,
                        _ => ExoBiologyVariant::Unknown,
                    };
                    exobios.push(ExoBiologySpecies {
                        genus: Bacterium,
                        species: Species::Nebulus,
                        variants: vec![variant],
                    });
                }
            }
            AtmosphereType::Neon | AtmosphereType::NeonRich => {
                if let Some(materials) = planet.materials.as_ref() {
                    let variant = match () {
                        _ if check_material(materials, Antimony) => ExoBiologyVariant::Cyan,
                        _ if check_material(materials, Polonium) => ExoBiologyVariant::Magenta,
                        _ if check_material(materials, Ruthenium) => ExoBiologyVariant::Cobalt,
                        _ if check_material(materials, Technetium) => ExoBiologyVariant::Lime,
                        _ if check_material(materials, Tellurium) => ExoBiologyVariant::White,
                        _ if check_material(materials, Yttrium) => ExoBiologyVariant::Aquamarine,
                        _ => ExoBiologyVariant::Unknown,
                    };
                    exobios.push(ExoBiologySpecies {
                        genus: Bacterium,
                        species: Species::Acies,
                        variants: vec![variant],
                    });
                }

                //Omentum - material determinant, requires Nitrogen or Ammonia volcanism
                if matches!(
                    volcanism,
                    Volcanism::NitrogenGeysers
                        | Volcanism::MajorNitrogenGeysers
                        | Volcanism::MinorNitrogenGeysers
                        | Volcanism::NitrogenMagma
                        | Volcanism::MinorNitrogenMagma
                        | Volcanism::MajorNitrogenMagma
                        | Volcanism::Ammonia
                        | Volcanism::MajorAmmonia
                        | Volcanism::MinorAmmonia
                ) {
                        let variant = match () {
                            _ if check_material(materials, Cadmium) => ExoBiologyVariant::Lime,
                            _ if check_material(materials, Mercury) => ExoBiologyVariant::White,
                            _ if check_material(materials, Molybdenum) => {
                                ExoBiologyVariant::Aquamarine
                            }
                            _ if check_material(materials, Niobium) => ExoBiologyVariant::Peach,
                            _ if check_material(materials, Tin) => ExoBiologyVariant::Red,
                            _ if check_material(materials, Tungsten) => ExoBiologyVariant::Blue,
                            _ => ExoBiologyVariant::Unknown,
                        };
                        exobios.push(ExoBiologySpecies {
                            genus: Bacterium,
                            species: Species::Omentum,
                            variants: vec![variant],
                        });
                }

                //Scopulum - material determinant, requires Carbon(dioxide) or Methane volcanism
                if matches!(
                    volcanism,
                    Volcanism::CarbonDioxide
                        | Volcanism::MajorCarbonDioxide
                        | Volcanism::MinorCarbonDioxide
                        | Volcanism::Methane
                        | Volcanism::MajorMethane
                        | Volcanism::MinorMethane
                ) {
                    if let Some(materials) = planet.materials.as_ref() {
                        let variant = match () {
                            _ if check_material(materials, Cadmium) => ExoBiologyVariant::White,
                            _ if check_material(materials, Mercury) => ExoBiologyVariant::Peach,
                            _ if check_material(materials, Molybdenum) => ExoBiologyVariant::Lime,
                            _ if check_material(materials, Niobium) => ExoBiologyVariant::Red,
                            _ if check_material(materials, Tin) => ExoBiologyVariant::Mulberry,
                            _ if check_material(materials, Tungsten) => {
                                ExoBiologyVariant::Aquamarine
                            }
                            _ => ExoBiologyVariant::Unknown,
                        };
                        exobios.push(ExoBiologySpecies {
                            genus: Bacterium,
                            species: Species::Scopulum,
                            variants: vec![variant],
                        });
                    }
                }

                //Verrata - material determinant, requires Water volcanism
                if matches!(
                    volcanism,
                    Volcanism::WaterGeysers
                        | Volcanism::MajorWaterGeysers
                        | Volcanism::MinorWaterGeysers
                ) {
                        let variant = match () {
                            _ if check_material(materials, Cadmium) => ExoBiologyVariant::Peach,
                            _ if check_material(materials, Mercury) => ExoBiologyVariant::Red,
                            _ if check_material(materials, Molybdenum) => ExoBiologyVariant::White,
                            _ if check_material(materials, Niobium) => ExoBiologyVariant::Mulberry,
                            _ if check_material(materials, Tin) => ExoBiologyVariant::Blue,
                            _ if check_material(materials, Tungsten) => ExoBiologyVariant::Lime,
                            _ => ExoBiologyVariant::Unknown,
                        };
                        exobios.push(ExoBiologySpecies {
                            genus: Bacterium,
                            species: Species::Verrata,
                            variants: vec![variant],
                        });
                }
            }
            AtmosphereType::Methane | AtmosphereType::MethaneRich => {
                //Bullaris
                    let variant = match () {
                        _ if check_material(materials, Antimony) => ExoBiologyVariant::Cobalt,
                        _ if check_material(materials, Polonium) => ExoBiologyVariant::Yellow,
                        _ if check_material(materials, Ruthenium) => ExoBiologyVariant::Aquamarine,
                        _ if check_material(materials, Technetium) => ExoBiologyVariant::Gold,
                        _ if check_material(materials, Tellurium) => ExoBiologyVariant::Lime,
                        _ if check_material(materials, Yttrium) => ExoBiologyVariant::Red,
                        _ => ExoBiologyVariant::Unknown,
                    };
                    exobios.push(ExoBiologySpecies {
                        genus: Bacterium,
                        species: Species::Bullaris,
                        variants: vec![variant],
                    });
            }
            AtmosphereType::Argon | AtmosphereType::ArgonRich => {
                //Vesicula
                    let variant = match () {
                        _ if check_material(materials, Antimony) => ExoBiologyVariant::Cyan,
                        _ if check_material(materials, Polonium) => ExoBiologyVariant::Orange,
                        _ if check_material(materials, Ruthenium) => ExoBiologyVariant::Mulberry,
                        _ if check_material(materials, Technetium) => ExoBiologyVariant::Gold,
                        _ if check_material(materials, Tellurium) => ExoBiologyVariant::Red,
                        _ if check_material(materials, Yttrium) => ExoBiologyVariant::Lime,
                        _ => ExoBiologyVariant::Unknown,
                    };
                    exobios.push(ExoBiologySpecies {
                        genus: Bacterium,
                        species: Species::Vesicula,
                        variants: vec![variant],
                    });
            }
            AtmosphereType::Nitrogen => {
                //Informem
                    let variant = match () {
                        _ if check_material(materials, Antimony) => ExoBiologyVariant::Red,
                        _ if check_material(materials, Polonium) => ExoBiologyVariant::Lime,
                        _ if check_material(materials, Ruthenium) => ExoBiologyVariant::Gold,
                        _ if check_material(materials, Technetium) => ExoBiologyVariant::Aquamarine,
                        _ if check_material(materials, Tellurium) => ExoBiologyVariant::Yellow,
                        _ if check_material(materials, Yttrium) => ExoBiologyVariant::Cobalt,
                        _ => ExoBiologyVariant::Unknown,
                    };
                    exobios.push(ExoBiologySpecies {
                        genus: Bacterium,
                        species: Species::Informem,
                        variants: vec![variant],
                    });
            }
            AtmosphereType::Oxygen => {
                //Volu
                    let variant = match () {
                        _ if check_material(materials, Antimony) => ExoBiologyVariant::Red,
                        _ if check_material(materials, Polonium) => ExoBiologyVariant::Aquamarine,
                        _ if check_material(materials, Ruthenium) => ExoBiologyVariant::Cobalt,
                        _ if check_material(materials, Technetium) => ExoBiologyVariant::Lime,
                        _ if check_material(materials, Tellurium) => ExoBiologyVariant::Cyan,
                        _ if check_material(materials, Yttrium) => ExoBiologyVariant::Gold,
                        _ => ExoBiologyVariant::Unknown,
                    };
                    exobios.push(ExoBiologySpecies {
                        genus: Bacterium,
                        species: Species::Volu,
                        variants: vec![variant],
                    });
            }
            AtmosphereType::Ammonia => {
                //Alcyoneum
                    exobios.push(ExoBiologySpecies {
                        genus: Bacterium,
                        species: Species::Alcyoneum,
                        variants: bacterium_starvariants(star_parents)
                    });
            }
            AtmosphereType::CarbonDioxide | AtmosphereType::CarbonDioxideRich => {
                //Aurasus
                    exobios.push(ExoBiologySpecies {
                        genus: Bacterium,
                        species: Species::Aurasus,
                        variants: bacterium_starvariants(star_parents)
                    });
            }
            AtmosphereType::Water | AtmosphereType::WaterRich | AtmosphereType::SulphurDioxide => {
                //Cerbrus
                    exobios.push(ExoBiologySpecies {
                        genus: Bacterium,
                        species: Species::Cerbrus,
                        variants: bacterium_starvariants(star_parents)
                    });
            }
            _ => {}
        }
    }
    exobios
}
