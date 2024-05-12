use crate::modules::shared::exploration::genus::Genus;
use crate::modules::shared::exploration::planetary_signal_type::PlanetarySignalType;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SAASignalsFoundEvent {
    pub body_name: String,
    pub system_address: u64,

    #[serde(rename = "BodyID")]
    pub body_id: u8,
    pub signals: Vec<SAASignalsFoundEventSignal>,
    pub genuses: Vec<SAASignalsFoundEventGenus>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SAASignalsFoundEventSignal {
    #[serde(rename = "Type")]
    pub kind: PlanetarySignalType,

    #[serde(rename = "Type_Localised")]
    pub type_localized: Option<String>,
    pub count: u8,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SAASignalsFoundEventGenus {
    pub genus: Genus,

    #[serde(rename = "Genus_Localised")]
    pub genus_localized: Option<String>,
}
