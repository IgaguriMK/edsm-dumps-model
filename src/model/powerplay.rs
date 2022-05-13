use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::EnumIter;
use variant_count::VariantCount;

use super::bgs;
use super::dec::date_format;
use super::system;
use super::RootEntry;

use super::util::DisplayViaSerde;
use crate::display_via_serde;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "type_hash", derive(type_hash::TypeHash))]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct PowerPlay {
    pub id: u64,
    // Attributes
    pub allegiance: Option<bgs::Allegiance>,
    pub coords: system::Coords,
    pub government: Option<bgs::Government>,
    pub id64: u64,
    pub name: String,
    pub power: Power,
    pub power_state: PowerState,
    pub state: Option<bgs::State>,
    // Metadata
    #[serde(with = "date_format")]
    #[cfg_attr(feature = "type_hash", type_hash(foreign_type))]
    pub date: DateTime<Utc>,
}

impl RootEntry for PowerPlay {
    fn entry_id(&self) -> u64 {
        self.id
    }

    fn type_name() -> &'static str {
        "power_play"
    }

    fn time(&self) -> DateTime<Utc> {
        self.date
    }
}

// Field Type

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, VariantCount)]
#[cfg_attr(feature = "type_hash", derive(type_hash::TypeHash))]
#[serde(deny_unknown_fields)]
pub enum Power {
    #[serde(rename = "A. Lavigny-Duval")]
    ALavignyDuval,
    #[serde(rename = "Aisling Duval")]
    AislingDuval,
    #[serde(rename = "Archon Delaine")]
    ArchonDelaine,
    #[serde(rename = "Denton Patreus")]
    DentonPatreus,
    #[serde(rename = "Edmund Mahon")]
    EdmundMahon,
    #[serde(rename = "Felicia Winters")]
    FeliciaWinters,
    #[serde(rename = "Li Yong-Rui")]
    LiYongRui,
    #[serde(rename = "Pranav Antal")]
    PranavAntal,
    #[serde(rename = "Yuri Grom")]
    YuriGrom,
    #[serde(rename = "Zachary Hudson")]
    ZacharyHudson,
    #[serde(rename = "Zemina Torval")]
    ZeminaTorval,
}

display_via_serde!(Power);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter, VariantCount)]
#[cfg_attr(feature = "type_hash", derive(type_hash::TypeHash))]
#[serde(deny_unknown_fields)]
pub enum PowerState {
    Contested,
    Controlled,
    Exploited,
    HomeSystem,
    InPrepareRadius,
    Prepared,
    Turmoil,
}

display_via_serde!(PowerState);
