use chrono::{DateTime, Utc};
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
    system_address: u32,
    #[serde(rename = "StarClass")]
    star_class: String,
    #[serde(rename = "RemainingJumpsInRoute")]
    remaining_jumps_in_route: u32,
}
#[derive(Deserialize)]
pub struct StartJump{
    timestamp: DateTime<Utc>,
    #[serde(rename = "JumpType")]
    jump_type: JumpType,
    #[serde(rename = "Taxi")]
    taxi: bool,
    ///Only available if the JumpType is `Hyperspace`
    #[serde(rename = "StarSystem")]
    star_system: Option<String>,
    ///Only available if the JumpType is `Hyperspace`
    #[serde(rename = "SystemAddress")]
    system_address: Option<u64>,
    ///Only available if the JumpType is `Hyperspace`
    #[serde(rename = "StarClass")]
    star_class: Option<StarClass>,
}
///Very large struct that covers almost everything about the system that you are currently jumping to
#[derive(Deserialize)]
pub struct FSDJump{
    timestamp: DateTime<Utc>,
    #[serde(rename = "Taxi")]
    taxi: bool,
    #[serde(rename = "Multicrew")]
    multicrew: bool,
    #[serde(rename = "StarSystem")]
    pub star_system: String,
    #[serde(rename = "SystemAddress")]
    system_address: u64,
    #[serde(rename = "StarPos")]
    star_pos: (f64, f64, f64),
    #[serde(rename = "SystemAllegiance")]
    system_allegiance: Allegiance,
    #[serde(rename = "SystemEconomy")]
    system_economy: Economy,
    #[serde(rename = "SystemEconomy_Localised")]
    system_economy_localised: String,
    #[serde(rename = "SystemSecondEconomy")]
    system_second_economy: Economy,
    #[serde(rename = "SystemSecondEconomy_Localised")]
    system_second_economy_localised: String,
    #[serde(rename = "SystemGovernment")]
    system_government: Government,
    #[serde(rename = "SystemGovernment_Localised")]
    system_government_localised: String,
    #[serde(rename = "SystemSecurity")]
    system_security: SystemSecurity,
    #[serde(rename = "SystemSecurity_Localised")]
    system_security_localised: String,
    #[serde(rename = "Population")]
    population: u64,
    #[serde(rename = "Body")]
    body: String,
    #[serde(rename = "BodyID")]
    body_id: u64,
    #[serde(rename = "ControllingPower")]
    controlling_power: PowerplayPower,
    #[serde(rename = "Powers")]
    powers: Vec<PowerplayPower>,
    #[serde(rename = "PowerplayState")]
    powerplay_state: PowerplayState,
    #[serde(rename = "PowerplayStateControlProgress")]
    powerplay_state_control_progress: f64,
    #[serde(rename = "PowerplayStateReinforcement")]
    powerplay_state_reinforcement: u64,
    #[serde(rename = "PowerplayStateUndermining")]
    powerplay_state_undermining: u64,
    #[serde(rename = "JumpDist")]
    jump_dist: f64,
    #[serde(rename = "FuelUsed")]
    fuel_used: f64,
    #[serde(rename = "FuelLevel")]
    fuel_level: f64,
    #[serde(rename = "Factions")]
    factions: Vec<Faction>,
    #[serde(rename = "SystemFaction")]
    system_faction: SystemFaction,
}
#[derive(Deserialize)]
pub struct SuperCruiseEntry{
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
    body_id: u32,
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
    threat: u32,
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
    is_station: bool,
}
#[derive(Deserialize)]
pub struct FSSSignalDiscovered {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "SystemAddress")]
    system_address: u64,
    #[serde(rename = "SignalName")]
    signal_name: String,
    #[serde(rename = "SignalType")]
    signal_type: SignalType,
    #[serde(rename = "IsStation")]
    is_station: bool,
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
//endregion
//region - Inventory -
#[derive(Deserialize)]
pub struct ShipLocker {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Items")]
    items: Vec<Item>,
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
//endregion
//region - Misc -
#[derive(Deserialize)]
pub struct LoadGame {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "FID")]
    fid: String, //Frontier ID
    #[serde(rename = "Commander")]
    commander: String,
    #[serde(rename = "Horizons")]
    has_horizons: bool,
    #[serde(rename = "Odyssey")]
    has_odyssey: bool,
    #[serde(rename = "Ship")]
    ship: String,
    #[serde(rename = "ShipLocalised")]
    ship_localised: String,
    #[serde(rename = "ShipId")]
    ship_id: u32,
    #[serde(rename = "ShipName")]
    ship_name: String,
    #[serde(rename = "ShipIdent")]
    ship_identity: String,
    #[serde(rename = "FuelLevel")]
    fuel_level: f32,
    #[serde(rename = "FuelCapacity")]
    fuel_capacity: f32,
    #[serde(rename = "GameMode")]
    game_mode: GameMode,
    #[serde(rename = "Group")]
    group: String,
    #[serde(rename = "Credits")]
    credits: u32,
    #[serde(rename = "Loan")]
    loan: u32,
    language: String,
    gameversion: String,
    build: String,
}
#[derive(Deserialize)]
pub struct Shutdown{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
}
#[derive(Deserialize)]
pub struct Music{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "MusicTrack")]
    pub(crate) music_track: String,
}
//endregion
//region - Docking -
#[derive(Deserialize)]
pub struct DockingRequested {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "MarketID")]
    market_id: u32,
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
    landing_pad: u32,
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
    market_id: u32,
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
    body_id: u32,
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
    id: u32,
    #[serde(rename = "StarSystem")]
    star_system: String,
    #[serde(rename = "SystemAddress")]
    system_address: u64,
    #[serde(rename = "Body")]
    body: String,
    #[serde(rename = "BodyID")]
    body_id: u32,
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
    star_system: String, //This is actually the body name
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
pub struct CommunityGoal{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "CurrentGoals")]
    current_goals: Vec<CGGoal>,
}
//endregion
//region - Social -
#[derive(Deserialize)]
pub struct ReceiveText {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "From")]
    from: String,
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "Message_Localised")]
    message_localised: String,
    #[serde(rename = "Channel")]
    channel: String,
}
#[derive(Deserialize)]
pub struct Friends{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Status")]
    status: FriendStatus
}
#[derive(Deserialize)]
pub struct WingLeave{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
}
//endregion
