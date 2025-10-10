# Genshin Artifact Calculator Product Requirements Document (PRD)

## 1. Project Overview
The calculator couples a Vue 3 front end with a Rust/wasm core (`mona_core`/`mona_wasm`) to evaluate Genshin Impact builds. Generated metadata in `mona_generate/output` feeds the UI, while wasm bindings expose high-performance simulation and optimization routines to JavaScript. This PRD captures the current structure, data requirements, and extension guidelines needed to keep the project maintainable.

## 2. Domain Data Modeling
### 2.1 Characters
* Generated metadata (`src/assets/_gen_character.js`) enumerates every playable character with localized names, rarity, element, weapon type, art assets, skill labels, and configurable skill parameters used by the UI selectors.【F:src/assets/character.js†L1-L30】【F:mona_generate/output/_gen_character.js†L160-L320】
* Runtime character instances are built in wasm through `CharacterInterface`, which serializes level, ascension, constellation, talent levels, and configuration params for wasm consumption.【F:mona_wasm/src/applications/common.rs†L18-L47】
* Underlying combat logic lives in `mona_core` per-character modules (for example, Amber) that define base stats, skill scaling tables, passive effects, and damage enum maps. These modules implement `CharacterStaticData`, attach talent toggles, and expose damage enumerations consumed by the optimizer and damage calculator.【F:mona_core/src/character/characters/pyro/amber.rs†L50-L139】

### 2.2 Weapons
* Weapon metadata (`mona_generate/output/_gen_weapon.js`) includes internal name, rarity, weapon class, passive description, thumbnail, and configurable parameters (e.g., stack counts or passive uptime sliders) for the front end.【F:mona_generate/output/_gen_weapon.js†L620-L689】
* The wasm layer expects `WeaponInterface` objects containing name, level, ascension flag, refine rank, and configuration payloads; it instantiates concrete weapon effects using these parameters alongside the bound character.【F:mona_wasm/src/applications/common.rs†L50-L72】

### 2.3 Artifacts
* Artifact set metadata (`mona_generate/output/_gen_artifact.js`) captures localization, star ranges, 2/4-piece effects, per-piece imagery, and optional configurable parameters (e.g., elemental choice or uptime ratios) that feed UI forms and effect calculators.【F:mona_generate/output/_gen_artifact.js†L600-L700】
* In storage and wasm, artifacts are modeled by slot, main stat tuple, sub-stat array, star/level, and unique IDs. The Pinia artifact store initializes from serialized payloads grouped by slot (`flower`, `feather`, etc.) and ensures IDs plus content hashes exist for deduplication.【F:src/store/pinia/artifact.ts†L18-L88】

## 3. Buff and Debuff Modeling
* UI-facing buff definitions are generated in `_gen_buff.js`, supplying localized names, icons, genres (character/common), descriptions, and configurable inputs (percentage sliders, stacks, etc.).【F:mona_generate/output/_gen_buff.js†L225-L288】
* The Rust side defines the `Buff` trait and metadata contract (`BuffMeta`) describing how buffs mutate attributes or enemies and how configuration schemas are exposed to the UI.【F:mona_core/src/buffs/buff.rs†L1-L19】
* Example: Bennett’s burst buff reads base ATK, constellation, and talent level to grant ATK% or Pyro bonus via `change_attribute`, while metadata specifies the config UI elements.【F:mona_core/src/buffs/buffs/character/bennett.rs†L12-L112】
* During optimization, buffs are materialized from `BuffInterface` and injected into value calculations, ensuring stat modifications are included when scoring builds.【F:mona_wasm/src/applications/common.rs†L74-L101】【F:mona_wasm/src/applications/optimize_artifacts/interface_wasm.rs†L34-L64】

