use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::dec::{date_format, date_format_opt};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct StationBody {
    pub id: u64,
    // Attributes
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
