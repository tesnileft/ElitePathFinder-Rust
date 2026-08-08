use std::fmt::Display;
use chrono::{DateTime, Utc};
use gdk::pango::Language;
use serde::Deserialize;
use crate::elite_events::enums::*;
use crate::elite_events::substructs::*;

//region - FSD -
#[derive(Deserialize)]
pub struct FSDTarget {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "SystemAddress")]
    system_address: u64,
    #[serde(rename = "StarClass")]
    star_class: String,
    #[serde(rename = "RemainingJumpsInRoute")]
    remaining_jumps_in_route: u64,
}
#[derive(Deserialize)]
pub struct StartJump{
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "JumpType")]
    pub jump_type: JumpType,
    #[serde(rename = "Taxi")]
    pub taxi: bool,
    ///Only available if the JumpType is `Hyperspace`
    #[serde(rename = "StarSystem")]
    pub star_system: Option<String>,
    ///Only available if the JumpType is `Hyperspace`
    #[serde(rename = "SystemAddress")]
    pub system_address: Option<u64>,
    ///Only available if the JumpType is `Hyperspace`
    #[serde(rename = "StarClass")]
    pub star_class: Option<StarClass>,
}
///Very large struct that covers almost everything about the system that you are currently jumping to
#[derive(Deserialize)]
pub struct FSDJump{
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "Taxi")]
    pub taxi: bool,
    #[serde(rename = "Multicrew")]
    pub multicrew: bool,
    #[serde(rename = "StarSystem")]
    pub star_system: String,
    #[serde(rename = "SystemAddress")]
    pub system_address: u64,
    #[serde(rename = "StarPos")]
    pub star_pos: (f64, f64, f64),
    #[serde(rename = "SystemAllegiance")]
    pub system_allegiance: Allegiance,
    #[serde(rename = "SystemEconomy")]
    pub system_economy: Economy,
    #[serde(rename = "SystemEconomy_Localised")]
    pub system_economy_localised: String,
    #[serde(rename = "SystemSecondEconomy")]
    pub system_second_economy: Economy,
    #[serde(rename = "SystemSecondEconomy_Localised")]
    pub system_second_economy_localised: String,
    #[serde(rename = "SystemGovernment")]
    pub system_government: Government,
    #[serde(rename = "SystemGovernment_Localised")]
    pub system_government_localised: String,
    #[serde(rename = "SystemSecurity")]
    pub system_security: SystemSecurity,
    #[serde(rename = "SystemSecurity_Localised")]
    pub system_security_localised: String,
    #[serde(rename = "Population")]
    pub population: u64,
    #[serde(rename = "Body")]
    pub body: String,
    #[serde(rename = "BodyID")]
    pub body_id: u64,
    #[serde(rename = "ControllingPower")]
    #[serde(default)]
    pub controlling_power: PowerplayPower,
    #[serde(rename = "Powers")]
    pub powers: Option<Vec<PowerplayPower>>,
    #[serde(rename = "PowerplayState")]
    #[serde(default)]
    pub powerplay_state: PowerplayState,
    #[serde(rename = "PowerplayStateControlProgress")]
    #[serde(default)]
    pub powerplay_state_control_progress: f64,
    #[serde(rename = "PowerplayStateReinforcement")]
    #[serde(default)]
    pub powerplay_state_reinforcement: u64,
    #[serde(rename = "PowerplayStateUndermining")]
    #[serde(default)]
    pub powerplay_state_undermining: u64,
    #[serde(rename = "JumpDist")]
    pub jump_dist: f64,
    #[serde(rename = "FuelUsed")]
    pub fuel_used: f64,
    #[serde(rename = "FuelLevel")]
    pub fuel_level: f64,
    #[serde(rename = "Factions")]
    #[serde(default)]
    pub factions: Vec<Faction>,
    #[serde(rename = "SystemFaction")]
    pub system_faction: Option<SystemFaction>,
}
#[derive(Deserialize)]
pub struct SupercruiseEntry {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Taxi")]
    taxi: bool,
    #[serde(rename = "Multicrew")]
    multicrew: bool,
    #[serde(rename = "StarSystem")]
    star_system: String,
    #[serde(rename = "SystemAddress")]
    system_address: u64,
}
#[derive(Deserialize)]
pub struct SupercruiseExit{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Taxi")]
    taxi: bool,
    #[serde(rename = "Multicrew")]
    multicrew: bool,
    #[serde(rename = "StarSystem")]
    star_system: String,
    #[serde(rename = "SystemAddress")]
    system_address: u64,
    #[serde(rename = "Body")]
    body: String,
    #[serde(rename = "BodyID")]
    body_id: u64,
    #[serde(rename = "BodyType")]
    body_type: BodyType,
}
#[derive(Deserialize)]
pub struct SupercruiseDestinationDrop{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Type")]
    type_: String,
    #[serde(rename = "Threat")]
    threat: u64,
    #[serde(rename = "MarketID")]
    market_id: u64,
}
//endregion
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
pub struct FSSDiscoveryScan{
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
pub struct FSSBodySignals{
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
    pub atmosphere_composition: Option<Vec<AtmosphericGas>> ,
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
    pub rotation_period: f64,
    #[serde(rename = "AxialTilt")]
    pub axial_tilt: f64,
    #[serde(rename = "WasDiscovered")]
    pub was_discovered: bool,
    #[serde(rename = "WasMapped")]
    pub was_mapped: bool,
    #[serde(rename = "WasFootfalled")]
    pub was_footfalled: bool,
    //TODO
}
#[derive(Deserialize)]
pub struct CelestialRings{
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "RingClass")]
    pub ring_class: String,
    #[serde(rename = "MassMT")]
    pub mass_megatons: f64,
    #[serde(rename = "InnerRad")]
    pub inner_radius: f64,
    #[serde(rename = "OuterRad")]
    pub outer_radius: f64,
}
#[derive(Deserialize)]
pub struct RawMaterialInfo{
    #[serde(rename = "Name")]
    pub name: RawMaterial,
    #[serde(rename = "Name_Localised")]
    #[serde(default)]
    pub name_localized: String,
    #[serde(rename = "Percent")]
    pub percent: f64,
}
#[derive(Deserialize, Eq, PartialEq, Clone)]
pub enum RawMaterial{
    //Grade 1
    #[serde(rename = "carbon")]
    Carbon,
    #[serde(rename = "iron")]
    Iron,
    #[serde(rename = "lead")]
    Lead,
    #[serde(rename = "nickel")]
    Nickel,
    #[serde(rename = "phosphorus")]
    Phosphorus,
    #[serde(rename = "rhenium")]
    Rhenium,
    #[serde(rename = "sulphur")]
    Sulphur,
    //Grade 2
    #[serde(rename = "arsenic")]
    Arsenic,
    #[serde(rename = "chromium")]
    Chromium,
    #[serde(rename = "germanium")]
    Germanium,
    #[serde(rename = "manganese")]
    Manganese,
    #[serde(rename = "vanadium")]
    Vanadium,
    #[serde(rename = "zinc")]
    Zinc,
    #[serde(rename = "zirconium")]
    Zirconium,
    //Grade 3
    #[serde(rename = "cadmium")]
    Cadmium,
    #[serde(rename = "mercury")]
    Mercury,
    #[serde(rename = "molybdenum")]
    Molybdenum,
    #[serde(rename = "niobium")]
    Niobium,
    #[serde(rename = "tin")]
    Tin,
    #[serde(rename = "tungsten")]
    Tungsten,
    #[serde(rename = "boron")]
    Boron,
    //Grade 4
    #[serde(rename = "antimony")]
    Antimony,
    #[serde(rename = "polonium")]
    Polonium,
    #[serde(rename = "ruthenium")]
    Ruthenium,
    #[serde(rename = "selenium")]
    Selenium,
    #[serde(rename = "technetium")]
    Technetium,
    #[serde(rename = "tellurium")]
    Tellurium,
    #[serde(rename = "yttrium")]
    Yttrium,
}
#[derive(Deserialize)]
pub struct FSSAllBodiesFound{
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "SystemName")]
    pub system_name: String,
    #[serde(rename = "SystemAddress")]
    pub system_address: u64,
    #[serde(rename = "Count")]
    pub count: u64
}
//endregion
//region - Inventory -
#[derive(Deserialize)]
pub struct ShipLocker {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Items")]
    items: Option<Vec<Item>>,
}
#[derive(Deserialize)]
pub struct Cargo{
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "Vessel")]
    pub vessel: String,
    #[serde(rename = "Count")]
    pub count: u64,
    #[serde(rename = "Inventory")]
    pub inventory: Option<Vec<CargoItem>>,
}

