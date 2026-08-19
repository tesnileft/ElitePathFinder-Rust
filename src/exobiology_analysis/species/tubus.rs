use crate::custom_structs::exobiospecies::ExoBiologySpecies;
use crate::custom_structs::materials::PlanetRawMaterial;
use crate::custom_structs::system_info::{AtmosphereInfo, AtmosphereQuality, Star};
use crate::elite_journal_data::enums::body_data::{AtmosphereType, PlanetClass, StarClass};
use crate::elite_journal_data::enums::exobiology::Genus::Tubus;
use crate::elite_journal_data::enums::exobiology::{ExoBiologyVariant, Species};

fn tubus_stellar_variant(class: &StarClass) -> ExoBiologyVariant {
    match class {
        StarClass::O => ExoBiologyVariant::Green,
        StarClass::B => ExoBiologyVariant::Emerald,
        StarClass::A => ExoBiologyVariant::Indigo,
        StarClass::F => ExoBiologyVariant::Grey,
        StarClass::G => ExoBiologyVariant::Red,
        StarClass::K => ExoBiologyVariant::Maroon,
        StarClass::M => ExoBiologyVariant::Teal,
        StarClass::L => ExoBiologyVariant::Turquoise,
        StarClass::T => ExoBiologyVariant::Mauve,
        StarClass::TTS => ExoBiologyVariant::Ocher,
        StarClass::W | StarClass::WN | StarClass::WNC | StarClass::WC | StarClass::WO => {
            ExoBiologyVariant::Lime
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
        | StarClass::DCV => ExoBiologyVariant::Yellow,
        StarClass::N => ExoBiologyVariant::Amethyst,
        _ => ExoBiologyVariant::Unknown,
    }
}

pub fn predict_tubus(atmosphere: &AtmosphereInfo, planet_class: &PlanetClass, temperature: f64, parent_stars: & Vec<&Star>) -> Vec<ExoBiologySpecies> {
    let mut exobios = Vec::new();
    if atmosphere.quality.contains(&AtmosphereQuality::Thin)
        && (160.0..=190.0).contains(&temperature)
    {
        if matches!(planet_class, PlanetClass::HMC)
            && matches!(
    atmosphere.atmosphere_type,
    AtmosphereType::Ammonia
    | AtmosphereType::CarbonDioxide
    | AtmosphereType::CarbonDioxideRich
    )
        {
            exobios.push(ExoBiologySpecies {
                genus: Tubus,
                species: Species::Sororibus,
                variants: parent_stars.iter().map(|star| tubus_stellar_variant(&star.class)).collect(),
            })
        }
        if matches!(planet_class, PlanetClass::Rocky) {
            match atmosphere.atmosphere_type {
                AtmosphereType::Ammonia => exobios.push(ExoBiologySpecies {
                    genus: Tubus,
                    species: Species::Rosarium,
                    variants: parent_stars.iter().map(|star| tubus_stellar_variant(&star.class)).collect(),
                }),
                AtmosphereType::CarbonDioxide | AtmosphereType::CarbonDioxideRich => {
                    exobios.push(ExoBiologySpecies {
                        genus: Tubus,
                        species: Species::Cavas,
                        variants: parent_stars.iter().map(|star| tubus_stellar_variant(&star.class)).collect(),
                    });
                    exobios.push(ExoBiologySpecies {
                        genus: Tubus,
                        species: Species::Compagibus,
                        variants: parent_stars.iter().map(|star| tubus_stellar_variant(&star.class)).collect(),
                    });
                    exobios.push(ExoBiologySpecies {
                        genus: Tubus,
                        species: Species::Conifer,
                        variants: parent_stars.iter().map(|star| tubus_stellar_variant(&star.class)).collect(),
                    })
                }
                _ => {}
            }
        }

    }
    exobios
}