use std::fmt;
use std::fmt::Display;
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
#[derive(Deserialize, Default)]
pub enum StationType{
    FleetCarrier,
    Coriolis,
    Orbis,
    #[default]
    None
}
#[derive(Deserialize)]
pub enum BodyType{
    Star,
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
    O,
    A,
    B,
    D,
    DA,
    G,
    F,
    K,
    L,
    T,
    TTS,
    Ae,
    Y,
    W,
    N,
    M,
    DB,
    DC,
    DO,
    DQ,
    DX,
    DAV,
    DBV,
    DCV,
    WO,
    WC,
    WNC,
    WN,
    AeBe,
    H,
    SupermassiveBlackHole,
    X,
    RoguePlanet,
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
#[derive(Deserialize, Default)]
pub enum Economy{
    #[serde(rename = "$economy_Carrier;")]
    Carrier,
    #[serde(rename = "$economy_Industrial;")]
    Industrial,
    #[serde(rename = "$economy_Military;")]
    Military,
    #[serde(rename = "$economy_Extraction;")]
    Extraction,
    #[default]
    #[serde(rename = "$economy_None;")]
    None,

}
#[derive(Deserialize)]
pub enum FriendStatus {
    Online,
    Offline,
}
//region - System Factions -
///System Security states, Anarchy is lowest
#[derive(Deserialize, Default)]
pub enum SystemSecurity{
    #[default]
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
#[derive(Deserialize, Default)]
pub enum Allegiance{
    Empire,
    Federation,
    Independent,
    #[default]
    #[serde(rename="")]
    None,
}
///All states a faction (and thus system at large) can be in, will be represented with an `Option<FactionState>` field, since there can be no active state.
#[derive(Deserialize, Default)]
pub enum FactionState {
    #[default]
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
#[derive(Deserialize, Default)]
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
    Theocracy,
    #[default]
    #[serde(alias="$government_None;")]
    None
}
///Current Powerplay State a system can be in, only covers the state the ruling faction is part of, so will never be "Exploiting" or "Undermining"
#[derive(Deserialize, Default)]
pub enum PowerplayState{
    #[default]
    Unoccupied,
    Stronghold,
    Exploited,
    Fortified,
}
/// Enum of all Powerplay factions, Spaces and dashes removed. Use <enum value>.to_string() for the full name.
#[derive(Deserialize, Default)]
pub enum PowerplayPower{
    #[serde(rename = "A. Lavigny-Duval")]
    ALavignyDuval,
    #[serde(rename = "Aisling Duval")]
    AislingDuval,
    #[serde(rename = "Denton Patreus")]
    DentonPatreus,
    #[serde(rename = "Zemina Torval")]
    ZeminaTorval,
    #[default]
    None
    //TODO Fill out
}
impl fmt::Display for PowerplayPower {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PowerplayPower::ALavignyDuval => write!(f, "A. Lavigny-Duval"),
            PowerplayPower::AislingDuval => write!(f, "Aisling Duval"),
            PowerplayPower::DentonPatreus => write!(f, "Denton Patreus"),
            PowerplayPower::ZeminaTorval => write!(f, "Zemina Torval"),
            _ => write!(f, "Undefined")
        }
    }
}
#[derive(Deserialize)]
pub enum EngineerUnlockedStatus{
    Known,
    Invited,
    Unlocked
}

#[derive(Deserialize)]
pub enum SRVType{
    #[serde(rename = "lander01")]
    Nomad,
}

#[derive(Deserialize, Default, Debug, Clone)]
pub enum AtmosphereType {
    Argon,
    ArgonRich,
    Methane,
    MethaneRich,
    SilicateVapour,
    CarbonDioxide,
    CarbonDioxideRich,
    Nitrogen,
    SulphurDioxide,
    Helium,
    Neon,
    NeonRich,
    Oxygen,
    Ammonia,
    Water,
    WaterRich,
    #[default]
    None
}
impl fmt::Display for AtmosphereType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Deserialize, Debug, Clone)]
pub enum Species{
    //Bacterium
    Acies,
    Alcyoneum,
    Aurasus,
    Bullaris,
    Cerbrus,
    Informem,
    Nebulus,
    Omentum,
    Scopulum,
    Tela,
    Verrata,
    #[serde(rename="$Codex_Ent_Bacterial_05_Name;")]
    Vesicula,
    Volu,
    //Fungoida
    Bullarum,
    Gelata,
    Setisis,
    Stabitis
}


