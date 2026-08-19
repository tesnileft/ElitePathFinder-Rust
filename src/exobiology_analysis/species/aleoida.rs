use crate::custom_structs::exobiospecies::ExoBiologySpecies;
use crate::custom_structs::system_info::{AtmosphereInfo, Star};
use crate::elite_journal_data::enums::body_data::{AtmosphereType, PlanetClass, StarClass, Volcanism};
use crate::elite_journal_data::enums::exobiology::{ExoBiologyVariant, Species};
use crate::elite_journal_data::enums::exobiology::ExoBiologyVariant::{Green, Teal, Turquoise, Unknown, Yellow, Emerald, Lime, Sage, Mauve, Grey, Indigo, Ocher};
use crate::elite_journal_data::enums::exobiology::Genus::Aleoida;
use crate::elite_journal_data::enums::exobiology::Species::{Arcus, Coronamus, Gravis, Laminiae, Spica};

fn aleoida_stellar_variant(star_class: &StarClass) -> ExoBiologyVariant{
    match star_class{
        StarClass::B => Yellow,
        StarClass::A => Green,
        StarClass::F => Teal,
        StarClass::K => Turquoise,
        StarClass::M => Emerald,
        StarClass::L => Lime,
        StarClass::T => Sage,
        StarClass::TTS => Mauve,
        StarClass::Y => Teal,
        StarClass::W => Grey,
        StarClass::D => Indigo,
        StarClass::N => Ocher,
        _ => Unknown
    }
}
fn new_aleoida(species: Species, parent_stars: &Vec<&Star>) -> ExoBiologySpecies{
    ExoBiologySpecies{
        genus: Aleoida,
        species,
        variants: parent_stars.iter().map(|startype| aleoida_stellar_variant(&startype.class)).collect()
    }
}

pub fn predict_aleoida(atmosphere: &AtmosphereInfo, temperature: f64, parent_stars: &Vec<&Star>) -> Vec<ExoBiologySpecies> {
    let mut exobios = Vec::new();
        match atmosphere.atmosphere_type{
            AtmosphereType::Ammonia => {
                exobios.push(new_aleoida(Laminiae, parent_stars));
                exobios.push(new_aleoida(Spica, parent_stars));
            },
            AtmosphereType::CarbonDioxide | AtmosphereType::CarbonDioxideRich => {
                if (175.0..=180.0).contains(&temperature) {
                    exobios.push(new_aleoida(Arcus, parent_stars));
                }
                if (180.0..=190.0).contains(&temperature){
                    exobios.push(new_aleoida(Coronamus, parent_stars));
                }
                if (190.0..=195.0).contains(&temperature){
                    exobios.push(new_aleoida(Gravis, parent_stars));
                }
            },
            _ => {}
        }

    exobios
}