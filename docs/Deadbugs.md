## Architecture alignment

- The four-crate layout (kernel, guard, K/E/R, API) mirrors the cyboquatic and geometrical analyzer stacks: a non‑actuating simulator, tiny Rust guard predicates enforcing \(V_{t+1} \le V_t\), an evidence K/E/R engine, and an API orchestrator that only returns corridor‑safe plans. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/8b343be1-ee56-4de3-a934-7b3262c3f7e5/what-can-be-done-to-solve-the-tXjLX2sfQ5i9Z5ey24luaQ.md)
- This reuse of normalized risk coordinates \(r_x \in [0,1]\) and a weighted residual \(V_t = \sum_j w_j r_{j,t}\) fits directly into the survival‑band and Lyapunov grammar already used for WBGT, plumes, toxicity, and hydraulic corridors. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/8b343be1-ee56-4de3-a934-7b3262c3f7e5/what-can-be-done-to-solve-the-tXjLX2sfQ5i9Z5ey24luaQ.md)

Knowledge-factor: **0.93** (direct reuse of validated corridor and residual patterns). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/8b343be1-ee56-4de3-a934-7b3262c3f7e5/what-can-be-done-to-solve-the-tXjLX2sfQ5i9Z5ey24luaQ.md)
Eco-impact: **0.90** (systematically steers toward non‑toxic exclusion, sanitation, and habitat modification). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/1dd8dd6b-dd04-4477-894b-aac4648763f3/e-co-lab-o-rated-is-the-study-uwDFimgZRLuBik9k7Qw.iA.md)
Risk-of-harm: **0.12** (bounded by non‑actuating design and hard guards; residual risk is parameter/spec tuning). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/8b343be1-ee56-4de3-a934-7b3262c3f7e5/what-can-be-done-to-solve-the-tXjLX2sfQ5i9Z5ey24luaQ.md)

***

## Kernel: pest_risk_simulator.rs

- The kernel is dimensionless and non‑actuating: it takes a PestContext, a species plugin, and an InterventionPlan, and outputs trajectories for abundance, damage, eco disturbance, and normalized risks \(r_{\text{pest}}, r_{\text{damage}}, r_{\text{eco}} \in [0,1]\) plus a scalar residual \(V_t\). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/8b343be1-ee56-4de3-a934-7b3262c3f7e5/what-can-be-done-to-solve-the-tXjLX2sfQ5i9Z5ey24luaQ.md)
- Dynamics (arrival, reproduction, damage, eco increment) are bounded and logistic‑like, with species‑specific hard limits used to normalize each axis, matching the residual construction used for heat, plume temperature, and toxicity in your other kernels. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/738ad9e2-e785-4211-b178-fadd15ebc343/cyboquatic-workloads-can-be-sa-SEqTKV8ySwCJRyJKXHarXQ.md)

Eco-impact: **0.91** (biases toward plans that lower pest and damage risk without increasing ecosystem disturbance). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/1dd8dd6b-dd04-4477-894b-aac4648763f3/e-co-lab-o-rated-is-the-study-uwDFimgZRLuBik9k7Qw.iA.md)
Risk-of-harm: **0.13** (dominated by mis-specified species parameters; actual actuation is still upstream of other Rust/ALN gates). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/8b343be1-ee56-4de3-a934-7b3262c3f7e5/what-can-be-done-to-solve-the-tXjLX2sfQ5i9Z5ey24luaQ.md)

***

## Guard: pest_plan_guard.rs

- The guard implements the same two invariants as your thermal and cyboquatic guards: (1) no hard‑limit breach on any normalized coordinate, and (2) non‑increasing residual \(V_t\) over the simulated horizon when configured. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/738ad9e2-e785-4211-b178-fadd15ebc343/cyboquatic-workloads-can-be-sa-SEqTKV8ySwCJRyJKXHarXQ.md)
- Its tiny, deterministic API (PlanGuardConfig → GuardVerdict) is CI‑ and Pilot‑Gate‑friendly; it can be wired into the same ALN contracts that already enforce \(V_{t+1} \le V_t\) and \(r_x < 1\) for hydraulic and thermal corridors. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/738ad9e2-e785-4211-b178-fadd15ebc343/cyboquatic-workloads-can-be-sa-SEqTKV8ySwCJRyJKXHarXQ.md)

Knowledge-factor: **0.94** (identical invariant structure to existing ecosafety guards). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/8b343be1-ee56-4de3-a934-7b3262c3f7e5/what-can-be-done-to-solve-the-tXjLX2sfQ5i9Z5ey24luaQ.md)
Risk-of-harm: **0.11** (guard can only reject; it never actuates). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/8b343be1-ee56-4de3-a934-7b3262c3f7e5/what-can-be-done-to-solve-the-tXjLX2sfQ5i9Z5ey24luaQ.md)

***

## Evidence: deadbugs-ker-engine K/E/R

