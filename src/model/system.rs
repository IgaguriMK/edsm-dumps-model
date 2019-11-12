use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::dec::date_format;
use super::RootEntry;

// Main Type

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SystemWithCoordinates {
    pub id: u64,
    // Attributes
    pub coords: Coords,
    pub id64: Option<u64>,
    pub name: String,
    // Metadata
    #[serde(with = "date_format")]
    pub date: DateTime<Utc>,
}

impl RootEntry for SystemWithCoordinates {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SystemWithoutCoordinates {
    pub id: u64,
    // Attributes
    pub estimated_coordinates: Option<EstimatedCoords>,
    pub id64: Option<u64>,
    pub name: String,
    // Metadata
    #[serde(with = "date_format")]
    pub date: DateTime<Utc>,
}

impl RootEntry for SystemWithoutCoordinates {}

// Field Type

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Coords {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct EstimatedCoords {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub precision: f32,
}
