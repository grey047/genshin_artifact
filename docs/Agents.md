# Agents Overview

This document describes the conceptual "agents" (responsibility-centric modules) in the **Genshin Artifact Calculator** and how they collaborate. It gives Codex/Copilot-style code intelligence a high-level map so it can generate code and refactors that respect module boundaries and data contracts.

> **Scope:** This is documentation only; no runtime agent framework is introduced. The term *agent* here denotes a cohesive responsibility with clear inputs/outputs and ownership.

---

## System Topology (High Level)

```
[UI (Vue 3 + Pinia)]
   │  uses generated metadata (_gen_*.js)
   ▼
[WASM Bridge (mona_wasm)]  ⇄  [Core Logic (mona_core)]
   ▲                               ▲
   │ assembles interfaces          │ exposes stat/skills/buffs/sets
   │ and optimizer requests        │ and search algorithms
   ▼                               ▼
[Optimizer Orchestrator]  ⇄  [Value/Target Functions]
   │
   ▼
[Result Recorder / IDs]  →  [UI Results]
```

---

## Agent Catalog

### 1) **UI Agent** (Vue Front End)

* **Purpose:** Presents selectors, artifact inventories, optimizer controls; renders results.
* **Inputs:** Generated metadata (`_gen_character.js`, `_gen_weapon.js`, `_gen_artifact.js`, `_gen_buff.js`), user selections, imported artifacts JSON.
* **Outputs:** `OptimizeArtifactInterface` payloads, visualization of ranked results.
* **Key Files:** `src/store/pinia/artifact.ts`, `src/assets/_gen_*.js`, `src/types/*.ts`.
* **Notes:** Must treat artifact IDs/contentHash as stable keys for diffing and dedupe.

### 2) **Metadata Generator Agent** (`mona_generate`)

* **Purpose:** Produces localized, UI-ready metadata for characters, weapons, artifacts, and buffs.
* **Inputs:** Rust enums/definitions in `mona_core` and static assets.
* **Outputs:** `_gen_character.js`, `_gen_weapon.js`, `_gen_artifact.js`, `_gen_buff.js` under `mona_generate/output`.
* **Contracts:** Regenerate outputs whenever core enums/effects change to avoid drift.

### 3) **WASM Bridge Agent** (`mona_wasm`)

* **Purpose:** Validates and transforms UI payloads into Rust structs, calls core logic, returns results.
* **Inputs:** `CharacterInterface`, `WeaponInterface`, `BuffInterface`, `TargetFunctionInterface`, `ConstraintConfig`, artifacts array.
* **Outputs:** Ranked artifact combinations with normalized scores + selected IDs.
* **Key Files:** `mona_wasm/src/applications/common.rs`, `.../optimize_artifacts/interface_wasm.rs`, `.../inter.rs`.

### 4) **Core Character Agent** (`mona_core/character/*`)

* **Purpose:** Encapsulates per-character base stats, talent scaling, passives, damage enums, and UI config schema.
* **Inputs:** Character selection (level/ascension/talents/config), weapon state, buffs.
* **Outputs:** Damage calculations, stateful toggles, metadata (`CharacterStaticData`).
* **Extension:** Add new character module and register via `mona_derive` macros; regenerate metadata.

### 5) **Core Weapon Agent** (`mona_core/weapon/*`)

* **Purpose:** Models weapon stats and passives, including refinement scaling and configurable parameters.
* **Inputs:** Weapon selection (level/ascension/refine/config), bound character context.
* **Outputs:** Attribute mutations/effects applied to character/attacks.

### 6) **Core Artifact Agent** (`mona_core/artifacts/*`)

* **Purpose:** Implements artifact set effects and configuration surfaces.
* **Inputs:** Artifact slot/main/substats, set bonuses, optional toggles.
* **Outputs:** Attribute changes and conditional effects for scoring.

### 7) **Buff System Agent** (`mona_core/buffs/*`)

* **Purpose:** Formalizes buffs/debuffs via `Buff` trait + `BuffMeta` for UI exposure.
* **Inputs:** Configurable UI inputs (stacks/percent/element/etc.), character/team state.
* **Outputs:** Deterministic attribute and enemy-state mutations.

### 8) **Target Function Agent** (`ValueFunction` / goals)

* **Purpose:** Encodes "what to optimize for" (e.g., expected DPS, reaction-specific damage, survivability blend).
* **Inputs:** Character + weapon + artifact state; environment assumptions.
* **Outputs:** Scalar score used by the optimizer heap.

