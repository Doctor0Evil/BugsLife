use crate::types::CorridorBands;
use crate::var_ids::VarId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Collection of corridor bands for a single BugsLife node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorridorSet {
    pub bands: HashMap<VarId, CorridorBands>,
}

#[derive(Debug, Error)]
pub enum CorridorValidationError {
    #[error("missing mandatory corridor for {0}")]
    MissingMandatory(&'static str),
    #[error("invalid corridor bands: {0}")]
    InvalidBands(String),
}

impl CorridorSet {
    pub fn new(bands: HashMap<VarId, CorridorBands>) -> Self {
        CorridorSet { bands }
    }

    /// Validate all bands and ensure all mandatory entries are present.
    pub fn validate(&self) -> Result<(), CorridorValidationError> {
        use VarId::*;

        let mandatory_ids: &[VarId] = &[
            r_noise_human,
            r_noise_pet,
            r_noise_wildlife,
            r_ultra_pest,
            r_vib_pest,
            r_light_eye,
            r_light_seizure,
            r_laser_class,
            r_odor_tox,
            r_odor_nuisance,
            r_thermal_body,
            r_thermal_material,
            r_struct_vib,
            r_multimodal,
        ];

        for id in mandatory_ids {
            if !self.bands.contains_key(id) {
                return Err(CorridorValidationError::MissingMandatory(id.as_str()));
            }
        }

        for bands in self.bands.values() {
            if let Err(e) = bands.validate() {
                return Err(CorridorValidationError::InvalidBands(e));
            }
        }

        Ok(())
    }
}
