use strum::IntoEnumIterator;

use crate::logs::content::log_event_content::scan_event::{
    ScanEvent, ScanEventKind, ScanEventPlanet, ScanEventStar,
};
use crate::models::exploration::species::Species;
use crate::models::galaxy::atmosphere::AtmosphereDensity;
use crate::models::galaxy::atmosphere_type::AtmosphereType;
use crate::models::galaxy::star_luminosity::StarLuminosity;
use crate::models::galaxy::volcanism::Volcanism;
use crate::models::galaxy::volcanism_type::VolcanismType;
use crate::modules::models::galaxy::atmosphere::Atmosphere;
use crate::modules::models::galaxy::planet_class::PlanetClass;
use crate::modules::models::galaxy::star_class::StarClass;

use super::spawn_condition::SpawnCondition;

#[derive(Debug)]
pub struct SpawnSource {
    body_name: String,
    pub star: Option<Star>,
    pub target_planet: Option<TargetPlanet>,
    pub bodies: Vec<Body>,
    pub nebula_distance: Option<f32>,
}

impl SpawnSource {
    pub fn new(body_name: impl Into<String>) -> SpawnSource {
        SpawnSource {
            body_name: body_name.into(),
            star: None,
            target_planet: None,
            bodies: Vec::new(),
            nebula_distance: None,
        }
    }

    pub fn supply_scan_event(&mut self, scan: &ScanEvent) {
        // TODO: Check if the scan event is actually relevant for this spawn source.
        //  Perhaps check the star system name, or the body name, etc.
        if !self.body_name.starts_with(&scan.star_system) {
            return;
        }

        match &scan.kind {
            ScanEventKind::Star(scan) => {
                self.supply_star_scan_event(&scan);
            }
            ScanEventKind::Planet(scan) => {
                self.supply_planet_scan_event(&scan);
            }
            _ => {} // Ignore belt clusters, etc.
        }
    }

    fn supply_star_scan_event(&mut self, scan: &ScanEventStar) {
        self.star = Some(Star {
            class: scan.star_type.clone(),
            luminosity: scan.luminosity.clone(),
        });
    }

    fn supply_planet_scan_event(&mut self, scan: &ScanEventPlanet) {
        self.target_planet = Some(TargetPlanet {
            atmosphere: scan.atmosphere.clone(),
            gravity: scan.surface_gravity,
            class: scan.planet_class.clone(),
            surface_temperature: scan.surface_temperature,
            volcanism: scan.volcanism.clone(),
            distance_from_star: 0.0, // FIXME: Find a way to get this value from the scan event.
            geological_signals_present: GeologicalSignalsPresent::Unknown, // FIXME: Find a way to get this value from the scan event.
        });
    }

    /// Returns a list of species that could spawn on this spawn source.
    pub fn get_spawnable_species(&self) -> Vec<Species> {
        Species::iter()
            .filter(|species| self.can_spawn_species(species))
            .collect()
    }

    /// Checks if the given species can spawn on this spawn source.
    pub fn can_spawn_species(&self, species: &Species) -> bool {
        species
            .spawn_conditions()
            .iter()
            .all(|condition| self.satisfies_spawn_condition(condition))
    }

