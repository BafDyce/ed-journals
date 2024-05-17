use std::fmt::{Display, Formatter};
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use thiserror::Error;

use crate::from_str_deserialize_impl;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum Item {
    // Data
    AccidentLogs,
    AirQualityReports,
    AtmosphericData,
    AudioLogs,
    AxCombatLogs,
    BallisticsData,
    BiologicalWeaponData,
    BiometricData,
    BlacklistData,
    BloodTestResults,
    CampaignPlans,
    CatMedia,
    CensusData,
    ChemicalExperimentData,
    ChemicalFormulae,
    ChemicalInventory,
    ChemicalPatents,
    ChemicalWeaponData,
    ClassicEntertainment,
    ClinicalTrialRecords,
    CocktailRecipes,
    CombatTrainingMaterial,
    CombatantPerformance,
    ConflictHistory,
    CriminalRecords,
    CropYieldAnalysis,
    CulinaryRecipes,
    DigitalDesigns,
    DutyRota,
    EmployeeDirectory,
    EmployeeExpenses,
    EmployeeGeneticData,
    EmploymentHistory,
    EnhancedInterrogationRecordings,
    EspionageMaterial,
    EvacuationProtocols,
    ExplorationJournals,
    ExtractionYieldData,
    FactionAssociates,
    FactionDonatorList,
    FactionNews,
    FinancialProjections,
    FleetRegistry,
    GeneSequencingData,
    GeneticResearch,
    GeologicalData,
    HydroponicData,
    IncidentLogs,
    InfluenceProjections,
    InternalCorrespondence,
    InterrogationRecordings,
    InterviewRecordings,
    JobApplications,
    Kompromat,
    LiteraryFiction,
    MaintenanceLogs,
    ManufacturingInstructions,
    MedicalRecords,
    MeetingMinutes,
    MineralSurvey,
    MiningAnalytics,
    MultimediaEntertainment,
    NetworkAccessHistory,
    NetworkSecurityProtocols,
    NextOfKinRecords,
    NocData,
    OperationalManual,
    OpinionPolls,
    PatientHistory,
    PatrolRoutes,
    PayrollInformation,
    PersonalLogs,
    PharmaceuticalPatents,
    PhotoAlbums,
    PlantGrowthCharts,
    PoliticalAffiliations,
    PrisonerLogs,
    ProductionReports,
    ProductionSchedule,
    Propaganda,
    PurchaseRecords,
    PurchaseRequests,
    RadioactivityData,
    ReactorOutputReview,
    RecyclingLogs,
    ResidentialDirectory,
    RiskAssessments,
    SalesRecords,
    SecurityExpenses,
    SeedGeneaology,
    SettlementAssaultPlans,
    SettlementDefencePlans,
    ShareholderInformation,
    SlushFundLogs,
    SmearCampaignPlans,
    SpectralAnalysisData,
    Spyware,
    StellarActivityLogs,
    SurveillanceLogs,
    TacticalPlans,
    TaxRecords,
    TopographicalSurveys,
    TravelPermits,
    TroopDeploymentRecords,
    UnionMembership,
    VaccinationRecords,
    VaccineResearch,
    VipSecurityDetail,
    VirologyData,
    Virus,
    VisitorRegister,
    WeaponInventory,
    WeaponTestData,
    XenoDefenceProtocols,

    // Goods
    AgriculturalProcessSample,
    BiochemicalAgent,
    BiologicalSample,
    BuildingSchematic,
    Californium,
    CastFossil,
    ChemicalProcessSample,
    ChemicalSample,
    CompactLibrary,
    CompressionLiquefiedGas,
    ContaminatedSpireRefineryCompound,
    DeepMantleSample,
    DegradedPowerRegulator,
    GMeds,
    GeneticRepairMeds,
    HealthMonitor,
    Hush,
    InertiaCanister,
    Infinity,
    InorganicContaminant,
    Insight,
    InsightDataBank,
    InsightEntertainmentSuite,
    IonisedGas,
    Lazarus,
    MicrobialInhibitor,
    MutagenicCatalyst,
    NutritionalConcentrate,
    PersonalComputer,
    PersonalDocuments,
    PetrifiedFossil,
    PowerRegulator,
    Push,
    PyrolyticCatalyst,
    RefinementProcessSample,
    ShipSchematic,
    SpireRefineryCompound,
    SuitSchematic,
    SurveillanceEquipment,
    SyntheticGenome,
    SyntheticPathogen,
    TrueFormFossil,
    UniversalTranslator,
    VehicleSchematic,
    WeaponSchematic,

    // Chemicals
    Aerogel,
    ChemicalCatalyst,
    ChemicalSuperbase,
    Epinephrine,
    EpoxyAdhesive,
    Graphene,
    OxygenicBacteria,
    PHNeutraliser,
    RDX,
    ViscoelasticPolymer,

    // Circuits
    CircuitBoard,
    CircuitSwitch,
    ElectricalFuse,
    ElectricalWiring,
    Electromagnet,
    IonBattery,
    MetalCoil,
    MicroSupercapacitor,
    MicroTransformer,
    Microelectrode,
    Motor,
    OpticalFibre,

    // Tech
    CarbonFibrePlating,
    EncryptedMemoryChip,
    MemoryChip,
    MicroHydraulics,
    MicroThrusters,
    OpticalLens,
    Scrambler,
    TitaniumPlating,
    Transmitter,
    TungstenCarbide,
    WeaponComponent,

    // Consumables
    EnergyCell,
    FragGranade,
    Medkit,
    ShieldDisruptor,
    ShieldProjector,
    EBreach,

    #[cfg(not(feature = "strict"))]
    Unknown(String),
}

