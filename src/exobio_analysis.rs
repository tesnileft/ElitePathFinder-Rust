use crate::{check_material, ExoBiologyVariant, ExoBiologySpecies, Species, StarSystem};
use crate::custom_structs::system_info::Body;
use crate::elite_events::enums::{AtmosphereType, Volcanism};
use crate::elite_events::events::Genus::Bacterium;
use crate::elite_events::events::RawMaterial::{Cadmium, Mercury, Molybdenum, Niobium, Tin, Tungsten};

pub fn determine_exobio_species(system: &StarSystem, body_id: u64) -> Option<Vec<ExoBiologySpecies>> {
    let body = &system.bodies[&body_id];
    let mut exobios: Vec<ExoBiologySpecies> = Vec::new();
    if let Body::Planet(planet) = body {
        //Bacterium
        if matches!(planet.volcanism.as_ref().clone()?, //Excludes others
                Volcanism::Helium
                | Volcanism::Iron
                | Volcanism::MajorSilicateVapour
                | Volcanism::MinorSilicateVapour){
            //Tela
            let materials = planet.materials.as_ref()?;
            let variant = match () {
                _ if check_material(materials, Cadmium) => ExoBiologyVariant::Gold,
                _ if check_material(materials, Mercury) => ExoBiologyVariant::Orange,
                _ if check_material(materials, Molybdenum) => ExoBiologyVariant::Yellow,
                _ if check_material(materials, Niobium) => ExoBiologyVariant::Magenta,
                _ if check_material(materials, Tungsten) => ExoBiologyVariant::Green,
                _ if check_material(materials, Tin) => ExoBiologyVariant::Cobalt,
                _ => ExoBiologyVariant::Unknown,
            };
            exobios.push(ExoBiologySpecies{
                genus: Bacterium,
                species: Species::Tela,
                variants: vec![variant],
            });
        }
        match planet.atmosphere_type.as_ref()? {
            AtmosphereType::Helium =>{
                //Nebulus
            }
            AtmosphereType::Neon | AtmosphereType::NeonRich => {
                //Acies
                //Omentum
                //Scopulum
                //Verrata
            }

            AtmosphereType::Methane | AtmosphereType::MethaneRich => {
                //Bullaris
            }
            AtmosphereType::Argon | AtmosphereType::ArgonRich => {
                //Vesicula
            }
            AtmosphereType::Nitrogen => {
                //Informem
            }
            AtmosphereType::Oxygen => {
                //Volu
            }
            AtmosphereType::Ammonia => {
                //Alcyoneum
            }
            AtmosphereType::CarbonDioxide | AtmosphereType::CarbonDioxideRich => {
                //Aurasus
            }
            AtmosphereType::Water | AtmosphereType::WaterRich | AtmosphereType::SulphurDioxide => {
                //Cerbrus
            }
            other => {}
        }
    }
    Some(exobios)
}