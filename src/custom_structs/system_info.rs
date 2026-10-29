use crate::custom_structs::exobiospecies::{ExoBiologySpecies, exobio_species_to_string};
use crate::custom_structs::materials::PlanetRawMaterial;
use crate::elite_journal_data::enums::body_data::{AtmosphereType, BodyParent, Geological, LuminosityClass, PlanetClass, StarClass, TerraformState, Volcanism};
use crate::elite_journal_data::substructs::body_data::{AtmosphericGas, BodyComposition};
use std::fmt::Display;

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
///
pub struct Planet {
    pub body_name: String,
    pub body_id: u64,
    pub system_address: u64,
    pub parents: Vec<BodyParent>,
    pub planet_class: PlanetClass,
    pub terraform_state: TerraformState,
    pub gravity: f64,
    pub mean_temperature: f64,
    pub volcanism: Volcanism,
    pub distance_from_arrival_ls: f64,

    pub atmosphere: AtmosphereInfo,
    pub orbit_data: OrbitalData,
    pub biological_signals: u64,
    pub geological_signals: u64,
    pub human_signals: u64,
    pub thargoid_signals: u64,
    pub guardian_signals: u64,

    pub potential_species: Vec<ExoBiologySpecies>,
    pub confirmed_species: Vec<ExoBiologySpecies>,
    pub confirmed_geology: Vec<Geological>,
    pub body_composition: BodyComposition,
    pub materials: Vec<PlanetRawMaterial>,

    pub was_discovered: bool,
    pub was_mapped: bool,
    pub was_footfalled: bool,
}
#[derive(Default)]
pub struct AtmosphereInfo {
    pub atmosphere_type: AtmosphereType,
    pub quality: Vec<AtmosphereQuality>,
    pub composition: Vec<AtmosphericGas>,
}

pub struct OrbitalData {

}

impl AtmosphereInfo {
    pub fn new(
        atmosphere_type: AtmosphereType,
        atmosphere_quality: String,
        composition_vec: Vec<AtmosphericGas>,
    ) -> Self {
        let quality = AtmosphereQuality::parse_all(&atmosphere_quality);
        AtmosphereInfo {
            atmosphere_type,
            quality,
            composition: composition_vec,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum AtmosphereQuality {
    Thin,
    Thick,
    Hot,
}
impl AtmosphereQuality {
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
impl Display for AtmosphereQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Display for Planet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        string.push_str(&format!("{} - {} \n", self.body_name, self.body_id));
        let atmosphere_traits = self
            .atmosphere
            .quality
            .iter()
            .fold(String::new(), |acc, q| {
                if acc.is_empty() {
                    q.to_string()
                } else {
                    acc + " " + &q.to_string()
                }
            });
        string.push_str(&format!(
            " Atmosphere: {} {} \n",
            atmosphere_traits,
            self.atmosphere.atmosphere_type.to_string()
        ));

        string.push_str(&format!(" Surface Gravity: {}\n", &self.gravity));
        if self.biological_signals > 0 {
            string.push_str(&format!(" Biological signals: {}\n", self.biological_signals));
        }
        if self.potential_species.len() > 0 {
            string.push_str(&format!(
                " Potential species:\n{}",
                exobio_species_to_string(&self.potential_species)
            ));
        }
        write!(f, "{}", string)
    }
}
