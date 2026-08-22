use crate::elite_journal_data::enums::misc::FriendStatus;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SendText {
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "To")]
    to: String,
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "Sent")]
    sent: bool,
}

#[derive(Deserialize)]
pub struct ReceiveText {
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "From")]
    pub from: String,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "Message_Localised")]
    pub message_localised: String,
    #[serde(rename = "Channel")]
    pub channel: String,
}

#[derive(Deserialize)]
pub struct Friends {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Status")]
    status: FriendStatus,
}

#[derive(Deserialize)]
pub struct SquadronStartup {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "SquadronID")]
    squadron_id: u64,
    #[serde(rename = "SquadronName")]
    squadron_name: String,
    #[serde(rename = "CurrentRank")]
    current_rank: u64,
    #[serde(rename = "CurrentRankName")]
    current_rank_name: String,
}

#[derive(Deserialize)]
pub struct WingInvite {
    timestamp: DateTime<Utc>,
    #[serde(rename = "Name")]
    name: String,
}

#[derive(Deserialize)]
pub struct WingJoin{
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Others")]
    others: Vec<String>
}

#[derive(Deserialize)]
pub struct WingAdd {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "Name")]
    name: String,
}

#[derive(Deserialize)]
pub struct WingLeave {
    #[serde(rename = "timestamp")]
    timestamp: DateTime<Utc>,
}