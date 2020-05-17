use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use super::dec::date_format;
use super::RootEntry;

use super::util::DisplayViaSerde;
use crate::display_via_serde;

// Main Type

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(tag = "type")]
#[allow(clippy::large_enum_variant)]
pub enum Body {
    Planet(Planet),
    Star(Star),
    #[serde(rename = "null")]
    Unknown(Unknown),
}

macro_rules! body_common_field {
    ($f:ident, $ty:ty ) => {
        pub fn $f(&self) -> $ty {
            match self {
                Body::Planet(x) => x.$f,
                Body::Star(x) => x.$f,
                Body::Unknown(x) => x.$f,
            }
        }
    };
}

impl Body {
    body_common_field!(id, u64);
    body_common_field!(id64, Option<u64>);
    body_common_field!(system_id, Option<u64>);
    body_common_field!(system_id64, Option<u64>);
    body_common_field!(update_time, DateTime<Utc>);

    pub fn name(&self) -> &str {
        match self {
            Body::Planet(x) => &x.name,
            Body::Star(x) => &x.name,
            Body::Unknown(x) => &x.name,
        }
    }

    pub fn system_name(&self) -> Option<&str> {
        match self {
            Body::Planet(x) => x.system_name.as_deref(),
            Body::Star(x) => x.system_name.as_deref(),
            Body::Unknown(x) => x.system_name.as_deref(),
        }
    }
}

