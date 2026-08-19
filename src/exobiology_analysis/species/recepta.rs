use crate::custom_structs::exobiospecies::ExoBiologySpecies;
use crate::custom_structs::materials::PlanetRawMaterial;
use crate::custom_structs::system_info::{AtmosphereInfo, Star};
use crate::elite_journal_data::enums::body_data::{AtmosphereType, PlanetClass, StarClass, Volcanism};
use crate::elite_journal_data::enums::body_data::RawMaterial::{Antimony, Cadmium, Mercury, Molybdenum, Niobium, Polonium, Technetium, Tellurium, Tungsten, Yttrium};
use crate::elite_journal_data::enums::exobiology::{ExoBiologyVariant, Species};
use crate::elite_journal_data::enums::exobiology::Genus::Recepta;
use crate::exobiology_analysis::determine_species::check_material;

fn recepta_stellar_variant(class: &StarClass) -> ExoBiologyVariant {
    match class {
        StarClass::B => ExoBiologyVariant::Turquoise,
        StarClass::A => ExoBiologyVariant::Amethyst,
        StarClass::F => ExoBiologyVariant::Mauve,
        StarClass::G => ExoBiologyVariant::Orange,
        StarClass::K => ExoBiologyVariant::Red,
        StarClass::M => ExoBiologyVariant::Maroon,
        StarClass::T => ExoBiologyVariant::Teal,
        StarClass::TTS => ExoBiologyVariant::Sage,
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
        StarClass::N => ExoBiologyVariant::Emerald,
        _ => ExoBiologyVariant::Unknown,
    }
}
pub fn predict_recepta(planet_class: &PlanetClass, atmosphere: &AtmosphereInfo, materials: &Vec<PlanetRawMaterial>, parent_stars: &Vec<&Star>) -> Vec<ExoBiologySpecies> {
    let mut exobios = Vec::new();
    if !matches!(atmosphere.atmosphere_type, AtmosphereType::SulphurDioxide){
        return exobios
    }
    match planet_class {
        PlanetClass::Rocky | PlanetClass::HMC => {
            let variant = match () {
                _ if check_material(materials, Cadmium) => ExoBiologyVariant::Lime,
                _ if check_material(materials, Mercury) => ExoBiologyVariant::Gold,
                _ if check_material(materials, Molybdenum) => ExoBiologyVariant::Orange,
                _ if check_material(materials, Niobium) => ExoBiologyVariant::Mulberry,
                _ if check_material(materials, Tungsten) => ExoBiologyVariant::Red,
                _ => ExoBiologyVariant::Unknown,
            };
            exobios.push(ExoBiologySpecies {
                genus: Recepta,
                species: Species::Deltahedronix,
                variants: vec![variant],
            });
                exobios.push(ExoBiologySpecies {
                    genus: Recepta,
                    species: Species::Umbrux,
                    variants: parent_stars.iter().map(|star| recepta_stellar_variant(&star.class)).collect(),
                });
        }
        PlanetClass::Icy | PlanetClass::RockyIce => {
            let variant = match () {
                _ if check_material(materials, Antimony) => ExoBiologyVariant::Lime,
                _ if check_material(materials, Polonium) => ExoBiologyVariant::White,
                _ if check_material(materials, Technetium) => {
                    ExoBiologyVariant::Aquamarine
                }
                _ if check_material(materials, Tellurium) => ExoBiologyVariant::Cyan,
                _ if check_material(materials, Yttrium) => ExoBiologyVariant::Green,
                _ => ExoBiologyVariant::Unknown,
            };
            exobios.push(ExoBiologySpecies {
                genus: Recepta,
                species: Species::Deltahedronix,
                variants: vec![variant],
            });
                exobios.push(ExoBiologySpecies {
                    genus: Recepta,
                    species: Species::Umbrux,
                    variants: parent_stars.iter().map(|star| recepta_stellar_variant(&star.class)).collect(),
                });
        }
        _ => {}
    }
    exobios
}