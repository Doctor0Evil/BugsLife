use crate::types::{Residual, RiskCoord};
use crate::var_ids::VarId;
use std::collections::HashMap;

/// Compute V_t = Σ w_j r_j given risk coordinates.
pub fn compute_residual(coords: &HashMap<VarId, RiskCoord>) -> Residual {
    let mut v_t = 0.0_f32;
    for rc in coords.values() {
        v_t += rc.weight * rc.r;
    }
    Residual { v_t, u_t: 0.0 }
}

/// Helper to merge previous residual for monotonic checks if needed.
pub fn residual_non_increasing(prev: Residual, next: Residual, eps: f32) -> bool {
    next.v_t <= prev.v_t + eps
}
