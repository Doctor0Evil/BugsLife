use crate::contracts::safe_step;
use crate::corridors::CorridorSet;
use crate::env_inputs::BugsLifeEnvInputs;
use crate::residual::compute_residual;
use crate::types::{CorridorDecision, Residual, RiskCoord};
use crate::var_ids::VarId;
use std::collections::HashMap;

/// Safety kernel trait for BugsLife nodes.
pub trait BugsLifeSafetyKernel {
    /// Map raw environment + proposed actuation to normalized risk coordinates.
    fn compute_coords(
        &self,
        env: &BugsLifeEnvInputs,
        act: &crate::actuation::BugsLifeActuation,
    ) -> HashMap<VarId, RiskCoord>;

    /// Access the corridor set used by this kernel.
    fn corridors(&self) -> &CorridorSet;

    /// Safety check wrapper: produce decision and next residual state.
    fn check_step(
        &self,
        env: &BugsLifeEnvInputs,
        act: &crate::actuation::BugsLifeActuation,
        prev_residual: Residual,
    ) -> (CorridorDecision, Residual) {
        let coords = self.compute_coords(env, act);
        let next_residual = compute_residual(&coords);

        // Safety coordinates: those that protect humans, pets, wildlife, infrastructure.
        let safety_ids: &[VarId] = &[
            VarId::r_noise_human,
            VarId::r_noise_pet,
            VarId::r_noise_wildlife,
            VarId::r_light_eye,
            VarId::r_light_seizure,
            VarId::r_laser_class,
            VarId::r_odor_tox,
            VarId::r_bioaccumulation,
            VarId::r_thermal_body,
            VarId::r_thermal_material,
            VarId::r_struct_vib,
            VarId::r_multimodal,
        ];

        let decision = safe_step(prev_residual, next_residual, &coords, safety_ids, 1e-4);
        (decision, next_residual)
    }
}

/// Default Phoenix 2026 kernel using corridor bands loaded from CSV or config.
pub struct DefaultBugsLifeKernelPhoenix2026 {
    pub corridors: CorridorSet,
}

impl DefaultBugsLifeKernelPhoenix2026 {
    pub fn new(corridors: CorridorSet) -> Self {
        DefaultBugsLifeKernelPhoenix2026 { corridors }
    }
}

