# GOOD Artifact Format Support Plan

## Overview
Add support for importing artifacts from the GOOD (Genshin Open Object Description) format, commonly exported by tools like Inventory Kamera and netcrawler. The GOOD format will be parsed in the **TypeScript frontend** alongside existing mona.json parsing, sharing the same entry point and converging to the same `IArtifact` interface before reaching the Rust/WASM optimization core.

## Architecture

### Current Data Flow (mona.json only)
```
File Upload (ArtifactsPage.vue → ImportBlock.vue)
    ↓
importJson() → JSON.parse()
    ↓
importMonaJson()        [src/utils/artifacts.ts]
    ↓  (iterates artifacts, dedup/upgrade/add to store)
    ↓
IArtifact[] → artifactStore
    ↓
convertArtifact()       [src/utils/converter.ts]
    ↓
IArtifactWasm → WASM/Rust (optimization only)
```

> **Note on `checkImportJson.js`**: This file exports a `checkImportJson()` validation
> function, but it is **never imported or called** anywhere in the codebase. `importMonaJson()`
> does not invoke it. Validation currently relies on the artifact store's own logic and
> `artifactsData` lookups. This is a pre-existing gap — not introduced by this plan.

### Proposed Data Flow (with GOOD support)
```
File Upload (ArtifactsPage.vue → ImportBlock.vue)
    ↓
importJson() → JSON.parse()
    ↓
detectFormat(rawObj)     [NEW - src/utils/goodFormat.ts]
    ↓
    ├─→ mona.json detected (has "flower"/"feather"/... keys)
    │       ↓
    │   importMonaJson()        [src/utils/artifacts.ts - existing]
    │
    └─→ GOOD format detected (has "format": "GOOD")
            ↓
        convertGoodToMona()     [NEW - src/utils/goodFormat.ts]
            ↓  (validates + converts each artifact; skips invalid)
        mona-format rawObj      (same shape as mona.json)
            ↓
        importMonaJson()        [src/utils/artifacts.ts - reused]
    ↓
[Both converge to same IArtifact[] → artifactStore]
    ↓
convertArtifact()       [src/utils/converter.ts - no changes]
    ↓
IArtifactWasm → WASM/Rust (no changes needed)
```

### Key Design Decisions

1. **TypeScript-only implementation** — All import parsing currently lives in TypeScript (`src/utils/`). GOOD parsing follows the same pattern. No changes to `mona_core` (Rust).

2. **Format detection gateway** — A single `detectFormat()` function inspects the parsed JSON to route to the correct parser. Both paths converge to the existing `importMonaJson()` function.

3. **GOOD → mona-format conversion** — Rather than creating a parallel import pipeline, GOOD artifacts are converted to the mona.json shape (`{flower: [...], feather: [...], ...}`), then reuse `importMonaJson()` for deduplication, upgrade detection, and equip/kumi handling.

4. **Validation in `convertGoodToMona()`** — Since `checkImportJson.js` is orphaned (exported but never called), all input validation for GOOD artifacts is handled inside `convertGoodToMona()`. This includes structural checks (required fields, types, value ranges) in addition to mapping lookups. Invalid artifacts are skipped with console warnings.

5. **No Rust changes** — `mona_core` only receives `IArtifactWasm` via the existing `convertArtifact()` path. The Rust side is format-agnostic.

## Format Comparison

### mona.json Format (Current)
```json
{
  "version": 1,
  "flower": [{
    "setName": "gladiatorFinale",
    "position": "flower",
    "mainTag": { "name": "lifeStatic", "value": 4780 },
    "normalTags": [
      { "name": "critical", "value": 0.035 },
      { "name": "criticalDamage", "value": 0.194 }
    ],
    "star": 5,
    "level": 20,
    "equip": "RaidenShogun"
  }],
  "feather": [{...}],
  "sand": [{...}],
  "cup": [{...}],
  "head": [{...}]
}
```

