use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ActiveState {
    pub state: State,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Allegiance {
    Federation,
    Independent,
    #[serde(rename = "Pilots Federation")]
    PilotsFederation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ControllingFaction {
    pub id: u64,
    // Attributes
    pub allegiance: Option<Allegiance>,
    pub government: Option<Government>,
    pub is_player: Option<bool>,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Economy {
    Industrial,
    Refinery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Government {
    Anarchy,
    Confederacy,
    Corporate,
    Democracy,
    Dictatorship,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Happiness {
    None,
    Happy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct PendingState {
    pub state: State,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct RecoveringState {
    pub state: State,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Security {
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum State {
    #[serde(rename = "Civil war")]
    CivilWar,
    Expansion,
    None,
}