    /// Checks if the spawn source satisfies the given condition.
    pub fn satisfies_spawn_condition(&self, condition: &SpawnCondition) -> bool {
        match condition {
            SpawnCondition::MinMeanTemperature(min_temp) => {
                if let Some(target_planet) = &self.target_planet {
                    target_planet.surface_temperature >= *min_temp
                } else {
                    false
                }
            }
            SpawnCondition::MaxMeanTemperature(max_temp) => {
                if let Some(target_planet) = &self.target_planet {
                    target_planet.surface_temperature <= *max_temp
                } else {
                    false
                }
            }
            SpawnCondition::NoAtmosphere => {
                if let Some(target_planet) = &self.target_planet {
                    target_planet.atmosphere.kind == AtmosphereType::None
                } else {
                    false
                }
            }
            SpawnCondition::AnyThinAtmosphere => {
                if let Some(target_planet) = &self.target_planet {
                    target_planet.atmosphere.density == AtmosphereDensity::Thin
                } else {
                    false
                }
            }
            SpawnCondition::ThinAtmosphere(atmosphere_type) => {
                if let Some(target_planet) = &self.target_planet {
                    target_planet.atmosphere.density == AtmosphereDensity::Thin
                        && target_planet.atmosphere.kind == *atmosphere_type
                } else {
                    false
                }
            }
            SpawnCondition::MinGravity(min_gravity) => {
                if let Some(target_planet) = &self.target_planet {
                    target_planet.gravity >= *min_gravity
                } else {
                    false
                }
            }
            SpawnCondition::MaxGravity(max_gravity) => {
                if let Some(target_planet) = &self.target_planet {
                    target_planet.gravity <= *max_gravity
                } else {
                    false
                }
            }
            SpawnCondition::PlanetClass(planet_class) => {
                if let Some(target_planet) = &self.target_planet {
                    target_planet.class == *planet_class
                } else {
                    false
                }
            }
            SpawnCondition::ParentStarClass(star_class) => {
                if let Some(star) = &self.star {
                    star.class == *star_class
                } else {
                    false
                }
            }
            SpawnCondition::ParentStarLuminosity(star_luminosity) => {
                if let Some(star) = &self.star {
                    star.luminosity == *star_luminosity
                } else {
                    false
                }
            }
            SpawnCondition::MinOrEqualParentStarLuminosity(star_luminosity) => {
                if let Some(star) = &self.star {
                    // FIXME: This requires the luminosity enum to be ordered from weak to strong luminosity.
                    //  This could be a potential bug if that enum is ever refactored.
                    star.luminosity >= *star_luminosity
                } else {
                    false
                }
            }
            SpawnCondition::SystemContainsPlanetClass(planet_class) => {
                self.bodies.iter().any(|b: &Body| b.class == *planet_class)
            }
            SpawnCondition::VolcanismType(volcanism_type) => {
                if let Some(target_planet) = &self.target_planet {
                    target_planet.volcanism.kind == *volcanism_type
                } else {
                    false
                }
            }
            SpawnCondition::MinDistanceFromParentSun(min_distance) => {
                if let Some(target_planet) = &self.target_planet {
                    target_planet.distance_from_star >= *min_distance
                } else {
                    false
                }
            }
            SpawnCondition::AnyVolcanism => {
                if let Some(target_planet) = &self.target_planet {
                    target_planet.volcanism.kind != VolcanismType::None
                } else {
                    false
                }
            }
            SpawnCondition::WithinNebulaRange(nebula_range) => {
                if let Some(nebula_distance) = self.nebula_distance {
                    nebula_distance <= *nebula_range
                } else {
                    false
                }
            }
            SpawnCondition::GeologicalSignalsPresent => {
                if let Some(target_planet) = &self.target_planet {
                    // TODO: Quickly draw the NotPresent conclusion if geological signals could not even
                    //  possibly exist on the target planet (atmosphere?), instead of waiting for the
                    //  result of planetary mapping to confirm signals are not present.

                    // NOTE: Do we want to consider the Unknown state (not yet mapped) as a valid datapoint?
                    //  If so, we should not return a simple boolean, but an enum with a 'Maybe' value to
                    //  indicate uncertainty about this spawn condition.
                    target_planet.geological_signals_present == GeologicalSignalsPresent::Present
                } else {
                    false
                }
            }
            SpawnCondition::Any(conditions) => conditions
                .iter()
                .any(|condition| self.satisfies_spawn_condition(condition)),
            SpawnCondition::All(conditions) => conditions
                .iter()
                .all(|condition| self.satisfies_spawn_condition(condition)),
        }
    }
}

#[derive(Debug)]
pub struct TargetPlanet {
    pub atmosphere: Atmosphere,
    pub gravity: f32,
    pub class: PlanetClass,
    pub surface_temperature: f32,
    pub volcanism: Volcanism,
    /// The distance from the parent star in AU
    pub distance_from_star: f32,
    pub geological_signals_present: GeologicalSignalsPresent,
}

