use crate::custom_structs::exobiospecies::ExoBiologySpecies;
use crate::custom_structs::system_info::{AtmosphereInfo, Star};
use crate::elite_journal_data::enums::body_data::{
    AtmosphereType, PlanetClass, StarClass, Volcanism,
};
use crate::elite_journal_data::enums::exobiology::Genus::Frutexa;
use crate::elite_journal_data::enums::exobiology::{ExoBiologyVariant, Species};

fn frutexa_stellar_variant(class: &StarClass) -> ExoBiologyVariant {
    match class {
        StarClass::O => ExoBiologyVariant::Yellow,
        StarClass::B => ExoBiologyVariant::Lime,
        StarClass::F => ExoBiologyVariant::Green,
        StarClass::G => ExoBiologyVariant::Emerald,
        StarClass::M => ExoBiologyVariant::Grey,
        StarClass::L => ExoBiologyVariant::Teal,
        StarClass::TTS => ExoBiologyVariant::Mauve,
        StarClass::W | StarClass::WN | StarClass::WNC | StarClass::WC | StarClass::WO => {
            ExoBiologyVariant::Orange
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
        | StarClass::DCV => ExoBiologyVariant::Indigo,
        StarClass::N => ExoBiologyVariant::Red,
        _ => ExoBiologyVariant::Unknown,
    }
}

pub fn predict_frutexa(
    planet_class: &PlanetClass,
    atmosphere: &AtmosphereInfo,
    temperature: f64,
    parent_stars: &Vec<&Star>,
) -> Vec<ExoBiologySpecies> {
    let mut exobios = Vec::new();
    let is_rocky = matches!(planet_class, PlanetClass::Rocky);
    let is_hmc = matches!(planet_class, PlanetClass::HMC);
    match atmosphere.atmosphere_type {
        AtmosphereType::Ammonia => {
            if is_rocky {
                //Flabellum, Flammasis
                exobios.push(ExoBiologySpecies {
                    genus: Frutexa,
                    species: Species::Flabellum,
                    variants: parent_stars
                        .iter()
                        .map(|star| frutexa_stellar_variant(&star.class))
                        .collect(),
                });
                exobios.push(ExoBiologySpecies {
                    genus: Frutexa,
                    species: Species::Flammasis,
                    variants: parent_stars
                        .iter()
                        .map(|star| frutexa_stellar_variant(&star.class))
                        .collect(),
                });
            }
            if is_hmc {
                //Metallicum
                exobios.push(ExoBiologySpecies {
                    genus: Frutexa,
                    species: Species::Metallicum,
                    variants: parent_stars
                        .iter()
                        .map(|star| frutexa_stellar_variant(&star.class))
                        .collect(),
                });
            }
        }
        AtmosphereType::CarbonDioxide | AtmosphereType::CarbonDioxideRich => {
            if temperature < 195.0 {
                if is_rocky {
                    //Acus, Fera
                    exobios.push(ExoBiologySpecies {
                        genus: Frutexa,
                        species: Species::Acus,
                        variants: parent_stars
                            .iter()
                            .map(|star| frutexa_stellar_variant(&star.class))
                            .collect(),
                    });
                    exobios.push(ExoBiologySpecies {
                        genus: Frutexa,
                        species: Species::Fera,
                        variants: parent_stars
                            .iter()
                            .map(|star| frutexa_stellar_variant(&star.class))
                            .collect(),
                    });
                }
                if is_hmc {
                    //Metallicum
                    exobios.push(ExoBiologySpecies {
                        genus: Frutexa,
                        species: Species::Metallicum,
                        variants: parent_stars
                            .iter()
                            .map(|star| frutexa_stellar_variant(&star.class))
                            .collect(),
                    });
                }
            }
        }
        AtmosphereType::Water | AtmosphereType::WaterRich => {
            if is_rocky {
                //Sponsae
                exobios.push(ExoBiologySpecies {
                    genus: Frutexa,
                    species: Species::Sponsae,
                    variants: parent_stars
                        .iter()
                        .map(|star| frutexa_stellar_variant(&star.class))
                        .collect(),
                });
            }
        }
        AtmosphereType::SulphurDioxide => {
            if is_rocky {
                //Collum
                exobios.push(ExoBiologySpecies {
                    genus: Frutexa,
                    species: Species::Collum,
                    variants: parent_stars
                        .iter()
                        .map(|star| frutexa_stellar_variant(&star.class))
                        .collect(),
                });
            }
        }
        _ => {}
    }
    exobios
}
