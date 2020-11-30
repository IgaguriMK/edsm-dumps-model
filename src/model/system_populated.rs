use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::bgs;
use super::body;
use super::dec::date_format;
use super::station;
use super::system;
use super::RootEntry;

// Main Type

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SystemPopulated {
    pub id: u64,
    // Attributes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allegiance: Option<bgs::Allegiance>,

    pub bodies: Vec<body::Body>,
    pub controlling_faction: bgs::ControllingFaction,
    pub coords: system::Coords,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub economy: Option<bgs::Economy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factions: Option<Vec<FactionInPopulated>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub government: Option<bgs::Government>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id64: Option<u64>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub population: Option<u64>,
    pub security: bgs::Security,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<bgs::State>,
    pub stations: Vec<StationInPopulated>,
    // Metadata
    #[serde(with = "date_format")]
    pub date: DateTime<Utc>,
}

impl RootEntry for SystemPopulated {
    fn entry_id(&self) -> u64 {
        self.id
    }

    fn type_name() -> &'static str {
        "system_populated"
    }

    fn time(&self) -> DateTime<Utc> {
        self.date
    }
}

// Field Type

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct FactionInPopulated {
    pub id: u64,
    // Attributes
    pub active_states: Vec<bgs::ActiveState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allegiance: Option<bgs::Allegiance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub government: Option<bgs::Government>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub happiness: Option<bgs::Happiness>,
    pub influence: f32,
    pub is_player: bool,
    pub name: String,
    pub pending_states: Vec<bgs::PendingState>,
    pub recovering_states: Vec<bgs::RecoveringState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<bgs::State>,
    // Metadata
    #[serde(with = "ts_seconds")]
    pub last_update: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct StationInPopulated {
    pub id: u64,
    // Attributes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allegiance: Option<bgs::Allegiance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<station::StationBody>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controlling_faction: Option<bgs::ControllingFaction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_to_arrival: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub economy: Option<bgs::Economy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub government: Option<bgs::Government>,
    pub have_market: bool,
    pub have_outfitting: bool,
    pub have_shipyard: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_id: Option<u64>,
    pub name: String,
    pub other_services: Vec<station::OtherService>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_economy: Option<bgs::Economy>,
    #[serde(rename = "type")]
    pub st_type: station::StationType,
    // Metadata
    pub update_time: station::UpdateTime,
}
