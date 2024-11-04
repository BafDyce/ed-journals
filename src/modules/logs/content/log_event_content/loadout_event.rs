use serde::{Deserialize, Serialize};

use crate::modules::ship::{ShipModule, ShipSlot, ShipType};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LoadoutEvent {
    pub ship: ShipType,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,
    pub ship_name: String,
    pub ship_ident: String,

    pub hull_value: Option<u64>,
    pub modules_value: Option<u64>,
    pub hull_health: f32,
    pub unladen_mass: f32,
    pub cargo_capacity: u64,
    pub max_jump_range: f32,
    pub fuel_capacity: LoadoutFuelCapacity,
    pub rebuy: u64,
    pub hot: Option<bool>,

    pub modules: Vec<LoadoutModule>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LoadoutFuelCapacity {
    pub main: f32,
    pub reserve: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LoadoutModule {
    pub slot: ShipSlot,
    pub item: ShipModule,
    pub on: bool,
    pub priority: u8,
    pub health: f32,

    // TODO check when this value is used
    pub value: Option<u32>,
    pub ammo_in_clip: Option<u32>,
}
