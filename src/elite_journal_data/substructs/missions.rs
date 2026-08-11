use chrono::{DateTime, Utc};
use serde::Deserialize;


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