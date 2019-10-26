use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "camelCase")]
#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
pub struct SystemWithCoordinates {}
