use serde::Deserialize;

#[derive(Deserialize, Default)]
pub enum StationType {
    Coriolis,
    Orbis,
    Ocellus,
    Bernal,
    FleetCarrier,
    MegaShip,
    CraterPort,
    #[default]
    None,
}

#[derive(Deserialize)]
pub enum StationService {
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
    #[serde(rename = "blackmarket")]
    BlackMarket,
    #[serde(rename = "facilitator")]
    InterstellarFactor,
    #[serde(rename = "techBroker")]
    TechnologyBroker
}

#[derive(Deserialize)]
pub enum EngineerUnlockedStatus {
    Known,
    Invited,
    Unlocked,
}