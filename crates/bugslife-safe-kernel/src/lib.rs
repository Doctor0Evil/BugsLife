#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// Dimensionless normalized risk coordinate in [0,1].
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Risk(pub f32);

impl Risk {
    pub fn zero() -> Self { Risk(0.0) }
    pub fn one()  -> Self { Risk(1.0) }
    pub fn clamp01(self) -> Self { Risk(self.0.clamp(0.0, 1.0)) }
}

/// Canonical BugsLife risk coordinates (expand as needed).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum VarId {
    RNoiseHuman,
    RNoiseRodent,
    RThermalBody,
    RVibrationStruct,
    RLightFlicker,
    ROdorTox,
    RSoilBiota,
    RBee,
    RBird,
}

/// One row of the corridor bands table.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CorridorRow {
    pub varid: VarId,
    pub safe: f32,
    pub gold: f32,
    pub hard: f32,
    pub weight: f32,
    pub mandatory: bool,
}

/// Table of corridor bands for a node/firmware image.
pub type CorridorBands = Vec<CorridorRow>;

/// Full risk state at one timestep.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RiskState {
    pub rnoise_human: Risk,
    pub rnoise_rodent: Risk,
    pub rthermal_body: Risk,
    pub rvibration_struct: Risk,
    pub rlight_flicker: Risk,
    pub rodor_tox: Risk,
    pub rsoil_biota: Risk,
    pub rbee: Risk,
    pub rbird: Risk,
}

impl RiskState {
    pub fn maxcoord(&self) -> Risk {
        let vals = [
            self.rnoise_human.0,
            self.rnoise_rodent.0,
            self.rthermal_body.0,
            self.rvibration_struct.0,
            self.rlight_flicker.0,
            self.rodor_tox.0,
            self.rsoil_biota.0,
            self.rbee.0,
            self.rbird.0,
        ];
        Risk(vals.iter().cloned().fold(0.0_f32, f32::max))
    }
}

/// Lyapunov residual weights, K/E/R-style.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResidualWeights {
    pub w_noise_human: f32,
    pub w_noise_rodent: f32,
    pub w_thermal_body: f32,
    pub w_vibration_struct: f32,
    pub w_light_flicker: f32,
    pub w_odor_tox: f32,
    pub w_soil_biota: f32,
    pub w_bee: f32,
    pub w_bird: f32,
}

impl ResidualWeights {
    pub fn residual(&self, s: &RiskState) -> f32 {
        let v =
            self.w_noise_human * s.rnoise_human.0 * s.rnoise_human.0 +
            self.w_noise_rodent * s.rnoise_rodent.0 * s.rnoise_rodent.0 +
            self.w_thermal_body * s.rthermal_body.0 * s.rthermal_body.0 +
            self.w_vibration_struct * s.rvibration_struct.0 * s.rvibration_struct.0 +
            self.w_light_flicker * s.rlight_flicker.0 * s.rlight_flicker.0 +
            self.w_odor_tox * s.rodor_tox.0 * s.rodor_tox.0 +
            self.w_soil_biota * s.rsoil_biota.0 * s.rsoil_biota.0 +
            self.w_bee * s.rbee.0 * s.rbee.0 +
            self.w_bird * s.rbird.0 * s.rbird.0;
        if v.is_nan() { 0.0 } else { v }
    }
}

/// Ecosafety shell for BugsLife; all invariants live here.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EcoShell {
    /// Max allowed risk for any coordinate (e.g. 1.0).
    pub hardcap: f32,
    /// Bee corridor ceiling (e.g. 0.10 for BeeRoH).
    pub bee_max: f32,
    /// Bird corridor ceiling (e.g. 0.10 for avian stress).
    pub bird_max: f32,
    /// Soil biota ceiling.
    pub soil_max: f32,
    /// Residual "safe interior" threshold.
    pub safe_residual: f32,
}

impl EcoShell {
    pub fn validate(&self, rs: &RiskState, vt: f32) -> bool {
        if rs.maxcoord().0 > self.hardcap { return false; }
        if rs.rbee.0       > self.bee_max { return false; }
        if rs.rbird.0      > self.bird_max { return false; }
        if rs.rsoil_biota.0> self.soil_max { return false; }
        if vt.is_nan() || vt < 0.0 { return false; }
        true
    }
}

/// Safe-step decision for deterrent profiles.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SafeDecision {
    RejectHard,
    RejectLyapunov,
    Accept,
}

/// Input to the BugsLife safety kernel.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SafeStepInput {
    pub prev: RiskState,
    pub next: RiskState,
    pub weights: ResidualWeights,
    pub shell: EcoShell,
}

/// Core contract: no corridor violation, no Lyapunov increase outside safe interior.
pub fn safestep(input: &SafeStepInput) -> SafeDecision {
    let v_prev = input.weights.residual(&input.prev);
    let v_next = input.weights.residual(&input.next);

    // 1. Hard ecosafety shell: no corridor, no deployment.
    if !input.shell.validate(&input.next, v_next) {
        return SafeDecision::RejectHard;
    }

    // 2. Lyapunov residual non-increase outside safe interior.
    if v_prev > input.shell.safe_residual + f32::EPSILON {
        if v_next > v_prev + 1.0e-6 {
            return SafeDecision::RejectLyapunov;
        }
    }

    SafeDecision::Accept
}

/// Piecewise linear normalizer for a single coordinate.
pub fn normalize_piecewise(x: f32, safe: f32, hard: f32) -> Risk {
    if x <= safe {
        Risk::zero()
    } else if x >= hard {
        Risk::one()
    } else {
        let r = (x - safe) / (hard - safe);
        Risk(r).clamp01()
    }
}

/// Trait that any BugsLife controller must implement.
pub trait BugsLifeSafetyKernel {
    /// Given an intent and current risk state, compute a safe actuation profile or reject.
    fn safestep_intent(
        &self,
        current: &RiskState,
        intent: &DeterrentIntent,
    ) -> Result<DeterrentActuation, SafeDecision>;
}

/// High-level deterrent intent (abstract PDSS profile).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeterrentIntent {
    pub profile_id: String,
    pub intensity_pct: f32,
    pub duration_s: f32,
}

/// Opaque actuation profile (device-specific).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeterrentActuation {
    // e.g. PWM duty cycles, IR codes, relay timings, all hidden
    // behind the adapter layer so raw commands never bypass safestep.
    pub payload: Vec<u8>,
}
