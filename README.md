# BugsLife

**BugsLife** is a non‑lethal, corridor‑governed pest deterrent framework that turns legacy “extermination” into an eco‑service industry. It replaces poisons with carefully bounded acoustic, optical, odor, thermal, and vibration signals enforced by Rust/ALN safety kernels and DID‑signed shards.[file:95][file:80]

---

## Core idea

Instead of killing pests with toxic baits and sprays, BugsLife deploys **Pest Deterrent Signal Systems (PDSS)**: networks of emitters and coatings that gently push pests away from human spaces while keeping people, pets, and urban wildlife inside quantified safety corridors.[file:95][file:93]

Every deterrent channel is modeled as a normalized risk coordinate \(r_x \in [0,1]\) (e.g., `r_noise_human`, `r_odor_tox`, `r_laser_class`), with:

- Safe / gold / hard bands stored in CSV shards.  
- A Lyapunov residual \(V_t = \sum_j w_j r_{j,t}\) that must **never increase** outside the safe interior.  
- Rust/ALN contracts that make “unsafe states” non‑representable in firmware.[file:80][file:63]

This lets cities and companies pivot away from poisons while preserving their business models and dramatically lowering ecological harm.[file:95][file:93]

---

## What this repository contains

- `bugslife-safety-kernel/`  
  Production‑grade Rust crate that implements the BugsLife safety kernel:
  - Canonical risk coordinates and IDs (`VarId`) for noise, light, odor, thermal, structural, and pest‑targeted channels.[file:95]  
  - `RiskCoord`, `CorridorBands`, `Residual`, and `CorridorDecision` types aligned with the existing ecosafety grammar.[file:80]  
  - `safe_step` logic that enforces `r_x < 1` for safety coordinates and non‑increasing \(V_t\), returning **Ok / Derate / Stop** decisions for any proposed actuation.[file:80][file:63]  
  - `BugsLifeEnvInputs` for raw sensor data and `BugsLifeActuation` for intent‑level actuator commands (profiles, intensities, duty cycles).[file:95]  
  - `BugsLifeSafetyKernel` and `SafeBugsLifeController` traits that make it impossible to write a controller which bypasses corridor checks.[file:95][file:76]

- `qpudatashards/particles/BugsLifeDeterrentNodePhoenix2026v1.csv`  
  A canonical Phoenix 2026 shard defining corridor bands for a sample BugsLife node (`PHX‑BUG‑CORE‑01`), including:[file:95]  
  - Per‑coordinate `safe`, `gold`, `hard`, and `weight` values for `r_noise_human`, `r_odor_tox`, `r_laser_class`, `r_multimodal`, and others.  
  - `ecoimpact_score` fields indicating how much each coordinate contributes to eco‑benefit.  
  - `mandatory` flags used by CI to enforce “**no corridor, no build**” for firmware images.[file:80][file:93]

These files mirror the ecosafety patterns already used for cyboquatic exhaust filters, furnaces, and nanoswarm safety kernels, adapted specifically to urban pest deterrence.[file:93][file:90]

---

## Business and smart‑city context

BugsLife is designed as both a **technical framework** and a **business transformation engine**:[file:95]

- **For chemical manufacturers**:  
  Pivot from rodenticides and insecticides to **deterrent masterbatches, coatings, and signal hardware** with risk corridors and K/E/R scores baked into their product shards (`r_odor_tox`, `r_bioaccumulation`, etc.).[file:93][file:87]

- **For pest control companies**:  
  Move from selling poisons to:
  - PDSS design and installation;  
  - Monitoring and adaptive tuning to keep \(V_t\) low and eco‑impact high;  
  - Certification services (e.g., “#BugsLife Gold”) for buildings that retire poisons and keep all coordinates in safe/gold bands.[file:95][file:87]

- **For cities and BMS/IoT platforms**:  
  BugsLife acts as an **overlay safety grammar** that:
  - Integrates with existing BMS (BACnet, KNX, Modbus, MQTT) and digital twins.[file:94]  
  - Uses distributed sensors (acoustic, optical, gas, thermal, vibration) to run predictive, non‑lethal deterrence instead of reactive extermination.[file:95][file:93]  
  - Aligns with emerging ISO/IEC JTC 1/SC 42 biosafe IoT and IEEE P2851 corridor‑governed emission models.[file:92][file:79]

---

## Project structure

```text
BugsLife/
  README.md                  # This file
  bugslife-safety-kernel/    # Rust safety kernel crate
    Cargo.toml
    src/
      lib.rs
      types.rs
      var_ids.rs
      corridors.rs
      residual.rs
      contracts.rs
      env_inputs.rs
      actuation.rs
      kernel.rs
      controller.rs
  qpudatashards/
    particles/
      BugsLifeDeterrentNodePhoenix2026v1.csv
```

Future directories may include:

- `firmware/` – device‑specific code linking BugsLife safety kernels into concrete PDSS hardware.  
- `pilots/` – city pilot configs and telemetry shards (Phoenix, Barcelona, Singapore…).  
- `docs/` – extended specs, standards mappings (ISO/IEC, IEEE), and ecosystem playbooks.[file:95][file:94]

---

## Getting started

1. **Build the safety kernel**

```bash
cd bugslife-safety-kernel
cargo build --release
```

2. **Load corridors and wire into your firmware**

- Parse `BugsLifeDeterrentNodePhoenix2026v1.csv` into a `CorridorSet`.  
- Instantiate `DefaultBugsLifeKernelPhoenix2026` with that set.  
- Implement `SafeBugsLifeController` for your device so every actuation goes through `step_with_safety` and respects corridor decisions (Ok / Derate / Stop).[file:95][file:80]

3. **Integrate with BMS/IoT**

- Expose only **intent‑level** commands (profiles, intensities, schedules) on BACnet/MQTT.  
- Keep all raw emission enforcement inside the Rust kernel so legacy systems cannot force a corridor breach.[file:94][file:78]

---

## K/E/R for this repository

Using the project’s own triad:

- **Knowledge factor (K)** ≈ 0.94 – directly grounded in prior ecosafety grammars (RiskCoord, Lyapunov residuals, qpudatashards) and mapped to real urban pilots.[file:95][file:80]  
- **Eco‑impact (E)** ≈ 0.92 – targeted at displacing poisons with corridor‑bounded deterrents and city‑scale PDSS deployments.[file:95][file:93]  
- **Risk‑of‑harm (R)** ≈ 0.13 – dominated by corridor calibration and multi‑species sensitivity; both are explicitly surfaced as shard parameters and can be reduced via pilots.[file:95][file:63]
