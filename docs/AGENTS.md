# Documentation Agent Guidelines

These instructions apply to all content under the `docs/` tree, including the PRD and any supplemental onboarding guides.

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
