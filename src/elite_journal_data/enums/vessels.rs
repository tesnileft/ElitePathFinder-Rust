use serde::Deserialize;

#[derive(Deserialize)]
pub enum ShipType {
    #[serde(rename = "anaconda")]
    Anaconda,
    #[serde(rename = "sidewinder")]
    SideWinder,
    CobraMkV,
    Corsair,
    #[serde(rename = "type7")]
    Type7,
    Type8,
    #[serde(rename = "python")]
    Python,
    PythonNX,
    SmallCombat01NX,
    #[serde(rename = "explorer_nx")]
    ExplorerNX,
    Mandalay,
    #[serde(rename = "viper")]
    ViperMkIII,
    #[serde(rename = "viper_mkiv")]
    ViperMkIV,
    #[serde(rename = "vulture")]
    Vulture,
}

#[derive(Deserialize)]
pub enum SRVType {
    #[serde(rename = "lander01")]
    Nomad,
}