#[derive(Deserialize)]
pub struct Backpack{
    timestamp: DateTime<Utc>,
    #[serde(rename = "Items")]
    items: Vec<Item>,
    #[serde(rename = "Components")]
    components: Vec<Component>,
    #[serde(rename = "Consumables")]
    consumables: Vec<Consumable>,
    #[serde(rename = "Data")]
    data: Vec<Data>
}
#[derive(Deserialize)]
pub struct Materials{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Raw")]
    raw: Vec<RawMaterialInventory>,
    #[serde(rename = "Manufactured")]
    manufactured: Vec<LocalisedMaterialInventory>,
    #[serde(rename = "Encoded")]
    encoded: Vec<LocalisedMaterialInventory>
}
#[derive(Deserialize)]
pub struct Loadout{
    timestamp: DateTime<Utc>,
    #[serde(rename = "Ship")]
    ship: String,
    #[serde(rename = "ShipID")]
    ship_id: u64,
    #[serde(rename = "ShipName")]
    ship_name: String,
    #[serde(rename = "ShipIdent")]
    ship_ident: String,
    #[serde(rename = "HullValue")]
    hull_value: u64,
    #[serde(rename = "HullHealth")]
    hull_health: f64,
    #[serde(rename = "UnladenMass")]
    unladen_mass: f64,
    #[serde(rename = "CargoCapacity")]
    cargo_capacity: u64,
    #[serde(rename = "MaxJumpRange")]
    max_jump_range: f64,
    #[serde(rename = "FuelCapacity")]
    fuel_capacity: FuelCapacity,
    #[serde(rename = "Rebuy")]
    rebuy: u64,
    #[serde(rename = "Modules")]
    modules: Vec<Module>

}
#[derive(Deserialize)]
pub struct SuitLoadout{
    timestamp: DateTime<Utc>,
    #[serde(rename = "SuitID")]
    suit_id: u64,
    #[serde(rename = "SuitName")]
    suit_name: String,
    #[serde(rename = "SuitName_Localised")]
    suit_name_localised: String,
    #[serde(rename = "SuitMods")]
    suit_mods: Vec<String>,
    #[serde(rename = "LoadoutID")]
    loadout_id: u64,
    #[serde(rename = "LoadoutName")]
    loadout_name: String,
    #[serde(rename = "Modules")]
    modules: Vec<SuitModule> // Actually weapons
}
#[derive(Deserialize)]
pub struct ReservoirReplenished{
    timestamp: DateTime<Utc>,
    #[serde(rename = "FuelMain")]
    fuel_main: f64,
    #[serde(rename = "FuelReservoir")]
    fuel_reservoir: f64
}
#[derive(Deserialize)]
pub struct FuelScoop{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Scooped")]
    scooped: f64,
    #[serde(rename = "Total")]
    total: f64,
}
#[derive(Deserialize)]
pub struct JetConeBoost{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "BoostValue")]
    boost_value: f64,
}
//endregion
//region - Misc -
#[derive(Deserialize)]
pub struct Statistics{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Bank_Account")]
    bankaccount: BankAccountStats,
    #[serde(rename = "Combat")]
    combat: CombatStats,
    #[serde(rename = "Crime")]
    crime: CrimeStats,
    #[serde(rename = "Trading")]
    trading: TradingStats,
    #[serde(rename = "Smuggling")]
    smuggling: SmugglingStats,
    #[serde(rename = "Mining")]
    mining: MiningStats,
    #[serde(rename = "Exploration")]
    exploration: ExplorationStats,
    #[serde(rename = "Passengers")]
    passengers: PassengersStats,
    #[serde(rename = "Search_And_Rescue")]
    search_and_recue: SearchAndRecueStats,
    #[serde(rename = "Squadron")]
    squadron: SquadronStats,
    #[serde(rename = "Crafting")]
    crafting: CraftingStats,
    #[serde(rename = "Crew")]
    crew: CrewStats,
    #[serde(rename = "Material_Trader_Stats")]
    material_trader_stats: MaterialTraderStats,
    #[serde(rename = "Exobiology")]
    exobiology: ExobiologyStats
}
#[derive(Deserialize)]
pub struct Music{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "MusicTrack")]
    pub(crate) music_track: String,
}
#[derive(Deserialize)]
pub struct ShipTargeted{
    timestamp: DateTime<Utc>,
    #[serde(rename = "TargetLocked")]
    target_locked: bool,
    #[serde(rename = "Ship")]
    ship: ShipType,
    #[serde(rename = "Ship_Localised")]
    ship_localised: String,
    #[serde(rename = "ScanStage")]
    scan_stage: u64,
    #[serde(rename = "PilotName")]
    pilot_name: String,
    #[serde(rename = "PilotName_Localised")]
    pilot_localised: String,
    #[serde(rename = "PilotRank")]
    pilot_rank: PilotRank,
    #[serde(rename = "SquadronID")]
    squadron_id: String,
    #[serde(rename = "ShieldHealth")]
    shield_health: f64,
    #[serde(rename = "HullHealth")]
    hull_health: f64,
    #[serde(rename = "LegalStatus")]
    legal_status: LegalStatus,
    #[serde(rename = "Power")]
    power: PowerplayPower,
}
#[derive(Deserialize)]
pub struct NavRoute{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>
}
#[derive(Deserialize)]
pub struct NavRouteClear{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>
}
#[derive(Deserialize)]
pub enum LegalStatus{
    Lawless,
    Wanted
}
#[derive(Deserialize)]
pub enum PilotRank
{
    Novice,
    Elite
}
//endregion
//region - Docking -
#[derive(Deserialize)]
pub struct DockingRequested {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "MarketID")]
    market_id: u64,
    #[serde(rename = "StationName")]
    station_name: String,
    #[serde(rename = "StationType")]
    station_type: StationType,
    #[serde(rename = "LandingPads")]
    landing_pads: LandingPads,
}
#[derive(Deserialize)]
pub struct DockingGranted{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "LandingPad")]
    landing_pad: u64,
    #[serde(rename = "MarketID")]
    market_id: u64,
    #[serde(rename = "StationName")]
    station_name: String,
    #[serde(rename = "StationType")]
    station_type: StationType,
}
#[derive(Deserialize)]
pub struct Docked{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "StationName")]
    station_name: String,
    #[serde(rename = "StationType")]
    station_type: StationType,
    #[serde(rename = "Taxi")]
    taxi: bool,
    #[serde(rename = "Multicrew")]
    multicrew: bool,
    #[serde(rename = "StarSystem")]
    star_system: String,
    #[serde(rename = "SystemAddress")]
    system_address: u64,
    #[serde(rename = "MarketID")]
    market_id: u64,
    #[serde(rename = "StationFaction")]
    station_faction: StationFaction,
    #[serde(rename = "StationGovernment")]
    station_government: String,
    #[serde(rename = "StationGovernment_Localised")]
    station_government_localised: String,
    #[serde(rename = "StationServices")]
    station_services: Vec<StationService>,
    #[serde(rename = "StationEconomy")]
    station_economy: String,
    #[serde(rename = "StationEconomy_Localised")]
    station_economy_localised: String,
    #[serde(rename = "StationEconomies")]
    station_economies: Vec<StationEconomy>,
    #[serde(rename = "DistFromStarLS")]
    dist_star_ls: f32,
    #[serde(rename = "LandingPads")]
    landing_pads: LandingPads
}
#[derive(Deserialize)]
pub struct Embark{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "SRV")]
    srv: bool,
    #[serde(rename = "Taxi")]
    taxi: bool,
    #[serde(rename = "Multicrew")]
    multicrew: bool,
    #[serde(rename = "ID")]
    id: u64,
    #[serde(rename = "StarSystem")]
    star_system: String,
    #[serde(rename = "SystemAddress")]
    system_address: u64,
    #[serde(rename = "Body")]
    body: String,
    #[serde(rename = "BodyID")]
    body_id: u64,
    #[serde(rename = "OnStation")]
    on_station: bool,
    #[serde(rename = "OnPlanet")]
    on_planet: bool
}
#[derive(Deserialize)]
pub struct Disembark {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "SRV")]
    srv: bool,
    #[serde(rename = "Taxi")]
    taxi: bool,
    #[serde(rename = "Multicrew")]
    multicrew: bool,
    #[serde(rename = "ID")]
    id: u64,
    #[serde(rename = "StarSystem")]
    star_system: String,
    #[serde(rename = "SystemAddress")]
    system_address: u64,
    #[serde(rename = "Body")]
    body: String,
    #[serde(rename = "BodyID")]
    body_id: u64,
    #[serde(rename = "OnStation")]
    on_station: bool,
    #[serde(rename = "OnPlanet")]
    on_planet: bool
}
#[derive(Deserialize)]
pub struct Undocked{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "StationName")]
    station_name: String,
    #[serde(rename = "StationType")]
    station_type: StationType,
    #[serde(rename = "Taxi")]
    taxi: bool,
    #[serde(rename = "Multicrew")]
    multicrew: bool,
}
#[derive(Deserialize)]
pub struct Touchdown {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "PlayerControlled")]
    player_controlled: bool,
    #[serde(rename = "Taxi")]
    taxi: bool,
    #[serde(rename = "Multicrew")]
    multicrew: bool,
    #[serde(rename = "StarSystem")]
    star_system: String,
    #[serde(rename = "SystemAddress")]
    system_address: u64,
    #[serde(rename = "Body")]
    body: String,
    #[serde(rename = "BodyID")]
    body_id: u64,
    #[serde(rename = "OnStation")]
    on_station: bool,
    #[serde(rename = "OnPlanet")]
    on_planet: bool,
    #[serde(rename = "Latitude")]
    latitude: f32,
    #[serde(rename = "Longitude")]
    longitude: f32,
}
#[derive(Deserialize)]
pub struct Liftoff {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "PlayerControlled")]
    player_controlled: bool,
    #[serde(rename = "Taxi")]
    taxi: bool,
    #[serde(rename = "Multicrew")]
    multicrew: bool,
    #[serde(rename = "StarSystem")]
    star_system: String,
    #[serde(rename = "SystemAddress")]
    system_address: u64,
    #[serde(rename = "Body")]
    body: String,
    #[serde(rename = "BodyID")]
    body_id: u64,
    #[serde(rename = "OnStation")]
    on_station: bool,
    #[serde(rename = "OnPlanet")]
    on_planet: bool,
    #[serde(rename = "Latitude")]
    latitude: f32,
    #[serde(rename = "Longitude")]
    longitude: f32,
}
#[derive(Deserialize)]
pub struct DockSRV {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "SRVType")]
    srv_type: SRVType,
    #[serde(rename = "SRVType_Localised")]
    srv_type_localised: String,
    #[serde(rename = "ID")]
    id: u64,
}
#[derive(Deserialize)]
pub struct ApproachBody {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "StarSystem")]
    star_system: String,
    #[serde(rename = "SystemAddress")]
    system_address: u64,
    #[serde(rename = "Body")]
    body: String,
    #[serde(rename = "BodyID")]
    body_id: u64,
}
#[derive(Deserialize)]
pub struct LeaveBody {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "StarSystem")]
    star_system: String,
    #[serde(rename = "SystemAddress")]
    system_address: u64,
    #[serde(rename = "Body")]
    body: String,
    #[serde(rename = "BodyID")]
    body_id: u64,
}
//endregion
//region - Station Features -
#[derive(Deserialize)]
pub struct RefuelAll{
    timestamp: DateTime<Utc>,
    #[serde(rename = "Cost")]
    cost: u64,
    #[serde(rename = "Amount")]
    amount: f64,
}
#[derive(Deserialize)]
pub struct Shipyard{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "MarketID")]
    market_id: u64,
    #[serde(rename = "StationName")]
    station_name: String,
    #[serde(rename = "StarSystem")]
    star_system: String, //This is actually the body name
}
#[derive(Deserialize)]
pub struct StoredShips{
    timestamp: DateTime<Utc>,
    #[serde(rename = "StationName")]
    station_name: String,
    #[serde(rename = "MarketID")]
    market_id: u64,
    #[serde(rename = "StarSystem")]
    star_system: Option<String>, //This is actually the body name
    #[serde(rename = "ShipsHere")]
    ships_here: Vec<StoredShip>,
}
#[derive(Deserialize)]
pub struct ShipyardTransfer{
    timestamp: DateTime<Utc>,
    #[serde(rename = "ShipType")]
    ship_type: ShipType,
    #[serde(rename = "ShipID")]
    ship_id: u64,
    #[serde(rename = "System")]
    system_name: String,
    #[serde(rename = "ShipMarketID")]
    ship_market_id: u64,
    #[serde(rename = "Distance")]
    distance: f64,
    #[serde(rename = "TransferPrice")]
    transfer_price: u64,
    #[serde(rename = "TransferTime")]
    transfer_time: u64,
    #[serde(rename = "MarketID")]
    market_id: u64,
}
#[derive(Deserialize)]
pub struct ShipyardSwap {
    timestamp: DateTime<Utc>,
    #[serde(rename = "ShipType")]
    ship_type: ShipType,
    #[serde(rename = "ShipType_Localised")]
    ship_type_localised: String,
    #[serde(rename = "ShipID")]
    ship_id: u64,
    #[serde(rename = "StoreOldShip")]
    store_old_ship: ShipType,
    #[serde(rename = "StoreShipID")]
    store_ship_id: u64,
    #[serde(rename = "MarketID")]
    market_id: u64,
}
#[derive(Deserialize)]
pub struct CommunityGoal{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "CurrentGoals")]
    current_goals: Vec<CGGoal>,
}
//endregion
//region - Social -
#[derive(Deserialize)]
pub struct Commander{
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "FID")]
    pub fid: String,
    #[serde(rename = "Name")]
    pub name: String,
}
#[derive(Deserialize)]
pub struct ReceiveText {
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "From")]
    pub from: String,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "Message_Localised")]
    pub message_localised: String,
    #[serde(rename = "Channel")]
    pub channel: String,
}
#[derive(Deserialize)]
pub struct Friends{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Status")]
    status: FriendStatus
}
#[derive(Deserialize)]
pub struct SquadronStartup{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "SquadronID")]
    squadron_id: u64,
    #[serde(rename = "SquadronName")]
    squadron_name: String,
    #[serde(rename = "CurrentRank")]
    current_rank: u64,
    #[serde(rename = "CurrentRankName")]
    current_rank_name: String
}
#[derive(Deserialize)]
pub struct WingInvite{
    timestamp: DateTime<Utc>,
    #[serde(rename = "Name")]
    name: String,
}
#[derive(Deserialize)]
pub struct WingAdd{
    timestamp: DateTime<Utc>,
    #[serde(rename = "Name")]
    name: String,
}
#[derive(Deserialize)]
pub struct WingLeave{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
}
//endregion
//region - Rank -
#[derive(Deserialize)]
pub struct Rank{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Combat")]
    combat: u64,
    #[serde(rename = "Trade")]
    trade: u64,
    #[serde(rename = "Explore")]
    explorer: u64,
    #[serde(rename = "Soldier")]
    mercenary: u64,
    #[serde(rename = "Exobiologist")]
    exobiologist: u64,
    #[serde(rename = "CQC")]
    cqc: u64,
    #[serde(rename = "Empire")]
    empire: u64,
    #[serde(rename = "Federation")]
    federation: u64,

}
#[derive(Deserialize)]
pub struct Progress{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Combat")]
    combat: u64,
    #[serde(rename = "Trade")]
    trade: u64,
    #[serde(rename = "Explore")]
    explorer: u64,
    #[serde(rename = "Soldier")]
    mercenary: u64,
    #[serde(rename = "Exobiologist")]
    exobiologist: u64,
    #[serde(rename = "CQC")]
    cqc: u64,
    #[serde(rename = "Empire")]
    empire: u64,
    #[serde(rename = "Federation")]
    federation: u64,
}
#[derive(Deserialize)]
pub struct Reputation{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Empire")]
    empire: f64,
    #[serde(rename = "Federation")]
    federation: f64,
    #[serde(rename = "Independent")]
    independent: f64,
    #[serde(rename = "Alliance")]
    alliance: f64,
}
#[derive(Deserialize)]
pub struct EngineerProgress{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Engineers")]
    engineers: Vec<Engineer>
}


