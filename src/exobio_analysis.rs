use crate::{check_material, ExoBiologyVariant, ExoBiologySpecies, Species, StarSystem};
use crate::custom_structs::system_info::{Body, Planet, Star};
use crate::elite_events::enums::{AtmosphereType, BodyParent, StarClass, Volcanism};
use crate::elite_events::events::Genus::Bacterium;
use crate::elite_events::events::RawMaterial::{Antimony, Cadmium, Mercury, Molybdenum, Niobium, Polonium, Ruthenium, Technetium, Tellurium, Tin, Tungsten, Yttrium};

fn stellar_class_variant(class: &StarClass) -> ExoBiologyVariant {
    match class {
        StarClass::O => ExoBiologyVariant::Turquoise,
        StarClass::B => ExoBiologyVariant::Grey,
        StarClass::A => ExoBiologyVariant::Yellow,
        StarClass::F => ExoBiologyVariant::Lime,
        StarClass::G => ExoBiologyVariant::Emerald,
        StarClass::K => ExoBiologyVariant::Green,
        StarClass::M => ExoBiologyVariant::Teal,
        StarClass::L => ExoBiologyVariant::Sage,
        StarClass::T => ExoBiologyVariant::Russet,
        StarClass::Y => ExoBiologyVariant::Mauve,
        StarClass::TTS => ExoBiologyVariant::Maroon,
        StarClass::AeBe => ExoBiologyVariant::Orange,
        StarClass::W
        | StarClass::WN
        | StarClass::WNC
        | StarClass::WC
        | StarClass::WO => ExoBiologyVariant::Amber,
        StarClass::D
        | StarClass::DA
        | StarClass::DB
        | StarClass::DC
        | StarClass::DO
        | StarClass::DQ
        | StarClass::DX
        | StarClass::DAV
        | StarClass::DBV
        | StarClass::DCV => ExoBiologyVariant::Ochre,
        StarClass::N => ExoBiologyVariant::Indigo,
        StarClass::H
        | StarClass::SupermassiveBlackHole
        | StarClass::X
        | StarClass::RoguePlanet => ExoBiologyVariant::Unknown,
        _=> {
            println!("Unsupported star type");
            ExoBiologyVariant::Unknown},
    }
}
fn find_parent_star<'a>(system: &'a StarSystem, planet: &Planet) -> Option<&'a Star> {
    let parents = planet.parents.as_ref()?;
    for p in parents {
        if let BodyParent::Star(star_id) = p {
            if let Some(Body::Star(star)) = system.bodies.get(star_id) {
                return Some(star);
            }
        }
    }
    None
}