### 9) **Optimizer Orchestrator Agent** (`SingleOptimizeAlgorithm` family)

* **Purpose:** Plans and executes artifact search (heuristics, pruning, cutoffs).
* **Inputs:** Candidate artifacts by slot, constraints, target function.
* **Outputs:** Top-N combinations with scores and references to artifact IDs.
* **Key Files:** `algorithms/common.rs`, `algorithms/cutoff_algo2.rs`.

### 10) **Result Recorder Agent**

* **Purpose:** Tracks and emits stable artifact IDs + scores for UI display.
* **Inputs:** Optimizer-produced candidates referencing imported IDs.
* **Outputs:** UI-consumable arrays; preserves identity across sessions if IDs are stable.

---

## Cross-Agent Contracts (What Codex Should Respect)

1. **Interfaces are source of truth:**

   * `CharacterInterface`, `WeaponInterface`, `BuffInterface`, `TargetFunctionInterface`, `ConstraintConfig` define the only legal way for UI → WASM → Core.
2. **Stable IDs:**

   * Artifact `id` and `contentHash` must persist between import/export and optimization cycles.
3. **Regeneration Discipline:**

   * Anytime core enums or effect logic changes, run `mona_generate` to refresh `_gen_*` outputs before committing.
4. **Config Exposure Parity:**

   * If Rust adds a toggle, ensure metadata describes its label/range/default so the UI shows it.
5. **No hidden globals:**

   * Buffs, passives, and toggles must be modeled through explicit interfaces for determinism.

---

## Extension Playbooks (for Codex to follow)

### Add a Character

1. Implement module under `mona_core/src/character/characters/<element>/<name>.rs`.
2. Provide `CharacterStaticData`, scaling arrays, passives (`ChangeAttribute`), damage enum, config schema.
3. Register via `mona_derive` and update enums/maps.
4. Run `mona_generate` to refresh `_gen_character.js` & assets.
5. Add images in `src/images/characters` aligned with generator expectations.

### Add a Weapon

1. Implement `mona_core/src/weapon/weapons/<type>/<name>.rs` with stat progression and passive logic.
2. Expose config params (stacks/uptime/etc.).
3. Update enums, run `mona_generate` to refresh `_gen_weapon.js`.

### Add an Artifact Set

1. Implement effect in `mona_core/src/artifacts/effects/*` and update `ArtifactSetName`.
2. Define `ArtifactEffectConfig` for toggles.
3. Run `mona_generate` to refresh `_gen_artifact.js`.

### Add a Buff

1. Implement under `mona_core/src/buffs/buffs/*` with `Buff` + `BuffMeta`.
2. Regenerate `_gen_buff.js`.

---

## Data Contracts & Serialization

* **Artifacts Import/Export JSON**

  * Keys: `flower`, `feather`, `sand`, `cup`, `head`; values: arrays of `IArtifact`.
  * Each `IArtifact` has `{ setName, position, star, mainTag, normalTags[], level, omit?, id?, contentHash? }`.
  * Missing `id`/`contentHash` are backfilled, but external tools should preserve them.

* **Optimizer Request**

  * Must serialize full context via wasm interfaces so core can reconstruct combat state accurately.

---

## Conventions & Guardrails (for Code Generation)

* Keep per-agent files small and cohesive; prefer pure functions for scoring paths.
* When adding toggles, include:

  * **UI label (localized)**, **range**, **default**, and a **doc comment** explaining semantics.
* For performance-sensitive loops in optimizer/wasm/core, favor stack allocation and precomputed tables.
* No silent fallbacks: if candidates for a slot are empty, log/return a typed error; the UI may surface guidance.

---

## Worked Example (Bennett Buff → Score Path)

1. UI toggles Bennett burst config (stacks/talent level) based on `_gen_buff.js`.
2. WASM builds a `BuffInterface` and passes it to core.
3. Core `bennett.rs` implements `Buff` to mutate attributes.
4. `ValueFunction` reads attributes → emits scalar.
5. Optimizer evaluates combinations with pruning; heap stores `(score, [artifactIDs])`.
6. UI renders ranked list with stable IDs and normalized scores.

---

## Open Questions

* Should we version metadata outputs to prevent accidental drift when users import legacy JSON?
* Do we want a DSL for target functions exposed to UI for power users?

---

## Changelog Intent

* Keep this file updated alongside:

  * new characters/weapons/artifact sets,
  * new optimizer algorithms,
  * any interface changes (add a short diff under this section).