### GOOD Format (Target)
```json
{
  "format": "GOOD",
  "version": 2,
  "artifacts": [{
    "setKey": "GladiatorsFinale",
    "slotKey": "flower",
    "rarity": 5,
    "level": 20,
    "mainStatKey": "hp",
    "substats": [
      { "key": "critRate_", "value": 3.5 },
      { "key": "critDMG_", "value": 19.4 }
    ],
    "location": "RaidenShogun",
    "lock": true,
    "id": 12345
  }]
}
```

## Field Mapping

### GOOD → mona internal format (IArtifact)

| GOOD Field | mona Field | Type | Notes |
|------------|------------|------|-------|
| `setKey` | `setName` | string → string | GOOD `"GladiatorsFinale"` → mona `"gladiatorFinale"` via `setNameMap` |
| `slotKey` | `position` | string → ArtifactPosition | `"flower"` → `"flower"`, `"goblet"` → `"cup"` |
| `rarity` | `star` | number | Direct |
| `level` | `level` | number | Direct |
| `mainStatKey` | `mainTag.name` | string → ArtifactMainStatName | `"hp"` → `"lifeStatic"`, `"critRate_"` → `"critical"` |
| (derived) | `mainTag.value` | number | Computed from `mainStatKey` + `star` + `level` via lookup table |
| `substats[].key` | `normalTags[].name` | string → ArtifactSubStatName | `"critRate_"` → `"critical"` |
| `substats[].value` | `normalTags[].value` | number | **Needs normalization**: GOOD uses `3.5` (display %), mona uses `0.035` (decimal) for percentage stats |
| `location` | `equip` | string or "" | Direct — **transient property**, used only during import-time kumi grouping; not stored in `IArtifact` |
| `lock` | — | boolean | **IGNORED** — not relevant to optimization |
| `id` | — | — | **IGNORED** — mona assigns its own IDs internally via `idProvider` |
| `substats[].increaseType` | — | number | **IGNORED** — roll count not needed for optimization |

### Ignored Fields

1. **`lock`** — Lock status metadata. Not relevant for artifact optimization. Can be added later if UI needs to display lock state.

2. **`substats[].increaseType`** — Roll count (0-4) for each substat. Not needed for current optimization logic which only uses stat values.

3. **`id`** — GOOD's artifact IDs. The mona system assigns its own IDs via `idProvider.ts`. GOOD IDs are not preserved.

## Character-Based Equip Grouping

Both formats support character-based grouping. GOOD's `location` maps to mona's `equip`, and the existing `importMonaJson()` already handles this:

```typescript
// In importMonaJson() — existing logic, works for both formats:
if (artifact.equip && artifact.equip !== "") {
    const equipCharacter = artifact.equip
    let arr = equips.get(equipCharacter)
    if (arr === undefined) {
        arr = []
        equips.set(equipCharacter, arr)
    }
    arr.push(artifactId)
}
// Later: kumiStore.addKumi(1, equipName, artifacts)
```

The GOOD → mona conversion simply maps `location` → `equip` so this existing logic is reused.

> **Note**: `equip` is a **transient property** on the raw JSON object consumed during import.
> It is **not** part of the `IArtifact` or `IArtifactContentOnly` interfaces. It is only read
> by `importMonaJson()` to build kumi groups, then discarded. The GOOD conversion sets it on
> the raw object so the existing import logic picks it up.

## Mapping Tables

All mappings live as TypeScript objects in `src/utils/goodFormat.ts`.

### 1. Slot Key Mapping (`slotKeyMap`)

| GOOD `slotKey` | mona `position` (ArtifactPosition) |
|----------------|-------------------------------------|
| `"flower"` | `"flower"` |
| `"plume"` | `"feather"` |
| `"sands"` | `"sand"` |
| `"goblet"` | `"cup"` |
| `"circlet"` | `"head"` |

### 2. Stat Name Mapping (`statNameMap`)

