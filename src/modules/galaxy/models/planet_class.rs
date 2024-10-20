use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum PlanetClass {
    #[serde(rename = "Metal rich body")]
    MetalRichBody,

    #[serde(rename = "High metal content body")]
    HighMetalContentBody,

    #[serde(rename = "Rocky body")]
    RockyBody,

    #[serde(rename = "Icy body")]
    IcyBody,

    #[serde(rename = "Rocky ice body")]
    RockyIceBody,

    #[serde(rename = "Earthlike body")]
    EarthlikeBody,

    #[serde(rename = "Water world")]
    WaterWorld,

    #[serde(rename = "Ammonia world")]
    AmmoniaWorld,

    #[serde(rename = "Water giant")]
    WaterGiant,

    #[serde(rename = "Water giant with life")]
    WaterGiantWithLife,

    #[serde(rename = "Gas giant with water based life")]
    GasGiantWithWaterBasedLife,

    #[serde(rename = "Gas giant with ammonia based life")]
    GasGiantWithAmmoniaBasedLife,

    #[serde(rename = "Sudarsky class I gas giant")]
    SudarskyClassIGasGiant,

    #[serde(rename = "Sudarsky class II gas giant")]
    SudarskyClassIiGasGiant,

    #[serde(rename = "Sudarsky class III gas giant")]
    SudarskyClassIiiGasGiant,

    #[serde(rename = "Sudarsky class IV gas giant")]
    SudarskyClassIvGasGiant,

    #[serde(rename = "Sudarsky class V gas giant")]
    SudarskyClassVGasGiant,

    #[serde(rename = "Helium rich gas giant")]
    HeliumRichGasGiant,

    #[serde(rename = "Helium gas giant")]
    HeliumGasGiant,
}

impl PlanetClass {
    /// Returns the base exploration value of the star planet class.
    pub fn base_value(&self) -> u64 {
        match self {
            PlanetClass::MetalRichBody => 21_790,
            PlanetClass::AmmoniaWorld => 96_932,
            PlanetClass::SudarskyClassIGasGiant => 1_656,
            PlanetClass::SudarskyClassIiGasGiant => 9_654,
            PlanetClass::HighMetalContentBody => 9_654,
            PlanetClass::WaterWorld => 64_831,
            PlanetClass::EarthlikeBody => 64_831,
            _ => 300,
        }
    }

    /// Returns the bonus exploration value if the planet is terraformable.
    pub fn terraformable_bonus(&self) -> u64 {
        match self {
            PlanetClass::MetalRichBody => 65_631,
            PlanetClass::AmmoniaWorld => 0,
            PlanetClass::SudarskyClassIGasGiant => 0,
            PlanetClass::SudarskyClassIiGasGiant => 100_677,
            PlanetClass::HighMetalContentBody => 100_677,
            PlanetClass::WaterWorld => 116_295,
            PlanetClass::EarthlikeBody => 116_295,
            _ => 93_328,
        }
    }
}

impl Display for PlanetClass {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let string = match self {
            Self::MetalRichBody => "Metal rich body",
            Self::HighMetalContentBody => "High metal content body",
            Self::RockyBody => "Rocky body",
            Self::IcyBody => "Icy body",
            Self::RockyIceBody => "Rocky ice body",
            Self::EarthlikeBody => "Earthlike body",
            Self::WaterWorld => "Water world",
            Self::AmmoniaWorld => "Ammonia world",
            Self::WaterGiant => "Water giant",
            Self::WaterGiantWithLife => "Water giant with life",
            Self::GasGiantWithWaterBasedLife => "Gas giant with water based life",
            Self::GasGiantWithAmmoniaBasedLife => "Gas giant with ammonia based life",
            Self::SudarskyClassIGasGiant => "Sudarsky class I gas giant",
            Self::SudarskyClassIiGasGiant => "Sudarsky class II gas giant",
            Self::SudarskyClassIiiGasGiant => "Sudarsky class III gas giant",
            Self::SudarskyClassIvGasGiant => "Sudarsky class IV gas giant",
            Self::SudarskyClassVGasGiant => "Sudarsky class V gas giant",
            Self::HeliumRichGasGiant => "Helium rich gas giant",
            Self::HeliumGasGiant => "Helium gas giant",
        };
        write!(f, "{}", string)
    }
}
