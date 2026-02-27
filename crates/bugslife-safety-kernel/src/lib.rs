pub mod types;
pub mod var_ids;
pub mod corridors;
pub mod residual;
pub mod contracts;
pub mod env_inputs;
pub mod actuation;
pub mod kernel;
pub mod controller;

pub use crate::types::{CorridorBands, CorridorDecision, RiskCoord, Residual};
pub use crate::var_ids::VarId;
pub use crate::corridors::{CorridorSet, CorridorValidationError};
pub use crate::contracts::{corridor_present, safe_step};
pub use crate::env_inputs::BugsLifeEnvInputs;
pub use crate::actuation::BugsLifeActuation;
pub use crate::kernel::{BugsLifeSafetyKernel, DefaultBugsLifeKernelPhoenix2026};
pub use crate::controller::SafeBugsLifeController;