Maps GOOD stat keys → mona `ArtifactStatName` (used in `mainTag.name` and `normalTags[].name`).

| GOOD Stat Key | mona ArtifactStatName | Is Percentage? |
|---------------|----------------------|----------------|
| `"hp"` | `"lifeStatic"` | No |
| `"atk"` | `"attackStatic"` | No |
| `"def"` | `"defendStatic"` | No |
| `"hp_"` | `"lifePercentage"` | Yes |
| `"atk_"` | `"attackPercentage"` | Yes |
| `"def_"` | `"defendPercentage"` | Yes |
| `"eleMas"` | `"elementalMastery"` | No |
| `"enerRech_"` | `"recharge"` | Yes |
| `"critRate_"` | `"critical"` | Yes |
| `"critDMG_"` | `"criticalDamage"` | Yes |
| `"heal_"` | `"cureEffect"` | Yes |
| `"pyro_dmg_"` | `"fireBonus"` | Yes |
| `"electro_dmg_"` | `"thunderBonus"` | Yes |
| `"hydro_dmg_"` | `"waterBonus"` | Yes |
| `"cryo_dmg_"` | `"iceBonus"` | Yes |
| `"anemo_dmg_"` | `"windBonus"` | Yes |
| `"geo_dmg_"` | `"rockBonus"` | Yes |
| `"dendro_dmg_"` | `"dendroBonus"` | Yes |
| `"physical_dmg_"` | `"physicalBonus"` | Yes |

**Percentage normalization**: GOOD stores percentage stats as display values (e.g., `critRate_ = 3.5` meaning 3.5%). Mona stores them as decimals (e.g., `critical = 0.035`). The conversion divides by 100 for all stats where `artifactTags[name].percentage === true`.

### 3. Set Name Mapping (`setNameMap`)

Maps GOOD `setKey` → mona `setName` (matching keys in `artifactsData` from `src/assets/_gen_artifact`).

The `_gen_artifact.js` data uses **mixed casing**: older sets (~37 sets) use camelCase (e.g., `"gladiatorFinale"`), while newer sets (~22 sets from EchoesOfAnOffering onward) use PascalCase (e.g., `"EchoesOfAnOffering"`, `"VermillionHereafter"`). The mapping below is verified against the actual `src/assets/_gen_artifact.js` keys.

