# Varesa Implementation Plan

## Task Info
- **Task ID**: 90
- **Character**: Varesa (Electro / Claymore)
- **Research**: `.research_info/varesa/`

## Character Info
- **Element**: Electro
- **Weapon**: Claymore (NOTE: existing code says Catalyst - need to verify)
- **Rarity**: 5
- **Base Stats (Lv90)**: HP 10264, ATK 572.38, DEF 576
- **Special Mechanic**: Fiery Passion state (Passion mode with enhanced attacks)

## Existing Files Status
| File | Status | Notes |
|------|--------|-------|
| `character/varesa.rs` | MOSTLY DONE | Multipliers, damage_internal, VaresaEffect all implemented |
| `target_functions/varesa_default.rs` | EXISTS | `get_target_function_opt_config` is `unimplemented!()` |

## Issues to Fix

### 1. Weapon Type Verification
**Current**: `WeaponType::Catalyst`
**Likely**: `WeaponType::Claymore` (based on fighting style: headbutts, wrestling)

### 2. Implement `get_target_function_by_role` in varesa.rs
**Current**: `unimplemented!()`
**Fix**: Return `VaresaDefaultTargetFunction::new(config)`

### 3. Implement `get_target_function_opt_config` in varesa_default.rs
**Current**: `unplemented!()`
**Fix**: Return proper TargetFunctionOptConfig with:
- `atk_fixed: 0.0` (flat ATK has ~36% value)
- `atk_percentage: 1.0`
- `recharge: 0.5` (N Nightsoul system needs energy)
- `elemental_mastery: 0.3` (Electro reactions bonus)
- `bonus_electro: 1.0` (main damage source)
- `critical: 1.0`
- `critical_damage: 1.0`

## Implementation Steps
1. Verify weapon type (Claymore vs Catalyst)
2. Fix `get_target_function_by_role` in varesa.rs
3. Implement `get_target_function_opt_config` in varesa_default.rs
4. Compile and verify
5. Self-review → Claude Code review

## Files to Modify
- `mona_core/src/character/characters/electro/varesa.rs`
- `mona_core/src/target_functions/target_functions/electro/varesa_default.rs`

## Branch
`feature/varesa-20260207`
