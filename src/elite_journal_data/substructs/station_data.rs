use serde::Deserialize;
use crate::elite_journal_data::enums::station_data::EngineerUnlockedStatus;
use crate::elite_journal_data::enums::system_data::Economy;
use crate::elite_journal_data::enums::vessels::ShipType;

//endregion
//region - Ship Related -
#[derive(Deserialize)]
pub struct LandingPads {
    #[serde(rename = "Small")]
    small: u32,
    #[serde(rename = "Medium")]
    medium: u32,
    #[serde(rename = "Large")]
    large: u32,
}

#[derive(Deserialize)]
pub struct StoredShip{
    #[serde(rename = "ShipID")]
    ship_id: u64,
    #[serde(rename = "ShipType")]
    ship_type: ShipType,
    #[serde(rename = "ShipType_Localised")]
    ship_type_localised: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "StarSystem")]
    star_system: String,
    #[serde(rename = "ShipMarketID")]
    ship_market_id: u64,
    #[serde(rename = "TransferPrice")]
    transfer_price: u64,
    #[serde(rename = "Value")]
    value: u64,
    #[serde(rename = "Hot")]
    hot: bool,
}

//endregion
//region - Locations -
#[derive(Deserialize)]
pub struct StationFaction {
    #[serde(rename = "Name")]
    name: String,
}

#[derive(Deserialize)]
pub struct StationEconomy{
    #[serde(rename = "Name")]
    name: Economy,
    #[serde(rename = "Name_Localised")]
    name_localised: String,
    #[serde(rename = "Proportion")]
    proportion: f32,
}

//endregion
//region - Engineer -
#[derive(Deserialize)]
pub struct Engineer{
    #[serde(rename = "Engineer")]
    engineer: String,
    #[serde(rename = "EngineerID")]
    id: u64,
    #[serde(rename = "Progress")]
    progress: EngineerUnlockedStatus,
    #[serde(rename = "RankProgress")]
    rank_progress: Option<u64>,
    #[serde(rename = "Rank")]
    rank: Option<u16>
}