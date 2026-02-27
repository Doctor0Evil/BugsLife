use crate::types::{CorridorBands, RiskCoord};
use crate::varids::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Normalize a physical measurement x into r_j ∈ [0,1] using corridor bands.
/// Piecewise-linear: r = 0 in the safe band, r = 1 at the hard edge;
/// values beyond hard are clamped.
pub fn normalize_metric(x: f32, bands: &CorridorBands) -> RiskCoord {
    // Assumes "higher is worse"; for inverted metrics, pre-transform x
    // or extend this function.
    let (safe_hi, gold_hi, hard_hi) = (bands.safe_hi, bands.gold_hi, bands.hard_hi);

    let r = if x <= safe_hi {
        0.0
    } else if x >= hard_hi {
        1.0
    } else if x <= gold_hi {
        // Map safe_hi..gold_hi → 0..0.5
        0.5 * (x - safe_hi) / (gold_hi - safe_hi + f32::EPSILON)
    } else {
        // Map gold_hi..hard_hi → 0.5..1.0
        0.5 + 0.5 * (x - gold_hi) / (hard_hi - gold_hi + f32::EPSILON)
    };

    RiskCoord(r).clamp()
}

/// Compute r_multimodal = max(...) from current coordinates.
pub fn compute_r_multimodal(
    r: &HashMap<String, RiskCoord>,
) -> RiskCoord {
    let keys = [
        R_NOISE_HUMAN,
        R_NOISE_ANNOY,
        R_STRUCTVIB_HUMAN,
        R_STRUCTVIB_PET,
        R_LIGHT_EYE,
        R_LASER_CLASS,
        R_THERMAL_BODY,
        R_GAP_RAT,
        R_GAP_ROACH,
        R_SERVICE_HUMAN,
        R_SERVICE_TOOL,
    ];

    let mut max_r = 0.0f32;
    for key in keys.iter() {
        if let Some(rc) = r.get(*key) {
            if rc.0 > max_r {
                max_r = rc.0;
            }
        }
    }
    RiskCoord(max_r)
}

/// Collection of corridor bands for a single BugsLife node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorridorSet {
    pub bands: HashMap<String, CorridorBands>,
}

#[derive(Debug, Error)]
pub enum CorridorValidationError {
    #[error("missing mandatory corridor for {0}")]
    MissingMandatory(&'static str),
    #[error("invalid corridor bands: {0}")]
    InvalidBands(String),
}

impl CorridorSet {
    pub fn new(bands: HashMap<String, CorridorBands>) -> Self {
        CorridorSet { bands }
    }

    /// Validate all bands and ensure all mandatory entries are present.
    pub fn validate(&self) -> Result<(), CorridorValidationError> {
        // Mandatory varids for this BugsLife layer
        let mandatory_ids: &[&str] = &[
            R_NOISE_HUMAN,
            R_NOISE_PET,
            R_NOISE_WILDLIFE,
            R_ULTRA_PEST,
            R_VIB_PEST,
            R_LIGHT_EYE,
            R_LIGHT_SEIZURE,
            R_LASER_CLASS,
            R_ODOR_TOX,
            R_ODOR_NUISANCE,
            R_THERMAL_BODY,
            R_THERMAL_MATERIAL,
            R_STRUCT_VIB,
            R_MULTIMODAL,
        ];

        for &id in mandatory_ids {
            if !self.bands.contains_key(id) {
                return Err(CorridorValidationError::MissingMandatory(id));
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
