use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
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
    StationONeilCylinder,
    StationDodec,
    StationAsteroid,
    TouristBeacon,
    Combat,
    Titan,

}

#[derive(Deserialize, Serialize)]
pub enum SAASignalType {
    #[serde(alias = "$SAA_SignalType_Biological;")]
    Biological,
    #[serde(rename = "$SAA_SignalType_Geological;")]
    Geological,
    #[serde(rename = "$SAA_SignalType_Human;")]
    Human,
    #[serde(rename = "$SAA_SignalType_Thargoid;")]
    Thargoid,
    #[serde(rename = "$PlanetaryMiningLocation_Name;")]
    PlanetaryMining,
    #[serde(rename = "$SAA_SignalType_Other;")]
    Other,
}