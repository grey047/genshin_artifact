# Task 76: Lunar Reactions & Moonsign System Implementation

**Task ID**: 92
**Related**: Lunar Reactions Guide + Damage Formulas

## References

**KQM Guide (Lunar Reactions System)**:
- https://keqingmains.com/misc/lunar-reactions/

**Damage Formulas (User-Provided)**:
- Reaction LC: https://kqm-uploads.keqingmains.com/wp-content/uploads/2026/01/Reaction-LC.webp
- LC Final: https://kqm-uploads.keqingmains.com/wp-content/uploads/2026/01/LC-Final.webp
- Direct LC: https://kqm-uploads.keqingmains.com/wp-content/uploads/2026/01/Direct-LC.webp
- Direct LB: https://kqm-uploads.keqingmains.com/wp-content/uploads/2026/01/Direct-LB.webp
- Reaction LCrys: https://kqm-uploads.keqingmains.com/wp-content/uploads/2026/01/Reaction-LCr.webp
- LCrys Final: https://kqm-uploads.keqingmains.com/wp-content/uploads/2026/01/LCr-Final.webp
- Direct LCrys: https://kqm-uploads.keqingmains.com/wp-content/uploads/2026/01/Direct-LCr.webp
- EM Bonus: https://kqm-uploads.keqingmains.com/wp-content/uploads/2026/01/EM-Bonus.webp

## Overview

Implement Lunar Reactions system for genshin_artifact, including:
1. Three new reaction types: Lunar-Charged, Lunar-Bloom, Lunar-Crystallize
2. Moonsign configuration (Level 1/2)
3. Moonsign support for 6 characters

## Characters Requiring Updates

| Character | Element | Has Moonsign | Reaction Enabled |
|-----------|---------|--------------|------------------|
| Flins | Electro | Yes | Lunar-Charged |
| Ineffa | Electro | Yes | Lunar-Charged |
| Columbina | Hydro | Yes | All three |
| Lauma | Dendro | Yes | Lunar-Bloom |
| Nefer | Dendro | Yes | Lunar-Bloom |
| Aino | Hydro | Yes | Moonsign Lv2 effect |

## Implementation Steps

### Phase 1: Core System Changes

#### 1.1 Add New Reaction Types
**Files to modify:**
- `mona_core/src/reaction/reaction.rs` - Add reaction types
- `mona_core/src/reaction/reaction_data.rs` - Add reaction data

**New Reactions:**
```
LunarCharged    // Electro + Hydro
LunarBloom      // Hydro + Dendro
LunarCrystallize // Geo + Hydro
```

#### 1.2 Add Moonsign Configuration
**Files to modify:**
- `mona_core/src/character/character_config.rs`
  - Add `moonsign_level: u8` field (0, 1, 2)
  - Add `has_moonsign_benediction: bool` field

#### 1.3 Add Damage Formulas
**Files to modify:**
- `mona_core/src/damage/damage_builder.rs`
- `mona_core/src/damage/simple_damage_builder.rs`

**Formulas:**
- Reaction LC: Individual × ranking weighting (1st×1, 2nd×½, others×1/12)
- Reaction LCrys: Same as LC but multiplier 0.96
- Direct LC: multiplier 3
- Direct LCrys: multiplier 1.6
- EM Bonus: Apply to all three reactions

#### 1.4 Add Level Multiplier
**From KQM Guide:**
| Level | Multiplier |
|-------|------------|
| 90 | 1561.47 |
| 100 | 1674.81 |

Add lookup table for Lunar Reaction level scaling.

### Phase 2: Character Updates

#### 2.1 Flins (Electro/Catalyst)
**Files:**
- `mona_core/src/character/characters/electro/flins.rs`
- `mona_core/src/target_functions/target_functions/electro/flins_default.rs`

**Changes:**
- Add `moonsign_level` to config
- Add `has_moonsign_benediction: true`
- Update target function with moonsign parameter
- Apply Lunar-Charged bonuses

#### 2.2 Ineffa (Electro/Polearm)
**Files:**
- `mona_core/src/character/characters/electro/ineffa.rs`
- `mona_core/src/target_functions/target_functions/electro/ineffa_default.rs`

**Changes:**
- Add `moonsign_level` to config
- Add `has_moonsign_benediction: true`
- Update target function with moonsign parameter
- Apply Lunar-Charged bonuses (Direct LC damage)

#### 2.3 Columbina (Hydro/Catalyst)
**Files:**
- `mona_core/src/character/characters/hydro/columbina.rs`
- `mona_core/src/target_functions/target_functions/hydro/columbina_default.rs`

**Changes:**
- Add `moonsign_level` to config
- Add `has_moonsign_benediction: true`
- Update target function with moonsign parameter
- Apply all three Lunar Reaction bonuses (LC, LB, LCrys)

#### 2.4 Lauma (Dendro/Catalyst)
**Files:**
- `mona_core/src/character/characters/dendro/lauma.rs`
- `mona_core/src/target_functions/target_functions/dendro/lauma_default.rs`

**Changes:**
- Add `moonsign_level` to config
- Add `has_moonsign_benediction: true`
- Update target function with moonsign parameter
- Apply Lunar-Bloom bonuses (Direct LB damage)

#### 2.5 Nefer (Dendro/Catalyst)
**Files:**
- `mona_core/src/character/characters/dendro/nefer.rs`
- `mona_core/src/target_functions/target_functions/dendro/nefer_default.rs`

**Changes:**
- Add `moonsign_level` to config
- Add `has_moonsign_benediction: true`
- Update target function with moonsign parameter
- Apply Lunar-Bloom bonuses

#### 2.6 Aino (Hydro/Bow)
**Files:**
- `mona_core/src/character/characters/hydro/aino.rs`
- `mona_core/src/target_functions/target_functions/hydro/aino_default.rs`

**Changes:**
- Add `moonsign_level` to config
- Add `has_moonsign_benediction: true`
- Update target function with moonsign parameter
- Apply Moonsign Lv2 effect (increased AoE and attack frequency on Burst)

### Phase 3: Artifact Updates (Optional - Future)

**Artifact sets affected by Moonsign:**
- Night of the Sky's Unveiling (4pc effect changes based on Moonsign Level)
- Silken Moon's Serenade (4pc effect changes based on Moonsign Level)

**Note**: These can be implemented in a separate task.

## Branch
`feature/lunar-reactions-76`

## Dependencies

None - this is a core system addition.

## Testing

1. Verify `cargo check --lib` passes
2. Verify all 6 characters compile with new config
3. Verify target functions accept moonsign_level parameter

## Notes

- Moonsign Team Bonus (non-Moonsign characters providing Lunar Reaction DMG Bonus) is a team-level buff - may require additional team configuration that is out of scope for this task
- Focus on character-level Moonsign effects and Lunar Reaction damage calculations