#[derive(Debug, Error)]
pub enum ItemError {
    #[error("Unknown item: '{0}'")]
    UnknownItem(String),
}

impl Item {
    fn name_to_item(name: &str) -> Result<Item, ItemError> {
        let lower_case: &str = &name.to_ascii_lowercase();

        Ok(match lower_case {
            // Data
            "accidentlogs" => Item::AccidentLogs,
            "airqualityreports" => Item::AirQualityReports,
            "atmosphericdata" => Item::AtmosphericData,
            "audiologs" => Item::AudioLogs,
            "axcombatlogs" => Item::AxCombatLogs,
            "ballisticsdata" => Item::BallisticsData,
            "biologicalweapondata" => Item::BiologicalWeaponData,
            "biometricdata" => Item::BiometricData,
            "blacklistdata" => Item::BlacklistData,
            "bloodtestresults" => Item::BloodTestResults,
            "campaignplans" => Item::CampaignPlans,
            "catmedia" => Item::CatMedia,
            "censusdata" => Item::CensusData,
            "chemicalexperimentdata" => Item::ChemicalExperimentData,
            "chemicalformulae" => Item::ChemicalFormulae,
            "chemicalinventory" => Item::ChemicalInventory,
            "chemicalpatents" => Item::ChemicalPatents,
            "chemicalweapondata" => Item::ChemicalWeaponData,
            "classicentertainment" => Item::ClassicEntertainment,
            "medicaltrialrecords" => Item::ClinicalTrialRecords,
            "cocktailrecipes" => Item::CocktailRecipes,
            "combattrainingmaterial" => Item::CombatTrainingMaterial,
            "combatantperformance" => Item::CombatantPerformance,
            "conflicthistory" => Item::ConflictHistory,
            "criminalrecords" => Item::CriminalRecords,
            "cropyieldanalysis" => Item::CropYieldAnalysis,
            "culinaryrecipes" => Item::CulinaryRecipes,
            "digitaldesigns" => Item::DigitalDesigns,
            "dutyrota" => Item::DutyRota,
            "employeedirectory" => Item::EmployeeDirectory,
            "employeeexpenses" => Item::EmployeeExpenses,
            "employeegeneticdata" => Item::EmployeeGeneticData,
            "employmenthistory" => Item::EmploymentHistory,
            "enhancedinterrogationrecordings" => Item::EnhancedInterrogationRecordings,
            "espionagematerial" => Item::EspionageMaterial,
            "evacuationprotocols" => Item::EvacuationProtocols,
            "explorationjournals" => Item::ExplorationJournals,
            "extractionyielddata" => Item::ExtractionYieldData,
            "factionassociates" => Item::FactionAssociates,
            "factiondonatorlist" => Item::FactionDonatorList,
            "factionnews" => Item::FactionNews,
            "financialprojections" => Item::FinancialProjections,
            "fleetregistry" => Item::FleetRegistry,
            "genesequencingdata" => Item::GeneSequencingData,
            "geneticresearch" => Item::GeneticResearch,
            "geologicaldata" => Item::GeologicalData,
            "hydroponicdata" => Item::HydroponicData,
            "incidentlogs" => Item::IncidentLogs,
            "influenceprojections" => Item::InfluenceProjections,
            "internalcorrespondence" => Item::InternalCorrespondence,
            "interrogationrecordings" => Item::InterrogationRecordings,
            "interviewrecordings" => Item::InterviewRecordings,
            "jobapplications" => Item::JobApplications,
            "kompromat" => Item::Kompromat,
            "literaryfiction" => Item::LiteraryFiction,
            "maintenancelogs" => Item::MaintenanceLogs,
            "manufacturinginstructions" => Item::ManufacturingInstructions,
            "medicalrecords" => Item::MedicalRecords,
            "meetingminutes" => Item::MeetingMinutes,
            "mineralsurvey" => Item::MineralSurvey,
            "mininganalytics" => Item::MiningAnalytics,
            "multimediaentertainment" => Item::MultimediaEntertainment,
            "networkaccesshistory" => Item::NetworkAccessHistory,
            "networksecurityprotocols" => Item::NetworkSecurityProtocols,
            "nextofkinrecords" => Item::NextOfKinRecords,
            "nocdata" => Item::NocData,
            "operationalmanual" => Item::OperationalManual,
            "opinionpolls" => Item::OpinionPolls,
            "patienthistory" => Item::PatientHistory,
            "patrolroutes" => Item::PatrolRoutes,
            "payrollinformation" => Item::PayrollInformation,
            "personallogs" => Item::PersonalLogs,
            "pharmaceuticalpatents" => Item::PharmaceuticalPatents,
            "photoalbums" => Item::PhotoAlbums,
            "plantgrowthcharts" => Item::PlantGrowthCharts,
            "politicalaffiliations" => Item::PoliticalAffiliations,
            "prisonerlogs" => Item::PrisonerLogs,
            "productionreports" => Item::ProductionReports,
            "productionschedule" => Item::ProductionSchedule,
            "propaganda" => Item::Propaganda,
            "purchaserecords" => Item::PurchaseRecords,
            "purchaserequests" => Item::PurchaseRequests,
            "radioactivitydata" => Item::RadioactivityData,
            "reactoroutputreview" => Item::ReactorOutputReview,
            "recyclinglogs" => Item::RecyclingLogs,
            "residentialdirectory" => Item::ResidentialDirectory,
            "riskassessments" => Item::RiskAssessments,
            "salesrecords" => Item::SalesRecords,
            "securityexpenses" => Item::SecurityExpenses,
            "seedgeneaology" => Item::SeedGeneaology,
            "settlementassaultplans" => Item::SettlementAssaultPlans,
            "settlementdefenceplans" => Item::SettlementDefencePlans,
            "shareholderinformation" => Item::ShareholderInformation,
            "slushfundlogs" => Item::SlushFundLogs,
            "smearcampaignplans" => Item::SmearCampaignPlans,
            "spectralanalysisdata" => Item::SpectralAnalysisData,
            "spyware" => Item::Spyware,
            "stellaractivitylogs" => Item::StellarActivityLogs,
            "surveilleancelogs" => Item::SurveillanceLogs,
            "tacticalplans" => Item::TacticalPlans,
            "taxrecords" => Item::TaxRecords,
            "topographicalsurveys" => Item::TopographicalSurveys,
            "travelpermits" => Item::TravelPermits,
            "troopdeploymentrecords" => Item::TroopDeploymentRecords,
            "unionmembership" => Item::UnionMembership,
            "vaccinationrecords" => Item::VaccinationRecords,
            "vaccineresearch" => Item::VaccineResearch,
            "vipsecuritydetail" => Item::VipSecurityDetail,
            "virologydata" => Item::VirologyData,
            "virus" => Item::Virus,
            "visitorregister" => Item::VisitorRegister,
            "weaponinventory" => Item::WeaponInventory,
            "weapontestdata" => Item::WeaponTestData,
            "xenodefenceprotocols" => Item::XenoDefenceProtocols,

            // Goods
            "agriculturalprocesssample" => Item::AgriculturalProcessSample,
            "biochemicalagent" => Item::BiochemicalAgent,
            "geneticsample" => Item::BiologicalSample,
            "buildingschematic" => Item::BuildingSchematic,
            "californium" => Item::Californium,
            "castfossil" => Item::CastFossil,
            "chemicalprocesssample" => Item::ChemicalProcessSample,
            "chemicalsample" => Item::ChemicalSample,
            "compactlibrary" => Item::CompactLibrary,
            "compressionliquefiedgas" => Item::CompressionLiquefiedGas,
            "contaminatedspirerefinerycompound" => Item::ContaminatedSpireRefineryCompound,
            "deepmantlesample" => Item::DeepMantleSample,
            "degradedpowerregulator" => Item::DegradedPowerRegulator,
            "gmeds" => Item::GMeds,
            "geneticrepairmeds" => Item::GeneticRepairMeds,
            "healthmonitor" => Item::HealthMonitor,
            "hush" => Item::Hush,
            "inertiacanister" => Item::InertiaCanister,
            "infinity" => Item::Infinity,
            "inorganiccontaminant" => Item::InorganicContaminant,
            "insight" => Item::Insight,
            "insightdatabank" => Item::InsightDataBank,
            "insightentertainmentsuite" => Item::InsightEntertainmentSuite,
            "ionisedgas" => Item::IonisedGas,
            "lazarus" => Item::Lazarus,
            "microbialinhibitor" => Item::MicrobialInhibitor,
            "mutageniccatalyst" => Item::MutagenicCatalyst,
            "nutritionalconcentrate" => Item::NutritionalConcentrate,
            "personalcomputer" => Item::PersonalComputer,
            "personaldocuments" => Item::PersonalDocuments,
            "petrifiedfossil" => Item::PetrifiedFossil,
            "largecapacitypowerregulator" => Item::PowerRegulator,
            "push" => Item::Push,
            "pyrolyticcatalyst" => Item::PyrolyticCatalyst,
            "refinementprocesssample" => Item::RefinementProcessSample,
            "shipschematic" => Item::ShipSchematic,
            "spirerefinerycompound" => Item::SpireRefineryCompound,
            "suitschematic" => Item::SuitSchematic,
            "surveillanceequipment" => Item::SurveillanceEquipment,
            "syntheticgenome" => Item::SyntheticGenome,
            "syntheticpathogen" => Item::SyntheticPathogen,
            "trueformfossil" => Item::TrueFormFossil,
            "universaltranslator" => Item::UniversalTranslator,
            "vehicleschematic" => Item::VehicleSchematic,
            "weaponschematic" => Item::WeaponSchematic,

            // Chemicals
            "aerogel" => Item::Aerogel,
            "chemicalcatalyst" => Item::ChemicalCatalyst,
            "chemicalsuperbase" => Item::ChemicalSuperbase,
            "epinephrine" => Item::Epinephrine,
            "epoxyadhesive" => Item::EpoxyAdhesive,
            "graphene" => Item::Graphene,
            "oxygenicbacteria" => Item::OxygenicBacteria,
            "phneutraliser" => Item::PHNeutraliser,
            "rdx" => Item::RDX,
            "viscoelasticpolymer" => Item::ViscoelasticPolymer,

            "circuitboard" => Item::CircuitBoard,
            "circuitswitch" => Item::CircuitSwitch,
            "electricalfuse" => Item::ElectricalFuse,
            "electricalwiring" => Item::ElectricalWiring,
            "electromagnet" => Item::Electromagnet,
            "ionbattery" => Item::IonBattery,
            "metalcoil" => Item::MetalCoil,
            "microsupercapacitor" => Item::MicroSupercapacitor,
            "microtransformer" => Item::MicroTransformer,
            "microelectrode" => Item::Microelectrode,
            "motor" => Item::Motor,
            "opticalfibre" => Item::OpticalFibre,

            "carbonfibreplating" => Item::CarbonFibrePlating,
            "encryptedmemorychip" => Item::EncryptedMemoryChip,
            "memorychip" => Item::MemoryChip,
            "microhydraulics" => Item::MicroHydraulics,
            "microthrusters" => Item::MicroThrusters,
            "opticallens" => Item::OpticalLens,
            "scrambler" => Item::Scrambler,
            "titaniumplating" => Item::TitaniumPlating,
            "transmitter" => Item::Transmitter,
            "tungstencarbide" => Item::TungstenCarbide,
            "weaponcomponent" => Item::WeaponComponent,

            "energycell" => Item::EnergyCell,
            "amm_grenade_frag" => Item::FragGranade,
            "healthpack" => Item::Medkit,
            "amm_grenade_emp" => Item::ShieldDisruptor,
            "amm_grenade_shield" => Item::ShieldProjector,
            "bypass" => Item::EBreach,

            #[cfg(not(feature = "strict"))]
            _ => Item::Unknown(name.to_string()),

            #[cfg(feature = "strict")]
            _ => return Err(ItemError::UnknownItem(name.to_string())),
        })
    }
}

