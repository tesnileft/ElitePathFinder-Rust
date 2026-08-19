use crate::GeologicalThings;
use crate::custom_structs::exobiospecies::{ExoBiologySpecies, exobio_species_to_string};
use crate::custom_structs::materials::PlanetRawMaterial;
use crate::elite_journal_data::enums::body_data::{AtmosphereType, BodyParent, PlanetClass, StarClass, LuminosityClass, Volcanism};
use std::fmt::Display;
use crate::elite_journal_data::substructs::body_data::{AtmosphericGas, BodyComposition};

pub enum Body {
    Star(Star),
    Planet(Planet),
}
pub struct Star {
    pub body_name: String,
    pub body_id: u64,
    pub class: StarClass,
    pub luminosity: LuminosityClass,
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

    pub atmosphere: Option<AtmosphereInfo>,
    pub biological_signals: Option<u64>,
    pub potential_species: Option<Vec<ExoBiologySpecies>>,
    pub confirmed_species: Vec<ExoBiologySpecies>,
    pub geological_signals: Option<u64>,
    pub confirmed_geology: Option<Vec<GeologicalThings>>,
    pub body_composition: Option<BodyComposition>,
    pub materials: Option<Vec<PlanetRawMaterial>>
}
#[derive(Default)]
pub struct AtmosphereInfo{
    pub atmosphere_type: AtmosphereType,
    pub quality: Vec<AtmosphereQuality>,
    pub composition: Vec<AtmosphericGas>,
}

impl AtmosphereInfo{
    pub fn new(atmosphere_type: AtmosphereType, atmosphere_quality: String, composition_vec: Vec<AtmosphericGas>) -> Self{

        let quality = AtmosphereQuality::parse_all(&atmosphere_quality);
        AtmosphereInfo{
            atmosphere_type,
            quality,
            composition: composition_vec
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum AtmosphereQuality{
    Thin,
    Thick,
    Hot,
}
impl AtmosphereQuality{
    fn parse_all(atmosphere_quality: &str) -> Vec<AtmosphereQuality> {
        let lower = atmosphere_quality.to_lowercase();
        lower
            .split_whitespace()
            .filter_map(|word| match word {
                "thin" => Some(AtmosphereQuality::Thin),
                "thick" => Some(AtmosphereQuality::Thick),
                "hot" => Some(AtmosphereQuality::Hot),
                _ => None,
            })
            .collect()
    }
}
impl Display for AtmosphereQuality{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
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
        let atmosphere_type = self.atmosphere.as_ref().map(|a| a.atmosphere_type.clone()).unwrap_or(AtmosphereType::None);
        let atmosphere_traits = self.atmosphere.as_ref()
            .map(|a| a.quality.iter().fold(String::new(), |acc, q| {
                if acc.is_empty() { q.to_string() } else { acc + " " + &q.to_string() }
            }))
            .unwrap_or_default();
        string.push_str(&format!(" Atmosphere: {} {} \n", atmosphere_traits, atmosphere_type.to_string()));
        if let Some(gravity) = &self.gravity{
            string.push_str(&format!(" Surface Gravity: {}\n", gravity));
        }
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