//endregion
//region - Missions -
#[derive(Deserialize)]
pub struct Missions{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Active")]
    active: Vec<Mission>,
    #[serde(rename = "Failed")]
    failed: Vec<Mission>,
    #[serde(rename = "Complete")]
    complete: Vec<Mission>,
}

//endregion
//region - Scans -
#[derive(Deserialize)]
pub struct ScanOrganic {
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "ScanType")]
    pub scan_type: String,
    #[serde(rename = "Genus")]
    pub genus: String,
    #[serde(rename = "Genus_Localised")]
    pub genus_localised: String,
    #[serde(rename = "Species")]
    pub species: Species,
    #[serde(rename = "Species_Localised")]
    pub species_localised: String,
    #[serde(rename = "Variant")]
    pub variant: String,
    #[serde(rename = "Variant_Localised")]
    pub variant_localised: String,
    #[serde(rename = "WasLogged")]
    pub was_logged: bool,
    #[serde(rename = "SystemAddress")]
    pub system_address: u64,
    #[serde(rename = "Body")]
    pub body_id: u64,
}

#[derive(Deserialize)]
pub struct CodexEntry {
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "EntryID")]
    pub entry_id: u64,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Name_Localised")]
    pub name_localised: String,
    #[serde(rename = "SubCategory")]
    pub sub_category: String,
    #[serde(rename = "SubCategory_Localised")]
    pub sub_category_localised: String,
    #[serde(rename = "Category")]
    pub category: String,
    #[serde(rename = "Category_Localised")]
    pub category_localised: String,
    #[serde(rename = "Region")]
    pub region: String,
    #[serde(rename = "Region_Localised")]
    pub region_localised: String,
    #[serde(rename = "System")]
    pub star_system: String,
    #[serde(rename = "SystemAddress")]
    pub system_address: u64,
    #[serde(rename = "BodyID")]
    pub body_id: u64,
    #[serde(rename = "NearestDestination")]
    pub nearest_destination: String,
    #[serde(rename = "Latitude")]
    pub latitude: f64,
    #[serde(rename = "Longitude")]
    pub longitude: f64,
    #[serde(rename = "IsNewEntry")]
    pub is_new_entry: bool,
}