impl RootEntry for Body {
    fn pre_filter(s: &str) -> Cow<'_, str> {
        let null_pos = s.find(r#""type":null"#);
        let first_compound = match (s.find(":{"), s.find('[')) {
            (None, None) => None,
            (Some(x), None) => Some(x),
            (None, Some(y)) => Some(y),
            (Some(x), Some(y)) => Some(x.min(y)),
        };

        let type_is_null = match (null_pos, first_compound) {
            (None, _) => false,
            (Some(_), None) => true,
            (Some(n), Some(o)) => n < o,
        };

        if type_is_null {
            Cow::Owned(s.replacen(r#""type":null"#, r#""type":"null""#, 1))
        } else {
            Cow::Borrowed(s)
        }
    }
}

/// Surrogate type for some encodings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(clippy::large_enum_variant)]
pub enum BodyS {
    Planet(Planet),
    Star(Star),
    #[serde(rename = "null")]
    Unknown(Unknown),
}

macro_rules! body_s_common_field {
    ($f:ident, $ty:ty ) => {
        pub fn $f(&self) -> $ty {
            match self {
                BodyS::Planet(x) => x.$f,
                BodyS::Star(x) => x.$f,
                BodyS::Unknown(x) => x.$f,
            }
        }
    };
}

impl BodyS {
    body_s_common_field!(id, u64);
    body_s_common_field!(id64, Option<u64>);
    body_s_common_field!(system_id, Option<u64>);
    body_s_common_field!(system_id64, Option<u64>);
    body_s_common_field!(update_time, DateTime<Utc>);

    pub fn name(&self) -> &str {
        match self {
            BodyS::Planet(x) => &x.name,
            BodyS::Star(x) => &x.name,
            BodyS::Unknown(x) => &x.name,
        }
    }

    pub fn system_name(&self) -> Option<&str> {
        match self {
            BodyS::Planet(x) => x.system_name.as_deref(),
            BodyS::Star(x) => x.system_name.as_deref(),
            BodyS::Unknown(x) => x.system_name.as_deref(),
        }
    }
}

impl From<Body> for BodyS {
    fn from(body: Body) -> BodyS {
        match body {
            Body::Planet(x) => BodyS::Planet(x),
            Body::Star(x) => BodyS::Star(x),
            Body::Unknown(x) => BodyS::Unknown(x),
        }
    }
}

impl From<BodyS> for Body {
    fn from(body: BodyS) -> Body {
        match body {
            BodyS::Planet(x) => Body::Planet(x),
            BodyS::Star(x) => Body::Star(x),
            BodyS::Unknown(x) => Body::Unknown(x),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Planet {
    pub id: u64,
    // Attributes
    pub arg_of_periapsis: Option<f32>,
    pub atmosphere_composition: Option<AtmosphereComposition>,
    pub atmosphere_type: Option<AtmosphereType>,
    pub axial_tilt: Option<f32>,
    pub belts: Option<Vec<Belt>>,
    pub body_id: Option<u64>,
    pub discovery: Option<Discovery>,
    pub distance_to_arrival: u64,
    pub earth_masses: f32,
    pub gravity: Option<f32>,
    pub id64: Option<u64>,
    pub is_landable: bool,
    pub materials: Option<Materials>,
    pub name: String,
    pub orbital_eccentricity: Option<f32>,
    pub orbital_inclination: Option<f32>,
    pub orbital_period: Option<f32>,
    pub parents: Option<Vec<Parent>>,
    pub radius: f32,
    pub reserve_level: Option<ReserveLevel>,
    pub rings: Option<Vec<Ring>>,
    pub rotational_period: Option<f32>,
    pub rotational_period_tidally_locked: bool,
    pub semi_major_axis: Option<f32>,
    pub solid_composition: Option<SolidComposition>,
    pub sub_type: PlanetSubType,
    pub surface_pressure: Option<f32>,
    pub surface_temperature: u64,
    pub system_id: Option<u64>,
    pub system_id64: Option<u64>,
    pub system_name: Option<String>,
    pub terraforming_state: Option<TerraformingState>,
    pub volcanism_type: Option<VolcanismType>,
    // Metadata
    #[serde(with = "date_format")]
    pub update_time: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Star {
    pub id: u64,
    // Attributes
    pub absolute_magnitude: Option<f32>,
    pub age: u64,
    pub arg_of_periapsis: Option<f32>,
    pub axial_tilt: Option<f32>,
    pub belts: Option<Vec<Belt>>,
    pub body_id: Option<u64>,
    pub discovery: Option<Discovery>,
    pub distance_to_arrival: u64,
    pub id64: Option<u64>,
    pub is_main_star: bool,
    pub is_scoopable: bool,
    pub luminosity: Option<Luminosity>,
    pub name: String,
    pub orbital_eccentricity: Option<f32>,
    pub orbital_inclination: Option<f32>,
    pub orbital_period: Option<f32>,
    pub parents: Option<Vec<Parent>>,
    pub reserve_level: Option<ReserveLevel>,
    pub rings: Option<Vec<Ring>>,
    pub rotational_period: Option<f32>,
    pub rotational_period_tidally_locked: bool,
    pub semi_major_axis: Option<f32>,
    pub solar_masses: f32,
    pub solar_radius: f32,
    pub spectral_class: Option<SpectralClass>,
    pub sub_type: StarSubType,
    pub surface_temperature: u64,
    pub system_id: Option<u64>,
    pub system_id64: Option<u64>,
    pub system_name: Option<String>,
    // Metadata
    #[serde(with = "date_format")]
    pub update_time: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Unknown {
    pub id: u64,
    // Attributes
    pub id64: Option<u64>,
    pub name: String,
    pub system_id: Option<u64>,
    pub system_id64: Option<u64>,
    pub system_name: Option<String>,
    // Metadata
    #[serde(with = "date_format")]
    pub update_time: DateTime<Utc>,
}

// Field Type

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, EnumIter,
)]
#[serde(deny_unknown_fields)]
pub enum AsteroidType {
    Icy,
    Rocky,
    #[serde(rename = "Metal Rich")]
    MetalRich,
    Metallic,
}

display_via_serde!(AsteroidType);

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AtmosphereComposition(BTreeMap<AtmosphereCompositionKey, f32>);

impl AtmosphereComposition {
    pub fn get(&self, key: AtmosphereCompositionKey) -> Option<f32> {
        self.0.get(&key).copied()
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, EnumIter,
)]
pub enum AtmosphereCompositionKey {
    Ammonia,
    Argon,
    #[serde(rename = "Carbon dioxide")]
    CarbonDioxide,
    Helium,
    Hydrogen,
    Iron,
    Methane,
    Neon,
    Nitrogen,
    Oxygen,
    Silicates,
    #[serde(rename = "Sulphur dioxide")]
    SulphurDioxide,
    Water,
}

display_via_serde!(AtmosphereCompositionKey);

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, EnumIter,
)]
pub enum AtmosphereType {
    /* normal */
    Ammonia,
    #[serde(rename = "Ammonia and Oxygen")]
    AmmoniaAndOxygen,
    #[serde(rename = "Ammonia-rich")]
    AmmoniaRich,
    Argon,
    #[serde(rename = "Argon-rich")]
    ArgonRich,
    #[serde(rename = "Carbon dioxide")]
    CarbonDioxide,
    #[serde(rename = "Carbon dioxide-rich")]
    CarbonDioxideRich,
    Helium,
    #[serde(rename = "Metallic vapour")]
    MetallicVapour,
    Methane,
    #[serde(rename = "Methane-rich")]
    MethaneRich,
    Neon,
    #[serde(rename = "Neon-rich")]
    NeonRich,
    Nitrogen,
    #[serde(rename = "No atmosphere")]
    NoAtmosphere,
    Oxygen,
    #[serde(rename = "Silicate vapour")]
    SilicateVapour,
    #[serde(rename = "Suitable for water-based life")]
    SuitableForWaterBasedLife,
    #[serde(rename = "Sulphur dioxide")]
    SulphurDioxide,
    Water,
    #[serde(rename = "Water-rich")]
    WaterRich,
    /* Hot */
    #[serde(rename = "Hot Argon")]
    HotArgon,
    #[serde(rename = "Hot Argon-rich")]
    HotArgonRich,
    #[serde(rename = "Hot Carbon dioxide")]
    HotCarbonDioxide,
    #[serde(rename = "Hot Carbon dioxide-rich")]
    HotCarbonDioxideRich,
    #[serde(rename = "Hot Metallic vapour")]
    HotMetallicVapour,
    #[serde(rename = "Hot Silicate vapour")]
    HotSilicateVapour,
    #[serde(rename = "Hot Sulphur dioxide")]
    HotSulphurDioxide,
    #[serde(rename = "Hot Water")]
    HotWater,
    #[serde(rename = "Hot Water-rich")]
    HotWaterRich,
    /* Hot thick */
    #[serde(rename = "Hot thick Ammonia")]
    HotThickAmmonia,
    #[serde(rename = "Hot thick Ammonia-rich")]
    HotThickAmmoniaRich,
    #[serde(rename = "Hot thick Argon")]
    HotThickArgon,
    #[serde(rename = "Hot thick Argon-rich")]
    HotThickArgonRich,
    #[serde(rename = "Hot thick Carbon dioxide")]
    HotThickCarbonDioxide,
    #[serde(rename = "Hot thick Carbon dioxide-rich")]
    HotThickCarbonDioxideRich,
    #[serde(rename = "Hot thick Metallic vapour")]
    HotThickMetallicVapour,
    #[serde(rename = "Hot thick Methane")]
    HotThickMethane,
    #[serde(rename = "Hot thick Methane-rich")]
    HotThickMethaneRich,
    #[serde(rename = "Hot thick Nitrogen")]
    HotThickNitrogen,
    #[serde(rename = "Hot thick No atmosphere")]
    HotThickNoAtmosphere,
    #[serde(rename = "Hot thick Silicate vapour")]
    HotThickSilicateVapour,
    #[serde(rename = "Hot thick Sulphur dioxide")]
    HotThickSulphurDioxide,
    #[serde(rename = "Hot thick Water")]
    HotThickWater,
    #[serde(rename = "Hot thick Water-rich")]
    HotThickWaterRich,
    /* Hot thin */
    #[serde(rename = "Hot thin Carbon dioxide")]
    HotThinCarbonDioxide,
    #[serde(rename = "Hot thin Metallic vapour")]
    HotThinMetallicVapour,
    #[serde(rename = "Hot thin Silicate vapour")]
    HotThinSilicateVapour,
    #[serde(rename = "Hot thin Sulphur dioxide")]
    HotThinSulphurDioxide,
    /* Thick */
    #[serde(rename = "Thick Ammonia")]
    ThickAmmonia,
    #[serde(rename = "Thick Ammonia and Oxygen")]
    ThickAmmoniaAndOxygen,
    #[serde(rename = "Thick Ammonia-rich")]
    ThickAmmoniaRich,
    #[serde(rename = "Thick Argon")]
    ThickArgon,
    #[serde(rename = "Thick Argon-rich")]
    ThickArgonRich,
    #[serde(rename = "Thick Carbon dioxide")]
    ThickCarbonDioxide,
    #[serde(rename = "Thick Carbon dioxide-rich")]
    ThickCarbonDioxideRich,
    #[serde(rename = "Thick Helium")]
    ThickHelium,
    #[serde(rename = "Thick Methane")]
    ThickMethane,
    #[serde(rename = "Thick Methane-rich")]
    ThickMethaneRich,
    #[serde(rename = "Thick Nitrogen")]
    ThickNitrogen,
    #[serde(rename = "Thick No atmosphere")]
    ThickNoAtmosphere,
    #[serde(rename = "Thick Suitable for water-based life")]
    ThickSuitableForWaterBasedLife,
    #[serde(rename = "Thick Sulphur dioxide")]
    ThickSulphurDioxide,
    #[serde(rename = "Thick Water")]
    ThickWater,
    #[serde(rename = "Thick Water-rich")]
    ThickWaterRich,
    /* Thin */
    #[serde(rename = "Thin Ammonia")]
    ThinAmmonia,
    #[serde(rename = "Thin Ammonia and Oxygen")]
    ThinAmmoniaAndOxygen,
    #[serde(rename = "Thin Ammonia-rich")]
    ThinAmmoniaRich,
    #[serde(rename = "Thin Argon")]
    ThinArgon,
    #[serde(rename = "Thin Argon-rich")]
    ThinArgonRich,
    #[serde(rename = "Thin Carbon dioxide")]
    ThinCarbonDioxide,
    #[serde(rename = "Thin Carbon dioxide-rich")]
    ThinCarbonDioxideRich,
    #[serde(rename = "Thin Helium")]
    ThinHelium,
    #[serde(rename = "Thin Methane")]
    ThinMethane,
    #[serde(rename = "Thin Methane-rich")]
    ThinMethaneRich,
    #[serde(rename = "Thin Neon")]
    ThinNeon,
    #[serde(rename = "Thin Neon-rich")]
    ThinNeonRich,
    #[serde(rename = "Thin Nitrogen")]
    ThinNitrogen,
    #[serde(rename = "Thin No atmosphere")]
    ThinNoAtmosphere,
    #[serde(rename = "Thin Oxygen")]
    ThinOxygen,
    #[serde(rename = "Thin Sulphur dioxide")]
    ThinSulphurDioxide,
    #[serde(rename = "Thin Water")]
    ThinWater,
    #[serde(rename = "Thin Water-rich")]
    ThinWaterRich,
}

display_via_serde!(AtmosphereType);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Belt {
    pub inner_radius: f32,
    pub mass: f32,
    pub name: String,
    pub outer_radius: f32,
    #[serde(rename = "type")]
    pub typ: Option<AsteroidType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Discovery {
    pub commander: String,
    #[serde(with = "date_format")]
    pub date: DateTime<Utc>,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, EnumIter,
)]
#[serde(deny_unknown_fields)]
pub enum Luminosity {
    VII,
    VI,
    Vz,
    Vb,
    Vab,
    Va,
    V,
    IVb,
    IVab,
    IVa,
    IV,
    IIIb,
    IIIab,
    IIIa,
    III,
    IIb,
    IIab,
    IIa,
    II,
    Ib,
    Iab,
    Ia0,
    Ia,
    I,
    O,
}

display_via_serde!(Luminosity);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct Materials(BTreeMap<MaterialsKey, f32>);

impl Materials {
    pub fn get(&self, key: MaterialsKey) -> Option<f32> {
        self.0.get(&key).copied()
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, EnumIter,
)]
pub enum MaterialsKey {
    Antimony,
    Arsenic,
    Cadmium,
    Carbon,
    Chromium,
    Germanium,
    Iron,
    Manganese,
    Mercury,
    Molybdenum,
    Nickel,
    Niobium,
    Phosphorus,
    Polonium,
    Ruthenium,
    Selenium,
    Sulphur,
    Technetium,
    Tellurium,
    Tin,
    Tungsten,
    Vanadium,
    Yttrium,
    Zinc,
    Zirconium,
}

display_via_serde!(MaterialsKey);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Parent {
    Null(u64),
    Planet(u64),
    Star(u64),
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, EnumIter,
)]
#[serde(deny_unknown_fields)]
pub enum PlanetSubType {
    // gas ginat
    #[serde(rename = "Class I gas giant")]
    ClassIGasGiant,
    #[serde(rename = "Class II gas giant")]
    ClassIiGasGiant,
    #[serde(rename = "Class III gas giant")]
    ClassIiiGasGiant,
    #[serde(rename = "Class IV gas giant")]
    ClassIvGasGiant,
    #[serde(rename = "Class V gas giant")]
    ClassVGasGiant,
    #[serde(rename = "Gas giant with ammonia-based life")]
    GasGiantWithAmmoniaBasedLife,
    #[serde(rename = "Gas giant with water-based life")]
    GasGiantWithWaterBasedLife,
    #[serde(rename = "Helium gas giant")]
    HeliumGasGiant,
    #[serde(rename = "Helium-rich gas giant")]
    HeliumRichGasGiant,
    #[serde(rename = "Water giant")]
    WaterGiant,
    // terrestrial planet
    #[serde(rename = "Ammonia world")]
    AmmoniaWorld,
    #[serde(rename = "Earth-like world")]
    EarthLikeWorld,
    #[serde(rename = "High metal content world")]
    HighMetalContentWorld,
    #[serde(rename = "Icy body")]
    IcyBody,
    #[serde(rename = "Metal-rich body")]
    MetalRichBody,
    #[serde(rename = "Rocky Ice world")]
    RockyIceWorld,
    #[serde(rename = "Rocky body")]
    RockyBody,
    #[serde(rename = "Water world")]
    WaterWorld,
}

display_via_serde!(PlanetSubType);

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, EnumIter,
)]
#[serde(deny_unknown_fields)]
pub enum ReserveLevel {
    Depleted,
    Low,
    Common,
    Major,
    Pristine,
}

