use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::Serialize;
use thiserror::Error;

use crate::from_str_deserialize_impl;
use crate::modules::materials::MaterialCategory;

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum Material {
    // Raw
    Antimony,
    Arsenic,
    Boron,
    Cadmium,
    Carbon,
    Chromium,
    Germanium,
    Iron,
    Lead,
    Manganese,
    Mercury,
    Molybdenum,
    Nickel,
    Niobium,
    Phosphorus,
    Polonium,
    Rhenium,
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

    // Manufactured
    BasicConductors,
    BioMechanicalConduits,
    BiotechConductors,
    CausticCrystal,
    CausticShard,
    ChemicalDistillery,
    ChemicalManipulators,
    ChemicalProcessors,
    ChemicalStorageUnits,
    CompactComposites,
    CompoundShielding,
    ConductiveCeramics,
    ConductiveComponents,
    ConductivePolymers,
    ConfigurableComponents,
    CoreDynamicsComposites,
    CorrosiveMechanisms,
    CrystalShards,
    ElectrochemicalArrays,
    ExquisiteFocusCrystals,
    FilamentComposites,
    FlawedFocusCrystals,
    FocusCrystals,
    GalvanisingAlloys,
    GridResistors,
    GuardianPowerCell,
    GuardianPowerConduit,
    GuardianSentinelWeaponParts,
    GuardianTechnologyComponent,
    GuardianWreckageComponents,
    HardenedSurfaceFragments,
    HeatConductionWiring,
    HeatDispersionPlate,
    HeatExchangers,
    HeatExposureSpecimen,
    HeatResistantCeramics,
    HeatVanes,
    HighDensityComposites,
    HybridCapacitors,
    ImperialShielding,
    ImprovisedComponents,
    MechanicalComponents,
    MechanicalEquipment,
    MechanicalScrap,
    MilitaryGradeAlloys,
    MilitarySupercapacitors,
    PharmaceuticalIsolators,
    PhaseAlloys,
    PhasingMembraneResidue,
    PolymerCapacitors,
    PrecipitatedAlloys,
    ProprietaryComposites,
    PropulsionElements,
    ProtoHeatRadiators,
    ProtoLightAlloys,
    ProtoRadiolicAlloys,
    RefinedFocusCrystals,
    SalvagedAlloys,
    SensorFragment,
    ShieldEmitters,
    ShieldingSensors,
    TacticalCoreChip,
    TemperedAlloys,
    ThargoidCarapace,
    ThargoidEnergyCell,
    ThargoidOrganicCircuitry,
    ThargoidTechnologicalComponents,
    ThermicAlloys,
    WeaponParts,
    WornShieldEmitters,
    WreckageComponents,

    // Encoded
    AberrantShieldPatternAnalysis,
    AbnormalCompactEmissionData,
    AdaptiveEncryptorsCapture,
    AnomalousBulkScanData,
    AnomalousFSDTelemetry,
    AtypicalDisruptedWakeEchoes,
    AtypicalEncryptionArchives,
    ClassifiedScanDatabanks,
    ClassifiedScanFragment,
    CrackedIndustrialFirmware,
    DataminedWakeExceptions,
    DecodedEmissionData,
    DistortedShieldCycleRecordings,
    DivergentScanData,
    EccentricHyperspaceTrajectories,
    ExceptionScrambledEmissionData,
    GuardianModuleBlueprintFragment,
    GuardianVesselBlueprintFragment,
    GuardianWeaponBlueprintFragment,
    InconsistentShieldSoakAnalysis,
    IrregularEmissionData,
    MassiveEnergySurgeAnalytics,
    ModifiedConsumerFirmware,
    ModifiedEmbeddedFirmware,
    OpenSymmetricKeys,
    PatternAlphaObeliskData,
    PatternBetaObeliskData,
    PatternDeltaObeliskData,
    PatternEpsilonObeliskData,
    PatternGammaObeliskData,
    PeculiarShieldFrequencyData,
    SecurityFirmwarePatch,
    ShipFlightData,
    ShipSystemsData,
    SpecializedLegacyFirmware,
    StrangeWakeSolutions,
    TaggedEncryptionCodes,
    ThargoidInterdictionTelemetry,
    ThargoidMaterialCompositionData,
    ThargoidResidueData,
    ThargoidShipSignature,
    ThargoidStructuralData,
    ThargoidWakeData,
    UnexpectedEmissionData,
    UnidentifiedScanArchives,
    UntypicalShieldScans,
    UnusualEncryptedFiles,

    #[cfg(not(feature = "strict"))]
    Unknown(String),
}

