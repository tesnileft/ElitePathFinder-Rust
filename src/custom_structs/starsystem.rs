use crate::custom_structs::exobiospecies::ExoBiologySpecies;
use crate::custom_structs::system_info::Body;
use crate::custom_structs::system_info::Body::{Planet, Star};
use crate::elite_journal_data::enums::body_data::PlanetClass;
use crate::elite_journal_data::enums::misc::Allegiance;
use crate::elite_journal_data::enums::system_data::{Economy, Government, SystemSecurity};
use crate::exobiology_analysis::determine_species::determine_exobio_species;
use std::collections::{HashMap, HashSet};
use gtk::AccessibleRole::Range;
use crate::elite_journal_data::enums::body_data::StarClass::N;
use crate::elite_journal_data::substructs::body_data::BodySurfaceSignal;

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
    ///Set by FSSAllBodiesFound. Exobio prediction depends on the whole system being scanned
    pub(crate) scan_body_count: Option<u64>,
    pub(crate) scan_body_progress: u64,
    pub(crate) cached_body_signals: HashMap<u64, Vec<BodySurfaceSignal>> //Since FSSBodySignals comes in before Scan does and we do want to have them stored
 }
impl StarSystem{
    ///Returns full list of potential species
    pub fn get_potential_exobio(&self, body_id: u64) -> Option<Vec<ExoBiologySpecies>>{
        determine_exobio_species(self, body_id)
    }
    ///Computes and stores potential species for a planet with biological signals.
    ///Returns (body_name, species) if the planet qualified
    pub fn update_potential_exobio(&mut self, body_id: u64) -> Option<(String, Vec<ExoBiologySpecies>)> {
        let has_bio = matches!(
            self.bodies.get(&body_id),
            Some(Planet(p)) if p.biological_signals > 0
        );
        if !has_bio {
            return None;
        }
        let potential = self.get_potential_exobio(body_id);
        if let Some(Planet(planet)) = self.bodies.get_mut(&body_id) {
            planet.potential_species = potential.clone()?;
            return potential.map(|species| (planet.body_name.clone(), species));
        }
        None
    }
    pub fn add_body(&mut self, id: u64, body: Body)
    {
        if let Planet(ref planet) = body{
            self.present_planets.insert(planet.planet_class.clone());
        }
        if self.bodies.insert(id, body).is_none() {
            //Increment amount of scanned bodies
            self.scan_body_progress += 1;
        }
    }
    pub fn compile_poi_list(& self){
        let mut list: Vec<String> = Vec::new();
        if let Some(body_count) = self.scan_body_count{
            for i in 0..body_count{
                match &self.bodies[&i] {
                    Planet(planet) => {},
                    Star(star) => {
                        if [N].contains(&star.class){
                            list.push(format!("Found a cool star: {}", star.body_name));
                        }
                    }
                }
            }
        }
    }
}