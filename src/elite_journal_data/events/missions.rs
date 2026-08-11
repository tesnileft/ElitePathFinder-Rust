use chrono::{DateTime, Utc};
use serde::Deserialize;
use crate::elite_journal_data::substructs::missions::{CGGoal, Mission};

#[derive(Deserialize)]
pub struct CommunityGoal {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "CurrentGoals")]
    current_goals: Vec<CGGoal>,
}

#[derive(Deserialize)]
pub struct CommunityGoalReward{
    timestamp: DateTime<Utc>,
    #[serde(rename="CGID")]
    cg_id: u64,
    #[serde(rename="Name")]
    name: String,
    #[serde(rename="System")]
    system: String,
    #[serde(rename="Reward")]
    reward: u64
}

#[derive(Deserialize)]
pub struct Missions {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Active")]
    active: Vec<Mission>,
    #[serde(rename = "Failed")]
    failed: Vec<Mission>,
    #[serde(rename = "Complete")]
    complete: Vec<Mission>,
}

#[derive(Deserialize)]
pub struct MissionAccepted {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Faction")]
    faction: String,
    #[serde(rename = "Name  ")]
    name: String, //TODO could become an enum
    #[serde(rename = "LocalisedName")]
    localised_name: String,
    #[serde(rename = "Donation")]
    donation: String, //Yes it's a string
    #[serde(rename = "Expiry")]
    expiry: DateTime<Utc>,
    #[serde(rename="Wing")]
    wing: bool, //Team
    #[serde(rename="Influence")]
    influence: String,
    #[serde(rename="Reputation")]
    reputation: String,
    #[serde(rename="MissionID")]
    mission_id: u64,
}
