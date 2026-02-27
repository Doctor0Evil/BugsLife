use serde::{Deserialize, Serialize};

/// Normalized risk coordinate r_j ∈ [0,1]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RiskCoord(pub f32);

impl RiskCoord {
    #[inline]
    pub fn clamp(self) -> Self {
        RiskCoord(self.0.max(0.0).min(1.0))
    }
}

/// Corridor bands for a single varid (safe/gold/hard in physical units)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorridorBands {
    pub varid: String,      // e.g. "r_gap_rat"
    pub units: String,      // e.g. "mm", "mm/s RMS", "dBA"
    pub safe_lo: f32,
    pub safe_hi: f32,
    pub gold_lo: f32,
    pub gold_hi: f32,
    pub hard_lo: f32,
    pub hard_hi: f32,
    pub weight: f32,        // w_j in residual
    pub lyap_channel: u8,   // optional channel grouping
    pub mandatory: bool,    // true for geometry/vibration core fields
}

/// Decision returned by the safety kernel for a proposed step
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CorridorDecision {
    Ok,
    Derate,
    Stop,
}

/// Core per-node state used by safestep
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BugsLifeDeterrentNode {
    // Identity / context
    pub node_id: String,
    pub region: String,          // e.g. "Phoenix-AZ-2026"
    pub module_type: String,     // e.g. "ApplianceBase", "DoorThreshold"
    pub did: String,             // DID/Bostrom identity

    // Corridor table for this node
    pub corridors: Vec<CorridorBands>,

    // Current normalized coordinates r_j (varid -> RiskCoord)
    pub r: std::collections::HashMap<String, RiskCoord>,

    // Lyapunov residual V_t
    pub residual_v: f32,

    // K/E/R scores for this node (research vs production lanes)
    pub k_knowledge: f32,
    pub e_eco_impact: f32,
    pub r_risk_of_harm: f32,

    // Evidence hex (transaction hash, trial bundle, etc.)
    pub evidence_hex: String,
}