display_via_serde!(ReserveLevel);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Ring {
    pub inner_radius: f32,
    pub mass: f32,
    pub name: String,
    pub outer_radius: f32,
    #[serde(rename = "type")]
    pub typ: Option<AsteroidType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct SolidComposition {
    #[serde(default)]
    pub ice: f32,
    #[serde(default)]
    pub metal: f32,
    #[serde(default)]
    pub rock: f32,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, EnumIter,
)]
pub enum StarClass {
    OTypeStars,
    BTypeStars,
    ATypeStars,
    FTypeStars,
    GTypeStars,
    KTypeStars,
    MTypeStars,
    LTypeStars,
    TTypeStars,
    YTypeStars,
    ProtoStars,
    CarbonStars,
    WolfRayetStars,
    WhiteDwarfStars,
    NonSequenceStars,
}

impl fmt::Display for StarClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            StarClass::OTypeStars => "O-Type Stars",
            StarClass::BTypeStars => "B-Type Stars",
            StarClass::ATypeStars => "A-Type Stars",
            StarClass::FTypeStars => "F-Type Stars",
            StarClass::GTypeStars => "G-Type Stars",
            StarClass::KTypeStars => "K-Type Stars",
            StarClass::MTypeStars => "M-Type Stars",
            StarClass::LTypeStars => "L-Type Stars",
            StarClass::TTypeStars => "T-Type Stars",
            StarClass::YTypeStars => "Y-Type Stars",
            StarClass::ProtoStars => "Proto Stars",
            StarClass::CarbonStars => "Carbon Stars",
            StarClass::WolfRayetStars => "Wolf-Rayet Stars",
            StarClass::WhiteDwarfStars => "White Dwarf Stars",
            StarClass::NonSequenceStars => "Non Sequence Stars",
        };

        write!(f, "{}", s)
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, EnumIter,
)]
#[serde(deny_unknown_fields)]
pub enum SpectralClass {
    O,
    O0,
    O1,
    O2,
    O3,
    O4,
    O5,
    O6,
    O7,
    O8,
    O9,
    B,
    B0,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    B9,
    A,
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,
    A9,
    F,
    F0,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    G,
    G0,
    G1,
    G2,
    G3,
    G4,
    G5,
    G6,
    G7,
    G8,
    G9,
    K,
    K0,
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    K8,
    K9,
    M,
    M0,
    M1,
    M2,
    M3,
    M4,
    M5,
    M6,
    M7,
    M8,
    M9,
    L,
    L0,
    L1,
    L2,
    L3,
    L4,
    L5,
    L6,
    L7,
    L8,
    L9,
    T,
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    T7,
    T8,
    T9,
    Y0,
    Y1,
    Y2,
    Y3,
    Y4,
    Y5,
    Y6,
    Y7,
    Y8,
    AeBe0,
    AeBe1,
    AeBe2,
    AeBe3,
    AeBe4,
    AeBe5,
    AeBe6,
    AeBe7,
    AeBe8,
    AeBe9,
    #[serde(rename = "TTS0")]
    Tts0,
    #[serde(rename = "TTS1")]
    Tts1,
    #[serde(rename = "TTS2")]
    Tts2,
    #[serde(rename = "TTS3")]
    Tts3,
    #[serde(rename = "TTS4")]
    Tts4,
    #[serde(rename = "TTS5")]
    Tts5,
    #[serde(rename = "TTS6")]
    Tts6,
    #[serde(rename = "TTS7")]
    Tts7,
    #[serde(rename = "TTS8")]
    Tts8,
    #[serde(rename = "TTS9")]
    Tts9,
}