impl Material {
    pub fn is_raw(&self) -> bool {
        matches!(self.into(), MaterialCategory::Raw)
    }

    pub fn is_manufactured(&self) -> bool {
        matches!(self.into(), MaterialCategory::Manufactured)
    }

    pub fn is_encoded(&self) -> bool {
        matches!(self.into(), MaterialCategory::Encoded)
    }

    pub fn id(&self) -> u64 {
        match self {
            Material::Iron => 128672318,
            Material::Nickel => 128672319,
            Material::Tin => 128672320,
            Material::Zinc => 128672321,
            Material::Carbon => 128672322,
            Material::Sulphur => 128672323,
            Material::Phosphorus => 128672324,
            Material::Manganese => 128672325,
            Material::Selenium => 128672326,
            Material::Chromium => 128672327,
            Material::Vanadium => 128672328,
            Material::Germanium => 128672329,
            Material::Cadmium => 128672330,
            Material::Tungsten => 128672331,
            Material::Arsenic => 128672332,
            Material::Molybdenum => 128672333,
            Material::Niobium => 128672334,
            Material::Zirconium => 128672335,
            Material::Mercury => 128672336,
            Material::Yttrium => 128672337,
            Material::Tellurium => 128672338,
            Material::Polonium => 128672339,
            Material::Technetium => 128672340,
            Material::Ruthenium => 128672341,
            Material::Antimony => 128672342,
            Material::GridResistors => 128673877,
            Material::CrystalShards => 128673878,
            Material::TemperedAlloys => 128673879,
            Material::BasicConductors => 128673880,
            Material::MechanicalScrap => 128673881,
            Material::HeatConductionWiring => 128673882,
            Material::WornShieldEmitters => 128673883,
            Material::CompactComposites => 128673884,
            Material::SalvagedAlloys => 128673885,
            Material::ChemicalStorageUnits => 128673886,
            Material::HybridCapacitors => 128673887,
            Material::FlawedFocusCrystals => 128673888,
            Material::HeatResistantCeramics => 128673889,
            Material::ConductiveComponents => 128673890,
            Material::MechanicalEquipment => 128673891,
            Material::HeatDispersionPlate => 128673892,
            Material::ShieldEmitters => 128673893,
            Material::FilamentComposites => 128673894,
            Material::GalvanisingAlloys => 128673895,
            Material::ChemicalProcessors => 128673896,
            Material::ElectrochemicalArrays => 128673897,
            Material::FocusCrystals => 128673898,
            Material::PrecipitatedAlloys => 128673899,
            Material::ConductiveCeramics => 128673900,
            Material::MechanicalComponents => 128673901,
            Material::HeatExchangers => 128673902,
            Material::ShieldingSensors => 128673903,
            Material::HighDensityComposites => 128673904,
            Material::PhaseAlloys => 128673905,
            Material::ChemicalDistillery => 128673906,
            Material::PolymerCapacitors => 128673907,
            Material::RefinedFocusCrystals => 128673908,
            Material::ThermicAlloys => 128673909,
            Material::ConductivePolymers => 128673910,
            Material::ConfigurableComponents => 128673911,
            Material::HeatVanes => 128673912,
            Material::CompoundShielding => 128673913,
            Material::ProprietaryComposites => 128673914,
            Material::ProtoLightAlloys => 128673915,
            Material::ChemicalManipulators => 128673916,
            Material::MilitarySupercapacitors => 128673917,
            Material::ExquisiteFocusCrystals => 128673918,
            Material::MilitaryGradeAlloys => 128673919,
            Material::BiotechConductors => 128673920,
            Material::ImprovisedComponents => 128673921,
            Material::ProtoHeatRadiators => 128673922,
            Material::ImperialShielding => 128673923,
            Material::CoreDynamicsComposites => 128673924,
            Material::ProtoRadiolicAlloys => 128673925,
            Material::PharmaceuticalIsolators => 128673926,
            Material::SpecializedLegacyFirmware => 128681610,
            Material::UnusualEncryptedFiles => 128681611,
            Material::AnomalousBulkScanData => 128681612,
            Material::AtypicalDisruptedWakeEchoes => 128681613,
            Material::ExceptionScrambledEmissionData => 128681614,
            Material::DistortedShieldCycleRecordings => 128681615,
            Material::ModifiedConsumerFirmware => 128681616,
            Material::TaggedEncryptionCodes => 128681617,
            Material::UnidentifiedScanArchives => 128681618,
            Material::AnomalousFSDTelemetry => 128681619,
            Material::IrregularEmissionData => 128681620,
            Material::InconsistentShieldSoakAnalysis => 128681621,
            Material::CrackedIndustrialFirmware => 128681622,
            Material::OpenSymmetricKeys => 128681623,
            Material::ClassifiedScanDatabanks => 128681624,
            Material::StrangeWakeSolutions => 128681625,
            Material::UnexpectedEmissionData => 128681626,
            Material::UntypicalShieldScans => 128681627,
            Material::SecurityFirmwarePatch => 128681628,
            Material::AtypicalEncryptionArchives => 128681629,
            Material::DivergentScanData => 128681630,
            Material::EccentricHyperspaceTrajectories => 128681631,
            Material::DecodedEmissionData => 128681632,
            Material::AberrantShieldPatternAnalysis => 128681633,
            Material::ModifiedEmbeddedFirmware => 128681634,
            Material::AdaptiveEncryptorsCapture => 128681635,
            Material::ClassifiedScanFragment => 128681636,
            Material::DataminedWakeExceptions => 128681637,
            Material::AbnormalCompactEmissionData => 128681638,
            Material::PeculiarShieldFrequencyData => 128681639,
            Material::SensorFragment => 128681640,
            Material::ThargoidShipSignature => 128731669,
            Material::ThargoidWakeData => 128731670,
            Material::PatternDeltaObeliskData => 128732197,
            Material::PatternAlphaObeliskData => 128732198,
            Material::PatternBetaObeliskData => 128732199,
            Material::PatternGammaObeliskData => 128732200,
            Material::PatternEpsilonObeliskData => 128732201,
            Material::ThargoidMaterialCompositionData => 128737280,
            Material::ThargoidResidueData => 128737281,
            Material::ThargoidStructuralData => 128737282,
            Material::ThargoidCarapace => 128737283,
            Material::ThargoidEnergyCell => 128737284,
            Material::ThargoidOrganicCircuitry => 128737285,
            Material::ThargoidTechnologicalComponents => 128737286,
            Material::BioMechanicalConduits => 128737287,
            Material::PropulsionElements => 128793132,
            Material::WeaponParts => 128793133,
            Material::WreckageComponents => 128793134,
            Material::ShipFlightData => 128793135,
            Material::ShipSystemsData => 128793136,
            Material::GuardianPowerCell => 128815023,
            Material::GuardianPowerConduit => 128815024,
            Material::GuardianTechnologyComponent => 128815025,
            Material::GuardianSentinelWeaponParts => 128815026,
            Material::GuardianWreckageComponents => 128815027,
            Material::GuardianWeaponBlueprintFragment => 128815028,
            Material::GuardianModuleBlueprintFragment => 128815029,
            Material::GuardianVesselBlueprintFragment => 128815030,
            Material::Rhenium => 128837857,
            Material::Lead => 128850245,
            Material::Boron => 128850246,

            // TODO these need values
            Material::CausticCrystal => 0,
            Material::CausticShard => 0,
            Material::CorrosiveMechanisms => 0,
            Material::HardenedSurfaceFragments => 0,
            Material::HeatExposureSpecimen => 0,
            Material::PhasingMembraneResidue => 0,
            Material::TacticalCoreChip => 0,
            Material::MassiveEnergySurgeAnalytics => 0,
            Material::ThargoidInterdictionTelemetry => 0,

            #[cfg(not(feature = "strict"))]
            Material::Unknown(_) => 0,
        }
    }
}

