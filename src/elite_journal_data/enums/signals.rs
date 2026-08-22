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
    Combat,
    Titan,

}

#[derive(Deserialize)]
pub enum SAASignalType {
    #[serde(alias = "$SAA_SignalType_Biological;")]
    Biological,
    #[serde(rename = "$SAA_SignalType_Geological;")]
    Geological,
    #[serde(rename = "$SAA_SignalType_Human;")]
    Human,
    #[serde(rename = "$SAA_SignalType_Thargoid;")]
    Thargoid,
    #[serde(rename = "$SAA_SignalType_Other;")]
    Other,
}