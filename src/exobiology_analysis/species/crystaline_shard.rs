use crate::custom_structs::exobiospecies::ExoBiologySpecies;
use crate::custom_structs::system_info::{Planet, Star};
use crate::elite_journal_data::enums::body_data::{AtmosphereType, PlanetClass, StarClass, Volcanism};
use crate::elite_journal_data::enums::exobiology::Genus::CrystalineShards;
use crate::elite_journal_data::enums::exobiology::Species;

/// Crystalline Shards require an airless, volcanically active Icy or Rocky Ice body
/// at least 12,000 Ls from a parent star of class A, F, G, K, M or S.
/// Unlike other genera, they have no color variant - individual shard clusters
/// just carry different raw materials, tracked separately via Planet::materials.
const MIN_DISTANCE_LS: f64 = 12_000.0;

fn is_qualifying_star(class: &StarClass) -> bool {
    matches!(
        class,
        StarClass::A | StarClass::F | StarClass::G | StarClass::K | StarClass::M | StarClass::S
    )
}

pub fn predict_shards(planet: &Planet, star_distance: f64, parent_stars: &Vec<&Star>) -> Vec<ExoBiologySpecies> {
    let mut exobios = Vec::new();

    if star_distance < MIN_DISTANCE_LS {
        return exobios;
    }

    let is_airless = planet
        .atmosphere
        .as_ref()
        .map(|atmosphere| matches!(atmosphere.atmosphere_type, AtmosphereType::None))
        .unwrap_or(false);
    let has_volcanism = !matches!(planet.volcanism, Some(Volcanism::None) | None);
    let is_icy_body = matches!(planet.planet_class, Some(PlanetClass::Icy) | Some(PlanetClass::RockyIce));
    let has_qualifying_star = parent_stars.iter().any(|star| is_qualifying_star(&star.class));

    if is_airless && has_volcanism && is_icy_body && has_qualifying_star {
        exobios.push(ExoBiologySpecies::new(CrystalineShards, Species::CrystalineShards, Vec::new()));
    }

    exobios
}
