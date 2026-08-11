use serde::Deserialize;

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