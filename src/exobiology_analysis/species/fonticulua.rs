use crate::custom_structs::exobiospecies::ExoBiologySpecies;
use crate::custom_structs::system_info::Star;
use crate::elite_journal_data::enums::body_data::{AtmosphereType, PlanetClass, StarClass};
use crate::elite_journal_data::enums::exobiology::Genus::Fonticulua;
use crate::elite_journal_data::enums::exobiology::{ExoBiologyVariant, Species};

fn fonticulua_stellar_variant(class: &StarClass) -> ExoBiologyVariant {
    match class {
        StarClass::O => ExoBiologyVariant::Grey,
        StarClass::B => ExoBiologyVariant::Lime,
        StarClass::A => ExoBiologyVariant::Green,
        StarClass::F => ExoBiologyVariant::Yellow,
        StarClass::G => ExoBiologyVariant::Teal,
        StarClass::K => ExoBiologyVariant::Emerald,
        StarClass::M => ExoBiologyVariant::Amethyst,
        StarClass::L => ExoBiologyVariant::Mauve,
        StarClass::T => ExoBiologyVariant::Orange,
        StarClass::TTS => ExoBiologyVariant::Red,
        StarClass::Y => ExoBiologyVariant::Ochre,
        StarClass::W | StarClass::WN | StarClass::WNC | StarClass::WC | StarClass::WO => {
            ExoBiologyVariant::Indigo
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
        | StarClass::DCV => ExoBiologyVariant::Turquoise,
        StarClass::N => ExoBiologyVariant::Sage,
        StarClass::AeBe => ExoBiologyVariant::Maroon,
        _ => ExoBiologyVariant::Unknown,
    }
}
pub fn predict_fonticulua(planet_class: &PlanetClass, atmosphere_type: &AtmosphereType, startypes: &Vec<&Star>) -> Vec<ExoBiologySpecies>{
    let mut exobios = Vec::new();
    if matches!( planet_class, PlanetClass::Icy | PlanetClass::RockyIce)
    {
        let species = match atmosphere_type {
            AtmosphereType::Neon | AtmosphereType::NeonRich => Some(Species::Segmentatus),
            AtmosphereType::Methane | AtmosphereType::MethaneRich => Some(Species::Digitos),
            AtmosphereType::Argon => Some(Species::Campestris),
            AtmosphereType::ArgonRich => Some(Species::Upupam),
            AtmosphereType::Nitrogen => Some(Species::Lapida),
            AtmosphereType::Oxygen => Some(Species::Fluctus),
            _ => None,
        };
        if let Some(species) = species {
            exobios.push(ExoBiologySpecies {
                genus: Fonticulua,
                species,
                variants: startypes.iter().map(|startype| fonticulua_stellar_variant(&startype.class)).collect(),
            });
            
        }
    }
    exobios
}