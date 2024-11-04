use serde::Serialize;

use crate::logs::loadout_event::{LoadoutEvent, LoadoutModule};

use super::ship_type::ShipType;

#[derive(Clone, Debug, Serialize)]
pub struct ShipInfo {
    pub ship_type: ShipType,
    pub id: u64,
    pub name: String,
    pub ident: String,
    pub value: u64,
    pub rebuy_value: u64,
    pub modules: Vec<LoadoutModule>,
    pub fuel_capacity: Option<f32>,
    pub fuel_level: Option<f32>,
    pub location: Option<ShipLocation>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ShipLocation {
    pub system: String,
    pub market_id: u64,
}

impl From<LoadoutEvent> for ShipInfo {
    fn from(loadout: LoadoutEvent) -> Self {
        Self {
            ship_type: loadout.ship,
            id: loadout.ship_id,
            name: loadout.ship_name,
            ident: loadout.ship_ident,
            value: loadout.hull_value.unwrap_or(0) + loadout.modules_value.unwrap_or(0),
            rebuy_value: loadout.rebuy,
            modules: loadout.modules,
            fuel_capacity: Some(loadout.fuel_capacity.main),
            fuel_level: None,
            location: None,
        }
    }
}

impl ShipInfo {
    pub fn update_location(&mut self, system: String, market_id: u64) {
        let _ = self.location.insert(ShipLocation { system, market_id });
    }
}
