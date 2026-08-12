use crate::custom_structs::exobiospecies::ExoBiologySpecies;
use crate::custom_structs::system_info::{Planet, Star};
use crate::elite_journal_data::enums::body_data::RawMaterial::{Cadmium, Mercury, Molybdenum, Niobium, Tin, Tungsten};
use crate::elite_journal_data::enums::exobiology::ExoBiologyVariant;
use crate::elite_journal_data::enums::exobiology::Genus::Fumerola;
use crate::elite_journal_data::enums::exobiology::Species::{Aquatis, Carbosis, Extremus, Nitris};
use crate::exobiology_analysis::determine_species::check_material;

pub fn predict_fumerola(planet: &Planet) -> Vec<ExoBiologySpecies> {
    let mut exobios = Vec::new();
    if let (Some(volcanism), Some(materials) )= (planet.volcanism.as_ref(), planet.materials.as_ref()) {
        let (species, variant) = match (){
            _ if volcanism.is_water() => { let variant = match () {
                _ if check_material(materials, Cadmium) => ExoBiologyVariant::Green,
                _ if check_material(materials, Mercury) => ExoBiologyVariant::Yellow,
                _ if check_material(materials, Molybdenum) => ExoBiologyVariant::Cyan,
                _ if check_material(materials, Niobium) => ExoBiologyVariant::Gold,
                _ if check_material(materials, Tungsten) => ExoBiologyVariant::Cobalt,
                _ if check_material(materials, Tin) => ExoBiologyVariant::Orange,
                _ => ExoBiologyVariant::Unknown,
                };
                (Aquatis, variant)},
            _ if volcanism.is_carbon_dioxide() | volcanism.is_methane() => { let variant = match () {
                _ if check_material(materials, Cadmium) => ExoBiologyVariant::Orange,
                _ if check_material(materials, Mercury) => ExoBiologyVariant::Magenta,
                _ if check_material(materials, Molybdenum) => ExoBiologyVariant::Gold,
                _ if check_material(materials, Niobium) => ExoBiologyVariant::Cobalt,
                _ if check_material(materials, Tungsten) => ExoBiologyVariant::Yellow,
                _ if check_material(materials, Tin) => ExoBiologyVariant::Teal,
                _ => ExoBiologyVariant::Unknown,
            };
                (Carbosis, variant)},
            _ if volcanism.is_carbon_dioxide() | volcanism.is_methane() => { let variant = match () {
                _ if check_material(materials, Cadmium) => ExoBiologyVariant::Aquamarine,
                _ if check_material(materials, Mercury) => ExoBiologyVariant::Lime,
                _ if check_material(materials, Molybdenum) => ExoBiologyVariant::Blue,
                _ if check_material(materials, Niobium) => ExoBiologyVariant::White,
                _ if check_material(materials, Tungsten) => ExoBiologyVariant::Mulberry,
                _ if check_material(materials, Tin) => ExoBiologyVariant::Peach,
                _ => ExoBiologyVariant::Unknown,
            };
                (Extremus, variant)},
            _ if volcanism.is_rocky() | volcanism.is_iron() | volcanism.is_silicate_vapours() => { let variant = match () {
                _ if check_material(materials, Cadmium) => ExoBiologyVariant::White,
                _ if check_material(materials, Mercury) => ExoBiologyVariant::Peach,
                _ if check_material(materials, Molybdenum) => ExoBiologyVariant::Lime,
                _ if check_material(materials, Niobium) => ExoBiologyVariant::Red,
                _ if check_material(materials, Tungsten) => ExoBiologyVariant::Aquamarine,
                _ if check_material(materials, Tin) => ExoBiologyVariant::Mulberry,
                _ => ExoBiologyVariant::Unknown,
            };
                (Nitris, variant)},
            _ => {return exobios}
        };

        exobios.push(ExoBiologySpecies{
            genus: Fumerola,
            species,
            variants: vec![variant],
        })

    }
    exobios
}