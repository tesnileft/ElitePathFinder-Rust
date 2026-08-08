use std::fmt::Display;
use crate::elite_events::enums::{AtmosphereType, BodyParent, PlanetClass, StarClass, Volcanism};
use crate::elite_events::substructs::{AtmosphericGas, BodyComposition};
use crate::{ExoBiologySpecies, GeologicalThings};
use crate::custom_structs::materials::PlanetRawMaterial;
use crate::exobio_analysis::get_species_value;

pub enum Body{
    Star(Star),
    Planet(Planet),
}
pub struct Star{
    pub body_name: String,
    pub body_id: u64,
    pub class: StarClass,
    pub subclass: u64,
    pub stellar_mass: f64,
}
#[derive(Default)]
pub struct Planet {
    pub body_name: String,
    pub body_id: u64,
    pub system_address: u64,
    pub parents: Option<Vec<BodyParent>>,
    pub planet_class: Option<PlanetClass>,
    pub gravity: Option<f64>,
    pub mean_temperature: Option<f64>,
    pub volcanism: Option<Volcanism>,
    pub atmosphere_type: Option<AtmosphereType>,
    pub atmosphere_composition: Option<Vec<AtmosphericGas>>,
    pub biological_signals: Option<u64>,
    pub potential_species: Option<Vec<ExoBiologySpecies>>,
    pub confirmed_species: Vec<ExoBiologySpecies>,
    pub geological_signals: Option<u64>,
    pub confirmed_geology: Option<Vec<GeologicalThings>>,
    pub body_composition: Option<BodyComposition>,
    pub materials: Option<Vec<PlanetRawMaterial>>
}
pub struct AtmosphereInfo{
    pub atmosphere_type: AtmosphereType,
    pub quality: AtmosphereQuality,
    pub composition: Vec<AtmosphericGas>,
}

#[derive(Default, PartialEq, Eq, Clone)]
enum AtmosphereQuality{
    Thin,
    Thick,
    #[default]
    Standard,
}
impl Planet {
    pub fn new(body_name: String, body_id: u64, system_address: u64) -> Planet {
        Planet {
            body_name,
            body_id,
            system_address,
            ..Default::default()
        }
    }
}
impl Display for Planet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        string.push_str(&format!("{} - {} \n", self.body_name, self.body_id));
        string.push_str(&format!(" Atmosphere: {} \n", self.atmosphere_type.clone().unwrap_or(AtmosphereType::None).to_string()));
        if let Some(bio_signals) = &self.biological_signals {
            if *bio_signals > 0{
                string.push_str(&format!(" Biological signals: {}\n", bio_signals));
            }
        }
        if let Some(v) = &self.potential_species {
            if v.len() > 0{
                string.push_str(&format!(" Potential species:\n{}", exobio_species_to_string(v)));
            }
        }

        write!(f, "{}", string)
    }
}
fn exobio_species_to_string(species: &Vec<ExoBiologySpecies>)-> String {
    let mut string = String::new();
    species.into_iter().for_each(|bio| {
        string.push_str(&format!(" {} ¢{}\n ", bio.to_string().as_str(), get_species_value(bio.species.clone())));
        bio.variants.iter().for_each(|variant| {
            string.push_str(&format!("/ {} ", variant.to_string()));
        });
        string.push_str("/\n");
    });
    string
}