lazy_static! {
    static ref ITEM_NAME_REGEX: Regex = Regex::new(r#"^\$?([a-zA-Z_]+?)(_[nN]ame;)?$"#).unwrap();
}

impl FromStr for Item {
    type Err = ItemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = ITEM_NAME_REGEX.captures(s) else {
            return Err(ItemError::UnknownItem(s.to_string()));
        };

        Item::name_to_item(
            captures
                .get(1)
                .expect("Should have been captured already")
                .as_str(),
        )
    }
}

from_str_deserialize_impl!(Item);

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                // Data
                Item::AccidentLogs => "Accident Logs",
                Item::AirQualityReports => "Air Quality Reports",
                Item::AtmosphericData => "Atmospheric Data",
                Item::AudioLogs => "Audio Logs",
                Item::AxCombatLogs => "AX Combat Logs",
                Item::BallisticsData => "Ballistics Data",
                Item::BiologicalWeaponData => "Biological Weapon Data",
                Item::BiometricData => "Biometric Data",
                Item::BlacklistData => "Blacklist Data",
                Item::BloodTestResults => "Blood Test Results",
                Item::CampaignPlans => "Campaign Plans",
                Item::CatMedia => "Cat Media",
                Item::CensusData => "Census Data",
                Item::ChemicalExperimentData => "Chemical Experiment Data",
                Item::ChemicalFormulae => "Chemical Formulae",
                Item::ChemicalInventory => "Chemical Inventory",
                Item::ChemicalPatents => "Chemical Patents",
                Item::ChemicalWeaponData => "Chemical Weapon Data",
                Item::ClassicEntertainment => "Classic Entertainment",
                Item::ClinicalTrialRecords => "Clinical Trial Records",
                Item::CocktailRecipes => "Cocktail Recipes",
                Item::CombatTrainingMaterial => "Combat Training Material",
                Item::CombatantPerformance => "Combatant Performance",
                Item::ConflictHistory => "Conflict History",
                Item::CriminalRecords => "Criminal Records",
                Item::CropYieldAnalysis => "Crop Yield Analysis",
                Item::CulinaryRecipes => "Culinary Recipes",
                Item::DigitalDesigns => "Digital Designs",
                Item::DutyRota => "Duty Rota",
                Item::EmployeeDirectory => "Employee Directory",
                Item::EmployeeExpenses => "Employee Expenses",
                Item::EmployeeGeneticData => "Employee Genetic Data",
                Item::EmploymentHistory => "Employment History",
                Item::EnhancedInterrogationRecordings => "Enhanced Interrogation Recordings",
                Item::EspionageMaterial => "Espionage Material",
                Item::EvacuationProtocols => "Evacuation Protocols",
                Item::ExplorationJournals => "Exploration Journals",
                Item::ExtractionYieldData => "Extraction Yield Data",
                Item::FactionAssociates => "Faction Associates",
                Item::FactionDonatorList => "Faction Donator List",
                Item::FactionNews => "Faction News",
                Item::FinancialProjections => "Financial Projections",
                Item::FleetRegistry => "Fleet Registry",
                Item::GeneSequencingData => "Gene Sequencing Data",
                Item::GeneticResearch => "Genetic Research",
                Item::GeologicalData => "Geological Data",
                Item::HydroponicData => "Hydroponic Data",
                Item::IncidentLogs => "Incident Logs",
                Item::InfluenceProjections => "Influence Projections",
                Item::InternalCorrespondence => "Internal Correspondence",
                Item::InterrogationRecordings => "Interrogation Recordings",
                Item::InterviewRecordings => "Interview Recordings",
                Item::JobApplications => "Job Applications",
                Item::Kompromat => "Kompromat",
                Item::LiteraryFiction => "Literary Fiction",
                Item::MaintenanceLogs => "Maintenance Logs",
                Item::ManufacturingInstructions => "Manufacturing Instructions",
                Item::MedicalRecords => "Medical Records",
                Item::MeetingMinutes => "Meeting Minutes",
                Item::MineralSurvey => "Mineral Survey",
                Item::MiningAnalytics => "Mining Analytics",
                Item::MultimediaEntertainment => "Multimedia Entertainment",
                Item::NetworkAccessHistory => "Network Access History",
                Item::NetworkSecurityProtocols => "Network Security Protocols",
                Item::NextOfKinRecords => "Next of Kin Records",
                Item::NocData => "NOC Data",
                Item::OperationalManual => "Operational Manual",
                Item::OpinionPolls => "Opinion Polls",
                Item::PatientHistory => "Patient History",
                Item::PatrolRoutes => "Patrol Routes",
                Item::PayrollInformation => "Payroll Information",
                Item::PersonalLogs => "Personal Logs",
                Item::PharmaceuticalPatents => "Pharmaceutical Patents",
                Item::PhotoAlbums => "Photo Albums",
                Item::PlantGrowthCharts => "Plant Growth Charts",
                Item::PoliticalAffiliations => "Political Affiliations",
                Item::PrisonerLogs => "Prisoner Logs",
                Item::ProductionReports => "Production Reports",
                Item::ProductionSchedule => "Production Schedule",
                Item::Propaganda => "Propaganda",
                Item::PurchaseRecords => "Purchase Records",
                Item::PurchaseRequests => "Purchase Requests",
                Item::RadioactivityData => "Radioactivity Data",
                Item::ReactorOutputReview => "Reactor Output Review",
                Item::RecyclingLogs => "Recycling Logs",
                Item::ResidentialDirectory => "Residential Directory",
                Item::RiskAssessments => "Risk Assessments",
                Item::SalesRecords => "Sales Records",
                Item::SecurityExpenses => "Security Expenses",
                Item::SeedGeneaology => "Seed Geneaology",
                Item::SettlementAssaultPlans => "Settlement Assault Plans",
                Item::SettlementDefencePlans => "Settlement Defence Plans",
                Item::ShareholderInformation => "Shareholder Information",
                Item::SlushFundLogs => "Slush Fund Logs",
                Item::SmearCampaignPlans => "Smear Campaign Plans",
                Item::SpectralAnalysisData => "Spectral Analysis Data",
                Item::Spyware => "Spyware",
                Item::StellarActivityLogs => "Stellar Activity Logs",
                Item::SurveillanceLogs => "Surveillance Logs",
                Item::TacticalPlans => "Tactical Plans",
                Item::TaxRecords => "Tax Records",
                Item::TopographicalSurveys => "Topographical Surveys",
                Item::TravelPermits => "Travel Permits",
                Item::TroopDeploymentRecords => "Troop Deployment Records",
                Item::UnionMembership => "Union Membership",
                Item::VaccinationRecords => "Vaccination Records",
                Item::VaccineResearch => "Vaccine Research",
                Item::VipSecurityDetail => "VIP Security Detail",
                Item::VirologyData => "Virology Data",
                Item::Virus => "Virus",
                Item::VisitorRegister => "Visitor Register",
                Item::WeaponInventory => "Weapon Inventory",
                Item::WeaponTestData => "Weapon Test Data",
                Item::XenoDefenceProtocols => "Xeno-Defence Protocols",

                // Goods
                Item::AgriculturalProcessSample => "Agricultural Process Sample",
                Item::BiochemicalAgent => "Biochemical Agent",
                Item::BiologicalSample => "Biological Sample",
                Item::BuildingSchematic => "Building Schematic",
                Item::Californium => "Californium",
                Item::CastFossil => "Cast Fossil",
                Item::ChemicalProcessSample => "Chemical Process Sample",
                Item::ChemicalSample => "Chemical Sample",
                Item::CompactLibrary => "Compact Library",
                Item::CompressionLiquefiedGas => "Compression-Liquefied Gas",
                Item::ContaminatedSpireRefineryCompound => "Contaminated Spire Refinery Compound",
                Item::DeepMantleSample => "Deep Mantle Sample",
                Item::DegradedPowerRegulator => "Degraded Power Regulator",
                Item::GMeds => "G-Meds",
                Item::GeneticRepairMeds => "Genetic Repair Meds",
                Item::HealthMonitor => "Health Monitor",
                Item::Hush => "Hush",
                Item::InertiaCanister => "Inertia Canister",
                Item::Infinity => "Infinity",
                Item::InorganicContaminant => "Inorganic Contaminant",
                Item::Insight => "Insight",
                Item::InsightDataBank => "Insight Data Bank",
                Item::InsightEntertainmentSuite => "Insight Entertainment Suite",
                Item::IonisedGas => "Ionised Gas",
                Item::Lazarus => "Lazarus",
                Item::MicrobialInhibitor => "Microbial Inhibitor",
                Item::MutagenicCatalyst => "Mutagenic Catalyst",
                Item::NutritionalConcentrate => "Nutritional Concentrate",
                Item::PersonalComputer => "Personal Computer",
                Item::PersonalDocuments => "Personal Documents",
                Item::PetrifiedFossil => "Petrified Fossil",
                Item::PowerRegulator => "Power Regulator",
                Item::Push => "Push",
                Item::PyrolyticCatalyst => "Pyrolytic Catalyst",
                Item::RefinementProcessSample => "Refinement Process Sample",
                Item::ShipSchematic => "Ship Schematic",
                Item::SpireRefineryCompound => "Spire Refinery Compound",
                Item::SuitSchematic => "Suit Schematic",
                Item::SurveillanceEquipment => "Surveillance Equipment",
                Item::SyntheticGenome => "Synthetic Genome",
                Item::SyntheticPathogen => "Synthetic Pathogen",
                Item::TrueFormFossil => "True Form Fossil",
                Item::UniversalTranslator => "Universal Translator",
                Item::VehicleSchematic => "Vehicle Schematic",
                Item::WeaponSchematic => "Weapon Schematic",

                // Chemicals
                Item::Aerogel => "Aerogel",
                Item::ChemicalCatalyst => "Chemical Catalyst",
                Item::ChemicalSuperbase => "Chemical Superbase",
                Item::Epinephrine => "Epinephrine",
                Item::EpoxyAdhesive => "Epoxy Adhesive",
                Item::Graphene => "Graphene",
                Item::OxygenicBacteria => "Oxygenic Bacteria",
                Item::PHNeutraliser => "pH Neutraliser",
                Item::RDX => "RDX",
                Item::ViscoelasticPolymer => "Viscoelastic Polymer",

                // Circuits
                Item::CircuitBoard => "Circuit Board",
                Item::CircuitSwitch => "Circuit Switch",
                Item::ElectricalFuse => "Electrical Fuse",
                Item::ElectricalWiring => "Electrical Wiring",
                Item::Electromagnet => "Electromagnet",
                Item::IonBattery => "Ion Battery",
                Item::MetalCoil => "Metal Coil",
                Item::MicroSupercapacitor => "Micro Supercapacitor",
                Item::MicroTransformer => "Micro Transformer",
                Item::Microelectrode => "Microelectrode",
                Item::Motor => "Motor",
                Item::OpticalFibre => "Optical Fibre",

                // Tech
                Item::CarbonFibrePlating => "Carbon Fibre Plating",
                Item::EncryptedMemoryChip => "Encrypted Memory Chip",
                Item::MemoryChip => "Memory Chip",
                Item::MicroHydraulics => "Micro Hydraulics",
                Item::MicroThrusters => "Micro Thrusters",
                Item::OpticalLens => "Micro Thrusters",
                Item::Scrambler => "Optical Lens",
                Item::TitaniumPlating => "Titanium Plating",
                Item::Transmitter => "Scrambler",
                Item::TungstenCarbide => "Titanium Plating",
                Item::WeaponComponent => "Transmitter",

                // Consumables
                Item::EnergyCell => "Energy Cell",
                Item::FragGranade => "Frag Grenade",
                Item::Medkit => "Medkit",
                Item::ShieldDisruptor => "Shield Disruptor",
                Item::ShieldProjector => "Shield Projector",
                Item::EBreach => "E-Breach",

                #[cfg(not(feature = "strict"))]
                Item::Unknown(unknown) => return write!(f, "Unknown item: {}", unknown),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::modules::models::odyssey::item::Item;

    #[test]
    fn item_test_cases_are_parsed_correctly() {
        let test_cases = [
            (
                "$EnhancedInterrogationRecordings_Name;",
                Item::EnhancedInterrogationRecordings,
            ),
            (
                "$WeaponSchematic_Name;",
                Item::WeaponSchematic,
            ),
        ];

        for (case, expected) in test_cases {
            let result = Item::from_str(case);

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected);
        }
    }
}
