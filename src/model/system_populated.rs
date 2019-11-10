use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::bgs;
use super::body;
use super::dec::date_format;
use super::station;
use super::system;

// Main Type

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SystemPopulated {
    pub id: u64,
    // Attributes
    pub allegiance: bgs::Allegiance,
    pub bodies: Vec<body::Body>,
    pub controlling_faction: bgs::ControllingFaction,
    pub coords: system::Coords,
    pub economy: Option<bgs::Economy>,
    pub factions: Option<Vec<FactionInPopulated>>,
    pub government: bgs::Government,
    pub id64: Option<u64>,
    pub name: String,
    pub population: u64,
    pub security: bgs::Security,
    pub state: Option<bgs::State>,
    pub stations: Vec<StationInPopulated>,
    // Metadata
    #[serde(with = "date_format")]
    pub date: DateTime<Utc>,
}

// Field Type

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct FactionInPopulated {
    pub id: u64,
    // Attributes
    pub active_states: Vec<bgs::ActiveState>,
    pub allegiance: bgs::Allegiance,
    pub government: bgs::Government,
    pub happiness: bgs::Happiness,
    pub influence: f32,
    pub is_player: bool,
    pub name: String,
    pub pending_states: Vec<bgs::PendingState>,
    pub recovering_states: Vec<bgs::RecoveringState>,
    pub state: bgs::State,
    // Metadata
    #[serde(with = "ts_seconds")]
    pub last_update: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct StationInPopulated {
    pub id: u64,
    // Attributes
    pub allegiance: bgs::Allegiance,
    pub body: Option<station::StationBody>,
    pub controlling_faction: Option<bgs::ControllingFaction>,
    pub distance_to_arrival: f32,
    pub economy: bgs::Economy,
    pub government: bgs::Government,
    pub have_market: bool,
    pub have_outfitting: bool,
    pub have_shipyard: bool,
    pub market_id: u64,
    pub name: String,
    pub other_services: Vec<station::OtherService>,
    pub second_economy: Option<bgs::Economy>,
    #[serde(rename = "type")]
    pub st_type: station::StationType,
    // Metadata
    pub update_time: station::UpdateTime,
}
