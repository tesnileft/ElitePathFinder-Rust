use crate::custom_structs::exobiospecies::ExoBiologySpecies;
use crate::custom_structs::system_info::Body;
use crate::elite_journal_data::enums::system_data::{Economy, Government, SystemSecurity};
use crate::exobiology_analysis::determine_species::determine_exobio_species;
use std::collections::{HashMap, HashSet};
use crate::custom_structs::system_info::Body::Planet;
use crate::elite_journal_data::enums::body_data::{BodyType, PlanetClass};
use crate::elite_journal_data::enums::misc::Allegiance;

#[derive(Default)]
pub struct StarSystem{
    pub(crate) name: String,
    pub(crate) address: u64,
    pub(crate) bodies: HashMap<u64, Body>,
    pub(crate) star_position: (f64, f64, f64), //Galactic Coordinates
    pub(crate) security: SystemSecurity,
    pub(crate) allegiance: Allegiance,
    pub(crate) economy: Economy,
    pub(crate) second_economy: Economy,
    pub(crate) government: Government,
    pub(crate) present_planets: HashSet<PlanetClass>,
 }
impl StarSystem{
    ///Returns full list of potential species
    pub fn get_potential_exobio(&self, body_id: u64) -> Option<Vec<ExoBiologySpecies>>{
        determine_exobio_species(self, body_id)
    }
    pub fn add_body(&mut self, id: u64, body: Body)
    {

        if let Planet(ref planet) = body{
            if let Some(planet_type) = &planet.planet_class{
                self.present_planets.insert(*planet_type);
            }
        }
        self.bodies.insert(id, body);
    }
}