display_via_serde!(SpectralClass);

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, EnumIter,
)]
#[serde(deny_unknown_fields)]
pub enum StarSubType {
    // Main sequence
    #[serde(rename = "O (Blue-White) Star")]
    OBlueWhiteStar,
    #[serde(rename = "B (Blue-White) Star")]
    BBlueWhiteStar,
    #[serde(rename = "A (Blue-White) Star")]
    ABlueWhiteStar,
    #[serde(rename = "F (White) Star")]
    FWhiteStar,
    #[serde(rename = "G (White-Yellow) Star")]
    GWhiteYellowStar,
    #[serde(rename = "K (Yellow-Orange) Star")]
    KYellowOrangeStar,
    #[serde(rename = "M (Red dwarf) Star")]
    MRedDwarfStar,
    // Giants
    #[serde(rename = "K (Yellow-Orange giant) Star")]
    KYellowOrangeGiantStar,
    #[serde(rename = "M (Red giant) Star")]
    MRedGiantStar,
    // Supergiants
    #[serde(rename = "B (Blue-White super giant) Star")]
    BBlueWhiteSuperGiantStar,
    #[serde(rename = "A (Blue-White super giant) Star")]
    ABlueWhiteSuperGiantStar,
    #[serde(rename = "F (White super giant) Star")]
    FWhiteSuperGiantStar,
    #[serde(rename = "G (White-Yellow super giant) Star")]
    GWhiteYellowSuperGiantStar,
    #[serde(rename = "M (Red super giant) Star")]
    MRedSuperGiantStar,
    // Brown dwarf
    #[serde(rename = "L (Brown dwarf) Star")]
    LBrownDwarfStar,
    #[serde(rename = "T (Brown dwarf) Star")]
    TBrownDwarfStar,
    #[serde(rename = "Y (Brown dwarf) Star")]
    YBrownDwarfStar,
    // Proto star
    #[serde(rename = "Herbig Ae/Be Star")]
    HerbigAeBeStar,
    #[serde(rename = "T Tauri Star")]
    TTauriStar,
    // Carbon star
    #[serde(rename = "C Star")]
    CStar,
    #[serde(rename = "CJ Star")]
    CJStar,
    #[serde(rename = "CN Star")]
    CNStar,
    #[serde(rename = "MS-type Star")]
    MSTypeStar,
    #[serde(rename = "S-type Star")]
    STypeStar,
    // Wolf-Rayet star
    #[serde(rename = "Wolf-Rayet Star")]
    WolfRayetStar,
    #[serde(rename = "Wolf-Rayet C Star")]
    WolfRayetCStar,
    #[serde(rename = "Wolf-Rayet N Star")]
    WolfRayetNStar,
    #[serde(rename = "Wolf-Rayet NC Star")]
    WolfRayetNCStar,
    #[serde(rename = "Wolf-Rayet O Star")]
    WolfRayetOStar,
    // White dwarf
    #[serde(rename = "White Dwarf (D) Star")]
    WhiteDwarfDStar,
    #[serde(rename = "White Dwarf (DA) Star")]
    WhiteDwarfDAStar,
    #[serde(rename = "White Dwarf (DAB) Star")]
    WhiteDwarfDABStar,
    #[serde(rename = "White Dwarf (DAV) Star")]
    WhiteDwarfDAVStar,
    #[serde(rename = "White Dwarf (DAZ) Star")]
    WhiteDwarfDAZStar,
    #[serde(rename = "White Dwarf (DB) Star")]
    WhiteDwarfDBStar,
    #[serde(rename = "White Dwarf (DBV) Star")]
    WhiteDwarfDBVStar,
    #[serde(rename = "White Dwarf (DBZ) Star")]
    WhiteDwarfDBZStar,
    #[serde(rename = "White Dwarf (DC) Star")]
    WhiteDwarfDCStar,
    #[serde(rename = "White Dwarf (DCV) Star")]
    WhiteDwarfDCVStar,
    #[serde(rename = "White Dwarf (DQ) Star")]
    WhiteDwarfDQStar,
    // Non sequence
    #[serde(rename = "Neutron Star")]
    NeutronStar,
    #[serde(rename = "Black Hole")]
    BlackHole,
    #[serde(rename = "Supermassive Black Hole")]
    SupermassiveBlackHole,
}