#[derive(Deserialize)]
pub struct SAAScanComplete{ //Surface Scan
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "BodyName")]
    pub body_name: String,
    #[serde(rename = "SystemAddress")]
    pub system_address: u64,
    #[serde(rename = "BodyID")]
    pub body_id: u64,
    #[serde(rename = "ProbesUsed")]
    pub probes_used: u64,
    #[serde(rename = "EfficiencyTarget")]
    pub efficiency_target: f64,
}
#[derive(Deserialize)]
///Surface scan signal results (Bio, geo)
pub struct SAASignalsFound{
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    pub body_name: String,
    #[serde(rename = "SystemAddress")]
    pub system_address: u64,
    #[serde(rename = "BodyID")]
    pub body_id: u64,
    #[serde(rename = "Signals")]
    pub signals: Vec<BodySurfaceSignal>,
    ///Potentially empty
    #[serde(rename = "Genuses")]
    pub genuses: Vec<ExobioGenus>,

}
#[derive(Deserialize)]
pub struct ScanBaryCentre{
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

#[derive(Deserialize)]
pub struct ExobioGenus{
    ///Enum of the genus type
    #[serde(rename = "Genus")]
    pub genus: Genus,
    #[serde(rename = "Genus_Localised")]
    pub genus_localised: String,
}
#[derive(Deserialize)]
pub struct BodySurfaceSignal {
    #[serde(rename = "Type")]
    pub type_: SAASignalType,
    #[serde(rename = "Type_Localised")]
    pub type_localised: String,
    #[serde(rename = "Count")]
    pub count: u64,
}
#[derive(Deserialize, Debug)]
pub enum Genus {
    #[serde(rename = "$Codex_Ent_Bacterial_Genus_Name;")]
    Bacterium,
    #[serde(rename = "$Codex_Ent_Fungoids_Genus_Name;")]
    Fungoida,


}
impl Display for Genus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Deserialize)]
pub enum SAASignalType {
    #[serde(alias = "$SAA_SignalType_Biological;")]
    Biological,
    #[serde(rename = "$SAA_SignalType_Geological;")]
    Geological
}