```typescript
const setNameMap: Record<string, string> = {
    // --- Older sets: GOOD PascalCase → mona camelCase ---
    "Adventurer": "adventurer",
    "ArchaicPetra": "archaicPetra",
    "Berserker": "berserker",
    "BlizzardStrayer": "blizzardStrayer",
    "BloodstainedChivalry": "bloodstainedChivalry",
    "BraveHeart": "braveHeart",
    "CrimsonWitchOfFlames": "crimsonWitch",
    "DefendersWill": "defenderWill",
    "EmblemOfSeveredFate": "emblemOfSeveredFate",
    "Gambler": "gambler",
    "GladiatorsFinale": "gladiatorFinale",
    "HeartOfDepth": "heartOfDepth",
    "HuskOfOpulentDreams": "huskOfOpulentDreams",
    "Instructor": "instructor",
    "Lavawalker": "lavaWalker",
    "LuckyDog": "luckyDog",
    "MaidenBeloved": "maidenBeloved",
    "MartialArtist": "martialArtist",
    "NoblesseOblige": "noblesseOblige",
    "OceanHuedClam": "oceanHuedClam",
    "PaleFlame": "paleFlame",
    "PrayersForDestiny": "prayersForDestiny",
    "PrayersForIllumination": "prayersForIllumination",
    "PrayersForWisdom": "prayersForWisdom",
    "PrayersToSpringtime": "prayersToSpringtime",
    "ResolutionOfSojourner": "resolutionOfSojourner",
    "RetracingBolide": "retracingBolide",
    "Scholar": "scholar",
    "ShimenawasReminiscence": "shimenawaReminiscence",
    "TenacityOfTheMillelith": "tenacityOfTheMillelith",
    "TheExile": "exile",
    "ThunderingFury": "thunderingFury",
    "Thundersoother": "thunderSmoother",
    "TinyMiracle": "tinyMiracle",
    "TravelingDoctor": "travelingDoctor",
    "ViridescentVenerer": "viridescentVenerer",
    "WanderersTroupe": "wandererTroupe",

    // --- Newer sets: GOOD PascalCase = mona PascalCase (identity mapping) ---
    "DeepwoodMemories": "DeepwoodMemories",
    "EchoesOfAnOffering": "EchoesOfAnOffering",
    "FlowerOfParadiseLost": "FlowerOfParadiseLost",
    "DesertPavilionChronicle": "DesertPavilionChronicle",
    "GildedDreams": "GildedDreams",
    "GoldenTroupe": "GoldenTroupe",
    "MarechausseeHunter": "MarechausseeHunter",
    "NymphsDream": "NymphsDream",
    "VermillionHereafter": "VermillionHereafter",
    "VourukashasGlow": "VourukashasGlow",
    "SongOfDaysPast": "SongOfDaysPast",
    "NighttimeWhispersInTheEchoingWoods": "NighttimeWhispersInTheEchoingWoods",
    "FragmentOfHarmonicWhimsy": "FragmentOfHarmonicWhimsy",
    "UnfinishedReverie": "UnfinishedReverie",
    "ScrollOfTheHeroOfCinderCity": "ScrollOfTheHeroOfCinderCity",
    "ObsidianCodex": "ObsidianCodex",
    "LongNightsOath": "LongNightsOath",
    "FinaleOfTheDeepGalleries": "FinaleOfTheDeepGalleries",
    "NightOfTheSkysUnveiling": "NightOfTheSkysUnveiling",
    "SilkenMoonsSerenade": "SilkenMoonsSerenade",
    "ADayCarvedFromRisingWinds": "ADayCarvedFromRisingWinds",
    "AubadeOfMorningstarAndMoon": "AubadeOfMorningstarAndMoon",
}
```

> **Verified**: All values above match the actual keys in `src/assets/_gen_artifact.js`.
> Older sets use camelCase keys; newer sets (from `EchoesOfAnOffering` onward) use
> PascalCase keys identical to the GOOD format. If new sets are added to `_gen_artifact`,
> the mapping must be updated to match.

### 4. Main Stat Value Derivation

GOOD format does **not** include main stat values. These must be derived from:
- `mainStatKey` (stat type)
- `rarity` (star)
- `level`

**Source**: The existing `artifactTags` constant in `src/constants/artifact.ts` has `max` values per star for each stat. For intermediate levels, we need a lookup table.

**Approach**: Create `src/utils/mainStatValues.ts` with a **complete** pre-computed lookup table:
```typescript
// mainStatValues[statName][star][level] = value
// Built from AnimeGameData ReliquaryMainPropExcelConfig.json
export function getMainStatValue(
    statName: ArtifactMainStatName,
    star: number,
    level: number
): number
```

**This is a launch requirement, not post-MVP.** Scanned inventories commonly contain
un-leveled and partially-leveled artifacts (e.g., level 0 fodder, level 16 4-star pieces).
Only supporting 5-star level-20 would silently produce `mainTag.value = 0` for all other
artifacts, causing incorrect optimization results. The full table must cover at minimum
stars 3–5 and all levels (0 to star×4).

Source data: AnimeGameData `ReliquaryLevelExcelConfigData.json` contains main stat growth
curves per `propType` and `rank` (star). A build script should extract these into a static
TypeScript table at development time.

## Implementation Approach

### 1. Format Detection Gateway

**File:** `src/utils/goodFormat.ts`

