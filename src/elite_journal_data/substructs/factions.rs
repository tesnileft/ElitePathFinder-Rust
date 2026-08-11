use serde::Deserialize;
use crate::elite_journal_data::enums::misc::Allegiance;
use crate::elite_journal_data::enums::system_data::{FactionState, Government};

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