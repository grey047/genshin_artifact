# Documentation Agent Guidelines and Responsibility Map

These instructions apply to all content under the `docs/` tree, including the PRD and any supplemental onboarding guides. They also catalogue the conceptual "agents" (responsibility-centric modules) so code-generation assistants understand how the runtime pieces collaborate.

## Purpose of the Documentation Folder
- Capture the canonical product requirements, architectural overviews, and contributor playbooks for the optimizer.
- Mirror the runtime structure:
  - **Frontend** (`src/`, `src-tauri/`): Vue + Pinia UI that consumes generated metadata and wasm bindings.
  - **Core Simulation** (`mona_core`, `mona_wasm`, `mona_derive`): Rust crates that own combat logic, buffs, artifact optimizers, and FFI bindings.
  - **Code Generation** (`mona_generate`, `mona_generate/output`): pipelines that synthesize localized metadata consumed by the UI.

## Authoring Expectations
- Keep descriptions synchronized with the actual code layout and naming. When code moves or APIs change, update the PRD sections that reference them.
- Use bilingual terminology when the referenced source files are bilingual (English, Japanese, or Chinese). Otherwise, default to English.
- Reference concrete module paths or file names when describing data sources, configs, or algorithms.
- Note command names (`cargo test`, `cargo run -p mona_generate`, `yarn build`, etc.) exactly so contributors can copy-paste them.

## Structure & Style
- Start new documents with an H1 that states the document purpose.
- Prefer ordered lists for step-by-step onboarding checklists; use tables for enumerating required metadata fields (e.g., character configs, weapon passives).
- Cite the origin of gameplay scalars (talent tables, shield formulas) and where they live in the codebase.
- When documenting configuration schemas, include the Rust struct/enum names as well as the generated JSON keys.

## Maintenance Workflow
- After modifying gameplay metadata or optimizer logic, rerun `cargo run -p mona_generate` and update any sections that mention generated outputs (`_gen_*.js`).
- When wasm bindings change, reflect the new TypeScript interfaces exposed in `mona_wasm/pkg` and how the frontend consumes them.
- Track open technical debt or TODOs in a dedicated "Follow-ups" or "Maintenance" section at the end of the PRD.

## Review Checklist
Before approving documentation PRs under `docs/`, verify that:
1. Architecture narratives align with the current Rust and Vue module structure.
2. Checklists for adding characters, weapons, artifacts, or buffs enumerate every required field from the corresponding `CharacterStaticData`, `WeaponStaticData`, `ArtifactEffectConfig`, and `BuffMeta` definitions.
3. Optimization descriptions mention the algorithms implemented in `mona_wasm/src/applications/optimize_artifacts` and how they interact with Pinia stores.
4. Import/export data formats describe the expected JSON attribute names consumed by the loaders in `loaders/`.
5. Maintenance notes call out any scripts or tests that must be re-run after documentation changes.

---

## Agents Overview

This section describes the conceptual "agents" (responsibility-centric modules) in the **Genshin Artifact Calculator** and how they collaborate. It gives Codex/Copilot-style code intelligence a high-level map so it can generate code and refactors that respect module boundaries and data contracts. *Scope:* This is documentation only; no runtime agent framework is introduced. The term *agent* here denotes a cohesive responsibility with clear inputs/outputs and ownership.

### System Topology (High Level)

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

### Agent Catalog

1. **UI Agent** (Vue Front End)
   - **Purpose:** Presents selectors, artifact inventories, optimizer controls; renders results.
   - **Inputs:** Generated metadata (`_gen_character.js`, `_gen_weapon.js`, `_gen_artifact.js`, `_gen_buff.js`), user selections, imported artifacts JSON.
   - **Outputs:** `OptimizeArtifactInterface` payloads, visualization of ranked results.
   - **Key Files:** `src/store/pinia/artifact.ts`, `src/assets/_gen_*.js`, `src/types/*.ts`.
   - **Notes:** Must treat artifact IDs/contentHash as stable keys for diffing and dedupe.

2. **Metadata Generator Agent** (`mona_generate`)
   - **Purpose:** Produces localized, UI-ready metadata for characters, weapons, artifacts, and buffs.
   - **Inputs:** Rust enums/definitions in `mona_core` and static assets.
   - **Outputs:** `_gen_character.js`, `_gen_weapon.js`, `_gen_artifact.js`, `_gen_buff.js` under `mona_generate/output`.
   - **Contracts:** Regenerate outputs whenever core enums/effects change to avoid drift.

3. **WASM Bridge Agent** (`mona_wasm`)
   - **Purpose:** Validates and transforms UI payloads into Rust structs, calls core logic, returns results.
   - **Inputs:** `CharacterInterface`, `WeaponInterface`, `BuffInterface`, `TargetFunctionInterface`, `ConstraintConfig`, artifacts array.
   - **Outputs:** Ranked artifact combinations with normalized scores + selected IDs.
   - **Key Files:** `mona_wasm/src/applications/common.rs`, `.../optimize_artifacts/interface_wasm.rs`, `.../inter.rs`.

4. **Core Character Agent** (`mona_core/character/*`)
   - **Purpose:** Encapsulates per-character base stats, talent scaling, passives, damage enums, and UI config schema.
   - **Inputs:** Character selection (level/ascension/talents/config), weapon state, buffs.
   - **Outputs:** Damage calculations, stateful toggles, metadata (`CharacterStaticData`).
   - **Extension:** Add new character module and register via `mona_derive` macros; regenerate metadata.