impl Display for Species{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Deserialize, Default, Debug)]
pub enum ExoBiologyVariant {
    Emerald,
    Gold,
    Maroon,
    Cobalt,
    Green,
    Yellow,
    Orange,
    Red,
    Magenta,
    Lime,
    Peach,
    Mulberry,
    Cyan,
    White,
    Blue,
    Aquamarine,
    Turquoise,
    Grey,
    Teal,
    Sage,
    Russet,
    Mauve,
    Amber,
    Ochre,
    Indigo,
    #[default]
    Unknown,
}
impl Display for ExoBiologyVariant{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Deserialize, Eq, PartialEq)]
pub enum PlanetClass{
    #[serde(rename="High metal content body")]
    HMC,
    #[serde(rename="Metal rich body")]
    MetalRich,
    #[serde(rename="Rocky body")]
    Rocky,
    #[serde(rename="Icy body")]
    Icy,
    #[serde(rename="Rocky ice body")]
    RockyIce,
    #[serde(rename="Sudarsky class I gas giant")]
    GasGiantClass1,
    #[serde(rename="Sudarsky class II gas giant")]
    GasGiantClass2,
    #[serde(rename="Sudarsky class III gas giant")]
    GasGiantClass3,
    #[serde(rename="Sudarsky class IV gas giant")]
    GasGiantClass4,
    #[serde(rename="Gas giant with water based life")]
    GasGiantWBL,
    #[serde(rename="Gas giant with ammonia based life")]
    GasGiantABL,
    #[serde(rename="Helium rich gas giant")]
    GasGiantHeliumRich
}

#[derive(Deserialize, Default, Eq, PartialEq, Clone)]
pub enum Volcanism{
    #[serde(rename="minor silicate vapour geysers volcanism")]
    MinorSilicateVapour,
    #[serde(rename="major silicate vapour geysers volcanism")]
    MajorSilicateVapour,
    #[serde(rename="water geysers volcanism")]
    WaterGeysers,
    #[serde(rename="major water geysers volcanism")]
    MajorWaterGeysers,
    #[serde(rename="minor water geysers volcanism")]
    MinorWaterGeysers,
    #[serde(rename="water magma volcanism")]
    WaterMagma,
    #[serde(rename="major water magma volcanism")]
    MajorWaterMagma,
    #[serde(rename="minor water magma volcanism")]
    MinorWaterMagma,
    #[serde(rename="rocky magma volcanism")]
    RockyMagma,
    #[serde(rename="major rocky magma volcanism")]
    MajorRockyMagma,
    #[serde(rename="minor rocky magma geysers volcanism")]
    MinorRockyMagma,
    #[serde(rename="carbon dioxide geysers volcanism")]
    CarbonDioxide,
    #[serde(rename="carbon dioxide geysers volcanism")]
    MajorCarbonDioxide,
    #[serde(rename="minor carbon dioxide geysers volcanism")]
    MinorCarbonDioxide,
    #[serde(rename="methane geysers volcanism")]
    Methane,
    #[serde(rename="minor methane geysers volcanism")]
    MinorMethane,
    #[serde(rename="Major methane geysers volcanism")]
    MajorMethane,
    #[serde(rename="nitrogen geysers volcanism")]
    Nitrogen,
    #[serde(rename="minor nitrogen geysers volcanism")]
    MinorNitrogen,
    #[serde(rename="Major nitrogen geysers volcanism")]
    MajorNitrogen,
    #[serde(rename="ammonia geysers volcanism")]
    Ammonia,
    #[serde(rename="minor ammonia geysers volcanism")]
    MinorAmmonia,
    #[serde(rename="Major ammonia geysers volcanism")]
    MajorAmmonia,
    #[serde(rename="ammonia magma volcanism")]
    AmmoniaMagma,
    #[serde(rename="minor ammonia magma volcanism")]
    MinorAmmoniaMagma,
    #[serde(rename="major ammonia magma volcanism")]
    MajorAmmoniaMagma,
    #[serde(rename="metallic magma volcanism")]
    MetallicMagma,
    #[serde(rename="major metallic magma volcanism")]
    MajorMetallicMagma,
    #[serde(rename="minor metallic magma volcanism")]
    MinorMetallicMagma,
    Helium,
    Iron,
    #[serde(rename="")]
    #[default]
    None,
}


#[derive(Debug, Deserialize)]
pub enum BodyParent {
    Ring(u64),
    Star(u64),
    Null(u64),
    Planet(u64)
}