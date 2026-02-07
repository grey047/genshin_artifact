# Jahoda Implementation Plan

## Task Info
- **Task ID**: 89
- **Character**: Jahoda (Anemo / Bow)
- **Research**: `.research_info/jahoda/`

## Character Info
- **Element**: Anemo
- **Weapon**: Bow
- **Rarity**: 5
- **Base Stats (Lv90)**: HP 10264, ATK 572.38, DEF 576
- **Sub Stat**: CRIT Rate (placeholder)

## Existing Files Status
| File | Status | Notes |
|------|--------|-------|
| `character/jahoda.rs` | EXISTS | Has `unimplemented!()` |
| `target_functions/jahoda_default.rs` | EXISTS | Complete (no unimplemented) |

## Issues to Fix

### 1. Corrupted Skill Multipliers
Current code has garbage data:
```rust
normal_dmg1: [18.70, 48.64, 48.04, ...]  // WRONG!
```

**Correct multipliers from research (Lv1→Lv15)**:
| Skill | Values |
|-------|--------|
| Normal 1 | [0.3740, 0.4038, 0.4336, 0.4769, 0.5067, 0.5365, 0.5798, 0.6231, 0.6664, 0.7097, 0.7530, 0.7963, 0.8396, 0.8829, 0.9262] |
| Normal 2 | [0.3724, 0.4020, 0.4316, 0.4748, 0.5044, 0.5340, 0.5772, 0.6204, 0.6636, 0.7068, 0.7500, 0.7932, 0.8364, 0.8796, 0.9228] |
| Normal 3 | [0.4939, 0.5333, 0.5727, 0.6300, 0.6694, 0.7088, 0.7661, 0.8234, 0.8807, 0.9380, 0.9953, 1.0526, 1.1099, 1.1672, 1.2245] |
| Charged Aimed | [0.4400, 0.4752, 0.5104, 0.5614, 0.5966, 0.6318, 0.6828, 0.7338, 0.7848, 0.8466, 0.9084, 0.9702, 1.0320, 1.0938, 1.1556] |
| Fully Charged | [1.2400, 1.3392, 1.4384, 1.5822, 1.6814, 1.7806, 1.9244, 2.0682, 2.2120, 2.3874, 2.5628, 2.7382, 2.9136, 3.0890, 3.2644] |
| E (Smoke Bomb) | [1.1520, 1.2442, 1.3364, 1.4699, 1.5621, 1.6543, 1.7878, 1.9213, 2.0548, 2.2296, 2.4044, 2.5792, 2.7540, 2.9288, 3.1036] |
| Q (Burst) | [1.7280, 1.8662, 2.0044, 2.2049, 2.3431, 2.4813, 2.6818, 2.8823, 3.0828, 3.3444, 3.6060, 3.8676, 4.1292, 4.3908, 4.6524] |

### 2. Base Stats Wrong
**Current**: HP 1000-range (garbage)
**Correct**: HP 10264, ATK 572, DEF 576

### 3. Plunging Attack Multipliers
Standard Bow values:
- Plunging 1: 0.6393-1.7098
- Plunging 2: 1.2784-3.4189
- Plunging 3: 1.5968-4.2704

### 4. get_target_function_by_role has unimplemented!()
**Current**: `unimplemented!()`
**Fix**: Return `JahodaDefaultTargetFunction` (already exists!)

### 5. Charged Attack Structure
Jahoda has charged attacks (Aimed Shot + Fully Charged). Need to handle:
- `charged_dmg` for normal aimed shot
- `charged_dmg2` for fully charged aimed shot

## Files to Modify
- `mona_core/src/character/characters/anemo/jahoda.rs`

## Implementation Steps
1. Replace all corrupted multipliers with correct values
2. Fix HP/ATK/DEF base stats
3. Add `charged_dmg2` for fully charged aimed shot
4. Implement `get_target_function_by_role`
5. Compile and verify
6. Self-review → Claude Code review

## Branch
`feature/jahoda-20260207`
