use chrono::{DateTime, Utc};
use serde::Deserialize;
use crate::elite_events::enums::{Allegiance, Economy, EngineerUnlockedStatus, FactionState, Government, ShipType};

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
#[derive(Deserialize)]
pub struct CargoItem{
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Name_Localized")]
    pub name_localized: String,
    #[serde(rename = "Count")]
    pub count: u64,
    #[serde(rename = "Stolen")]
    pub stolen: u64,
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
#[derive(Deserialize)]
pub struct Mission{
    #[serde(rename = "MissionID")]
    mission_id: u64,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "PassengerMission")]
    passenger_mission: bool,
    #[serde(rename = "Expires")]
    expires: u64,
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
    active_states: Option<Vec<StateTrend>>,
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
    trend: Option<u64>
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
//endregion

//region - Statistics -
#[derive(Deserialize)]
#[serde(rename = "BankAccount")]
pub struct BankAccountStats {
    #[serde(rename = "Current_Wealth")]
    current_wealth: u64,
    #[serde(rename = "Spent_On_Ships")]
    spent_on_ships: u64,
    #[serde(rename = "Spent_On_Outfitting")]
    spent_on_outfitting: u64,
    #[serde(rename = "Spent_On_Repairs")]
    spent_on_repairs: u64,
    #[serde(rename = "Spent_On_Fuel")]
    spent_on_fuel: u64,
    #[serde(rename = "Spent_On_Ammo_Consumables")]
    spent_on_ammo_consumables: u64,
    #[serde(rename = "Insurance_Claims")]
    insurance_claims: u64,
    #[serde(rename = "Spent_On_Insurance")]
    spent_on_insurance: u64,
    #[serde(rename = "Owned_Ship_Count")]
    owned_ship_count: u64,
    #[serde(rename = "Spent_On_Suits")]
    spent_on_suits: u64,
    #[serde(rename = "Spent_On_Weapons")]
    spent_on_weapons: u64,
    #[serde(rename = "Spent_On_Suit_Consumables")]
    spent_on_suit_consumables: u64,
    #[serde(rename = "Suits_Owned")]
    suits_owned: u64,
    #[serde(rename = "Weapons_Owned")]
    weapons_owned: u64,
    #[serde(rename = "Spent_On_Premium_Stock")]
    spent_on_premium_stock: u64,
    #[serde(rename = "Premium_Stock_Bought")]
    premium_stock_bought: u64,
    #[serde(rename = "MercCoins_Current")]
    merccoins_current: u64,
    #[serde(rename = "MercCoins_Total_Earned")]
    merccoins_total_earned: u64,
    #[serde(rename = "MercCoins_Total_Spent")]
    merccoins_total_spent: u64,
    #[serde(rename = "MercCoins_Spent_On_MercGear")]
    merccoins_spent_on_mercgeear: u64,
    #[serde(rename = "MercCoins_Spent_On_Engineering")]
    merccoins_spent_on_engineering: u64,
}
//TODO fill out statistics structs
#[derive(Deserialize)]
#[serde(rename = "Combat")]
pub struct CombatStats{}
#[derive(Deserialize)]
#[serde(rename = "Crime")]
pub struct CrimeStats{}
#[derive(Deserialize)]
#[serde(rename = "Trading")]
pub struct TradingStats{}
#[derive(Deserialize)]
#[serde(rename = "Smuggling")]
pub struct SmugglingStats{}
#[derive(Deserialize)]
#[serde(rename = "Mining")]
pub struct MiningStats{}
#[derive(Deserialize)]
#[serde(rename = "Exploration")]
pub struct ExplorationStats{}
#[derive(Deserialize)]
#[serde(rename = "Passengers")]
pub struct PassengersStats{}
#[derive(Deserialize)]
#[serde(rename = "Search_And_Rescue")]
pub struct SearchAndRecueStats{}
#[derive(Deserialize)]
#[serde(rename = "Squadron")]
pub struct SquadronStats{}
#[derive(Deserialize)]
#[serde(rename = "Crafting")]
pub struct CraftingStats{}
#[derive(Deserialize)]
#[serde(rename = "Crew")]
pub struct CrewStats{}
#[derive(Deserialize)]
#[serde(rename = "Material_Trader_Stats")]
pub struct MaterialTraderStats{}
#[derive(Deserialize)]
#[serde(rename = "Exobiology")]
pub struct ExobiologyStats{}
//endregion