#[derive(Debug, PartialEq)]
pub enum GeologicalSignalsPresent {
    /// Mapping of the planet has concluded that geological signals are present.
    Present,
    /// The planet has not been mapped yet, so it is unknown if geological signals are present.
    Unknown,
    /// Mapping of the planet has concluded that geological signals are not present.
    NotPresent,
}

#[derive(Debug)]
pub struct Star {
    pub class: StarClass,
    pub luminosity: StarLuminosity,
}

#[derive(Debug, PartialEq)]
pub struct Body {
    pub class: PlanetClass,
}

mod tests {
    use crate::logs::content::LogEventContent;
    use crate::logs::LogDir;
    use crate::models::exploration::genus::Genus;
    use crate::models::exploration::species::Species;
    use crate::modules::exobiology::models::spawn_condition;
    use crate::modules::exobiology::models::spawn_source::SpawnSource;
    use std::collections::{HashMap, HashSet};
    use std::env::current_dir;

    use super::Body;
    // use crate::blocking::JournalDir;
    //
    // use crate::modules::logs::content::log_event_content::JournalEventContent;

    #[test]
    fn spawnable_species_are_generated_correctly() {
        let dir_path = current_dir().unwrap().join("test-files").join("journals");
        let log_dir = LogDir::new(dir_path);
        let logs = log_dir.journal_logs().unwrap();

        // Species found in the logs, grouped by body name.
        // These are the value we will compare against the calculated spawnable species.
        let mut expected_species = HashMap::<String, HashSet<Species>>::new();
        for journal in &logs {
            let reader = journal.create_blocking_reader().unwrap();

            let mut current_body_name = String::new();

            for entry in reader {
                if let LogEventContent::Touchdown(touchdown) = &entry.as_ref().unwrap().content {
                    current_body_name = touchdown.body.clone();
                }

                if let LogEventContent::ScanOrganic(organic) = &entry.as_ref().unwrap().content {
                    expected_species
                        .entry(current_body_name.clone())
                        .or_insert_with(HashSet::new)
                        .insert(organic.species.clone());
                }
            }
        }

        // Create a SpawnSource for each body in our test data that has an expected list of species.
        let mut spawn_sources = HashMap::<String, SpawnSource>::new();
        for (body, _) in &expected_species {
            spawn_sources.insert(body.clone(), SpawnSource::new(body.clone()));
        }

        // Supply the journal events to the spawn sources.
        for journal in &logs {
            let reader = journal.create_blocking_reader().unwrap();

            for entry in reader {
                if let LogEventContent::Scan(scan) = entry.unwrap().content {
                    let body_name = scan.body_name.clone();

                    // Skip scan events that are not relevant to our test data
                    if !spawn_sources.contains_key(&body_name) {
                        continue;
                    }

                    let spawn_source = spawn_sources.get_mut(&body_name).unwrap();

                    spawn_source.supply_scan_event(&scan);
                }
            }
        }

        // Check each spawn source to see if the calculated spawnable species match the expected species.
        for (body_name, expected_species) in expected_species {
            let spawn_source = spawn_sources.get(&body_name).unwrap();
            let spawnable_species = spawn_source.get_spawnable_species();

            let missing_matches = expected_species
                .iter()
                .filter(|species| !spawnable_species.contains(species))
                .collect::<Vec<&Species>>();

            // If it's not a match, figure out which conditions failed for debugging purposes.
            for species in &missing_matches {
                let conditions = species.spawn_conditions();

                let failing_conditions = conditions
                    .iter()
                    .filter(|condition| !spawn_source.satisfies_spawn_condition(condition))
                    .collect::<Vec<_>>();

                println!(
                    //&failing_conditions.is_empty(),
                    //"The following conditions failed for '{:?}' on body '{}': {:?}\n{:#?}",
                    "The following conditions failed for '{:?}' on body '{}': {:?}",
                    species,
                    body_name,
                    failing_conditions //, spawn_source
                );
            }
        }
    }
}
