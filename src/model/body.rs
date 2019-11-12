use std::borrow::Cow;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::dec::date_format;
use super::RootEntry;

// Main Type

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(tag = "type")]
pub enum Body {
    Planet(Planet),
    Star(Star),
    #[serde(rename = "null")]
    Unknown(Unknown),
}

impl RootEntry for Body {
    fn pre_filter(s: &str) -> Cow<'_, str> {
        let null_pos = s.find(r#""type":null"#);
        let first_compound = match (s.find(":{"), s.find("[")) {
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Planet {
    pub id: u64,
    // Attributes
    pub arg_of_periapsis: Option<f32>,
    pub atmosphere_composition: Option<AtmosphereComposition>,
    pub atmosphere_type: Option<String>,
    pub axial_tilt: Option<f32>,
    pub belts: Option<Vec<Belt>>,
    pub body_id: Option<u64>,
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
    pub spectral_class: Option<String>,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum AsteroidType {
    Icy,
    #[serde(rename = "Metal Rich")]
    MetalRich,
    Metallic,
    Rocky,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AtmosphereComposition {
    pub ammonia: Option<f32>,
    pub argon: Option<f32>,
    #[serde(rename = "Carbon dioxide")]
    pub carbon_dioxide: Option<f32>,
    pub helium: Option<f32>,
    pub hydrogen: Option<f32>,
    pub iron: Option<f32>,
    pub methane: Option<f32>,
    pub neon: Option<f32>,
    pub nitrogen: Option<f32>,
    pub oxygen: Option<f32>,
    pub silicates: Option<f32>,
    #[serde(rename = "Sulphur dioxide")]
    pub sulphur_dioxide: Option<f32>,
    pub water: Option<f32>,
}

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct Materials {
    pub antimony: Option<f32>,
    pub arsenic: Option<f32>,
    pub cadmium: Option<f32>,
    pub carbon: Option<f32>,
    pub chromium: Option<f32>,
    pub germanium: Option<f32>,
    pub iron: Option<f32>,
    pub manganese: Option<f32>,
    pub mercury: Option<f32>,
    pub molybdenum: Option<f32>,
    pub nickel: Option<f32>,
    pub niobium: Option<f32>,
    pub phosphorus: Option<f32>,
    pub polonium: Option<f32>,
    pub ruthenium: Option<f32>,
    pub selenium: Option<f32>,
    pub sulphur: Option<f32>,
    pub technetium: Option<f32>,
    pub tellurium: Option<f32>,
    pub tin: Option<f32>,
    pub tungsten: Option<f32>,
    pub vanadium: Option<f32>,
    pub yttrium: Option<f32>,
    pub zinc: Option<f32>,
    pub zirconium: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum Parent {
    Null(u64),
    Planet(u64),
    Star(u64),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum ReserveLevel {
    Depleted,
    Low,
    Common,
    Major,
    Pristine,
}

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum StarSubType {
    // Main sequence & giants (scoopable)
    #[serde(rename = "O (Blue-White) Star")]
    OBlueWhiteStar,
    #[serde(rename = "B (Blue-White super giant) Star")]
    BBlueWhiteSuperGiantStar,
    #[serde(rename = "B (Blue-White) Star")]
    BBlueWhiteStar,
    #[serde(rename = "A (Blue-White super giant) Star")]
    ABlueWhiteSuperGiantStar,
    #[serde(rename = "A (Blue-White) Star")]
    ABlueWhiteStar,
    #[serde(rename = "F (White super giant) Star")]
    FWhiteSuperGiantStar,
    #[serde(rename = "F (White) Star")]
    FWhiteStar,
    #[serde(rename = "G (White-Yellow super giant) Star")]
    GWhiteYellowSuperGiantStar,
    #[serde(rename = "G (White-Yellow) Star")]
    GWhiteYellowStar,
    #[serde(rename = "K (Yellow-Orange giant) Star")]
    KYellowOrangeGiantStar,
    #[serde(rename = "K (Yellow-Orange) Star")]
    KYellowOrangeStar,
    #[serde(rename = "M (Red dwarf) Star")]
    MRedDwarfStar,
    #[serde(rename = "M (Red giant) Star")]
    MRedGiantStar,
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
    CjStar,
    #[serde(rename = "CN Star")]
    CnStar,
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
    WolfRayetNcStar,
    #[serde(rename = "Wolf-Rayet O Star")]
    WolfRayetOStar,
    // White dwarf
    #[serde(rename = "White Dwarf (D) Star")]
    WhiteDwarfDStar,
    #[serde(rename = "White Dwarf (DA) Star")]
    WhiteDwarfDaStar,
    #[serde(rename = "White Dwarf (DAB) Star")]
    WhiteDwarfDabStar,
    #[serde(rename = "White Dwarf (DAV) Star")]
    WhiteDwarfDavStar,
    #[serde(rename = "White Dwarf (DAZ) Star")]
    WhiteDwarfDazStar,
    #[serde(rename = "White Dwarf (DB) Star")]
    WhiteDwarfDbStar,
    #[serde(rename = "White Dwarf (DBV) Star")]
    WhiteDwarfDbvStar,
    #[serde(rename = "White Dwarf (DBZ) Star")]
    WhiteDwarfDbzStar,
    #[serde(rename = "White Dwarf (DC) Star")]
    WhiteDwarfDcStar,
    #[serde(rename = "White Dwarf (DCV) Star")]
    WhiteDwarfDcvStar,
    #[serde(rename = "White Dwarf (DQ) Star")]
    WhiteDwarfDqStar,
    // Non sequence
    #[serde(rename = "Neutron Star")]
    NeutronStar,
    #[serde(rename = "Black Hole")]
    BlackHole,
    #[serde(rename = "Supermassive Black Hole")]
    SupermassiveBlackHole,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum TerraformingState {
    #[serde(rename = "Candidate for terraforming")]
    CandidateForTerraforming,
    #[serde(rename = "Not terraformable")]
    NotTerraformable,
    Terraformed,
    Terraforming,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