impl BugsLifeSafetyKernel for DefaultBugsLifeKernelPhoenix2026 {
    fn compute_coords(
        &self,
        env: &BugsLifeEnvInputs,
        act: &crate::actuation::BugsLifeActuation,
    ) -> HashMap<VarId, RiskCoord> {
        let mut map = HashMap::new();

        // These mappings are intentionally simple and conservative;
        // they can be refined as corridor calibration improves.

        // r_noise_human: SPL_A normalized vs corridor hard limit.
        let bands = &self.corridors.bands[&VarId::r_noise_human];
        // For now, assume corridor.hard corresponds to 100 dBA and safe ~ 0 dBA baseline.
        let r_noise_human = (env.spl_db_a / 100.0).clamp(0.0, 1.0);
        map.insert(
            VarId::r_noise_human,
            RiskCoord::new(r_noise_human, 0.05, bands.weight),
        );

        // r_noise_pet and r_noise_wildlife: reuse SPL_Z for conservative mapping.
        let bands_pet = &self.corridors.bands[&VarId::r_noise_pet];
        let r_noise_pet = (env.spl_db_z / 100.0).clamp(0.0, 1.0);
        map.insert(
            VarId::r_noise_pet,
            RiskCoord::new(r_noise_pet, 0.05, bands_pet.weight),
        );

        let bands_wild = &self.corridors.bands[&VarId::r_noise_wildlife];
        let r_noise_wild = (env.spl_db_z / 100.0).clamp(0.0, 1.0);
        map.insert(
            VarId::r_noise_wildlife,
            RiskCoord::new(r_noise_wild, 0.05, bands_wild.weight),
        );

        // r_ultra_pest: pest aversion band; we expect higher SPL_ultra at higher intensity_pct.
        let bands_ultra = &self.corridors.bands[&VarId::r_ultra_pest];
        let r_ultra = ((env.spl_ultra_db / 120.0) * act.intensity_pct).clamp(0.0, 1.0);
        map.insert(
            VarId::r_ultra_pest,
            RiskCoord::new(r_ultra, 0.10, bands_ultra.weight),
        );

        // r_vib_pest
        let bands_vib_pest = &self.corridors.bands[&VarId::r_vib_pest];
        let r_vib_pest = (env.struct_vib_mm_s / 20.0).clamp(0.0, 1.0);
        map.insert(
            VarId::r_vib_pest,
            RiskCoord::new(r_vib_pest, 0.10, bands_vib_pest.weight),
        );

        // r_light_eye: glare vs lux (simplified).
        let bands_light_eye = &self.corridors.bands[&VarId::r_light_eye];
        let r_light_eye = (env.illuminance_lux / 2000.0).clamp(0.0, 1.0);
        map.insert(
            VarId::r_light_eye,
            RiskCoord::new(r_light_eye, 0.05, bands_light_eye.weight),
        );

        // r_light_seizure: flicker risk vs frequency and duty cycle.
        let bands_light_seiz = &self.corridors.bands[&VarId::r_light_seizure];
        let r_light_seizure = if env.flicker_hz >= 3.0 && env.flicker_hz <= 70.0 {
            0.7
        } else {
            0.1
        };
        map.insert(
            VarId::r_light_seizure,
            RiskCoord::new(r_light_seizure, 0.10, bands_light_seiz.weight),
        );

        // r_laser_class: normalized from raw class.
        let bands_laser = &self.corridors.bands[&VarId::r_laser_class];
        let base = match env.laser_class_raw {
            0 | 1 => 0.1,
            2 => 0.3,
            3 => 0.7,
            _ => 1.0,
        };
        map.insert(
            VarId::r_laser_class,
            RiskCoord::new(base, 0.05, bands_laser.weight),
        );

        // r_odor_tox: VOC mg/m3 vs a conservative 1.0 mg/m3 threshold.
        let bands_odor_tox = &self.corridors.bands[&VarId::r_odor_tox];
        let r_odor_tox = (env.voc_mg_m3 / 1.0).clamp(0.0, 1.0);
        map.insert(
            VarId::r_odor_tox,
            RiskCoord::new(r_odor_tox, 0.10, bands_odor_tox.weight),
        );

        // r_odor_nuisance: odor units / 10.
        let bands_odor_nuis = &self.corridors.bands[&VarId::r_odor_nuisance];
        let r_odor_nuisance = (env.odor_units / 10.0).clamp(0.0, 1.0);
        map.insert(
            VarId::r_odor_nuisance,
            RiskCoord::new(r_odor_nuisance, 0.10, bands_odor_nuis.weight),
        );

        // r_bioaccumulation: use residual_mass_idx as proxy.
        let bands_bio = &self.corridors.bands[&VarId::r_bioaccumulation];
        let r_bio = env.residual_mass_idx.clamp(0.0, 1.0);
        map.insert(
            VarId::r_bioaccumulation,
            RiskCoord::new(r_bio, 0.10, bands_bio.weight),
        );

        // Thermal
        let bands_th_body = &self.corridors.bands[&VarId::r_thermal_body];
        let r_th_body = (env.delta_t_body_k / 10.0).clamp(0.0, 1.0);
        map.insert(
            VarId::r_thermal_body,
            RiskCoord::new(r_th_body, 0.05, bands_th_body.weight),
        );

        let bands_th_mat = &self.corridors.bands[&VarId::r_thermal_material];
        let r_th_mat = (env.delta_t_material_k / 30.0).clamp(0.0, 1.0);
        map.insert(
            VarId::r_thermal_material,
            RiskCoord::new(r_th_mat, 0.05, bands_th_mat.weight),
        );

        // Structural vibration
        let bands_struct = &self.corridors.bands[&VarId::r_struct_vib];
        let r_struct = (env.struct_vib_mm_s / 30.0).clamp(0.0, 1.0);
        map.insert(
            VarId::r_struct_vib,
            RiskCoord::new(r_struct, 0.10, bands_struct.weight),
        );

        // r_multimodal: simple max of core safety channels for now.
        let bands_multi = &self.corridors.bands[&VarId::r_multimodal];
        let mut r_multi = 0.0_f32;
        for (id, rc) in &map {
            match id {
                VarId::r_noise_human
                | VarId::r_noise_pet
                | VarId::r_noise_wildlife
                | VarId::r_light_eye
                | VarId::r_light_seizure
                | VarId::r_laser_class
                | VarId::r_odor_tox
                | VarId::r_thermal_body
                | VarId::r_thermal_material
                | VarId::r_struct_vib => {
                    if rc.r > r_multi {
                        r_multi = rc.r;
                    }
                }
                _ => {}
            }
        }
        map.insert(
            VarId::r_multimodal,
            RiskCoord::new(r_multi, 0.10, bands_multi.weight),
        );

        map
    }

    fn corridors(&self) -> &crate::corridors::CorridorSet {
        &self.corridors
    }
}