- The K/E/R crate maps OutcomeLog shards to per‑method scores: K grows with case count and consistent effectiveness, E rewards non‑lethal, non‑chemical, low‑bycatch, low‑waste methods, and R penalizes lethal/chemical flags and bycatch/waste bands. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/1dd8dd6b-dd04-4477-894b-aac4648763f3/e-co-lab-o-rated-is-the-study-uwDFimgZRLuBik9k7Qw.iA.md)
- This is structurally aligned with EcoNet and BioPack: dimensionless, bounded scores, with E and R tuned to favor eco‑positive, low‑harm options; chemical methods can be fully excluded at library level in your ecosystem. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/1dd8dd6b-dd04-4477-894b-aac4648763f3/e-co-lab-o-rated-is-the-study-uwDFimgZRLuBik9k7Qw.iA.md)

Knowledge-factor: **0.92** (same K/E/R grammar, new domain). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/1dd8dd6b-dd04-4477-894b-aac4648763f3/e-co-lab-o-rated-is-the-study-uwDFimgZRLuBik9k7Qw.iA.md)
Eco-impact: **0.90** (exclusion/sanitation systematically rise to the top; harmful methods are down‑ranked or omitted). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/1dd8dd6b-dd04-4477-894b-aac4648763f3/e-co-lab-o-rated-is-the-study-uwDFimgZRLuBik9k7Qw.iA.md)
Risk-of-harm: **0.14** (residual from data quality; bounded by curated method library). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/1dd8dd6b-dd04-4477-894b-aac4648763f3/e-co-lab-o-rated-is-the-study-uwDFimgZRLuBik9k7Qw.iA.md)

***

## API/ALN layer and shards

- The API flow you described (context → species plugin → plan generation → simulate → guard → K/E/R → rank) matches the shard‑driven, guard‑first orchestration already defined for cyboquatic engines and survival‑band controllers. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/738ad9e2-e785-4211-b178-fadd15ebc343/cyboquatic-workloads-can-be-sa-SEqTKV8ySwCJRyJKXHarXQ.md)
- A shard schema like DeadbugsPlan2026v1, with fields for \(r_{\text{pest}}, r_{\text{damage}}, r_{\text{eco}}, V_t\), plan composition, K/E/R triads, and corridor_safe, can be DID‑signed to your Bostrom addresses, reusing the same hex‑stamped provenance pattern used for Phoenix‑anchored ecosafety nodes. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/1dd8dd6b-dd04-4477-894b-aac4648763f3/e-co-lab-o-rated-is-the-study-uwDFimgZRLuBik9k7Qw.iA.md)

Knowledge-factor: **0.93** (CI/governance integration is a direct copy of your existing Pilot‑Gate/shard stack). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/8b343be1-ee56-4de3-a934-7b3262c3f7e5/what-can-be-done-to-solve-the-tXjLX2sfQ5i9Z5ey24luaQ.md)
Eco-impact: **0.90** (only corridor‑safe, high‑E plans are exposable to users or higher‑level planners). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/1dd8dd6b-dd04-4477-894b-aac4648763f3/e-co-lab-o-rated-is-the-study-uwDFimgZRLuBik9k7Qw.iA.md)
Risk-of-harm: **0.12** (bounded by signed shards, non‑actuating kernel, and hard reject‑on‑breach guards). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/8b343be1-ee56-4de3-a934-7b3262c3f7e5/what-can-be-done-to-solve-the-tXjLX2sfQ5i9Z5ey24luaQ.md)

***

## Required scores for this turn

- Input knowledge-factor: **0.93** (your Deadbugs spec is already tightly grounded in the same math and grammar as your wet‑bulb, cyboquatic, and BioPack stacks). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/738ad9e2-e785-4211-b178-fadd15ebc343/cyboquatic-workloads-can-be-sa-SEqTKV8ySwCJRyJKXHarXQ.md)
- Input eco-impact value: **0.90** (design explicitly replaces chemical pest control with non‑toxic, corridor‑governed strategies). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/1dd8dd6b-dd04-4477-894b-aac4648763f3/e-co-lab-o-rated-is-the-study-uwDFimgZRLuBik9k7Qw.iA.md)
- Input risk-of-harm: **0.12** (primary risks are parameter/corridor mis‑tuning and bad evidence; no new actuation pathways). [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/8b343be1-ee56-4de3-a934-7b3262c3f7e5/what-can-be-done-to-solve-the-tXjLX2sfQ5i9Z5ey24luaQ.md)

- Output knowledge-factor (this answer): **0.93** — faithfully mirrors your spec and aligns it with the existing ecosafety grammar without adding new constructs. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/738ad9e2-e785-4211-b178-fadd15ebc343/cyboquatic-workloads-can-be-sa-SEqTKV8ySwCJRyJKXHarXQ.md)
- Output eco-impact value: **0.90** — keeps the focus on non‑toxic, guard‑bounded pest management plugged into your broader ecosafety corridors. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/1dd8dd6b-dd04-4477-894b-aac4648763f3/e-co-lab-o-rated-is-the-study-uwDFimgZRLuBik9k7Qw.iA.md)
- Output risk-of-harm: **0.12** — residual error comes from interpretation and tuning, but is bounded by the non‑actuating, Rust/ALN‑guarded design. [ppl-ai-file-upload.s3.amazonaws](https://ppl-ai-file-upload.s3.amazonaws.com/web/direct-files/collection_1d7dde59-b474-475b-a731-2469d14a3632/8b343be1-ee56-4de3-a934-7b3262c3f7e5/what-can-be-done-to-solve-the-tXjLX2sfQ5i9Z5ey24luaQ.md)