display_via_serde!(StarSubType);

impl StarSubType {
    pub fn filter_star_class(self) -> StarClass {
        match self {
            StarSubType::OBlueWhiteStar => StarClass::OTypeStars,
            StarSubType::BBlueWhiteSuperGiantStar => StarClass::BTypeStars,
            StarSubType::BBlueWhiteStar => StarClass::BTypeStars,
            StarSubType::ABlueWhiteSuperGiantStar => StarClass::ATypeStars,
            StarSubType::ABlueWhiteStar => StarClass::ATypeStars,
            StarSubType::FWhiteSuperGiantStar => StarClass::FTypeStars,
            StarSubType::FWhiteStar => StarClass::FTypeStars,
            StarSubType::GWhiteYellowSuperGiantStar => StarClass::GTypeStars,
            StarSubType::GWhiteYellowStar => StarClass::GTypeStars,
            StarSubType::KYellowOrangeGiantStar => StarClass::KTypeStars,
            StarSubType::KYellowOrangeStar => StarClass::KTypeStars,
            StarSubType::MRedDwarfStar => StarClass::MTypeStars,
            StarSubType::MRedGiantStar => StarClass::MTypeStars,
            StarSubType::MRedSuperGiantStar => StarClass::MTypeStars,
            StarSubType::LBrownDwarfStar => StarClass::LTypeStars,
            StarSubType::TBrownDwarfStar => StarClass::TTypeStars,
            StarSubType::YBrownDwarfStar => StarClass::YTypeStars,
            StarSubType::HerbigAeBeStar => StarClass::ProtoStars,
            StarSubType::TTauriStar => StarClass::ProtoStars,
            StarSubType::CStar => StarClass::CarbonStars,
            StarSubType::CJStar => StarClass::CarbonStars,
            StarSubType::CNStar => StarClass::CarbonStars,
            StarSubType::MSTypeStar => StarClass::CarbonStars,
            StarSubType::STypeStar => StarClass::CarbonStars,
            StarSubType::WolfRayetStar => StarClass::WolfRayetStars,
            StarSubType::WolfRayetCStar => StarClass::WolfRayetStars,
            StarSubType::WolfRayetNStar => StarClass::WolfRayetStars,
            StarSubType::WolfRayetNCStar => StarClass::WolfRayetStars,
            StarSubType::WolfRayetOStar => StarClass::WolfRayetStars,
            StarSubType::WhiteDwarfDStar => StarClass::WhiteDwarfStars,
            StarSubType::WhiteDwarfDAStar => StarClass::WhiteDwarfStars,
            StarSubType::WhiteDwarfDABStar => StarClass::WhiteDwarfStars,
            StarSubType::WhiteDwarfDAVStar => StarClass::WhiteDwarfStars,
            StarSubType::WhiteDwarfDAZStar => StarClass::WhiteDwarfStars,
            StarSubType::WhiteDwarfDBStar => StarClass::WhiteDwarfStars,
            StarSubType::WhiteDwarfDBVStar => StarClass::WhiteDwarfStars,
            StarSubType::WhiteDwarfDBZStar => StarClass::WhiteDwarfStars,
            StarSubType::WhiteDwarfDCStar => StarClass::WhiteDwarfStars,
            StarSubType::WhiteDwarfDCVStar => StarClass::WhiteDwarfStars,
            StarSubType::WhiteDwarfDQStar => StarClass::WhiteDwarfStars,
            StarSubType::NeutronStar => StarClass::NonSequenceStars,
            StarSubType::BlackHole => StarClass::NonSequenceStars,
            StarSubType::SupermassiveBlackHole => StarClass::NonSequenceStars,
        }
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, EnumIter,
)]
#[serde(deny_unknown_fields)]
pub enum TerraformingState {
    #[serde(rename = "Candidate for terraforming")]
    CandidateForTerraforming,
    #[serde(rename = "Not terraformable")]
    NotTerraformable,
    Terraformed,
    Terraforming,
}

