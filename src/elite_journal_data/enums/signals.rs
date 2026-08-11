use serde::Deserialize;

#[derive(Deserialize)]
pub enum SignalType {
    Generic,
    Codex,
    FleetCarrier,
    NavBeacon,
    Megaship,
    ResourceExtraction,
    Installation,
    StationCoriolis,
    SquadronCarrier,
    Outpost,
    StationMegaShip,
    StationBernalSphere,
    StationONeilOrbis,
    StationAsteroid,
    TouristBeacon,
}

#[derive(Deserialize)]
pub enum SAASignalType {
    #[serde(alias = "$SAA_SignalType_Biological;")]
    Biological,
    #[serde(rename = "$SAA_SignalType_Geological;")]
    Geological,
}