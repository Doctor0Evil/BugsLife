use crate::residual::residual_non_increasing;
use crate::types::{CorridorBands, CorridorDecision, Residual, RiskCoord};
use crate::var_ids::VarId;
use std::collections::HashMap;

/// Check that all mandatory corridors are present ("no corridor, no build").
pub fn corridor_present(
    corridors: &HashMap<VarId, CorridorBands>,
    mandatory: &[VarId],
) -> bool {
    mandatory.iter().all(|id| corridors.contains_key(id))
}

/// Decide whether a proposed step is Ok / Derate / Stop.
///
/// - If any r_j >= 1.0 for safety coordinates, return Stop.
/// - Else, if residual increases outside safe interior, return Derate.
/// - Else, Ok.
pub fn safe_step(
    prev_residual: Residual,
    next_residual: Residual,
    coords: &HashMap<VarId, RiskCoord>,
    safety_ids: &[VarId],
    eps: f32,
) -> CorridorDecision {
    // Hard constraints: any r_j >= 1.0 for safety coordinates.
    for id in safety_ids {
        if let Some(rc) = coords.get(id) {
            if rc.r >= 1.0 {
                return CorridorDecision::Stop;
            }
        }
    }

    // Lyapunov constraint: non-increasing residual outside safe interior.
    if !residual_non_increasing(prev_residual, next_residual, eps) {
        CorridorDecision::Derate
    } else {
        CorridorDecision::Ok
    }
}