```typescript
export type ImportFormat = "mona" | "good" | "unknown"

/**
 * Detect whether parsed JSON is mona.json or GOOD format.
 * - GOOD: has `format === "GOOD"` or has `artifacts` array at top level
 * - mona: has any of "flower", "feather", "sand", "cup", "head" keys
 */
export function detectFormat(obj: any): ImportFormat {
    if (obj.format === "GOOD" || (Array.isArray(obj.artifacts) && !obj.flower)) {
        return "good"
    }
    if (obj.flower || obj.feather || obj.sand || obj.cup || obj.head) {
        return "mona"
    }
    return "unknown"
}
```

### 2. GOOD → mona Conversion

**File:** `src/utils/goodFormat.ts`

```typescript
import { artifactTags } from "@/constants/artifact"

/**
 * Convert a GOOD format JSON object into mona.json shape.
 * Returns an object with { flower: [...], feather: [...], ... , equip: [...] }
 * suitable for passing to importMonaJson().
 */
export function convertGoodToMona(goodObj: any): { result: any, skipped: number } {
    const result: any = {
        flower: [],
        feather: [],
        sand: [],
        cup: [],
        head: [],
    }
    let skipped = 0

    for (const artifact of goodObj.artifacts ?? []) {
        // --- Structural validation ---
        if (!artifact || typeof artifact !== "object") {
            console.warn("[GOOD import] skipping non-object artifact entry")
            skipped++
            continue
        }

        // --- Mapping lookups (skip unknown values) ---
        const position = slotKeyMap[artifact.slotKey]
        if (!position) {
            console.warn(`[GOOD import] unknown slotKey: ${artifact.slotKey}`)
            skipped++
            continue
        }

        const setName = setNameMap[artifact.setKey]
        if (!setName) {
            console.warn(`[GOOD import] unknown setKey: ${artifact.setKey}`)
            skipped++
            continue
        }

        const mainStatName = statNameMap[artifact.mainStatKey]
        if (!mainStatName) {
            console.warn(`[GOOD import] unknown mainStatKey: ${artifact.mainStatKey}`)
            skipped++
            continue
        }

        // --- Semantic validation (range checks) ---
        const star = typeof artifact.rarity === "number" ? artifact.rarity : 5
        const level = typeof artifact.level === "number" ? artifact.level : 0
        if (star < 1 || star > 5) {
            console.warn(`[GOOD import] invalid rarity: ${star}`)
            skipped++
            continue
        }
        if (level < 0 || level > star * 4) {
            console.warn(`[GOOD import] invalid level: ${level} for ${star}-star artifact`)
            skipped++
            continue
        }

        const mainStatValue = getMainStatValue(mainStatName, star, level)

        const normalTags = (artifact.substats ?? [])
            .filter((s: any) => {
                if (!s || typeof s !== "object") return false
                if (!s.key || !statNameMap[s.key]) return false
                if (typeof s.value !== "number" || !isFinite(s.value)) return false
                return true
            })
            .map((s: any) => {
                const name = statNameMap[s.key]
                const isPercentage = artifactTags[name]?.percentage ?? false
                return {
                    name,
                    value: isPercentage ? s.value / 100 : s.value,
                }
            })

        const monaArtifact: any = {
            setName,
            position,
            mainTag: { name: mainStatName, value: mainStatValue },
            normalTags,
            star,
            level,
            omit: false,
        }

        // Map location → equip (transient property for kumi grouping only)
        if (artifact.location && artifact.location !== "") {
            monaArtifact.equip = artifact.location
        }

        result[position].push(monaArtifact)
    }

    if (skipped > 0) {
        console.warn(`[GOOD import] ${skipped} artifact(s) skipped due to validation errors`)
    }

    return { result, skipped }
}
```

### 3. Modify Import Entry Point

**File:** `src/pages/ArtifactsPage/ArtifactsPage.vue`

Update `importJson()` to detect format and route accordingly:

