use crate::elite_journal_data::enums::body_data::{AtmosphereType, BodyParent, PlanetClass, StarClass, Volcanism};
use crate::elite_journal_data::enums::signals::SignalType;
use crate::elite_journal_data::substructs::body_data::{AtmosphericGas, BodyComposition, BodySurfaceSignal};
use crate::elite_journal_data::substructs::body_data::{CelestialRings, RawMaterialInfo};
use chrono::{DateTime, Utc};
use serde::Deserialize;

//region - FSS -
#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct FSSSignal {
    #[serde(rename = "SignalName")]
    signal_name: String,
    #[serde(rename = "SignalType")]
    signal_type: SignalType,
    #[serde(rename = "IsStation")]
    is_station: Option<bool>,
}

#[derive(Deserialize)]
pub struct FSSSignalDiscovered {
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "SystemAddress")]
    pub system_address: u64,
    #[serde(rename = "SignalName")]
    pub signal_name: String,
    #[serde(rename = "SignalType")]
    pub signal_type: SignalType,
    #[serde(rename = "IsStation")]
    pub is_station: Option<bool>,
}

#[derive(Deserialize)]
pub struct FSSDiscoveryScan {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Progress")]
    progress: f64,
    #[serde(rename = "BodyCount")]
    body_count: u64,
    #[serde(rename = "NonBodyCount")]
    non_body_count: u64,
    #[serde(rename = "SystemAddress")]
    system_address: u64,
}

#[derive(Deserialize)]
pub struct FSSBodySignals {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "BodyName")]
    pub(crate) body_name: String,
    #[serde(rename = "BodyID")]
    pub(crate) body_id: u64,
    #[serde(rename = "SystemAddress")]
    pub(crate) system_address: u64,
    #[serde(rename = "Signals")]
    pub signals: Vec<BodySurfaceSignal>,
}

///This struct is so omega dumb because the devs thought putting planets, stars, and EVERY OTHER ASTRAL BODY scans in one event was a GREAT idea!!! hence we have 1389040 optional fields that you need to check sdlknfasJDBLKJRGB;PIQWUBGPOIBDW
#[derive(Deserialize)]
pub struct Scan {
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "ScanType")]
    pub scan_type: String,
    #[serde(rename = "BodyName")]
    pub body_name: String,
    #[serde(rename = "BodyID")]
    pub body_id: u64,
    #[serde(rename = "Parents")]
    pub parents: Option<Vec<BodyParent>>,
    #[serde(rename = "StarSystem")]
    pub star_system: String,
    #[serde(rename = "SystemAddress")]
    pub system_address: u64,
    #[serde(rename = "DistanceFromArrivalLS")]
    pub distance_from_arrival_ls: f64,
    #[serde(rename = "TidalLock")]
    #[serde(default)]
    pub tidal_lock: bool,
    #[serde(rename = "TerraformState")]
    #[serde(default)]
    pub terraform_state: String,
    #[serde(rename = "PlanetClass")]
    pub planet_class: Option<PlanetClass>,
    #[serde(rename = "StarType")]
    pub star_class: Option<StarClass>,
    #[serde(rename = "Subclass")]
    pub star_subclass: Option<u64>,
    #[serde(rename = "Age_MY")]
    pub age_million_years: Option<u64>,
    #[serde(rename = "Luminosity")]
    pub luminosity: Option<String>,
    #[serde(rename = "Rings")]
    pub rings: Option<Vec<CelestialRings>>,
    #[serde(rename = "Atmosphere")]
    #[serde(default)]
    pub atmosphere: String,
    #[serde(rename = "AtmosphereType")]
    #[serde(default)]
    pub atmosphere_type: AtmosphereType,
    #[serde(rename = "AtmosphereComposition")]
    pub atmosphere_composition: Option<Vec<AtmosphericGas>>,
    #[serde(rename = "Volcanism")]
    #[serde(default)]
    pub volcanism: Volcanism,
    #[serde(rename = "StellarMass")]
    pub stellar_mass: Option<f64>,
    #[serde(rename = "MassEM")]
    pub mass_em: Option<f64>,
    #[serde(rename = "Radius")]
    pub radius: Option<f64>,
    #[serde(rename = "SurfaceGravity")]
    pub surface_gravity: Option<f64>, //Only for bodies that arent stars
    #[serde(rename = "SurfaceTemperature")]
    pub surface_temperature: Option<f64>, //Exists for all major bodies
    #[serde(rename = "SurfacePressure")]
    pub surface_pressure: Option<f64>, //Only exists for planets with an atmosphere
    #[serde(rename = "Landable")]
    #[serde(default)]
    pub landable: bool,
    #[serde(rename = "Materials")]
    #[serde(default)]
    pub materials: Vec<RawMaterialInfo>, //Only exists for planets that have materials on them
    #[serde(rename = "Composition")]
    pub composition: Option<BodyComposition>, //Only exists for some planets
    #[serde(rename = "SemiMajorAxis")]
    pub semimajor_axis: Option<f64>, // Does not exist for single major celestial bodies
    #[serde(rename = "Eccentricity")]
    pub eccentricity: Option<f64>, //How much the orbit deviates from a perfect circle, doesn't exist for non-orbiting bodies
    #[serde(rename = "OrbitalInclination")]
    pub orbital_inclination: Option<f64>, //How much a body swings above/below the plane of reference of its parent body
    #[serde(rename = "Periapsis")]
    pub periapsis: Option<f64>,
    #[serde(rename = "OrbitalPeriod")]
    pub orbital_period: Option<f64>, //How long one orbit takes
    #[serde(rename = "AscendingNode")]
    pub ascending_node: Option<f64>, //Where the celestial body moves north through the plane of reference
    #[serde(rename = "MeanAnomaly")]
    pub mean_anomaly: Option<f64>, //Fraction how far the elliptical orbit has gone since passing through it's periapsis (time wise, not distance)
    #[serde(rename = "RotationPeriod")]
    pub rotation_period: Option<f64>,
    #[serde(rename = "AxialTilt")]
    pub axial_tilt: Option<f64>,
    #[serde(rename = "WasDiscovered")]
    pub was_discovered: bool,
    #[serde(rename = "WasMapped")]
    pub was_mapped: bool,
    #[serde(rename = "WasFootfalled")]
    pub was_footfalled: bool,
    //TODO
}

#[derive(Deserialize)]
pub struct FSSAllBodiesFound {
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "SystemName")]
    pub system_name: String,
    #[serde(rename = "SystemAddress")]
    pub system_address: u64,
    #[serde(rename = "Count")]
    pub count: u64,
}

#[derive(Deserialize)]
pub struct ScanBaryCentre {
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "StarSystem")]
    pub star_system: String,
    #[serde(rename = "SystemAddress")]
    pub system_address: u64,
    #[serde(rename = "BodyID")]
    pub body_id: u64,
    #[serde(rename = "SemiMajorAxis")]
    pub semimajor_axis: f64,
    #[serde(rename = "Eccentricity")]
    pub eccentricity: f64,
    #[serde(rename = "OrbitalInclination")]
    pub orbital_inclination: f64,
    #[serde(rename = "Periapsis")]
    pub periapsis: f64,
    #[serde(rename = "OrbitalPeriod")]
    pub orbital_period: f64,
    #[serde(rename = "AscendingNode")]
    pub ascending_node: f64,
    #[serde(rename = "MeanAnomaly")]
    pub mean_anomaly: f64,
}