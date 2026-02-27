use serde_repr::{Deserialize_repr, Serialize_repr};

/// Canonical risk coordinate identifiers for #BugsLife deterrent nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum VarId {
    // Acoustic — safety
    r_noise_human = 1,
    r_noise_pet = 2,
    r_noise_wildlife = 3,

    // Pest-targeted acoustic / vibration
    r_ultra_pest = 10,
    r_vib_pest = 11,

    // Light / optical safety
    r_light_eye = 20,
    r_light_seizure = 21,
    r_laser_class = 22,

    // Odor / chemical
    r_odor_tox = 30,
    r_odor_nuisance = 31,
    r_bioaccumulation = 32,

    // Thermal + structural
    r_thermal_body = 40,
    r_thermal_material = 41,
    r_struct_vib = 42,

    // Combined cocktail / interaction axis
    r_multimodal = 50,
}

impl VarId {
    pub fn as_str(&self) -> &'static str {
        match self {
            VarId::r_noise_human => "r_noise_human",
            VarId::r_noise_pet => "r_noise_pet",
            VarId::r_noise_wildlife => "r_noise_wildlife",
            VarId::r_ultra_pest => "r_ultra_pest",
            VarId::r_vib_pest => "r_vib_pest",
            VarId::r_light_eye => "r_light_eye",
            VarId::r_light_seizure => "r_light_seizure",
            VarId::r_laser_class => "r_laser_class",
            VarId::r_odor_tox => "r_odor_tox",
            VarId::r_odor_nuisance => "r_odor_nuisance",
            VarId::r_bioaccumulation => "r_bioaccumulation",
            VarId::r_thermal_body => "r_thermal_body",
            VarId::r_thermal_material => "r_thermal_material",
            VarId::r_struct_vib => "r_struct_vib",
            VarId::r_multimodal => "r_multimodal",
        }
    }
}
/// Canonical varids for geometry + vibration + joint axis.
/// These names must match CorridorBands.varid and r-keys.

pub const R_GAP_RAT: &str           = "r_gap_rat";
pub const R_GAP_ROACH: &str         = "r_gap_roach";
pub const R_TAPER: &str             = "r_taper";
pub const R_ROUGH: &str             = "r_rough";
pub const R_SERVICE_HUMAN: &str     = "r_service_human";
pub const R_SERVICE_TOOL: &str      = "r_service_tool";

pub const R_STRUCTVIB_HUMAN: &str   = "r_structvib_human";
pub const R_STRUCTVIB_PET: &str     = "r_structvib_pet";
pub const R_STRUCTVIB_BEE: &str     = "r_structvib_bee";   // often fixed to 1 (disallowed)
pub const R_STRUCTVIB_PEST: &str    = "r_structvib_pest";  // research-only

pub const R_NOISE_HUMAN: &str       = "r_noisehuman";
pub const R_NOISE_ANNOY: &str       = "r_noiseannoyance";
pub const R_DIST_FREQ: &str         = "r_disturbancefreq";
pub const R_DIST_DUTY: &str         = "r_disturbanceduty";

pub const R_LIGHT_EYE: &str         = "r_lighteye";
pub const R_LASER_CLASS: &str       = "r_laserclass";
pub const R_THERMAL_BODY: &str      = "r_thermalbody";

pub const R_MULTIMODAL: &str        = "r_multimodal";

/// List of mandatory varids for BugsLifeDeterrentNode.v1 geometry/vibration layer
pub const MANDATORY_VARIDS: &[&str] = &[
    R_GAP_RAT,
    R_GAP_ROACH,
    R_SERVICE_HUMAN,
    R_SERVICE_TOOL,
    R_STRUCTVIB_HUMAN,
    R_STRUCTVIB_PET,
    R_NOISE_HUMAN,
    R_NOISE_ANNOY,
    R_DIST_FREQ,
    R_DIST_DUTY,
    R_MULTIMODAL,
];
