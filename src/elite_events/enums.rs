use std::fmt;
use serde::{Deserialize, };
#[derive(Deserialize)]
pub enum SignalType {
    FleetCarrier,
    NavBeacon,
    Megaship,
    ResourceExtraction,
    Installation,
    StationCoriolis,
    SquadronCarrier,
    Outpost,
    StationMegaShip
}
#[derive(Deserialize)]
pub enum StationType{
    FleetCarrier,
    Coriolis,
}
#[derive(Deserialize)]
pub enum BodyType{
    Planet,
    Station,
}
#[derive(Deserialize)]
pub enum ShipType{
    SideWinder,
    CobraMkV,
    Type8,
    Python_NX,
    SmallCombat01_NX,
    #[serde(rename = "explorer_nx")]
    Explorer_NX,
    Mandalay,
}
#[derive(Deserialize, PartialEq, Eq)]
pub enum JumpType{
    Supercruise,
    Hyperspace
}
#[derive(Deserialize)]
pub enum StarClass{
    DA,
    G,
    K,
    N,
    M
}
#[derive(Deserialize)]
pub enum GameMode {
    Solo,
    Group,
    Open,
}
#[derive(Deserialize)]
pub enum StationService{
    #[serde(rename = "dock")]
    Dock,
    #[serde(rename = "autodock")]
    AutoDock,
    #[serde(rename = "commodities")]
    Commodities,
    #[serde(rename = "voucherredemption")]
    VoucherRedemption,
    #[serde(rename = "vistagenomics")]
    VistaGenomics,
    #[serde(rename = "contacts")]
    Contacts,
    #[serde(rename = "exploration")]
    Exploration,
    #[serde(rename = "outfitting")]
    Outfitting,
    #[serde(rename = "crewlounge")]
    Crewlounge,
    #[serde(rename = "rearm")]
    Rearm,
    #[serde(rename = "refuel")]
    Refuel,
    #[serde(rename = "repair")]
    Repair,
    #[serde(rename = "shipyard")]
    ShipYard,
    #[serde(rename = "tuning")]
    Tuning,
    #[serde(rename = "missions")]
    Missions,
    #[serde(rename = "missionsgenerated")]
    MissionsGenerated,
    #[serde(rename = "engineer")]
    Engineer,
    #[serde(rename = "flightcontroller")]
    FlightController,
    #[serde(rename = "stationoperations")]
    StationOperations,
    #[serde(rename = "powerplay")]
    Powerplay,
    #[serde(rename = "searchrescue")]
    SearchResque,
    #[serde(rename = "materialtrader")]
    MaterialTrader,
    #[serde(rename = "stationMenu")]
    StationMenu,
    #[serde(rename = "carriermanagement")]
    CarrierManagement,
    #[serde(rename = "carrierfuel")]
    CarrierFuel,
    #[serde(rename = "shop")]
    Shop,
    #[serde(rename = "livery")]
    Livery,
    #[serde(rename = "socialspace")]
    SocialSpace,
    #[serde(rename = "bartender")]
    Bartender,
    #[serde(rename = "pioneersupplies")]
    PioneerSupplies,
    #[serde(rename = "apexinterstellar")]
    ApexInterstellar,
    #[serde(rename = "frontlinesolutions")]
    FrontlineSolutions,
    #[serde(rename = "registeringcolonisation")]
    RegisteringColonisation,
}
#[derive(Deserialize)]
pub enum Economy{
    #[serde(rename = "$economy_Carrier;")]
    Carrier,
    #[serde(rename = "$economy_Industrial;")]
    Industrial,
    #[serde(rename = "$economy_Military;")]
    Military,
    #[serde(rename = "$economy_Extraction;")]
    Extraction,

}
#[derive(Deserialize)]
pub enum FriendStatus {
    Online,
    Offline,
}
//region - System Factions -
///System Security states, Anarchy is lowest
#[derive(Deserialize)]
pub enum SystemSecurity{
    #[serde(rename = "$GAlAXY_MAP_INFO_state_anarchy;")]
    Anarchy,
    #[serde(rename = "$SYSTEM_SECURITY_low;")]
    Low,
    #[serde(rename = "$SYSTEM_SECURITY_medium;")]
    Medium,
    #[serde(rename = "$SYSTEM_SECURITY_high;")]
    High,
}
///Faction Allegiances
#[derive(Deserialize)]
pub enum Allegiance{
    Empire,
    Federation,
    Independent
}
///All states a faction (and thus system at large) can be in, will be represented with an `Option<FactionState>` field, since there can be no active state.
#[derive(Deserialize)]
pub enum FactionState {
    None,
    Boom,
    Bust,
    CivilUnrest,
    Famine,
    Lockdown,
    Outbreak,
    War,
    CivilWar,
    Election,
    Retreat,
    Expansion,
    Blight,
    Drought,
    InfrastructureFailure,
    Terrorism,
    NaturalDisaster,
    PublicHoliday
}
///Possible system governments
#[derive(Deserialize)]
pub enum Government{
    Anarchy,
    Communist,
    Confederacy,
    Cooperative,
    Corporate,
    Democracy,
    Dictatorship,
    Feudal,
    #[serde(alias="$government_Patronage;")]
    Patronage,
    PrisonColony,
    Theocracy
}
///Current Powerplay State a system can be in, only covers the state the ruling faction is part of, so will never be "Exploiting" or "Undermining"
#[derive(Deserialize)]
pub enum PowerplayState{
    Unoccupied,
    Stronghold,
    Exploited,
    Fortified,
}
/// Enum of all Powerplay factions, Spaces and dashes removed. Use <enum value>.ToString() for the full name.
#[derive(Deserialize)]
pub enum PowerplayPower{
    #[serde(rename = "A. Lavigny-Duval")]
    ALavignyDuval,
    #[serde(rename = "Aisling Duval")]
    AislingDuval,
    #[serde(rename = "Denton Patreus")]
    DentonPatreus,
    #[serde(rename = "Zemina Torval")]
    ZeminaTorval,
    //TODO Fill out
}
impl fmt::Display for PowerplayPower {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PowerplayPower::ALavignyDuval => write!(f, "A. Lavigny-Duval"),
            PowerplayPower::AislingDuval => write!(f, "Aisling Duval"),
            PowerplayPower::DentonPatreus => write!(f, "Denton Patreus"),
            PowerplayPower::ZeminaTorval => write!(f, "Zemina Torval"),
        }
    }
}
#[derive(Deserialize)]
pub enum EngineerUnlockedStatus{
    Known,
    Invited,
    Unlocked
}