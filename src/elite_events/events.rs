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
    pub controlling_power: PowerplayPower,
    #[serde(rename = "Powers")]
    pub powers: Vec<PowerplayPower>,
    #[serde(rename = "PowerplayState")]
    pub powerplay_state: PowerplayState,
    #[serde(rename = "PowerplayStateControlProgress")]
    pub powerplay_state_control_progress: f64,
    #[serde(rename = "PowerplayStateReinforcement")]
    pub powerplay_state_reinforcement: u64,
    #[serde(rename = "PowerplayStateUndermining")]
    pub powerplay_state_undermining: u64,
    #[serde(rename = "JumpDist")]
    pub jump_dist: f64,
    #[serde(rename = "FuelUsed")]
    pub fuel_used: f64,
    #[serde(rename = "FuelLevel")]
    pub fuel_level: f64,
    #[serde(rename = "Factions")]
    pub factions: Vec<Faction>,
    #[serde(rename = "SystemFaction")]
    pub system_faction: SystemFaction,
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
    timestamp: DateTime<Utc>,
    #[serde(rename = "SystemAddress")]
    system_address: u64,
    #[serde(rename = "SignalName")]
    signal_name: String,
    #[serde(rename = "SignalType")]
    signal_type: SignalType,
    #[serde(rename = "IsStation")]
    is_station: Option<bool>,
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
    raw: Vec<RawMaterial>,
    #[serde(rename = "Manufactured")]
    manufactured: Vec<LocalisedMaterial>,
    #[serde(rename = "Encoded")]
    encoded: Vec<LocalisedMaterial>
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
    distance_from_star_ls: f64,
    #[serde(rename = "Docked")]
    docked: bool,
    #[serde(rename = "StationName")]
    station_name: String,
    #[serde(rename = "StationType")]
    station_type: StationType,
    #[serde(rename = "MarketID")]
    market_id: u64,
    #[serde(rename = "StationFaction")]
    station_faction: SystemFaction,
    #[serde(rename = "StationGovernment")]
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