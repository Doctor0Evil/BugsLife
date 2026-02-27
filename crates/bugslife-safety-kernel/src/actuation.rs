use serde::{Deserialize, Serialize};

/// Intent-level actuation command for a BugsLife node.
/// This is a profile, not raw voltages or waveform samples.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BugsLifeActuation {
    /// High-level profile identifier (e.g., "bedbug_ceiling_low").
    pub profile_id: String,
    /// Overall intensity multiplier in [0,1].
    pub intensity_pct: f32,
    /// Fraction of time active in [0,1] for this schedule window.
    pub duty_cycle: f32,
    /// Optional schedule tag (e.g., "night", "day", "weekend").
    pub schedule_tag: String,
}

impl BugsLifeActuation {
    pub fn new(profile_id: impl Into<String>, intensity_pct: f32, duty_cycle: f32, schedule_tag: impl Into<String>) -> Self {
        Self {
            profile_id: profile_id.into(),
            intensity_pct: intensity_pct.clamp(0.0, 1.0),
            duty_cycle: duty_cycle.clamp(0.0, 1.0),
            schedule_tag: schedule_tag.into(),
        }
    }
}
