use serde::Deserialize;

#[derive(Deserialize)]
pub enum ShipType {
    #[serde(rename = "anaconda")]
    Anaconda,
    #[serde(rename = "sidewinder")]
    SideWinder,
    CobraMkV,
    Corsair,
    #[serde(rename = "type6")]
    Type6,
    #[serde(rename = "type7")]
    Type7,
    #[serde(rename = "type8")]
    Type8,
    #[serde(rename = "type9")]
    Type9,
    #[serde(rename = "python")]
    Python,
    PythonNX,
    SmallCombat01NX,
    #[serde(alias = "Explorer_NX", alias = "explorer_nx")]
    CaspianExplorer,
    Mandalay,
    #[serde(rename = "viper")]
    ViperMkIII,
    #[serde(rename = "viper_mkiv")]
    ViperMkIV,
    #[serde(rename = "vulture")]
    Vulture,
    #[serde(alias = "panthermkii", alias = "PantherMkII")]
    PantherClipperMkII,
    #[serde(rename = "asp")]
    AspExplorer,
    #[serde(rename = "krait_mkii")]
    KraitMkII,
}

#[derive(Deserialize)]
pub enum SRVType {
    #[serde(rename = "lander01")]
    Nomad,
    #[serde(rename = "testbuggy")]
    Scarab,
}

#[derive(Deserialize)]
pub enum SlotType{
    MainEngines,
    FrameShiftDrive,
    Thrusters,
}

#[derive(Deserialize)]
pub enum LimpetType{
    Collection,

}