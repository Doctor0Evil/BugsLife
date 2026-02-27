use serde::{Deserialize, Serialize};

/// Raw sensor inputs for one safety-tick of a BugsLife node.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BugsLifeEnvInputs {
    // Acoustic (SPL and spectra)
    pub spl_db_a: f32,
    pub spl_db_z: f32,
    pub spl_ultra_db: f32,
    pub noise_duty_cycle: f32,

    // Light / optical
    pub illuminance_lux: f32,
    pub flicker_hz: f32,
    pub laser_class_raw: u8,
    pub laser_irradiance_mw_cm2: f32,
    pub laser_exposure_ms: f32,

    // Odor / chemical
    pub voc_mg_m3: f32,
    pub odor_units: f32,
    pub bioaerosol_idx: f32,
    pub residual_mass_idx: f32,

    // Thermal
    pub delta_t_body_k: f32,
    pub delta_t_material_k: f32,

    // Structural vibration
    pub struct_vib_mm_s: f32,
}