display_via_serde!(TerraformingState);

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, EnumIter,
)]
#[serde(deny_unknown_fields)]
pub enum VolcanismType {
    #[serde(rename = "Ammonia Magma")]
    AmmoniaMagma,
    #[serde(rename = "Carbon Dioxide Geysers")]
    CarbonDioxideGeysers,
    #[serde(rename = "Major Carbon Dioxide Geysers")]
    MajorCarbonDioxideGeysers,
    #[serde(rename = "Major Metallic Magma")]
    MajorMetallicMagma,
    #[serde(rename = "Major Rocky Magma")]
    MajorRockyMagma,
    #[serde(rename = "Major Silicate Vapour Geysers")]
    MajorSilicateVapourGeysers,
    #[serde(rename = "Major Water Geysers")]
    MajorWaterGeysers,
    #[serde(rename = "Major Water Magma")]
    MajorWaterMagma,
    #[serde(rename = "Metallic Magma")]
    MetallicMagma,
    #[serde(rename = "Methane Magma")]
    MethaneMagma,
    #[serde(rename = "Minor Ammonia Magma")]
    MinorAmmoniaMagma,
    #[serde(rename = "Minor Carbon Dioxide Geysers")]
    MinorCarbonDioxideGeysers,
    #[serde(rename = "Minor Metallic Magma")]
    MinorMetallicMagma,
    #[serde(rename = "Minor Methane Magma")]
    MinorMethaneMagma,
    #[serde(rename = "Minor Nitrogen Magma")]
    MinorNitrogenMagma,
    #[serde(rename = "Minor Rocky Magma")]
    MinorRockyMagma,
    #[serde(rename = "Minor Silicate Vapour Geysers")]
    MinorSilicateVapourGeysers,
    #[serde(rename = "Minor Water Geysers")]
    MinorWaterGeysers,
    #[serde(rename = "Minor Water Magma")]
    MinorWaterMagma,
    #[serde(rename = "Nitrogen Magma")]
    NitrogenMagma,
    #[serde(rename = "No volcanism")]
    NoVolcanism,
    #[serde(rename = "Rocky Magma")]
    RockyMagma,
    #[serde(rename = "Silicate Vapour Geysers")]
    SilicateVapourGeysers,
    #[serde(rename = "Water Geysers")]
    WaterGeysers,
    #[serde(rename = "Water Magma")]
    WaterMagma,
}

display_via_serde!(VolcanismType);
