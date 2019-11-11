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
    Alliance,
    Empire,
    Federation,
    Independent,
    #[serde(rename = "Pilots Federation")]
    PilotsFederation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ControllingFaction {
    pub id: Option<u64>,
    // Attributes
    pub allegiance: Option<Allegiance>,
    pub government: Option<Government>,
    pub is_player: Option<bool>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Economy {
    Agriculture,
    Colony,
    Extraction,
    #[serde(rename = "High Tech")]
    HighTech,
    Industrial,
    Military,
    None,
    Prison,
    Refinery,
    Repair,
    Rescue,
    Service,
    Terraforming,
    Tourism,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Government {
    Anarchy,
    Communism,
    Confederacy,
    Cooperative,
    Corporate,
    Democracy,
    Dictatorship,
    Feudal,
    Patronage,
    Prison,
    #[serde(rename = "Prison colony")]
    PrisonColony,
    Theocracy,
    #[serde(rename = "Workshop (Engineer)")]
    WorkshopEngineer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Happiness {
    Despondent,
    Discontented,
    Elated,
    Happy,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct PendingState {
    pub state: State,
    pub trend: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct RecoveringState {
    pub state: State,
    pub trend: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Security {
    Anarchy,
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum State {
    Blight,
    Boom,
    Bust,
    #[serde(rename = "Civil liberty")]
    CivilLiberty,
    #[serde(rename = "Civil unrest")]
    CivilUnrest,
    #[serde(rename = "Civil war")]
    CivilWar,
    Election,
    Expansion,
    Famine,
    Investment,
    Lockdown,
    None,
    Outbreak,
    #[serde(rename = "Pirate attack")]
    PirateAttack,
    Retreat,
    War,
}
