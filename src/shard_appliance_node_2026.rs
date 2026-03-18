// src/shard_appliance_node_2026.rs
// ApplianceNodeShard2026v1 Rust binding (ecosafety2026.0)

use crate::contracts::{CorridorBands, RiskCoord, Residual, CorridorDecision, safestep};

#[derive(Clone, Debug)]
pub struct ApplianceCorridors {
    pub noise: CorridorBands,   // units: dBA
    pub dT:    CorridorBands,   // units: °C surface rise
    pub vib:   CorridorBands,   // units: normalized (dimensionless)
}

#[derive(Clone, Debug)]
pub struct ApplianceRisk {
    pub rnoise:   RiskCoord,
    pub rthermal: RiskCoord,
    pub rvib:     RiskCoord,
    pub vt:       Residual,
}

#[derive(Clone, Debug)]
pub struct ApplianceNodeShard2026v1 {
    // identity / location
    pub nodeid:                  String,
    pub region:                  String,
    pub houselabel:              String,
    pub roomlabel:               String,

    // metadata
    pub applianceclass:          String,
    pub adaptertype:             String,
    pub ratedpower_w:            f64,
    pub ratednoise_dba:          f64,

    // corridors (physical bands)
    pub corridors:               ApplianceCorridors,

    // current normalized risks
    pub rnoise:                  f64,
    pub rthermal:                f64,
    pub rvib:                    f64,

    // residuals
    pub residual_vt:             f64,
    pub residual_vt_prev:        f64,
    pub vt_ok:                   bool,

    // governance scores
    pub knowledgefactor01:       f64,
    pub ecoimpact01:             f64,
    pub riskofharm01:            f64,

    // PDSS usage
    pub pdss_duty_fraction:      f64,
    pub pdss_energy_kwh_per_day: f64,
    pub baseline_pesticide_g_per_day_avoided: f64,

    // anchors
    pub bostrom_tx_hex:          String,
    pub evidencehex:             String,
    pub epoch_grammar:           String,
    pub lane:                    String,
    pub kerdeployable:           bool,

    pub notes:                   String,
}

/// Piecewise-linear normalization from physical metric to r_j in [0,1],
/// shared with other ecosafety kernels.
fn to_r(measured: f64, bands: &CorridorBands) -> RiskCoord {
    let r_value = if measured <= bands.safe {
        0.0
    } else if measured >= bands.hard {
        1.0
    } else {
        (measured - bands.safe) / (bands.hard - bands.safe)
    };

    RiskCoord {
        value: r_value,
        sigma: 0.0,
        bands: bands.clone(),
    }
}

/// Compute normalized risks and Lyapunov residual for an appliance node,
/// given current physical telemetry.
pub fn normalize_appliance(
    noise_dba: f64,
    dT_c: f64,
    vib_norm: f64,
    corridors: &ApplianceCorridors,
) -> ApplianceRisk {
    let rnoise   = to_r(noise_dba, &corridors.noise);
    let rthermal = to_r(dT_c,      &corridors.dT);
    let rvib     = to_r(vib_norm,  &corridors.vib);

    let coords = vec![rnoise.clone(), rthermal.clone(), rvib.clone()];

    // Residual kernel: V_t = Σ w_j * r_j^2 (or linear, depending on contracts)
    let vt_val: f64 = coords
        .iter()
        .map(|rc| rc.bands.weight_w * rc.value * rc.value)
        .sum();

    let vt = Residual { vt: vt_val, coords };

    ApplianceRisk {
        rnoise,
        rthermal,
        rvib,
        vt,
    }
}

/// Safestep wrapper for PDSS control: enforces hard corridors and V_{t+1} <= V_t
pub fn safestep_appliance_tick(
    prev_vt: &Residual,
    next_risk: &ApplianceRisk,
) -> CorridorDecision {
    // Reuse canonical safestep from your ecosafety spine.
    safestep(prev_vt.clone(), next_risk.vt.clone())
}

/// Update shard from telemetry + safestep result.
pub fn update_shard_from_tick(
    shard: &mut ApplianceNodeShard2026v1,
    telemetry_noise_dba: f64,
    telemetry_dT_c: f64,
    telemetry_vib_norm: f64,
) {
    let corridors = shard.corridors.clone();
    let prev_residual = Residual {
        vt:    shard.residual_vt,
        coords: vec![], // can be left empty if not needed for governance at this level
    };

    let risk = normalize_appliance(
        telemetry_noise_dba,
        telemetry_dT_c,
        telemetry_vib_norm,
        &corridors,
    );

    let decision = safestep_appliance_tick(&prev_residual, &risk);

    shard.rnoise       = risk.rnoise.value;
    shard.rthermal     = risk.rthermal.value;
    shard.rvib         = risk.rvib.value;
    shard.residual_vt_prev = shard.residual_vt;
    shard.residual_vt      = risk.vt.vt;
    shard.vt_ok            = matches!(decision, CorridorDecision::Ok);

    // Example: adjust K/E/R heuristically; in production this should be
    // driven by city-scale governance logic, not local heuristics.
    if shard.vt_ok && shard.riskofharm01 > 0.0 {
        shard.knowledgefactor01 = (shard.knowledgefactor01 + 0.01).min(1.0);
        shard.ecoimpact01       = (shard.ecoimpact01 + 0.005).min(1.0);
        shard.riskofharm01      = (shard.riskofharm01 * 0.99).max(0.0);
    } else if !shard.vt_ok {
        shard.riskofharm01 = (shard.riskofharm01 + 0.02).min(1.0);
    }

    shard.kerdeployable =
        shard.knowledgefactor01 >= 0.90 &&
        shard.ecoimpact01       >= 0.90 &&
        shard.riskofharm01     <= 0.13 &&
        shard.vt_ok;
}
