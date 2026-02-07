# Nefer Implementation Plan

## Character Info
- **Name**: Nefer
- **Element**: Dendro
- **Weapon**: **Catalyst** (NOT Polearm - confirmed from Fandom Wiki)
- **Rarity**: 5
- **Base Stats (Lv90)**: HP 10264, ATK 572.38, DEF 576
- **Sub Stat**: CRIT Rate (placeholder)

## Issues to Fix

### 1. Weapon Type CORRECT (Catalyst)
The current code says Catalyst - this is CORRECT. Nefer uses kicks (Catalyst behavior).

### 2. Corrupted Skill Multipliers
Current code has garbage data:
```rust
normal_dmg1: [26.81, 62.22, 100.00, ...]  // WRONG!
```

**Correct multipliers from Fandom Wiki research**:
| Skill | Lv1 | Lv9 | Lv10 |
|-------|-----|-----|------|
| Normal 1 | 0.648 | 1.153 | 1.597 |
| Normal 2 | 0.576 | 1.024 | 1.420 |
| Normal 3 | 0.792 | 1.408 | 1.954 |
| Normal 4 | 0.896 | 1.600 | 2.224 |
| Charged | 1.472 | 2.624 | 3.956 |
| E (Senet) | 3.072 | 5.472 | 8.244 |
| Q (Sacred) | 4.992 | 8.928 | 13.464 |

### 3. Base Stats Wrong
**Current**: HP 1016, ATK 27, DEF 63 (completely wrong)
**Correct**: HP 10264, ATK 572, DEF 576

### 4. get_target_function_by_role has unimplemented!()
**Current**: `unimplemented!()`
**Fix**: Return `NeferDefaultTargetFunction` (already exists!)

### 5. Plunging Attack Multipliers
Standard Catalyst values:
- Plunging 1: 0.6393-1.7098
- Plunging 2: 1.2784-3.4189
- Plunging 3: 1.5968-4.2704

## Files to Modify
- `mona_core/src/character/characters/dendro/nefer.rs`

## Implementation Steps
1. Fix `weapon_type` from Catalyst → Polearm
2. Replace all skill multipliers with correct values
3. Fix HP/ATK/DEF base stats
4. Implement `get_target_function_by_role`
5. Compile and verify
6. Self-review → Claude Code review

## Branch
`feature/nefer-20260207`
