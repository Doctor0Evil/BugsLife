// Core shard schema + residual kernel for non-lethal pest deterrent deployments.

use serde::{Deserialize, Serialize};

/// Normalized risk triple for one corridor coordinate.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RiskCoord {
    /// Normalized risk coordinate r_j in [0.0, 1.0].
    pub r: f32,
    /// Uncertainty (sigma-like) in [0.0, 1.0].
    pub sigma: f32,
    /// Weight w_j >= 0.0 for residual aggregation.
    pub weight: f32,
}

impl RiskCoord {
    pub fn clipped(mut self) -> Self {
        self.r = self.r.clamp(0.0, 1.0);
        self.sigma = self.sigma.clamp(0.0, 1.0);
        self.weight = self.weight.max(0.0);
        self
    }
}

/// K/E/R triad for a BugsLife deployment.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct KER {
    /// Knowledge-factor in [0.0, 1.0].
    pub k: f32,
    /// Eco-impact value in [0.0, 1.0].
    pub e: f32,
    /// Risk-of-harm in [0.0, 1.0].
    pub r: f32,
}

impl KER {
    pub fn clipped(mut self) -> Self {
        self.k = self.k.clamp(0.0, 1.0);
        self.e = self.e.clamp(0.0, 1.0);
        self.r = self.r.clamp(0.0, 1.0);
        self
    }
}

/// BugsLife pest-deterrent deployment shard.
/// Version: BugsLifePestShard.v1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BugsLifePestShard {
    /// Shard identifier (DID/bostrom hash or UUID).
    pub shard_id: String,
    /// Site label (building, block, facility).
    pub site_id: String,
    /// ISO-8601 timestamp for this state snapshot.
    pub timestamp_utc: String,

    // --- Risk coordinates: all normalized [0.0, 1.0] ---

    /// Acoustic level vs corridor (A-weighted, exposure-aware).
    pub r_noise_level: RiskCoord,
    /// Odor intensity vs odor-hours corridor.
    pub r_odor_intensity: RiskCoord,
    /// Odor toxicity vs health-based limits.
    pub r_odor_tox: RiskCoord,
    /// Laser/light hazard index derived from class, beam geometry.
    pub r_laser_class: RiskCoord,
    /// Glare / light intrusion at receptors.
    pub r_light_glare: RiskCoord,
    /// Acute toxicity (signals + coatings) vs LC50/LD50 corridors.
    pub r_toxicity_acute: RiskCoord,
    /// Chronic toxicity vs NOAEL/RfD corridors.
    pub r_toxicity_chronic: RiskCoord,
    /// Persistence / bioaccumulation of additives.
    pub r_bioaccumulation: RiskCoord,
    /// Disturbance frequency (events/day or night).
    pub r_disturbance_freq: RiskCoord,
    /// Disturbance duty cycle (fraction of time active).
    pub r_disturbance_duty: RiskCoord,
    /// Habitat / ecosystem sensitivity weight (LifeEnvelope-derived).
    pub r_ecosystem_sensitivity: RiskCoord,

    // --- Aggregate residuals ---

    /// Lyapunov-style residual V_t = sum_j w_j * r_j.
    pub v_residual: f32,
    /// Uncertainty residual U_t = sum_j w_j * sigma_j.
    pub u_residual: f32,

    // --- K/E/R meta for this deployment state ---

    pub ker: KER,

    // --- Benefit tracking (eco-impact evidence) ---

    /// Cumulative kg of conventional poisons avoided (vs baseline).
    pub kg_poisons_avoided: f32,
    /// Cumulative count of verified non-target incidents (lower is better).
    pub non_target_incidents: u32,
    /// Cumulative number of complaint tickets resolved by tuning signals.
    pub complaints_resolved: u32,
}

impl BugsLifePestShard {
    /// Recompute V_t and U_t from the current risk coordinates.
    /// Must be called after updating any r_* fields.
    pub fn recompute_residuals(&mut self) {
        let coords = [
            self.r_noise_level.clipped(),
            self.r_odor_intensity.clipped(),
            self.r_odor_tox.clipped(),
            self.r_laser_class.clipped(),
            self.r_light_glare.clipped(),
            self.r_toxicity_acute.clipped(),
            self.r_toxicity_chronic.clipped(),
            self.r_bioaccumulation.clipped(),
            self.r_disturbance_freq.clipped(),
            self.r_disturbance_duty.clipped(),
            self.r_ecosystem_sensitivity.clipped(),
        ];

        let mut v_sum = 0.0_f32;
        let mut u_sum = 0.0_f32;

        for c in coords.iter() {
            v_sum += c.weight * c.r;
            u_sum += c.weight * c.sigma;
        }

        self.v_residual = v_sum;
        self.u_residual = u_sum;
    }

    /// Simple guard: check that all core corridors are present and within [0.0, 1.0].
    /// Returns false if any coordinate is out-of-bounds (violated corridor).
    pub fn corridors_ok(&self) -> bool {
        fn ok(c: &RiskCoord) -> bool {
            (0.0..=1.0).contains(&c.r)
                && (0.0..=1.0).contains(&c.sigma)
                && c.weight >= 0.0
        }

        ok(&self.r_noise_level)
            && ok(&self.r_odor_intensity)
            && ok(&self.r_odor_tox)
            && ok(&self.r_laser_class)
            && ok(&self.r_light_glare)
            && ok(&self.r_toxicity_acute)
            && ok(&self.r_toxicity_chronic)
            && ok(&self.r_bioaccumulation)
            && ok(&self.r_disturbance_freq)
            && ok(&self.r_disturbance_duty)
            && ok(&self.r_ecosystem_sensitivity)
    }

    /// Corridors must be present and K/E/R fields clipped before deployment.
    pub fn ker_clipped(mut self) -> Self {
        self.ker = self.ker.clipped();
        self
    }
}
