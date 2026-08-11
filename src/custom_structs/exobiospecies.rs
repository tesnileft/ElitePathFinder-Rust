use crate::exobiology_analysis::determine_species::get_species_value;
use std::fmt::{Display, Formatter};
use crate::elite_journal_data::enums::exobiology::*;

pub struct ExoBiologySpecies{
    pub(crate) genus: Genus,
    pub(crate) species: Species,
    pub(crate) variants: Vec<ExoBiologyVariant>,
}
impl Display for ExoBiologySpecies {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.genus.to_string(), self.species.to_string())
    }
}
pub fn exobio_species_to_string(species: &Vec<ExoBiologySpecies>)-> String {
    let mut string = String::new();
    species.into_iter().for_each(|bio| {
        string.push_str(&format!(" {} ", bio.to_string().as_str()));
        bio.variants.iter().for_each(|variant| {
            string.push_str(&format!("/ {} ", variant.to_string()));
        });
        string.push_str("/ ");
        string.push_str(&format!("¢{} \n", get_species_value(bio.species.clone())));

    });
    string
}