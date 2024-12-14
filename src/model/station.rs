use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::EnumIter;
use variant_count::VariantCount;

use super::bgs;
use super::dec::{date_format, date_format_opt};
use super::RootEntry;

use super::util::DisplayViaSerde;
use crate::display_via_serde;

// Main Type

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "type_hash", derive(type_hash::TypeHash))]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Station {
    pub id: u64,
    // Attributes
    pub allegiance: Option<bgs::Allegiance>,
    pub body: Option<StationBody>,
    pub commodities: Option<Vec<Commodity>>,
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
    pub outfitting: Option<Vec<Outfitting>>,
    pub second_economy: Option<bgs::Economy>,
    pub ships: Option<Vec<Ship>>,
    pub system_id: Option<u64>,
    pub system_id64: Option<u64>,
    pub system_name: Option<String>,
    #[serde(rename = "type")]
    pub typ: Option<StationType>,
    // Metadata
    pub update_time: UpdateTime,
}

impl RootEntry for Station {
    fn entry_id(&self) -> u64 {
        self.id
    }

    fn type_name() -> &'static str {
        "station"
    }

    fn time(&self) -> DateTime<Utc> {
        self.update_time.information
    }
}

// Filed Type

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "type_hash", derive(type_hash::TypeHash))]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Commodity {
    id: Option<String>,
    name: String,
    // Attributes
    buy_price: u64,
    demand: u64,
    sell_price: u64,
    stock: u64,
    stock_bracket: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, VariantCount)]
#[cfg_attr(feature = "type_hash", derive(type_hash::TypeHash))]
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
    #[serde(rename = "Apex Interstellar Transport")]
    ApexInterstellarTransport,
    #[serde(rename = "Frontline Solutions")]
    FrontlineSolutions,
    #[serde(rename = "Pioneer Supplies")]
    PioneerSupplies,
    #[serde(rename = "Vista Genomics")]
    VistaGenomics,
}

display_via_serde!(OtherService);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "type_hash", derive(type_hash::TypeHash))]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Outfitting {
    id: String,
    name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "type_hash", derive(type_hash::TypeHash))]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Ship {
    id: u64,
    name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "type_hash", derive(type_hash::TypeHash))]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct StationBody {
    pub id: u64,
    // Attributes
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, VariantCount)]
#[cfg_attr(feature = "type_hash", derive(type_hash::TypeHash))]
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
    #[serde(rename = "Odyssey Settlement")]
    OdysseySettlement,
    // Fleet Carrier
    #[serde(rename = "Fleet Carrier")]
    FleetCarrier,
}

display_via_serde!(StationType);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "type_hash", derive(type_hash::TypeHash))]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct UpdateTime {
    #[serde(with = "date_format")]
    #[cfg_attr(feature = "type_hash", type_hash(foreign_type))]
    pub information: DateTime<Utc>,
    #[serde(with = "date_format_opt")]
    #[serde(default = "option_none")]
    #[cfg_attr(feature = "type_hash", type_hash(foreign_type))]
    pub market: Option<DateTime<Utc>>,
    #[serde(with = "date_format_opt")]
    #[serde(default = "option_none")]
    #[cfg_attr(feature = "type_hash", type_hash(foreign_type))]
    pub outfitting: Option<DateTime<Utc>>,
    #[serde(with = "date_format_opt")]
    #[serde(default = "option_none")]
    #[cfg_attr(feature = "type_hash", type_hash(foreign_type))]
    pub shipyard: Option<DateTime<Utc>>,
}

fn option_none<T>() -> Option<T> {
    None
}
