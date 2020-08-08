use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use super::bgs;
use super::dec::{date_format, date_format_opt};
use super::RootEntry;

use super::util::DisplayViaSerde;
use crate::display_via_serde;

// Main Type

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Station {
    pub id: u64,
    // Attributes
    pub allegiance: Option<bgs::Allegiance>,
    pub body: Option<StationBody>,
    pub controlling_faction: Option<bgs::ControllingFaction>,
    pub distance_to_arrival: Option<f32>,
    pub economy: Option<bgs::Economy>,
    pub government: Option<bgs::Government>,
    pub have_market: bool,
    pub have_outfitting: bool,
    pub have_shipyard: bool,
    pub market_id: Option<u64>,
    pub name: String,
    pub other_services: Vec<OtherService>,
    pub second_economy: Option<bgs::Economy>,
    pub system_id: u64,
    pub system_id64: u64,
    pub system_name: String,
    #[serde(rename = "type")]
    pub typ: StationType,
    // Metadata
    pub update_time: UpdateTime,
}

impl RootEntry for Station {
    fn entry_id(&self) -> u64 {
        self.id
    }
}

// Filed Type

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
#[serde(deny_unknown_fields)]
pub enum OtherService {
    #[serde(rename = "Black Market")]
    BlackMarket,
    Contacts,
    #[serde(rename = "Crew Lounge")]
    CrewLounge,
    #[serde(rename = "Interstellar Factors Contact")]
    InterstellarFactorsContact,
    #[serde(rename = "Material Trader")]
    MaterialTrader,
    Missions,
    Refuel,
    Repair,
    Restock,
    #[serde(rename = "Search and Rescue")]
    SearchAndRescue,
    #[serde(rename = "Technology Broker")]
    TechnologyBroker,
    Tuning,
    #[serde(rename = "Universal Cartographics")]
    UniversalCartographics,
}

display_via_serde!(OtherService);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct StationBody {
    pub id: u64,
    // Attributes
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
#[serde(deny_unknown_fields)]
pub enum StationType {
    // Orbital Large
    #[serde(rename = "Ocellus Starport")]
    OcellusStarport,
    #[serde(rename = "Orbis Starport")]
    OrbisStarport,
    #[serde(rename = "Coriolis Starport")]
    CoriolisStarport,
    #[serde(rename = "Asteroid base")]
    AsteroidBase,
    #[serde(rename = "Mega ship")]
    MegaShip,
    // Orbital small
    Outpost,
    // Planetary
    #[serde(rename = "Planetary Port")]
    PlanetaryPort,
    #[serde(rename = "Planetary Outpost")]
    PlanetaryOutpost,
}

display_via_serde!(StationType);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct UpdateTime {
    #[serde(with = "date_format")]
    pub information: DateTime<Utc>,
    #[serde(with = "date_format_opt")]
    pub market: Option<DateTime<Utc>>,
    #[serde(with = "date_format_opt")]
    pub outfitting: Option<DateTime<Utc>>,
    #[serde(with = "date_format_opt")]
    pub shipyard: Option<DateTime<Utc>>,
}