## 4. Optimization Workflow
1. The front end assembles an `OptimizeArtifactInterface` (character, weapon, target function, buff list, optional artifact filters/constraints, and algorithm choice) and ships artifacts as an array of wasm-compatible structs.【F:mona_wasm/src/applications/optimize_artifacts/inter.rs†L33-L98】
2. `OptimizeSingleWasm.optimize` converts inputs to Rust structs, instantiates the requested target function (native or DSL), applies filters, and selects the configured search algorithm.【F:mona_wasm/src/applications/optimize_artifacts/interface_wasm.rs†L25-L65】
3. Algorithms implement `SingleOptimizeAlgorithm`. By default, `CutoffAlgo2` blends heuristic weighting with pruned search: it ranks main stats and artifacts via weight heuristics, builds “super artifacts” for upper-bound pruning, iterates combinations respecting set constraints, and records top results via a bounded heap.【F:mona_wasm/src/applications/optimize_artifacts/algorithm.rs†L1-L61】【F:mona_wasm/src/applications/optimize_artifacts/algorithms/common.rs†L1-L200】【F:mona_wasm/src/applications/optimize_artifacts/algorithms/cutoff_algo2.rs†L1-L517】
4. Constraints (minimum stats, forced set layouts) are enforced in `ValueFunction` checks before scoring, and the result heap returns artifact IDs plus normalized scores for the UI to display.【F:mona_wasm/src/applications/optimize_artifacts/algorithms/common.rs†L93-L183】【F:mona_wasm/src/applications/optimize_artifacts/algorithms/cutoff_algo2.rs†L300-L447】

## 5. Extensibility Requirements
### 5.1 Adding a Character
* Implement new character logic in `mona_core/src/character/characters/<element>/<name>.rs`: define `CharacterStaticData`, talent scaling arrays, passive effect struct (`ChangeAttribute`), damage enum, and configuration schema if needed.【F:mona_core/src/character/characters/pyro/amber.rs†L55-L139】
* Extend enums/maps via `mona_derive` macros so the character is discoverable by `CharacterName` and generation scripts.【F:mona_generate/src/gen_meta/gen_character_meta.rs†L11-L125】
* Regenerate metadata (`mona_generate`) to update `_gen_character.js`, localizations, and UI assets, then add card/avatar/splash imagery under `src/images/characters` as referenced by the generator.【F:mona_generate/output/_gen_character.js†L160-L268】

#### Required Inputs for a New Character
To add a specific character, gather the following beforehand so coding and metadata generation can proceed without guesswork:
* **Identity & classification:** Internal key, display names (all supported locales), element, weapon type, rarity, region/series tags, gender/pronoun for text replacements.【F:mona_generate/src/gen_meta/gen_character_meta.rs†L11-L125】【F:src/assets/character.js†L1-L30】
* **Base stats by ascension:** HP/ATK/DEF tables for each ascension, plus specialized sub-stat progression (e.g., CRIT Rate%).【F:mona_core/src/character/characters/pyro/amber.rs†L61-L110】
* **Talents:** Multiplier tables for normal/charged/plunging, skill, burst, and passives; cooldowns and snapshotting rules where relevant so the damage enum and rotation helpers can be authored.【F:mona_core/src/character/characters/pyro/amber.rs†L111-L139】
* **Internal toggles/config:** Any gameplay states that should be user-configurable (e.g., stack counts, infusion element, burst mode). Provide default values, ranges, and UI labels for these configs so `_gen_character.js` exposes the correct form inputs.【F:mona_generate/output/_gen_character.js†L200-L268】
* **Assets:** Card, avatar, and skill icon filenames that will be generated or copied into `src/images/characters` to keep the UI consistent with other heroes.
* **Buff interactions:** If the character supplies team buffs or debuffs, list their mechanical formulas (ATK%, DMG Bonus, RES shred, etc.) to implement associated entries under `mona_core/src/buffs` when needed.【F:mona_core/src/buffs/buffs/character/bennett.rs†L12-L112】

### 5.2 Adding a Weapon
* Create weapon effect logic in `mona_core/src/weapon/weapons/<type>/<name>.rs` (pattern mirrors characters) with base stats, passive scaling, and config definitions.
* Update weapon enums and run `mona_generate` to refresh `_gen_weapon.js` so the UI receives localized metadata and config sliders.【F:mona_generate/output/_gen_weapon.js†L626-L689】

