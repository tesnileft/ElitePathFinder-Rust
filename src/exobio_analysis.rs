use crate::{check_material, ExoBiologyVariant, ExoBiologySpecies, Species, StarSystem};
use crate::custom_structs::system_info::{AtmosphereQuality, Body, Planet, Star};
use crate::elite_events::enums::{AtmosphereType, BodyParent, PlanetClass, StarClass, Volcanism};
use crate::elite_events::enums::Volcanism::Methane;
use crate::elite_events::events::Genus::{Bacterium, Concha, Fonticulua, Frutexa, Fungoida, Tussock};
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
fn concha_stellar_variant(class: &StarClass) -> ExoBiologyVariant {
    match class {
        StarClass::B => ExoBiologyVariant::Indigo,
        StarClass::A => ExoBiologyVariant::Teal,
        StarClass::F => ExoBiologyVariant::Grey,
        StarClass::G => ExoBiologyVariant::Turquoise,
        StarClass::K => ExoBiologyVariant::Red,
        StarClass::L => ExoBiologyVariant::Orange,
        StarClass::Y => ExoBiologyVariant::Yellow,
        StarClass::N => ExoBiologyVariant::Emerald,
        StarClass::W
        | StarClass::WN
        | StarClass::WNC
        | StarClass::WC
        | StarClass::WO => ExoBiologyVariant::Lime,
        _ => ExoBiologyVariant::Unknown,
    }
}
fn fonticulua_stellar_variant(class: &StarClass) -> ExoBiologyVariant {
    match class {
        StarClass::O => ExoBiologyVariant::Grey,
        StarClass::B => ExoBiologyVariant::Lime,
        StarClass::A => ExoBiologyVariant::Green,
        StarClass::F => ExoBiologyVariant::Yellow,
        StarClass::G => ExoBiologyVariant::Teal,
        StarClass::K => ExoBiologyVariant::Emerald,
        StarClass::M => ExoBiologyVariant::Amethyst,
        StarClass::L => ExoBiologyVariant::Mauve,
        StarClass::T => ExoBiologyVariant::Orange,
        StarClass::TTS => ExoBiologyVariant::Red,
        StarClass::Y => ExoBiologyVariant::Ochre,
        StarClass::W
        | StarClass::WN
        | StarClass::WNC
        | StarClass::WC
        | StarClass::WO => ExoBiologyVariant::Indigo,
        StarClass::D
        | StarClass::DA
        | StarClass::DB
        | StarClass::DC
        | StarClass::DO
        | StarClass::DQ
        | StarClass::DX
        | StarClass::DAV
        | StarClass::DBV
        | StarClass::DCV => ExoBiologyVariant::Turquoise,
        StarClass::N => ExoBiologyVariant::Sage,
        StarClass::AeBe => ExoBiologyVariant::Maroon,
        _ => ExoBiologyVariant::Unknown,
    }
}
fn tussock_stellar_variant(class: &StarClass) -> ExoBiologyVariant {
    match class {
        StarClass::F => ExoBiologyVariant::Yellow,
        StarClass::G => ExoBiologyVariant::Lime,
        StarClass::K => ExoBiologyVariant::Green,
        StarClass::M => ExoBiologyVariant::Emerald,
        StarClass::L => ExoBiologyVariant::Sage,
        StarClass::T => ExoBiologyVariant::Teal,
        StarClass::W
        | StarClass::WN
        | StarClass::WNC
        | StarClass::WC
        | StarClass::WO => ExoBiologyVariant::Orange,
        StarClass::D
        | StarClass::DA
        | StarClass::DB
        | StarClass::DC
        | StarClass::DO
        | StarClass::DQ
        | StarClass::DX
        | StarClass::DAV
        | StarClass::DBV
        | StarClass::DCV => ExoBiologyVariant::Maroon,
        StarClass::H => ExoBiologyVariant::Red,
        _ => ExoBiologyVariant::Unknown,
    }
}
fn frutexa_stellar_variant(class: &StarClass) -> ExoBiologyVariant {
    match class {
        StarClass::O => ExoBiologyVariant::Yellow,
        StarClass::B => ExoBiologyVariant::Lime,
        StarClass::F => ExoBiologyVariant::Green,
        StarClass::G => ExoBiologyVariant::Emerald,
        StarClass::M => ExoBiologyVariant::Grey,
        StarClass::L => ExoBiologyVariant::Teal,
        StarClass::TTS => ExoBiologyVariant::Mauve,
        StarClass::W
        | StarClass::WN
        | StarClass::WNC
        | StarClass::WC
        | StarClass::WO => ExoBiologyVariant::Orange,
        StarClass::D
        | StarClass::DA
        | StarClass::DB
        | StarClass::DC
        | StarClass::DO
        | StarClass::DQ
        | StarClass::DX
        | StarClass::DAV
        | StarClass::DBV
        | StarClass::DCV => ExoBiologyVariant::Indigo,
        StarClass::N => ExoBiologyVariant::Red,
        _ => ExoBiologyVariant::Unknown,
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
        //Aleoida
        Species::Arcus => 7_252_500,
        Species::Coronamus => 6_284_600,
        Species::Gravis => 12_935_900,
        Species::Laminiae | Species::Spica => 3_385_200,
        //Bacterium
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
        //Cactoida
        Species::Cortexum | Species::Pullulanta=> 3_667_600,
        Species::Lapis | Species::Peperatus => 2_483_600,
        Species::Vermis => 16_202_800,
        //Concha
        Species::Aureolas => 7_774_700,
        Species::Biconcavis => 19_010_800,
        Species::Labiata => 2_352_400,
        Species::Renibus => 4_572_400,
        //Electricae
        Species::Pluma | Species::Radialem => 6_284_600,
        //Fonticulua
        Species::Campestris => 1_000_000,
        Species::Digitos => 1_804_100,
        Species::Fluctus => 20_000_000,
        Species::Lapida => 3_111_000,
        Species::Segmentatus => 19_010_800,
        Species::Upupam => 5_727_600,
        //Frutexa
        Species::Acus => 7_774_700,
        Species::Collum => 1_639_800,
        Species::Fera => 1_632_500,
        Species::Flabellum => 1_808_900,
        Species::Flammasis => 10_326_000,
        Species::Metallicum => 1_632_500,
        Species::Sponsae => 5_988_000,
        //Fumerola
        Species::Aquatis | Species::Carbosis => 6_284_600,
        Species::Extremus => 12_202_800,
        Species::Nitris => 7_500_900,
        //Fungoida
        Species::Bullarum => 3_703_200,
        Species::Gelata => 3_330_300,
        Species::Setisis => 1_670_100,
        Species::Stabitis => 2_680_300,
        //Osseus
        Species::Cornibus => 1_483_000,
        Species::Discus => 12_934_900,
        Species::Fractus => 4_027_800,
        Species::Pellebantus => 9_739_000,
        Species::Pumice => 3_156_300,
        Species::Spiralis => 2_404_700,
        //Recepta
        Species::Conditivus => 14_313_700,
        Species::Deltahedronix => 16_202_800,
        Species::Umbrux => 12_934_900,
        //Stratum
        Species::Araneamus | Species::Excutitus => 2_448_900,
        Species::Cucumisis => 16_202_800,
        Species::Frigus => 2_637_500,
        Species::Laminamus => 2_788_300,
        Species::Limaxus | Species::Paleas => 1_362_000,
        Species::Tectonicas => 19_010_800,
        //Tubus
        Species::Cavas => 11_873_200,
        Species::Compagibus => 7_774_700,
        Species::Conifer => 2_415_500,
        Species::Rosarium => 2_637_500,
        Species::Sororibus => 7_727_600,
        //Tussock
        Species::Albata => 3_252_500,
        Species::Capillum => 7_025_800,
        Species::Caputus => 3_472_400,
        Species::Catena => 1_766_600,
        Species::Cultro => 1_766_600,
        Species::Divisa => 1_766_600,
        Species::Ignis => 1_849_000,
        Species::Pennata => 5_853_800,
        Species::Pennatis => 1_000_000,
        Species::Propagito => 1_000_000,
        Species::Serrati => 4_447_100,
        Species::Stigmasis => 19_010_800,
        Species::Triticum => 7_774_700,
        Species::Ventusa => 3_227_700,
        Species::Virgam => 14_313_700,
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
        match &planet.atmosphere.as_ref()?.atmosphere_type {
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
            _ => {}
        }
        //Fungoida
        if let (Some(planet_gravity), Some(atmosphere)) = (planet.gravity, planet.atmosphere.as_ref()){
            if planet_gravity < 0.27 && atmosphere.quality.contains(&AtmosphereQuality::Thin){ // Gravity below 0.27, and thin atmo.
                match atmosphere.atmosphere_type {
                    AtmosphereType::Ammonia | AtmosphereType::Methane| AtmosphereType::MethaneRich=> {
                        //Setisis
                        if let Some(materials) = planet.materials.as_ref() {
                            let variant = match () {
                                _ if check_material(materials, Antimony) => ExoBiologyVariant::Peach,
                                _ if check_material(materials, Polonium) => ExoBiologyVariant::White,
                                _ if check_material(materials, Ruthenium) => ExoBiologyVariant::Gold,
                                _ if check_material(materials, Technetium) => ExoBiologyVariant::Lime,
                                _ if check_material(materials, Tellurium) => ExoBiologyVariant::Yellow,
                                _ if check_material(materials, Yttrium) => ExoBiologyVariant::Orange,
                                _ => ExoBiologyVariant::Unknown,
                            };
                            exobios.push(ExoBiologySpecies{
                                genus: Fungoida,
                                species: Species::Setisis,
                                variants: vec![variant],
                            });
                        }
                    }
                    AtmosphereType::Argon | AtmosphereType::ArgonRich =>{
                        //Bullarum
                        if let Some(materials) = planet.materials.as_ref() {
                            let variant = match () {
                                _ if check_material(materials, Antimony) => ExoBiologyVariant::Red,
                                _ if check_material(materials, Polonium) => ExoBiologyVariant::Mulberry,
                                _ if check_material(materials, Ruthenium) => ExoBiologyVariant::Magenta,
                                _ if check_material(materials, Technetium) => ExoBiologyVariant::Peach,
                                _ if check_material(materials, Tellurium) => ExoBiologyVariant::Gold,
                                _ if check_material(materials, Yttrium) => ExoBiologyVariant::Orange,
                                _ => ExoBiologyVariant::Unknown,
                            };
                            exobios.push(ExoBiologySpecies{
                                genus: Fungoida,
                                species: Species::Bullarum,
                                variants: vec![variant],
                            });
                        }
                    }
                    AtmosphereType::CarbonDioxide | AtmosphereType::CarbonDioxideRich | AtmosphereType::Water | AtmosphereType::WaterRich=> {
                        //Gelata and Stabitis
                        //Shared conditions
                        let can_occur = !matches!(atmosphere.atmosphere_type, AtmosphereType::CarbonDioxide | AtmosphereType::CarbonDioxideRich)
                            || planet.mean_temperature.is_some_and(|t| (180.0..=195.0).contains(&t));
                        if can_occur {
                            if let Some(materials) = planet.materials.as_ref() {
                                //Gelata
                                let gelata_variant = match () {
                                    _ if check_material(materials, Cadmium) => ExoBiologyVariant::Cyan,
                                    _ if check_material(materials, Mercury) => ExoBiologyVariant::Lime,
                                    _ if check_material(materials, Molybdenum) => ExoBiologyVariant::Mulberry,
                                    _ if check_material(materials, Niobium) => ExoBiologyVariant::Green,
                                    _ if check_material(materials, Tungsten) => ExoBiologyVariant::Orange,
                                    _ if check_material(materials, Tin) => ExoBiologyVariant::Red,
                                    _ => ExoBiologyVariant::Unknown,
                                };
                                exobios.push(ExoBiologySpecies{
                                    genus: Fungoida,
                                    species: Species::Gelata,
                                    variants: vec![gelata_variant],
                                });

                                //Stabitis
                                let stabitis_variant = match () {
                                    _ if check_material(materials, Cadmium) => ExoBiologyVariant::Blue,
                                    _ if check_material(materials, Mercury) => ExoBiologyVariant::Green,
                                    _ if check_material(materials, Molybdenum) => ExoBiologyVariant::Magenta,
                                    _ if check_material(materials, Niobium) => ExoBiologyVariant::White,
                                    _ if check_material(materials, Tungsten) => ExoBiologyVariant::Peach,
                                    _ if check_material(materials, Tin) => ExoBiologyVariant::Orange,
                                    _ => ExoBiologyVariant::Unknown,
                                };
                                exobios.push(ExoBiologySpecies{
                                    genus: Fungoida,
                                    species: Species::Stabitis,
                                    variants: vec![stabitis_variant],
                                });
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        //region Concha
        if let (Some(planet_gravity), Some(atmosphere)) = (planet.gravity, planet.atmosphere.as_ref()){
            if planet_gravity < 0.27 && atmosphere.quality.contains(&AtmosphereQuality::Thin){ // Gravity below 0.27, and thin atmo.
                match atmosphere.atmosphere_type {
                    AtmosphereType::Ammonia => {
                        //Aureolas
                        if let Some(star) = find_parent_star(system, planet) {
                            let variant = concha_stellar_variant(&star.class);
                            exobios.push(ExoBiologySpecies{
                                genus: Concha,
                                species: Species::Aureolas,
                                variants: vec![variant],
                            });
                        }
                    }
                    AtmosphereType::Nitrogen => {
                        //Biconcavis
                        if let Some(materials) = planet.materials.as_ref() {
                            let variant = match () {
                                _ if check_material(materials, Antimony) => ExoBiologyVariant::Peach,
                                _ if check_material(materials, Polonium) => ExoBiologyVariant::Red,
                                _ if check_material(materials, Ruthenium) => ExoBiologyVariant::Orange,
                                _ if check_material(materials, Tellurium) => ExoBiologyVariant::Yellow,
                                _ => ExoBiologyVariant::Unknown,
                            };
                            exobios.push(ExoBiologySpecies{
                                genus: Concha,
                                species: Species::Biconcavis,
                                variants: vec![variant],
                            });
                        }
                    }
                    AtmosphereType::CarbonDioxide | AtmosphereType::CarbonDioxideRich => {
                        //Labiata - mean temperature below 190 K
                        if planet.mean_temperature.is_some_and(|t| t < 190.0) {
                            if let Some(star) = find_parent_star(system, planet) {
                                let variant = concha_stellar_variant(&star.class);
                                exobios.push(ExoBiologySpecies{
                                    genus: Concha,
                                    species: Species::Labiata,
                                    variants: vec![variant],
                                });
                            }
                        }
                        //Renibus - mean temperature between 180 K and 195 K
                        if planet.mean_temperature.is_some_and(|t| (180.0..=195.0).contains(&t)) {
                            if let Some(materials) = planet.materials.as_ref() {
                                let variant = match () {
                                    _ if check_material(materials, Cadmium) => ExoBiologyVariant::Red,
                                    _ if check_material(materials, Mercury) => ExoBiologyVariant::Mulberry,
                                    _ if check_material(materials, Molybdenum) => ExoBiologyVariant::Peach,
                                    _ if check_material(materials, Niobium) => ExoBiologyVariant::Blue,
                                    _ if check_material(materials, Tungsten) => ExoBiologyVariant::White,
                                    _ if check_material(materials, Tin) => ExoBiologyVariant::Aquamarine,
                                    _ => ExoBiologyVariant::Unknown,
                                };
                                exobios.push(ExoBiologySpecies{
                                    genus: Concha,
                                    species: Species::Renibus,
                                    variants: vec![variant],
                                });
                            }
                        }
                    }
                    AtmosphereType::Water | AtmosphereType::WaterRich => {
                        //Renibus
                        if let Some(materials) = planet.materials.as_ref() {
                            let variant = match () {
                                _ if check_material(materials, Cadmium) => ExoBiologyVariant::Red,
                                _ if check_material(materials, Mercury) => ExoBiologyVariant::Mulberry,
                                _ if check_material(materials, Molybdenum) => ExoBiologyVariant::Peach,
                                _ if check_material(materials, Niobium) => ExoBiologyVariant::Blue,
                                _ if check_material(materials, Tungsten) => ExoBiologyVariant::White,
                                _ if check_material(materials, Tin) => ExoBiologyVariant::Aquamarine,
                                _ => ExoBiologyVariant::Unknown,
                            };
                            exobios.push(ExoBiologySpecies{
                                genus: Concha,
                                species: Species::Renibus,
                                variants: vec![variant],
                            });
                        }
                    }
                    _ => {}
                }
            }
        }
        //endregion
        //region Fonticulua
        if let (Some(planet_gravity), Some(atmosphere)) = (planet.gravity, planet.atmosphere.as_ref()){
            if planet_gravity < 0.27
                && atmosphere.quality.contains(&AtmosphereQuality::Thin)
                && matches!(planet.planet_class, Some(PlanetClass::Icy) | Some(PlanetClass::RockyIce))
            {
                let species = match atmosphere.atmosphere_type {
                    AtmosphereType::Neon | AtmosphereType::NeonRich => Some(Species::Segmentatus),
                    AtmosphereType::Methane | AtmosphereType::MethaneRich => Some(Species::Digitos),
                    AtmosphereType::Argon => Some(Species::Campestris),
                    AtmosphereType::ArgonRich => Some(Species::Upupam),
                    AtmosphereType::Nitrogen => Some(Species::Lapida),
                    AtmosphereType::Oxygen => Some(Species::Fluctus),
                    _ => None,
                };
                if let Some(species) = species {
                    if let Some(star) = find_parent_star(system, planet) {
                        let variant = fonticulua_stellar_variant(&star.class);
                        exobios.push(ExoBiologySpecies{
                            genus: Fonticulua,
                            species,
                            variants: vec![variant],
                        });
                    }
                }
            }
        }
        //endregion
        //region Tussock
        if let (Some(planet_gravity), Some(atmosphere)) = (planet.gravity, planet.atmosphere.as_ref()){
            if planet_gravity < 0.27
                && atmosphere.quality.contains(&AtmosphereQuality::Thin)
                && matches!(planet.planet_class, Some(PlanetClass::Rocky))
            {
                match atmosphere.atmosphere_type {
                    AtmosphereType::Methane | AtmosphereType::MethaneRich | AtmosphereType::Argon | AtmosphereType::ArgonRich => {
                        //Capillum
                        if let Some(star) = find_parent_star(system, planet) {
                            exobios.push(ExoBiologySpecies{
                                genus: Tussock,
                                species: Species::Capillum,
                                variants: vec![tussock_stellar_variant(&star.class)],
                            });
                        }
                    }
                    AtmosphereType::Ammonia => {
                        //Catena, Cultro, Divisa - all occur under identical conditions
                        if let Some(star) = find_parent_star(system, planet) {
                            for species in [Species::Catena, Species::Cultro, Species::Divisa] {
                                exobios.push(ExoBiologySpecies{
                                    genus: Tussock,
                                    species,
                                    variants: vec![tussock_stellar_variant(&star.class)],
                                });
                            }
                        }
                    }
                    AtmosphereType::CarbonDioxide | AtmosphereType::CarbonDioxideRich => {
                        if let Some(temp) = planet.mean_temperature {
                            if let Some(star) = find_parent_star(system, planet) {
                                //Propagito and Pennatis - anywhere below 195 K
                                if temp < 195.0 {
                                    exobios.push(ExoBiologySpecies{
                                        genus: Tussock,
                                        species: Species::Propagito,
                                        variants: vec![tussock_stellar_variant(&star.class)],
                                    });
                                    exobios.push(ExoBiologySpecies{
                                        genus: Tussock,
                                        species: Species::Pennatis,
                                        variants: vec![tussock_stellar_variant(&star.class)],
                                    });
                                }
                                //Temperature-banded species
                                let banded = match () {
                                    _ if (145.0..155.0).contains(&temp) => Some(Species::Pennata),
                                    _ if (155.0..160.0).contains(&temp) => Some(Species::Ventusa),
                                    _ if (160.0..170.0).contains(&temp) => Some(Species::Ignis),
                                    _ if (170.0..175.0).contains(&temp) => Some(Species::Serrati),
                                    _ if (175.0..180.0).contains(&temp) => Some(Species::Albata),
                                    _ if (180.0..190.0).contains(&temp) => Some(Species::Caputus),
                                    _ if (190.0..=195.0).contains(&temp) => Some(Species::Triticum),
                                    _ => None,
                                };
                                if let Some(species) = banded {
                                    exobios.push(ExoBiologySpecies{
                                        genus: Tussock,
                                        species,
                                        variants: vec![tussock_stellar_variant(&star.class)],
                                    });
                                }
                            }
                        }
                    }
                    AtmosphereType::Water | AtmosphereType::WaterRich => {
                        //Virgam
                        if let Some(star) = find_parent_star(system, planet) {
                            exobios.push(ExoBiologySpecies{
                                genus: Tussock,
                                species: Species::Virgam,
                                variants: vec![tussock_stellar_variant(&star.class)],
                            });
                        }
                    }
                    AtmosphereType::SulphurDioxide => {
                        //Stigmasis
                        if let Some(star) = find_parent_star(system, planet) {
                            exobios.push(ExoBiologySpecies{
                                genus: Tussock,
                                species: Species::Stigmasis,
                                variants: vec![tussock_stellar_variant(&star.class)],
                            });
                        }
                    }
                    _ => {}
                }
            }
        }
        //endregion
        //region Frutexa
        if let (Some(planet_gravity), Some(atmosphere)) = (planet.gravity, planet.atmosphere.as_ref()){
            if planet_gravity < 0.27 && atmosphere.quality.contains(&AtmosphereQuality::Thin) {
                let is_rocky = matches!(planet.planet_class, Some(PlanetClass::Rocky));
                let is_hmc = matches!(planet.planet_class, Some(PlanetClass::HMC));
                match atmosphere.atmosphere_type {
                    AtmosphereType::Ammonia => {
                        if let Some(star) = find_parent_star(system, planet) {
                            if is_rocky {
                                //Flabellum, Flammasis
                                exobios.push(ExoBiologySpecies{
                                    genus: Frutexa,
                                    species: Species::Flabellum,
                                    variants: vec![frutexa_stellar_variant(&star.class)],
                                });
                                exobios.push(ExoBiologySpecies{
                                    genus: Frutexa,
                                    species: Species::Flammasis,
                                    variants: vec![frutexa_stellar_variant(&star.class)],
                                });
                            }
                            if is_hmc {
                                //Metallicum
                                exobios.push(ExoBiologySpecies{
                                    genus: Frutexa,
                                    species: Species::Metallicum,
                                    variants: vec![frutexa_stellar_variant(&star.class)],
                                });
                            }
                        }
                    }
                    AtmosphereType::CarbonDioxide | AtmosphereType::CarbonDioxideRich => {
                        if planet.mean_temperature.is_some_and(|t| t < 195.0) {
                            if let Some(star) = find_parent_star(system, planet) {
                                if is_rocky {
                                    //Acus, Fera
                                    exobios.push(ExoBiologySpecies{
                                        genus: Frutexa,
                                        species: Species::Acus,
                                        variants: vec![frutexa_stellar_variant(&star.class)],
                                    });
                                    exobios.push(ExoBiologySpecies{
                                        genus: Frutexa,
                                        species: Species::Fera,
                                        variants: vec![frutexa_stellar_variant(&star.class)],
                                    });
                                }
                                if is_hmc {
                                    //Metallicum
                                    exobios.push(ExoBiologySpecies{
                                        genus: Frutexa,
                                        species: Species::Metallicum,
                                        variants: vec![frutexa_stellar_variant(&star.class)],
                                    });
                                }
                            }
                        }
                    }
                    AtmosphereType::Water | AtmosphereType::WaterRich => {
                        if is_rocky {
                            //Sponsae
                            if let Some(star) = find_parent_star(system, planet) {
                                exobios.push(ExoBiologySpecies{
                                    genus: Frutexa,
                                    species: Species::Sponsae,
                                    variants: vec![frutexa_stellar_variant(&star.class)],
                                });
                            }
                        }
                    }
                    AtmosphereType::SulphurDioxide => {
                        if is_rocky {
                            //Collum
                            if let Some(star) = find_parent_star(system, planet) {
                                exobios.push(ExoBiologySpecies{
                                    genus: Frutexa,
                                    species: Species::Collum,
                                    variants: vec![frutexa_stellar_variant(&star.class)],
                                });
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        //endregion
        //region Tubus

        //endregion
    }
    Some(exobios)
}