#[derive(Debug, Error)]
pub enum MaterialError {
    #[error("Unknown material: '{0}'")]
    UnknownMaterial(String),
}

impl FromStr for Material {
    type Err = MaterialError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let string: &str = &s.to_ascii_lowercase();

        Ok(match string {
            "carbon" => Material::Carbon,
            "vanadium" => Material::Vanadium,
            "niobium" => Material::Niobium,
            "yttrium" => Material::Yttrium,
            "phosphorus" => Material::Phosphorus,
            "chromium" => Material::Chromium,
            "molybdenum" => Material::Molybdenum,
            "technetium" => Material::Technetium,
            "sulphur" => Material::Sulphur,
            "manganese" => Material::Manganese,
            "cadmium" => Material::Cadmium,
            "ruthenium" => Material::Ruthenium,
            "iron" => Material::Iron,
            "zinc" => Material::Zinc,
            "tin" => Material::Tin,
            "selenium" => Material::Selenium,
            "nickel" => Material::Nickel,
            "germanium" => Material::Germanium,
            "tungsten" => Material::Tungsten,
            "tellurium" => Material::Tellurium,
            "rhenium" => Material::Rhenium,
            "arsenic" => Material::Arsenic,
            "mercury" => Material::Mercury,
            "polonium" => Material::Polonium,
            "lead" => Material::Lead,
            "zirconium" => Material::Zirconium,
            "boron" => Material::Boron,
            "antimony" => Material::Antimony,
            "chemicalstorageunits" => Material::ChemicalStorageUnits,
            "chemicalprocessors" => Material::ChemicalProcessors,
            "chemicaldistillery" => Material::ChemicalDistillery,
            "chemicalmanipulators" => Material::ChemicalManipulators,
            "pharmaceuticalisolators" => Material::PharmaceuticalIsolators,
            "temperedalloys" => Material::TemperedAlloys,
            "heatresistantceramics" => Material::HeatResistantCeramics,
            "precipitatedalloys" => Material::PrecipitatedAlloys,
            "thermicalloys" => Material::ThermicAlloys,
            "militarygradealloys" => Material::MilitaryGradeAlloys,
            "heatconductionwiring" => Material::HeatConductionWiring,
            "heatdispersionplate" => Material::HeatDispersionPlate,
            "heatexchangers" => Material::HeatExchangers,
            "heatvanes" => Material::HeatVanes,
            "protoheatradiators" => Material::ProtoHeatRadiators,
            "basicconductors" => Material::BasicConductors,
            "conductivecomponents" => Material::ConductiveComponents,
            "conductiveceramics" => Material::ConductiveCeramics,
            "conductivepolymers" => Material::ConductivePolymers,
            "biotechconductors" => Material::BiotechConductors,
            "mechanicalscrap" => Material::MechanicalScrap,
            "mechanicalequipment" => Material::MechanicalEquipment,
            "mechanicalcomponents" => Material::MechanicalComponents,
            "configurablecomponents" => Material::ConfigurableComponents,
            "improvisedcomponents" => Material::ImprovisedComponents,
            "gridresistors" => Material::GridResistors,
            "hybridcapacitors" => Material::HybridCapacitors,
            "electrochemicalarrays" => Material::ElectrochemicalArrays,
            "polymercapacitors" => Material::PolymerCapacitors,
            "militarysupercapacitors" => Material::MilitarySupercapacitors,
            "wornshieldemitters" => Material::WornShieldEmitters,
            "shieldemitters" => Material::ShieldEmitters,
            "shieldingsensors" => Material::ShieldingSensors,
            "compoundshielding" => Material::CompoundShielding,
            "imperialshielding" => Material::ImperialShielding,
            "compactcomposites" => Material::CompactComposites,
            "filamentcomposites" => Material::FilamentComposites,
            "highdensitycomposites" => Material::HighDensityComposites,
            "fedproprietarycomposites" => Material::ProprietaryComposites,
            "fedcorecomposites" => Material::CoreDynamicsComposites,
            "crystalshards" => Material::CrystalShards,
            "uncutfocuscrystals" => Material::FlawedFocusCrystals,
            "focuscrystals" => Material::FocusCrystals,
            "refinedfocuscrystals" => Material::RefinedFocusCrystals,
            "exquisitefocuscrystals" => Material::ExquisiteFocusCrystals,
            "salvagedalloys" => Material::SalvagedAlloys,
            "galvanisingalloys" => Material::GalvanisingAlloys,
            "phasealloys" => Material::PhaseAlloys,
            "protolightalloys" => Material::ProtoLightAlloys,
            "protoradiolicalloys" => Material::ProtoRadiolicAlloys,
            "hardenedsurfacefragments" => Material::HardenedSurfaceFragments,
            "tg_causticshard" => Material::CausticShard,
            "tacticalcorechip" => Material::TacticalCoreChip,
            "unknowncarapace" => Material::ThargoidCarapace,
            "tg_biomechanicalconduits" => Material::BioMechanicalConduits,
            "tg_causticgeneratorparts" => Material::CorrosiveMechanisms,
            "phasingmembraneresidue" => Material::PhasingMembraneResidue,
            "thargoidenergycell" => Material::ThargoidEnergyCell,
            "tg_wreckagecomponents" => Material::WreckageComponents,
            "tg_causticcrystal" => Material::CausticCrystal,
            "thargoidtechnologicalcomponents" => Material::ThargoidTechnologicalComponents,
            "tg_weaponparts" => Material::WeaponParts,
            "heatexposurespecimen" => Material::HeatExposureSpecimen,
            "tg_propulsionelement" => Material::PropulsionElements,
            "unknownenergysource" => Material::SensorFragment,
            "thargoidorganiccircuitry" => Material::ThargoidOrganicCircuitry,
            "guardian_powercell" => Material::GuardianPowerCell,
            "guardian_sentinel_wreckagecomponents" => Material::GuardianWreckageComponents,
            "guardian_powerconduit" => Material::GuardianPowerConduit,
            "guardian_sentinel_weaponparts" => Material::GuardianSentinelWeaponParts,
            "guardian_techcomponent" => Material::GuardianTechnologyComponent,
            "scrambledemissiondata" => Material::ExceptionScrambledEmissionData,
            "archivedemissiondata" => Material::IrregularEmissionData,
            "emissiondata" => Material::UnexpectedEmissionData,
            "decodedemissiondata" => Material::DecodedEmissionData,
            "compactemissionsdata" => Material::AbnormalCompactEmissionData,
            "disruptedwakeechoes" => Material::AtypicalDisruptedWakeEchoes,
            "fsdtelemetry" => Material::AnomalousFSDTelemetry,
            "wakesolutions" => Material::StrangeWakeSolutions,
            "hyperspacetrajectories" => Material::EccentricHyperspaceTrajectories,
            "dataminedwake" => Material::DataminedWakeExceptions,
            "shieldcyclerecordings" => Material::DistortedShieldCycleRecordings,
            "shieldsoakanalysis" => Material::InconsistentShieldSoakAnalysis,
            "shielddensityreports" => Material::UntypicalShieldScans,
            "shieldpatternanalysis" => Material::AberrantShieldPatternAnalysis,
            "shieldfrequencydata" => Material::PeculiarShieldFrequencyData,
            "encryptedfiles" => Material::UnusualEncryptedFiles,
            "encryptioncodes" => Material::TaggedEncryptionCodes,
            "symmetrickeys" => Material::OpenSymmetricKeys,
            "encryptionarchives" => Material::AtypicalEncryptionArchives,
            "adaptiveencryptors" => Material::AdaptiveEncryptorsCapture,
            "bulkscandata" => Material::AnomalousBulkScanData,
            "scanarchives" => Material::UnidentifiedScanArchives,
            "scandatabanks" => Material::ClassifiedScanDatabanks,
            "encodedscandata" => Material::DivergentScanData,
            "classifiedscandata" => Material::ClassifiedScanFragment,
            "legacyfirmware" => Material::SpecializedLegacyFirmware,
            "consumerfirmware" => Material::ModifiedConsumerFirmware,
            "industrialfirmware" => Material::CrackedIndustrialFirmware,
            "securityfirmware" => Material::SecurityFirmwarePatch,
            "embeddedfirmware" => Material::ModifiedEmbeddedFirmware,
            "tg_structuraldata" => Material::ThargoidStructuralData,
            "tg_shutdowndata" => Material::MassiveEnergySurgeAnalytics,
            "tg_shipflightdata" => Material::ShipFlightData,
            "tg_shipsystemsdata" => Material::ShipSystemsData,
            "tg_interdictiondata" => Material::ThargoidInterdictionTelemetry,
            "tg_compositiondata" => Material::ThargoidMaterialCompositionData,
            "unknownshipsignature" => Material::ThargoidShipSignature,
            "tg_residuedata" => Material::ThargoidResidueData,
            "unknownwakedata" => Material::ThargoidWakeData,
            "ancienthistoricaldata" => Material::PatternGammaObeliskData,
            "ancientculturaldata" => Material::PatternBetaObeliskData,
            "ancientbiologicaldata" => Material::PatternAlphaObeliskData,
            "ancientlanguagedata" => Material::PatternDeltaObeliskData,
            "ancienttechnologicaldata" => Material::PatternEpsilonObeliskData,
            "guardian_moduleblueprint" => Material::GuardianModuleBlueprintFragment,
            "guardian_vesselblueprint" => Material::GuardianVesselBlueprintFragment,
            "guardian_weaponblueprint" => Material::GuardianWeaponBlueprintFragment,

            #[cfg(not(feature = "strict"))]
            _ => Material::Unknown(s.to_string()),

            #[cfg(feature = "strict")]
            _ => return Err(MaterialError::UnknownMaterial(s.to_string())),
        })
    }
}