```typescript
// NOTE: importMonaJson() is synchronous (returns ImportJsonResult), but the
// existing code uses `async`/`await` on it. This is harmless (awaiting a
// non-Promise returns the value immediately) but is a pre-existing quirk.
function importJson(text: string, deleteUnseen: boolean, backupKumiDir: boolean) {
    try {
        const rawObj = JSON.parse(text)
        const format = detectFormat(rawObj)

        let monaObj: any
        if (format === "good") {
            const { result, skipped } = convertGoodToMona(rawObj)
            monaObj = result
            if (skipped > 0) {
                ElMessage({
                    message: `${skipped} artifact(s) skipped due to invalid data`,
                    type: "warning"
                })
            }
        } else if (format === "mona") {
            monaObj = rawObj
        } else {
            throw new Error("Unknown format")
        }

        importMonaJson(monaObj, deleteUnseen, backupKumiDir)
    } catch (e) {
        ElMessage({
            message: t("artPage.wrongFormat"),
            type: "error"
        })
    }
}
```

### 4. Main Stat Value Lookup

**File:** `src/utils/mainStatValues.ts`

```typescript
import type { ArtifactMainStatName } from "@/types/artifact"

/**
 * Pre-computed main stat values indexed by [statName][star][level].
 * Source: AnimeGameData ReliquaryMainPropExcelConfig.json
 *
 * For 5-star artifacts at level 20:
 *   lifeStatic: 4780, attackStatic: 311,
 *   attackPercentage: 0.466, lifePercentage: 0.466,
 *   defendPercentage: 0.583, elementalMastery: 186.5,
 *   recharge: 0.518, critical: 0.311, criticalDamage: 0.622,
 *   cureEffect: 0.359, fireBonus/waterBonus/...: 0.466,
 *   physicalBonus: 0.583
 */
const mainStatTable: Record<string, Record<number, Record<number, number>>> = {
    // Populated from AnimeGameData
    // mainStatTable["lifeStatic"][5][20] = 4780
    // mainStatTable["lifeStatic"][5][0] = 717
    // ... etc.
}

export function getMainStatValue(
    statName: ArtifactMainStatName,
    star: number,
    level: number
): number {
    return mainStatTable[statName]?.[star]?.[level] ?? 0
}
```

## Files to Create/Modify

| File | Change |
|------|--------|
| `src/utils/goodFormat.ts` | **NEW** — Format detection, mapping tables, GOOD → mona conversion |
| `src/utils/mainStatValues.ts` | **NEW** — Main stat value lookup table by stat/star/level |
| `src/pages/ArtifactsPage/ArtifactsPage.vue` | **MODIFY** — Update `importJson()` to call `detectFormat()` and route accordingly |

### Files NOT Modified

| File | Why |
|------|-----|
| `mona_core/` (any Rust file) | No changes — Rust only receives `IArtifactWasm` via `convertArtifact()` |
| `src/utils/artifacts.ts` | No changes — `importMonaJson()` is reused as-is |
| `src/utils/converter.ts` | No changes — converts `IArtifact` → `IArtifactWasm`, format-agnostic. Note: its `nameMap` only covers 39 older sets, but falls back to identity mapping for unknown names, so newer PascalCase set keys pass through correctly. Pre-existing limitation, not caused by this plan |
| `src/utils/checkImportJson.js` | No changes — **orphaned file**: exported but never imported/called anywhere. Validation for GOOD imports is handled in `convertGoodToMona()` instead |
| `src/types/artifact.ts` | No changes — existing `IArtifact` / `IArtifactContentOnly` types are sufficient |

## Estimated Complexity

| Component | Complexity | Lines of Code |
|-----------|------------|---------------|
| Format detection (`detectFormat`) | Low | ~15 |
| Slot/stat/set mapping tables | Low | ~100 |
| GOOD → mona conversion + validation (`convertGoodToMona`) | Low | ~100 |
| Main stat value lookup table | Medium | ~200 (data-heavy) |
| Entry point modification (`importJson`) | Low | ~15 |
| **Total** | **Low-Medium** | **~430** |