pub fn get_species_value(species: Species) -> u64
{
    match species {
        Species::Acies | Species::Aurasus | Species::Vesicula => 1_000_000,
        Species::Alcyoneum => 1_658_500,
        Species::Bullaris => 1_152_500,
        Species::Cerbrus => 1_689_800,
        Species::Informem => 8_418_000,
        Species::Nebulus => 5_289_900,
        Species::Omentum => 4_638_900,
        Species::Scopulum => 4_934_500,
        Species::Tela => 1_949_000,
        Species::Verrata => 3_897_000,
        Species::Volu => 7_774_700,
        Species::Bullarum => 3_703_200,
        Species::Gelata => 3_330_300,
        Species::Setisis => 1_670_100,
        Species::Stabitis => 2_680_300,

    }
}


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
                if let Some(materials) = planet.materials.as_ref() {
                    let variant = match () {
                        _ if check_material(materials, Antimony) => ExoBiologyVariant::Magenta,
                        _ if check_material(materials, Polonium) => ExoBiologyVariant::Gold,
                        _ if check_material(materials, Ruthenium) => ExoBiologyVariant::Orange,
                        _ if check_material(materials, Technetium) => ExoBiologyVariant::Cyan,
                        _ if check_material(materials, Tellurium) => ExoBiologyVariant::Green,
                        _ if check_material(materials, Yttrium) => ExoBiologyVariant::Cobalt,
                        _ => ExoBiologyVariant::Unknown,
                    };
                    exobios.push(ExoBiologySpecies{
                        genus: Bacterium,
                        species: Species::Nebulus,
                        variants: vec![variant],
                    });
                }
            }
            AtmosphereType::Neon | AtmosphereType::NeonRich => {
                if let Some(materials) = planet.materials.as_ref() {
                    let variant = match () {
                        _ if check_material(materials, Antimony) => ExoBiologyVariant::Cyan,
                        _ if check_material(materials, Polonium) => ExoBiologyVariant::Magenta,
                        _ if check_material(materials, Ruthenium) => ExoBiologyVariant::Cobalt,
                        _ if check_material(materials, Technetium) => ExoBiologyVariant::Lime,
                        _ if check_material(materials, Tellurium) => ExoBiologyVariant::White,
                        _ if check_material(materials, Yttrium) => ExoBiologyVariant::Aquamarine,
                        _ => ExoBiologyVariant::Unknown,
                    };
                    exobios.push(ExoBiologySpecies{
                        genus: Bacterium,
                        species: Species::Acies,
                        variants: vec![variant],
                    });
                }

                //Omentum - material determinant, requires Nitrogen or Ammonia volcanism
                if matches!(planet.volcanism.as_ref()?,
                        Volcanism::Nitrogen | Volcanism::MajorNitrogen | Volcanism::MinorNitrogen| Volcanism::Ammonia | Volcanism::MajorAmmonia | Volcanism::MinorAmmonia) {
                    if let Some(materials) = planet.materials.as_ref() {
                        let variant = match () {
                            _ if check_material(materials, Cadmium) => ExoBiologyVariant::Lime,
                            _ if check_material(materials, Mercury) => ExoBiologyVariant::White,
                            _ if check_material(materials, Molybdenum) => ExoBiologyVariant::Aquamarine,
                            _ if check_material(materials, Niobium) => ExoBiologyVariant::Peach,
                            _ if check_material(materials, Tin) => ExoBiologyVariant::Red,
                            _ if check_material(materials, Tungsten) => ExoBiologyVariant::Blue,
                            _ => ExoBiologyVariant::Unknown,
                        };
                        exobios.push(ExoBiologySpecies{
                            genus: Bacterium,
                            species: Species::Omentum,
                            variants: vec![variant],
                        });
                    }
                }

                //Scopulum - material determinant, requires Carbon(dioxide) or Methane volcanism
                if matches!(planet.volcanism.as_ref()?,
                        Volcanism::CarbonDioxide | Volcanism::MajorCarbonDioxide | Volcanism::MinorCarbonDioxide | Volcanism::Methane | Volcanism::MajorMethane | Volcanism::MinorMethane) {
                    if let Some(materials) = planet.materials.as_ref() {
                        let variant = match () {
                            _ if check_material(materials, Cadmium) => ExoBiologyVariant::White,
                            _ if check_material(materials, Mercury) => ExoBiologyVariant::Peach,
                            _ if check_material(materials, Molybdenum) => ExoBiologyVariant::Lime,
                            _ if check_material(materials, Niobium) => ExoBiologyVariant::Red,
                            _ if check_material(materials, Tin) => ExoBiologyVariant::Mulberry,
                            _ if check_material(materials, Tungsten) => ExoBiologyVariant::Aquamarine,
                            _ => ExoBiologyVariant::Unknown,
                        };
                        exobios.push(ExoBiologySpecies{
                            genus: Bacterium,
                            species: Species::Scopulum,
                            variants: vec![variant],
                        });
                    }
                }

                //Verrata - material determinant, requires Water volcanism
                if matches!(planet.volcanism.as_ref()?, Volcanism::WaterGeysers | Volcanism::MajorWaterGeysers | Volcanism::MinorWaterGeysers) {
                    if let Some(materials) = planet.materials.as_ref() {
                        let variant = match () {
                            _ if check_material(materials, Cadmium) => ExoBiologyVariant::Peach,
                            _ if check_material(materials, Mercury) => ExoBiologyVariant::Red,
                            _ if check_material(materials, Molybdenum) => ExoBiologyVariant::White,
                            _ if check_material(materials, Niobium) => ExoBiologyVariant::Mulberry,
                            _ if check_material(materials, Tin) => ExoBiologyVariant::Blue,
                            _ if check_material(materials, Tungsten) => ExoBiologyVariant::Lime,
                            _ => ExoBiologyVariant::Unknown,
                        };
                        exobios.push(ExoBiologySpecies{
                            genus: Bacterium,
                            species: Species::Verrata,
                            variants: vec![variant],
                        });
                    }
                }
            }

            AtmosphereType::Methane | AtmosphereType::MethaneRich => {
                //Bullaris
                if let Some(materials) = planet.materials.as_ref() {
                    let variant = match () {
                        _ if check_material(materials, Antimony) => ExoBiologyVariant::Cobalt,
                        _ if check_material(materials, Polonium) => ExoBiologyVariant::Yellow,
                        _ if check_material(materials, Ruthenium) => ExoBiologyVariant::Aquamarine,
                        _ if check_material(materials, Technetium) => ExoBiologyVariant::Gold,
                        _ if check_material(materials, Tellurium) => ExoBiologyVariant::Lime,
                        _ if check_material(materials, Yttrium) => ExoBiologyVariant::Red,
                        _ => ExoBiologyVariant::Unknown,
                    };
                    exobios.push(ExoBiologySpecies{
                        genus: Bacterium,
                        species: Species::Bullaris,
                        variants: vec![variant],
                    });
                }

            }
            AtmosphereType::Argon | AtmosphereType::ArgonRich => {
                //Vesicula
                if let Some(materials) = planet.materials.as_ref() {
                    let variant = match () {
                        _ if check_material(materials, Antimony) => ExoBiologyVariant::Cyan,
                        _ if check_material(materials, Polonium) => ExoBiologyVariant::Orange,
                        _ if check_material(materials, Ruthenium) => ExoBiologyVariant::Mulberry,
                        _ if check_material(materials, Technetium) => ExoBiologyVariant::Gold,
                        _ if check_material(materials, Tellurium) => ExoBiologyVariant::Red,
                        _ if check_material(materials, Yttrium) => ExoBiologyVariant::Lime,
                        _ => ExoBiologyVariant::Unknown,
                    };
                    exobios.push(ExoBiologySpecies{
                        genus: Bacterium,
                        species: Species::Vesicula,
                        variants: vec![variant],
                    });
                }

            }
            AtmosphereType::Nitrogen => {
                //Informem
                if let Some(materials) = planet.materials.as_ref() {
                    let variant = match () {
                        _ if check_material(materials, Antimony) => ExoBiologyVariant::Red,
                        _ if check_material(materials, Polonium) => ExoBiologyVariant::Lime,
                        _ if check_material(materials, Ruthenium) => ExoBiologyVariant::Gold,
                        _ if check_material(materials, Technetium) => ExoBiologyVariant::Aquamarine,
                        _ if check_material(materials, Tellurium) => ExoBiologyVariant::Yellow,
                        _ if check_material(materials, Yttrium) => ExoBiologyVariant::Cobalt,
                        _ => ExoBiologyVariant::Unknown,
                    };
                    exobios.push(ExoBiologySpecies{
                        genus: Bacterium,
                        species: Species::Informem,
                        variants: vec![variant],
                    });
                }

            }
            AtmosphereType::Oxygen => {
                //Volu
                if let Some(materials) = planet.materials.as_ref() {
                    let variant = match () {
                        _ if check_material(materials, Antimony) => ExoBiologyVariant::Red,
                        _ if check_material(materials, Polonium) => ExoBiologyVariant::Aquamarine,
                        _ if check_material(materials, Ruthenium) => ExoBiologyVariant::Cobalt,
                        _ if check_material(materials, Technetium) => ExoBiologyVariant::Lime,
                        _ if check_material(materials, Tellurium) => ExoBiologyVariant::Cyan,
                        _ if check_material(materials, Yttrium) => ExoBiologyVariant::Gold,
                        _ => ExoBiologyVariant::Unknown,
                    };
                    exobios.push(ExoBiologySpecies{
                        genus: Bacterium,
                        species: Species::Volu,
                        variants: vec![variant],
                    });
                }

            }
            AtmosphereType::Ammonia => {
                //Alcyoneum
                if let Some(star) = find_parent_star(system, planet) {
                    let variant = stellar_class_variant(&star.class);
                    exobios.push(ExoBiologySpecies{
                        genus: Bacterium,
                        species: Species::Alcyoneum,
                        variants: vec![variant],
                    });
                }

            }
            AtmosphereType::CarbonDioxide | AtmosphereType::CarbonDioxideRich => {
                //Aurasus
                if let Some(star) = find_parent_star(system, planet) {
                    let variant = stellar_class_variant(&star.class);
                    exobios.push(ExoBiologySpecies{
                        genus: Bacterium,
                        species: Species::Aurasus,
                        variants: vec![variant],
                    });
                }

            }
            AtmosphereType::Water | AtmosphereType::WaterRich | AtmosphereType::SulphurDioxide => {
                //Cerbrus
                if let Some(star) = find_parent_star(system, planet) {
                    let variant = stellar_class_variant(&star.class);
                    exobios.push(ExoBiologySpecies{
                        genus: Bacterium,
                        species: Species::Cerbrus,
                        variants: vec![variant],
                    });
                }

            }
            other => {}
        }
        //Fungoida
        if let Some(planet_gravity) = planet.gravity{
            if planet_gravity < 0.27{
                
            }
        }
        //
    }
    Some(exobios)
}