5. **Core Weapon Agent** (`mona_core/weapon/*`)
   - **Purpose:** Models weapon stats and passives, including refinement scaling and configurable parameters.
   - **Inputs:** Weapon selection (level/ascension/refine/config), bound character context.
   - **Outputs:** Attribute mutations/effects applied to character/attacks.

6. **Core Artifact Agent** (`mona_core/artifacts/*`)
   - **Purpose:** Implements artifact set effects and configuration surfaces.
   - **Inputs:** Artifact slot/main/substats, set bonuses, optional toggles.
   - **Outputs:** Attribute changes and conditional effects for scoring.

7. **Buff System Agent** (`mona_core/buffs/*`)
   - **Purpose:** Formalizes buffs/debuffs via `Buff` trait + `BuffMeta` for UI exposure.
   - **Inputs:** Configurable UI inputs (stacks/percent/element/etc.), character/team state.
   - **Outputs:** Deterministic attribute and enemy-state mutations.

8. **Target Function Agent** (`ValueFunction` / goals)
   - **Purpose:** Encodes "what to optimize for" (e.g., expected DPS, reaction-specific damage, survivability blend).
   - **Inputs:** Character + weapon + artifact state; environment assumptions.
   - **Outputs:** Scalar score used by the optimizer heap.

9. **Optimizer Orchestrator Agent** (`SingleOptimizeAlgorithm` family)
   - **Purpose:** Plans and executes artifact search (heuristics, pruning, cutoffs).
   - **Inputs:** Candidate artifacts by slot, constraints, target function.
   - **Outputs:** Top-N combinations with scores and references to artifact IDs.
   - **Key Files:** `algorithms/common.rs`, `algorithms/cutoff_algo2.rs`.

10. **Result Recorder Agent**
    - **Purpose:** Tracks and emits stable artifact IDs + scores for UI display.
    - **Inputs:** Optimizer-produced candidates referencing imported IDs.
    - **Outputs:** UI-consumable arrays; preserves identity across sessions if IDs are stable.

### Cross-Agent Contracts (What Documentation Must Respect)

1. **Interfaces are source of truth:** `CharacterInterface`, `WeaponInterface`, `BuffInterface`, `TargetFunctionInterface`, and `ConstraintConfig` define the only legal way for UI → WASM → Core communication.
2. **Stable IDs:** Artifact `id` and `contentHash` must persist between import/export and optimization cycles.
3. **Regeneration discipline:** Anytime core enums or effect logic changes, run `mona_generate` to refresh `_gen_*` outputs before committing.
4. **Config exposure parity:** If Rust adds a toggle, ensure metadata describes its label/range/default so the UI shows it.
5. **No hidden globals:** Buffs, passives, and toggles must be modeled through explicit interfaces for determinism.

### Extension Playbooks (for Authors & Code Assistants)

#### Add a Character
1. Implement module under `mona_core/src/character/characters/<element>/<name>.rs`.
2. Provide `CharacterStaticData`, scaling arrays, passives (`ChangeAttribute`), damage enum, config schema.
3. Register via `mona_derive` and update enums/maps.
4. Run `mona_generate` to refresh `_gen_character.js` & assets.
5. Add images in `src/images/characters` aligned with generator expectations.

#### Add a Weapon
1. Implement `mona_core/src/weapon/weapons/<type>/<name>.rs` with stat progression and passive logic.
2. Expose config params (stacks/uptime/etc.).
3. Update enums, run `mona_generate` to refresh `_gen_weapon.js`.

#### Add an Artifact Set
1. Implement effect in `mona_core/src/artifacts/effects/*` and update `ArtifactSetName`.
2. Define `ArtifactEffectConfig` for toggles.
3. Run `mona_generate` to refresh `_gen_artifact.js`.

#### Add a Buff
1. Implement under `mona_core/src/buffs/buffs/*` with `Buff` + `BuffMeta`.
2. Regenerate `_gen_buff.js`.

### Data Contracts & Serialization

- **Artifacts Import/Export JSON**
  - Keys: `flower`, `feather`, `sand`, `cup`, `head`; values: arrays of `IArtifact`.
  - Each `IArtifact` has `{ setName, position, star, mainTag, normalTags[], level, omit?, id?, contentHash? }`.
  - Missing `id`/`contentHash` are backfilled, but external tools should preserve them.

- **Optimizer Request**
  - Must serialize full context via wasm interfaces so core can reconstruct combat state accurately.

### Conventions & Guardrails (for Code Generation and Docs)

- Keep per-agent files small and cohesive; prefer pure functions for scoring paths.
- When adding toggles, include the UI label (localized), range, default, and a doc comment explaining semantics.
- For performance-sensitive loops in optimizer/wasm/core, favor stack allocation and precomputed tables.
- No silent fallbacks: if candidates for a slot are empty, log/return a typed error; the UI may surface guidance.

### Worked Example (Bennett Buff → Score Path)

1. UI toggles Bennett burst config (stacks/talent level) based on `_gen_buff.js`.
2. WASM builds a `BuffInterface` and passes it to core.
3. Core `bennett.rs` implements `Buff` to mutate attributes.
4. `ValueFunction` reads attributes → emits scalar.
5. Optimizer evaluates combinations with pruning; heap stores `(score, [artifactIDs])`.
6. UI renders ranked list with stable IDs and normalized scores.

### Open Questions

- Should we version metadata outputs to prevent accidental drift when users import legacy JSON?
- Do we want a DSL for target functions exposed to UI for power users?

### Changelog Intent

- Keep this file updated alongside new characters/weapons/artifact sets, new optimizer algorithms, and any interface changes (add a short diff under this section).