//endregion

//region - Logging Specific -
#[derive(Deserialize)]
pub struct FileHeader {
    pub part: u64,
    pub language: String,
    #[serde(rename = "Odyssey")]
    pub odyssey: bool,
    pub gameversion: String,
    pub build: String,
}
#[derive(Deserialize)]
pub struct LoadGame {
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "FID")]
    pub fid: String, //Frontier ID
    #[serde(rename = "Commander")]
    pub commander: String,
    #[serde(rename = "Horizons")]
    pub has_horizons: bool,
    #[serde(rename = "Odyssey")]
    pub has_odyssey: bool,
    #[serde(rename = "Ship")]
    pub ship: String,
    #[serde(rename = "ShipLocalised")]
    pub ship_localised: Option<String>,
    #[serde(rename = "ShipID")]
    pub ship_id: u64,
    #[serde(rename = "ShipName")]
    pub ship_name: String,
    #[serde(rename = "ShipIdent")]
    pub ship_identity: String,
    #[serde(rename = "FuelLevel")]
    pub fuel_level: f32,
    #[serde(rename = "FuelCapacity")]
    pub fuel_capacity: f32,
    #[serde(rename = "GameMode")]
    pub game_mode: GameMode,
    #[serde(rename = "Group")]
    pub group: String,
    #[serde(rename = "Credits")]
    pub credits: u64,
    #[serde(rename = "Loan")]
    pub loan: u64,
    pub language: String,
    pub gameversion: String,
    pub build: String,
}
#[derive(Deserialize)]
pub struct Shutdown{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
}
//endregion

