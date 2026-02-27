use crate::actuation::BugsLifeActuation;
use crate::env_inputs::BugsLifeEnvInputs;
use crate::kernel::BugsLifeSafetyKernel;
use crate::types::{CorridorDecision, Residual};

/// Trait for controllers that must route all actuator changes through a safety kernel.
pub trait SafeBugsLifeController<K: BugsLifeSafetyKernel> {
    fn kernel(&self) -> &K;

    /// Propose an actuation for the given environment.
    fn propose_actuation(&mut self, env: &BugsLifeEnvInputs) -> BugsLifeActuation;

    /// Execute one control step under safety constraints.
    fn step_with_safety(
        &mut self,
        env: &BugsLifeEnvInputs,
        prev_residual: Residual,
    ) -> Result<(BugsLifeActuation, Residual), CorridorDecision> {
        let act = self.propose_actuation(env);
        let (decision, next_residual) = self.kernel().check_step(env, &act, prev_residual);

        match decision {
            CorridorDecision::Ok => Ok((act, next_residual)),
            CorridorDecision::Derate | CorridorDecision::Stop => Err(decision),
        }
    }
}
