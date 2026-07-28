use chrono::{DateTime, Utc};
use serde::Deserialize;
use crate::elite_events::enums::{Allegiance, Economy, FactionState, Government, ShipType};

//region - Inventory Items -
#[derive(Deserialize)]
pub struct Item {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: Option<String>,
    #[serde(rename = "OwnerID")]
    owner_id: u32,
    #[serde(rename = "Count")]
    count: u32,
}
#[derive(Deserialize)]
pub struct RawMaterial {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Count")]
    count: u32,
}
#[derive(Deserialize)]
pub struct LocalisedMaterial {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: String,
    #[serde(rename = "Count")]
    count: u32,
}
#[derive(Deserialize)]
pub struct Component{
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Count")]
    count: u64
}
#[derive(Deserialize)]
pub struct Consumable{
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: String,
    #[serde(rename = "Count")]
    count: u64
}
#[derive(Deserialize)]
pub struct Data{
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Name_Localised")]
    name_localised: String,
    #[serde(rename = "Count")]
    count: u64
}
#[derive(Deserialize)]
pub struct Materials {
    #[serde(rename = "Raw")]
    raw: Vec<RawMaterial>,
    #[serde(rename = "Manufactured")]
    manufactured: Vec<LocalisedMaterial>,
    #[serde(rename = "Encoded")]
    encoded: Vec<LocalisedMaterial>,
}
#[derive(Deserialize)]
pub struct SuitModule{
    #[serde(rename = "SlotName")]
    slot_name: String,
    #[serde(rename = "SuitModuleID")]
    suit_module_id: u64,
    #[serde(rename = "ModuleName")]
    module_name: String,
    #[serde(rename = "ModuleName_Localised")]
    module_name_localised: String,
    #[serde(rename = "Class")]
    class: u64,
    #[serde(rename = "WeaponMods")]
    weapon_mods: Vec<String>
}
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
#[derive(Deserialize)]
pub struct Module{
    #[serde(rename = "Slot")]
    slot: String,
    #[serde(rename = "Item")]
    item: String,
    #[serde(rename = "On")]
    on: bool,
    #[serde(rename = "Priority")]
    priority: u64,
    #[serde(rename = "Health")]
    health: f64
}
#[derive(Deserialize)]
pub struct FuelCapacity{
    #[serde(rename = "Main")]
    main: f64,
    #[serde(rename = "Reserve")]
    reserve: f64,
}
//endregion
//region - Missions -
#[derive(Deserialize)]
pub struct CGGoal{
    #[serde(rename = "CGID")]
    cgid: u64,
    #[serde(rename = "Title")]
    title: String,
    #[serde(rename = "SystemName")]
    system_name: String,
    #[serde(rename = "MarketName")]
    market_name: String,
    #[serde(rename = "Expiry")]
    expiry: DateTime<Utc>,
    #[serde(rename = "IsComplete")]
    is_complete: bool,
    #[serde(rename = "CurrentTotal")]
    current_total: u64,
    #[serde(rename = "PlayerContribution")]
    player_contribution: u64,
    #[serde(rename = "NumContributors")]
    num_contributors: u64,
    #[serde(rename = "TopTier")]
    top_tier: CGTier,
    #[serde(rename = "TopRankSize")]
    top_rank_size: u32,
    #[serde(rename = "PlayerInTopRank")]
    player_in_top_rank: bool,
    #[serde(rename = "PlayerPercentileBand")]
    player_percentile_band: u32,
}
#[derive(Deserialize)]
pub struct CGTier{
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Bonus")]
    bonus: String,
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

//region - Faction Stuff -
#[derive(Deserialize)]
pub struct Conflict{
    #[serde(rename = "WarType")]
    war_type: String,
    #[serde(rename = "Status")]
    status: String,
    #[serde(rename = "Faction1")]
    faction1: ConflictFaction,
}
#[derive(Deserialize)]
pub struct ConflictFaction{
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Stake")]
    stake: String,
    #[serde(rename = "WonDays")]
    won_days: u64,
}
#[derive(Deserialize)]
pub struct SystemFaction{
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "FactionState")]
    faction_state: Option<FactionState>,
}
#[derive(Deserialize)]
pub struct Faction{
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "FactionState")]
    faction_state: FactionState,
    #[serde(rename = "Government")]
    government: Government,
    #[serde(rename = "Influence")]
    influence: f64,
    #[serde(rename = "Allegiance")]
    allegiance: Allegiance,
    #[serde(rename = "Happiness")]
    happiness: String,
    #[serde(rename = "MyReputation")]
    my_reputation: f64,
    #[serde(rename = "ActiveStates")]
    active_states: Option<Vec<FactionState>>,
    #[serde(rename = "RecoveringStates")]
    recovering_states: Option<Vec<StateTrend>>,
    #[serde(rename = "PendingStates")]
    pending_states: Option<Vec<StateTrend>>,
}
#[derive(Deserialize)]
pub struct StateTrend
{
    #[serde(rename = "State")]
    state: FactionState,
    #[serde(rename = "Trend")]
    trend: u64
}
//endregion
