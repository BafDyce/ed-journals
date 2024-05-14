use crate::shared::galaxy::atmosphere_type::AtmosphereType;
use crate::shared::galaxy::planet_class::PlanetClass;
use crate::shared::galaxy::star_class::StarClass;
use crate::shared::galaxy::star_luminosity::StarLuminosity;
use crate::shared::galaxy::volcanism_type::VolcanismType;

pub enum SpawnCondition {
    MinMeanTemperature(f32),
    MaxMeanTemperature(f32),
    NoAtmosphere,
    AnyThinAtmosphere,
    ThinAtmosphere(AtmosphereType),
    MinGravity(f32),
    MaxGravity(f32),
    PlanetClass(PlanetClass),
    ParentStarClass(StarClass),
    ParentStarLuminosity(StarLuminosity),
    MinOrEqualParentStarLuminosity(StarLuminosity),
    SystemContainsPlanetClass(PlanetClass),
    VolcanismType(VolcanismType),
    /// The minimum distance the planet needs to be from the sun in AU
    MinDistanceFromParentSun(f32),
    AnyVolcanism,

    WithinNebulaRange(f32),

    /// The target planet must have at least one geological signal present.
    GeologicalSignalsPresent,

    Any(Vec<SpawnCondition>),
    All(Vec<SpawnCondition>),

    /// If the first spawn condition is true, then the second condition must also return `true`, but
    /// if the first spawn condition is false, then it returns `true` and the second condition isn't
    /// tested.
    Conditional(SpawnCondition, SpawnCondition),
}