from_str_deserialize_impl!(Material);

impl Display for Material {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                // Raw
                Material::Antimony => "Antimony",
                Material::Arsenic => "Arsenic",
                Material::Boron => "Boron",
                Material::Cadmium => "Cadmium",
                Material::Carbon => "Carbon",
                Material::Chromium => "Chromium",
                Material::Germanium => "Germanium",
                Material::Iron => "Iron",
                Material::Lead => "Lead",
                Material::Manganese => "Manganese",
                Material::Mercury => "Mercury",
                Material::Molybdenum => "Molybdenum",
                Material::Nickel => "Nickel",
                Material::Niobium => "Niobium",
                Material::Phosphorus => "Phosphorus",
                Material::Polonium => "Polonium",
                Material::Rhenium => "Rhenium",
                Material::Ruthenium => "Ruthenium",
                Material::Selenium => "Selenium",
                Material::Sulphur => "Sulphur",
                Material::Technetium => "Technetium",
                Material::Tellurium => "Tellurium",
                Material::Tin => "Tin",
                Material::Tungsten => "Tungsten",
                Material::Vanadium => "Vanadium",
                Material::Yttrium => "Yttrium",
                Material::Zinc => "Zinc",
                Material::Zirconium => "Zirconium",

                // Manufactured
                Material::BasicConductors => "Basic Conductors",
                Material::BioMechanicalConduits => "Bio-Mechanical Conduits",
                Material::BiotechConductors => "Biotech Conductors",
                Material::CausticCrystal => "Caustic Crystal",
                Material::CausticShard => "Caustic Shard",
                Material::ChemicalDistillery => "Chemical Distillery",
                Material::ChemicalManipulators => "Chemical Manipulators",
                Material::ChemicalProcessors => "Chemical Processors",
                Material::ChemicalStorageUnits => "Chemical Storage Units",
                Material::CompactComposites => "Compact Composites",
                Material::CompoundShielding => "Compound Shielding",
                Material::ConductiveCeramics => "Conductive Ceramics",
                Material::ConductiveComponents => "Conductive Components",
                Material::ConductivePolymers => "Conductive Polymers",
                Material::ConfigurableComponents => "Configurable Components",
                Material::CoreDynamicsComposites => "Core Dynamics Composites",
                Material::CorrosiveMechanisms => "Corrosive Mechanisms",
                Material::CrystalShards => "Crystal Shards",
                Material::ElectrochemicalArrays => "Electrochemical Arrays",
                Material::ExquisiteFocusCrystals => "Exquisite Focus Crystals",
                Material::FilamentComposites => "Filament Composites",
                Material::FlawedFocusCrystals => "Flawed Focus Crystals",
                Material::FocusCrystals => "Focus Crystals",
                Material::GalvanisingAlloys => "Galvanising Alloys",
                Material::GridResistors => "Grid Resistors",
                Material::GuardianPowerCell => "Guardian Power Cell",
                Material::GuardianPowerConduit => "Guardian Power Conduit",
                Material::GuardianSentinelWeaponParts => "Guardian Sentinel Weapon Parts",
                Material::GuardianTechnologyComponent => "Guardian Technology Component",
                Material::GuardianWreckageComponents => "Guardian Wreckage Components",
                Material::HardenedSurfaceFragments => "Hardened Surface Fragments",
                Material::HeatConductionWiring => "Heat Conduction Wiring",
                Material::HeatDispersionPlate => "Heat Dispersion Plate",
                Material::HeatExchangers => "Heat Exchangers",
                Material::HeatExposureSpecimen => "Heat Exposure Specimen",
                Material::HeatResistantCeramics => "Heat Resistant Ceramics",
                Material::HeatVanes => "Heat Vanes",
                Material::HighDensityComposites => "High Density Composites",
                Material::HybridCapacitors => "Hybrid Capacitors",
                Material::ImperialShielding => "Imperial Shielding",
                Material::ImprovisedComponents => "Improvised Components",
                Material::MechanicalComponents => "Mechanical Components",
                Material::MechanicalEquipment => "Mechanical Equipment",
                Material::MechanicalScrap => "Mechanical Scrap",
                Material::MilitaryGradeAlloys => "Military Grade Alloys",
                Material::MilitarySupercapacitors => "Military Supercapacitors",
                Material::PharmaceuticalIsolators => "Pharmaceutical Isolators",
                Material::PhaseAlloys => "Phase Alloys",
                Material::PhasingMembraneResidue => "Phasing Membrane Residue",
                Material::PolymerCapacitors => "Polymer Capacitors",
                Material::PrecipitatedAlloys => "Precipitated Alloys",
                Material::ProprietaryComposites => "Proprietary Composites",
                Material::PropulsionElements => "Propulsion Elements",
                Material::ProtoHeatRadiators => "Proto Heat Radiators",
                Material::ProtoLightAlloys => "Proto Light Alloys",
                Material::ProtoRadiolicAlloys => "Proto Radiolic Alloys",
                Material::RefinedFocusCrystals => "Refined Focus Crystals",
                Material::SalvagedAlloys => "Salvaged Alloys",
                Material::SensorFragment => "Sensor Fragment",
                Material::ShieldEmitters => "Shield Emitters",
                Material::ShieldingSensors => "Shielding Sensors",
                Material::TacticalCoreChip => "Tactical Core Chip",
                Material::TemperedAlloys => "Tempered Alloys",
                Material::ThargoidCarapace => "Thargoid Carapace",
                Material::ThargoidEnergyCell => "Thargoid Energy Cell",
                Material::ThargoidOrganicCircuitry => "Thargoid Organic Circuitry",
                Material::ThargoidTechnologicalComponents => "Thargoid Technological Components",
                Material::ThermicAlloys => "Thermic Alloys",
                Material::WeaponParts => "Weapon Parts",
                Material::WornShieldEmitters => "Worn Shield Emitters",
                Material::WreckageComponents => "Wreckage Components",