## Validation Steps

All validation for GOOD artifacts is handled inside `convertGoodToMona()`.

> **Important**: `checkImportJson.js` is orphaned — it is **never called** by
> `importMonaJson()` or any other code. Do not rely on it as a safety net.
> All necessary checks must live in `convertGoodToMona()` itself.

**During GOOD → mona conversion** (`convertGoodToMona`):
   - ✅ Each entry is a non-null object (structural check)
   - ✅ `slotKey` exists in `slotKeyMap` → else skip + warn
   - ✅ `setKey` exists in `setNameMap` → else skip + warn
   - ✅ `mainStatKey` exists in `statNameMap` → else skip + warn
   - ✅ `rarity` is a number in range 1–5 → else skip + warn
   - ✅ `level` is a number in range 0 to star×4 → else skip + warn
   - ✅ Each substat is an object with valid `.key` and numeric `.value` → else filtered out
   - ✅ `location` → mapped to transient `equip` property for kumi grouping
   - ✅ Total skipped count returned to caller for user-facing warning

**During mona import** (`importMonaJson` — no additional validation):
   - `importMonaJson()` does not call `checkImportJson()`. It trusts that the
     converted mona-format objects are structurally valid. This is safe because
     `convertGoodToMona()` already validates all fields above.

## Example Usage

```typescript
// User uploads a GOOD format file via the existing Import dialog
// The flow is automatic — no user action needed to select format

// Internal flow:
const text = await fileUploader.getReadPromise()
const rawObj = JSON.parse(text)

// detectFormat checks for "format": "GOOD" or "artifacts" array
const format = detectFormat(rawObj)  // → "good"

// Convert to mona shape (returns { result, skipped })
const { result: monaObj, skipped } = convertGoodToMona(rawObj)
// monaObj = { flower: [...], feather: [...], sand: [...], cup: [...], head: [...] }
// skipped = number of artifacts that failed validation

// Reuse existing import pipeline
importMonaJson(monaObj, deleteUnseen, backupKumiDir)
// → dedup, upgrade detection, equip grouping all handled by existing code
```

## Future Enhancements (Post-MVP)

1. **`lock` status** — Display lock state in UI
   - Add `isLocked: boolean` to `IArtifact` interface
   - Pass through from GOOD conversion

2. **`increaseType`** — Roll count for substats
   - Could be used for more accurate optimization scoring
   - Would need a wrapper or extended `IArtifactTag` type

3. **GOOD export** — Allow exporting artifacts in GOOD format
   - Reverse the mapping tables
   - Add an export menu option alongside existing "export Mona JSON"

4. **Wire up `checkImportJson()`** — The orphaned `checkImportJson.js` could be
   integrated into the mona.json import path for parity with GOOD validation.
   Currently mona.json imports have no schema validation either.

5. **Backfill `converter.ts` `nameMap`** — Only covers 39 older sets. Newer
   PascalCase sets pass through via identity fallback, but explicit entries
   would be safer.

## Known Pre-Existing Issues

These issues exist in the current codebase and are **not introduced by this plan**, but
are documented here for awareness:

1. **`checkImportJson.js` is orphaned** — Exported but never imported/called.
   mona.json imports have no schema validation beyond `artifactStore` logic.

2. **`converter.ts` `nameMap` is incomplete** — Only maps 39 older camelCase
   sets. Falls back to identity mapping for unknown names, so PascalCase keys
   from newer sets pass through correctly. Not a blocking issue.

3. **`async`/`await` on synchronous `importMonaJson()`** — `importJson()` in
   `ArtifactsPage.vue` uses `async function` and `await importMonaJson(...)`,
   but `importMonaJson()` returns `ImportJsonResult` synchronously. Harmless
   (awaiting a non-Promise just returns the value), but inconsistent.