#[derive(Deserialize)]
pub struct Location{
    //
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "DistFromStarLS")]
    distance_from_star_ls: Option<f64>,
    #[serde(rename = "Docked")]
    docked: bool,
    #[serde(rename = "StationName")]
    station_name: Option<String>,
    #[serde(rename = "StationType")]
    #[serde(default)]
    station_type: StationType,
    #[serde(rename = "MarketID")]
    market_id: Option<u64>,
    #[serde(rename = "StationFaction")]
    station_faction: Option<SystemFaction>,
    #[serde(rename = "StationGovernment")]
    #[serde(default)]
    station_government: Government,
    #[serde(rename = "StationGovernment_Localised")]
    station_government_localized: String,
    #[serde(rename = "StationServices")]
    station_services: Vec<StationService>,
    #[serde(rename = "StationEconomy")]
    station_economy: String,
    #[serde(rename = "StationEconomy_Localised")]
    station_economy_localised: String,
    #[serde(rename = "StationAllegiance")]
    station_allegiance: Allegiance,
    #[serde(rename = "StationEconomies")]
    station_economies: Vec<StationEconomy>,
    #[serde(rename = "Taxi")]
    taxi: bool,
    #[serde(rename = "Multicrew")]
    multicrew: bool,
    #[serde(rename = "StarSystem")]
    star_system: String,
    #[serde(rename = "SystemAddress")]
    system_address: u64,
    #[serde(rename = "StarPos")]
    pub star_pos: (f64, f64, f64),
    #[serde(rename = "SystemAllegiance")]
    system_allegiance: Allegiance,
    #[serde(rename = "SystemEconomy")]
    system_economy: Economy,
    #[serde(rename = "SystemEconomy_Localised")]
    system_economy_localised: String,
    #[serde(rename = "SystemSecondEconomy")]
    pub system_second_economy: Economy,
    #[serde(rename = "SystemSecondEconomy_Localised")]
    pub system_second_economy_localised: String,
    #[serde(rename = "SystemGovernment")]
    pub system_government: Government,
    #[serde(rename = "SystemGovernment_Localised")]
    pub system_government_localised: String,
    #[serde(rename = "SystemSecurity")]
    pub system_security: SystemSecurity,
    #[serde(rename = "SystemSecurity_Localised")]
    pub system_security_localised: String,
    #[serde(rename = "Population")]
    pub population: u64,
    #[serde(rename = "Body")]
    pub body: String,
    #[serde(rename = "BodyID")]
    pub body_id: u64,
    #[serde(rename = "BodyType")]
    pub body_type: BodyType,
    #[serde(rename = "Powers")]
    pub powers: Vec<PowerplayPower>,
    #[serde(rename = "PowerplayState")]
    pub powerplay_state: PowerplayState,
    #[serde(rename = "PowerplayStateControlProgress")]
    pub powerplay_control_progress: f64,
    #[serde(rename = "PowerplayStateReinforcement")]
    pub powerplay_reinforcement: f64,
    #[serde(rename = "PowerplayStateUndermining")]
    pub powerplay_undermining: f64,
    #[serde(rename = "Factions")]
    pub factions: Vec<Faction>,
    #[serde(rename = "SystemFaction")]
    pub system_faction: SystemFaction,
}
#[derive(Deserialize)]
pub struct Powerplay {
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "Power")]
    pub power: PowerplayPower,
    #[serde(rename = "Rank")]
    pub rank: u64,
    #[serde(rename = "Merits")]
    pub merits: u64,
    #[serde(rename = "TimePledged")]
    pub time_pledged: u64
}

#[derive(Deserialize)]
pub struct LaunchFighter {
    timestamp: DateTime<Utc>,
    #[serde(rename = "Loadout")]
    loadout: String,
    #[serde(rename = "ID")]
    id: u64,
    #[serde(rename = "PlayerControlled")]
    player_controlled: bool,
}