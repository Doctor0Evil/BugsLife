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
