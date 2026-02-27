use serde::{Deserialize, Serialize};

/// Normalized risk coordinate r \in [0,1] with optional uncertainty and weight.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RiskCoord {
    /// Normalized risk position in [0,1].
    pub r: f32,
    /// One-sigma uncertainty on r (0 means no estimate).
    pub sigma: f32,
    /// Weight w_j used in Lyapunov residuals.
    pub weight: f32,
}

impl RiskCoord {
    pub fn new(r: f32, sigma: f32, weight: f32) -> Self {
        let r_clamped = r.clamp(0.0, 1.0);
        let sigma_clamped = sigma.max(0.0);
        let weight_clamped = weight.max(0.0);
        RiskCoord {
            r: r_clamped,
            sigma: sigma_clamped,
            weight: weight_clamped,
        }
    }
}

/// Single variable corridor band specification for one VarId.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorridorBands {
    /// Identifier for the risk coordinate (noise, light, odor, etc.).
    pub var_id: crate::var_ids::VarId,
    /// Engineering unit string (e.g., "dBA", "mg_m3", "lux").
    pub unit: String,
    /// Safe boundary in normalized [0,1] space.
    pub safe: f32,
    /// Gold (preferred) boundary in normalized [0,1] space.
    pub gold: f32,
    /// Hard limit boundary in normalized [0,1] space (must not be crossed).
    pub hard: f32,
    /// Weight for Lyapunov residual.
    pub weight: f32,
    /// Channel index in residual vector, useful for diagnostics.
    pub lyap_channel: u8,
    /// Mandatory corridor flag (no corridor, no build).
    pub mandatory: bool,
}

impl CorridorBands {
    pub fn validate(&self) -> Result<(), String> {
        if !(0.0 <= self.safe && self.safe <= self.gold && self.gold <= self.hard && self.hard <= 1.0) {
            return Err(format!(
                "Invalid corridor ordering for {:?}: safe={}, gold={}, hard={}",
                self.var_id, self.safe, self.gold, self.hard
            ));
        }
        if self.weight < 0.0 {
            return Err(format!(
                "Negative weight not allowed for {:?}: {}",
                self.var_id, self.weight
            ));
        }
        Ok(())
    }
}

/// Lyapunov-style residual V_t and optional uncertainty U_t.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Residual {
    /// V_t = Σ w_j r_j, aggregated risk.
    pub v_t: f32,
    /// Optional uncertainty channel, kept for compatibility with other ecosafety kernels.
    pub u_t: f32,
}

impl Residual {
    pub fn new(v_t: f32, u_t: f32) -> Self {
        Residual { v_t, u_t }
    }
}

/// Decision emitted by the safety kernel for a proposed step.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CorridorDecision {
    /// Step is inside corridors and non-increasing residual constraints.
    Ok,
    /// Step violates Lyapunov monotonicity but not hard limits; requires derating.
    Derate,
    /// Step would cross at least one hard corridor; must not be applied.
    Stop,
}