#### Required Inputs for a Set Weapon
* **Identity:** Internal weapon key, localized names, rarity, weapon class, series tag if relevant for filtering.【F:mona_generate/output/_gen_weapon.js†L626-L689】
* **Stat progression:** Base ATK and sub-stat values per level/ascension so `WeaponStaticData` can be populated.【F:mona_core/src/weapon/weapons/bows/amos_bow.rs†L21-L68】
* **Passive description & mechanics:** Clear formula for passive effects including trigger conditions, stack counts, cooldowns, and refinement scaling tables. Highlight any configurable parameters (uptime sliders, stack overrides) with desired min/max/default values for the UI config metadata.【F:mona_core/src/weapon/weapons/bows/amos_bow.rs†L69-L150】
* **Icon assets:** Weapon icon/thumbnail references that align with generator expectations in `mona_generate`.

### 5.3 Adding an Artifact Set
* Implement the artifact set’s effect in `mona_core/src/artifacts/effects` and register configuration metadata (set bonuses, toggles) through `ArtifactEffectConfig`.
* Update `ArtifactSetName` enum and regenerate `_gen_artifact.js` for UI descriptions and configuration options.【F:mona_core/src/artifacts/artifact.rs†L20-L70】【F:mona_generate/output/_gen_artifact.js†L600-L700】

#### Required Inputs for an Artifact Set
* **Set identity:** Internal key, localized 2-piece/4-piece names and descriptions, star rarity range, and story tag if applicable for filtering.【F:mona_generate/output/_gen_artifact.js†L600-L700】
* **Effect mechanics:** Mathematical breakdown of 2-piece and 4-piece bonuses, including any conditional logic (element checks, stack counts, uptime) plus suggested configuration toggles (e.g., element selector, stack slider) with UI ranges/defaults.【F:mona_core/src/artifacts/effects/viridescent_venerer.rs†L15-L140】
* **Piece metadata:** Names and icon references for flower, plume, sands, goblet, circlet as required by the generator outputs.
* **Synergy notes:** If the set introduces buffs/debuffs not already modeled, specify their formulas so corresponding buff entries can be added to keep optimizer calculations accurate.【F:mona_core/src/buffs/buff.rs†L1-L19】

## 6. Data Import Expectations
* Artifact import/export uses a JSON object with slot keys (`flower`, `feather`, `sand`, `cup`, `head`), each containing an array of artifacts shaped like `IArtifact`: `{ setName, position, star, mainTag: { name, value }, normalTags: [{ name, value }...], level, omit?, id?, contentHash? }`. Missing `id` or `contentHash` fields are backfilled during initialization, but providing them preserves stable references.【F:src/store/pinia/artifact.ts†L18-L88】【F:src/types/artifact.ts†L1-L47】
* Optimizer requests must serialize characters, weapons, buffs, and target functions using the wasm interfaces (`CharacterInterface`, `WeaponInterface`, `BuffInterface`, `TargetFunctionInterface`, `ConstraintConfig`) so the wasm core can reconstruct combat contexts accurately.【F:mona_wasm/src/applications/common.rs†L18-L123】【F:mona_wasm/src/applications/optimize_artifacts/inter.rs†L33-L98】

## 7. Maintenance Considerations
* Keep `mona_core` and `mona_generate` in sync: whenever Rust enums or effect logic change, rerun the generator to refresh JS assets before committing.
* `ResultRecorder` uses artifact IDs, so ensure imported artifacts carry stable IDs to avoid confusing duplicate detection during optimization.【F:mona_wasm/src/applications/optimize_artifacts/algorithms/common.rs†L133-L183】
* The heuristic optimizer falls back to a slower naive search if any slot lacks candidates; maintain artifact filters and datasets to prevent empty slots.【F:mona_wasm/src/applications/optimize_artifacts/algorithms/cutoff_algo2.rs†L460-L467】
* Buff/target function configs surface directly in the UI; when adding options, provide localized labels in the respective Rust metadata to keep `_gen_*` outputs user-friendly.【F:mona_core/src/buffs/buffs/character/bennett.rs†L28-L70】【F:mona_generate/src/gen_meta/gen_character_meta.rs†L42-L125】

## 8. Open Questions / Future Work
* Review outdated character/weapon rosters against current game releases and add missing assets to keep optimization relevant.
* Consider automating metadata regeneration (e.g., npm script) to reduce drift between Rust definitions and generated JS.
* Explore extending the data import format with versioning to guard against structural changes in artifact serialization.
