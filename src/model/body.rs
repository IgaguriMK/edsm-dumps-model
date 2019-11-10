use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::dec::date_format;

// Main Type

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(tag = "type")]
pub enum Body {
    Planet(Planet),
    Star(Star),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Planet {
    pub id: u64,
    // Attributes
    pub atmosphere_type: AtmosphereType,
    pub body_id: u64,
    pub distance_to_arrival: f32,
    pub earth_masses: f32,
    pub gravity: f32,
    pub id64: u64,
    pub is_landable: bool,
    pub name: String,
    #[serde(flatten)]
    pub orbital_elements: OrbitalElements,
    pub radius: f32,
    #[serde(flatten)]
    pub rotational_elements: RotationalElements,
    pub sub_type: PlanetSubType,
    pub surface_pressure: Option<f32>,
    pub surface_temperature: f32,
    pub volcanism_type: VolcanismType,
    // Metadata
    #[serde(with = "date_format")]
    pub update_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Star {
    pub id: u64,
    // Attributes
    pub absolute_magnitude: f32,
    pub age: u64,
    pub body_id: u64,
    pub distance_to_arrival: f32,
    pub id64: u64,
    pub is_main_star: bool,
    pub is_scoopable: bool,
    pub luminosity: Luminosity,
    pub name: String,
    #[serde(flatten)]
    pub orbital_elements: OrbitalElements,
    #[serde(flatten)]
    pub rotational_elements: RotationalElements,
    pub solar_masses: f32,
    pub solar_radius: f32,
    pub spectral_class: SpectralClass,
    pub sub_type: StarSubType,
    pub surface_temperature: f32,
    // Metadata
    #[serde(with = "date_format")]
    pub update_time: DateTime<Utc>,
}

// Field Type

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum AtmosphereType {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Luminosity {
    V,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct OrbitalElements {
    pub arg_of_periapsis: Option<f32>,
    pub orbital_eccentricity: Option<f32>,
    pub orbital_inclination: Option<f32>,
    pub orbital_period: Option<f32>,
    pub parents: Option<Vec<Parent>>,
    pub semi_major_axis: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Parent {
    Star(u64),
    Null(u64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum PlanetSubType {
    #[serde(rename = "High metal content world")]
    HighMetalContentWorld,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct RotationalElements {
    pub axial_tilt: Option<f32>,
    pub rotational_period: f32,
    pub rotational_period_tidally_locked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum SpectralClass {
    M9,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum StarSubType {
    #[serde(rename = "M (Red dwarf) Star")]
    MRedDwarfStar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum VolcanismType {
    #[serde(rename = "No Volcanism")]
    NoVolcanism,
}
