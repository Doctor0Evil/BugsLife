#![forbid(unsafe_code)]

use std::time::SystemTime;

/// Bostrom / DID metadata for full auditability.
#[derive(Clone, Debug)]
pub struct EvidenceMeta {
    /// Primary bostrom address of the operator or shard signer.
    pub bostrom_address: String,
    /// Optional alternate / secure address (e.g., RT-monitored).
    pub alt_address: Option<String>,
    /// Hex-stamp tying this record to a research shard or proof string.
    pub hex_stamp: String,
    /// Coarse geocell or facility identifier (e.g., "PHX-RES-01").
    pub location_cell: String,
    /// Timestamp in system time; for shards this is later normalized to UTC.
    pub timestamp: SystemTime,
}

/// Location class where the method is used.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LocationType {
    Home,
    Restaurant,
    Farm,
    Warehouse,
    Hospital,
    Other,
}

/// High-level pest category; extensible as needed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PestSpecies {
    Rodent,
    Cockroach,
    Fly,
    Mosquito,
    Termite,
    Ant,
    StoredProductInsect,
    Other,
}

/// Categorical effectiveness band recorded by the operator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EffectivenessBand {
    Low,
    Medium,
    High,
}

/// Qualitative side-effects observed for a method deployment.
#[derive(Clone, Debug, Default)]
pub struct SideEffects {
    /// Non-target animals captured or harmed (counts per deployment).
    pub non_target_kill_count: u32,
    /// Any pet incident (injury, stress, vet visit).
    pub pet_incident: bool,
    /// Any wildlife incident (protected or unprotected).
    pub wildlife_incident: bool,
    /// Physical injury to operator or bystander.
    pub human_injury: bool,
    /// Qualitative waste class: "low", "moderate", "high", optionally with notes.
    pub waste_burden: String,
    /// Notes on air-quality, odors, or indoor air complaints.
    pub air_quality_concern: bool,
}

/// Tags describing where humans and animals are present.
#[derive(Clone, Debug, Default)]
pub struct ProximityTags {
    pub children_present: bool,
    pub pets_present: bool,
    pub livestock_present: bool,
    pub wildlife_corridor: bool,
}

/// Physical / behavioral control families.
/// Explicitly excludes chemicals, toxins, or pathogens.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControlFamily {
    Exclusion,          // sealing, screening, door sweeps
    Sanitation,         // food/waste/moisture control
    MechanicalKill,     // snap traps, multi-capture traps
    LiveCapture,        // monitored live traps with release protocol
    HabitatChange,      // habitat reduction, harborage removal
    PredatorSupport,    // bat boxes, owl boxes, raptor perches
    MonitoringOnly,     // sticky monitors, sensors, cameras
}

/// Lure type restricted to food-grade / non-toxic materials.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LureType {
    None,
    FoodGradeBait,
    FoodWasteAttractant,
    SyntheticNonToxicLure,
}

/// Construction for exclusion / proofing.
#[derive(Clone, Debug, Default)]
pub struct ExclusionDetails {
    pub sealed_cracks: bool,
    pub door_sweeps_installed: bool,
    pub vents_screened: bool,
    pub pipe_entries_sealed: bool,
    pub notes: Option<String>,
}

/// Sanitation / hygiene context.
#[derive(Clone, Debug, Default)]
pub struct HygieneContext {
    pub food_left_out: bool,
    pub open_garbage: bool,
    pub standing_water: bool,
    pub organic_debris: bool,
    pub cleaning_frequency_per_week: u8,
}

/// Core definition of a non-toxic control method.
#[derive(Clone, Debug)]
pub struct ControlMethod {
    pub id: String,
    pub family: ControlFamily,
    pub trap_type: Option<String>,
    pub lure_type: LureType,
    pub exclusion: Option<ExclusionDetails>,
    /// True if disposable electronics / batteries are involved.
    pub uses_disposable_electronics: bool,
    /// True if persistent plastics are generated (non-biodegradable housings, liners, etc.).
    pub generates_persistent_plastic: bool,
    pub notes: Option<String>,
}

/// Context of a pest problem where the method is deployed.
#[derive(Clone, Debug)]
pub struct PestContext {
    pub location_type: LocationType,
    pub pest: PestSpecies,
    pub proximity: ProximityTags,
    pub hygiene: HygieneContext,
    /// Building envelope headline data.
    pub building_has_gaps: bool,
    pub moisture_high: bool,
    pub food_waste_available: bool,
}

/// Outcome log for a single deployment instance of a method.
#[derive(Clone, Debug)]
pub struct OutcomeLog {
    pub method_id: String,
    pub context: PestContext,
    pub effectiveness: EffectivenessBand,
    pub side_effects: SideEffects,
    /// Number of target pests removed (captures or kills) in this run.
    pub target_count: u32,
    /// Duration of observation window in days.
    pub observation_days: u32,
    pub meta: EvidenceMeta,
}

/// Normalized risk coordinates for a method under a given corpus of logs.
/// All values are in [0, 1] as in your corridor grammar.
#[derive(Clone, Debug, Default)]
pub struct RiskCoordinates {
    pub r_pets: f64,
    pub r_wildlife: f64,
    pub r_waste: f64,
    pub r_air: f64,
    pub r_human_injury: f64,
}

/// Aggregated K/E/R scores for a method.
#[derive(Clone, Debug)]
pub struct KerScore {
    /// Knowledge-factor K in [0,1].
    pub k: f64,
    /// Eco-impact value E in [0,1].
    pub e: f64,
    /// Risk-of-harm R in [0,1].
    pub r: f64,
    /// Normalized risk coordinates for corridors.
    pub coords: RiskCoordinates,
    /// Hard corridor flag: true if any protected corridor reached 1.0.
    pub hard_violation: bool,
}