                // Encoded
                Material::AberrantShieldPatternAnalysis => "Aberrant Shield Pattern Analysis",
                Material::AbnormalCompactEmissionData => "Abnormal Compact Emissions Data",
                Material::AdaptiveEncryptorsCapture => "Adaptive Encryptors Capture",
                Material::AnomalousBulkScanData => "Anomalous Bulk Scan Data",
                Material::AnomalousFSDTelemetry => "Anomalous FSD Telemetry",
                Material::AtypicalDisruptedWakeEchoes => "Atypical Disrupted Wake Echoes",
                Material::AtypicalEncryptionArchives => "Atypical Encryption Archives",
                Material::ClassifiedScanDatabanks => "Classified Scan Databanks",
                Material::ClassifiedScanFragment => "Classified Scan Fragment",
                Material::CrackedIndustrialFirmware => "Cracked Industrial Firmware",
                Material::DataminedWakeExceptions => "Datamined Wake Exceptions",
                Material::DecodedEmissionData => "Decoded Emission Data",
                Material::DistortedShieldCycleRecordings => "Distorted Shield Cycle Recordings",
                Material::DivergentScanData => "Divergent Scan Data",
                Material::EccentricHyperspaceTrajectories => "Eccentric Hyperspace Trajectories",
                Material::ExceptionScrambledEmissionData => "Exceptional Scrambled Emission Data",
                Material::GuardianModuleBlueprintFragment => "Guardian Module Blueprint Fragment",
                Material::GuardianVesselBlueprintFragment => "Guardian Vessel Blueprint Fragment",
                Material::GuardianWeaponBlueprintFragment => "Guardian Weapon Blueprint Fragment",
                Material::InconsistentShieldSoakAnalysis => "Inconsistent Shield Soak Analysis",
                Material::IrregularEmissionData => "Irregular Emission Data",
                Material::MassiveEnergySurgeAnalytics => "Massive Energy Surge Analytics",
                Material::ModifiedConsumerFirmware => "Modified Consumer Firmware",
                Material::ModifiedEmbeddedFirmware => "Modified Embedded Firmware",
                Material::OpenSymmetricKeys => "Open Symmetric Keys",
                Material::PatternAlphaObeliskData => "Pattern Alpha Obelisk Data",
                Material::PatternBetaObeliskData => "Pattern Beta Obelisk Data",
                Material::PatternDeltaObeliskData => "Pattern Delta Obelisk Data",
                Material::PatternEpsilonObeliskData => "Pattern Epsilon Obelisk Data",
                Material::PatternGammaObeliskData => "Pattern Gamma Obelisk Data",
                Material::PeculiarShieldFrequencyData => "Peculiar Shield Frequency Data",
                Material::SecurityFirmwarePatch => "Security Firmware Patch",
                Material::ShipFlightData => "Ship Flight Data",
                Material::ShipSystemsData => "Ship Systems Data",
                Material::SpecializedLegacyFirmware => "Specialised Legacy Firmware",
                Material::StrangeWakeSolutions => "Strange Wake Solutions",
                Material::TaggedEncryptionCodes => "Tagged Encryption Codes",
                Material::ThargoidInterdictionTelemetry => "Thargoid Interdiction Telemetry",
                Material::ThargoidMaterialCompositionData => "Thargoid Material Composition Data",
                Material::ThargoidResidueData => "Thargoid Residue Data",
                Material::ThargoidShipSignature => "Thargoid Ship Signature",
                Material::ThargoidStructuralData => "Thargoid Structural Data",
                Material::ThargoidWakeData => "Thargoid Wake Data",
                Material::UnexpectedEmissionData => "Unexpected Emission Data",
                Material::UnidentifiedScanArchives => "Unidentified Scan Archives",
                Material::UntypicalShieldScans => "Untypical Shield Scans",
                Material::UnusualEncryptedFiles => "Unusual Encrypted Files",

                #[cfg(not(feature = "strict"))]
                Material::Unknown(unknown) => return write!(f, "Unknown material: {}", unknown),
            }
        )
    }
}
