use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
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
    #[serde(rename = "lakonminer")]
    Type11,
    #[serde(rename = "python")]
    Python,
    PythonNX,
    SmallCombat01NX,
    #[serde(alias = "diamondbackxl")]
    DiamondbackExplorer,
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

#[derive(Deserialize, Serialize)]
pub enum SRVType {
    #[serde(rename = "lander01")]
    Nomad,
    #[serde(rename = "testbuggy")]
    Scarab,
}

#[derive(Deserialize, Serialize)]
pub enum SlotType{
    MainEngines,
    PowerPlant,
    FrameShiftDrive,
    Thrusters,
    Slot02_Size6, //Needs to be renamed to something more sensible
}

#[derive(Deserialize, Serialize)]
pub enum CarrierType{
    FleetCarrier,
    SquadronCarrier,
}

#[derive(Deserialize, Serialize)]
pub enum DockingAccess{
    #[serde(rename = "all")]
    All,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "friends")]
    Friends,
    #[serde(rename = "squadron")]
    Squadron,
    #[serde(rename = "squadronfriends")]
    SquadronFriends,
}

#[derive(Deserialize, Serialize)]
pub enum CarrierCrewRole{
    BlackMarket,
    Captain,
    Refuel,
    Repair,
    Rearm,
    Commodities,
    VoucherRedemption,
    Exploration,
    Shipyard,
    Outfitting,
    CarrierFuel,
    VistaGenomics,
    PioneerSupplies,
    Bartender,
}



#[derive(Deserialize, Serialize)]
pub enum LimpetType{
    Collection,

}