# Durin Implementation Plan

## Overview
Implement Durin (Pyro/Sword) into genshin_artifact optimizer.

## Reference Implementations
- Skirk (Cryo/Sword) - Already implemented, use as template
- Escoffier (Cryo/Polearm) - Already implemented

## Files to Create/Modify

### 1. Character Data File
**Path**: `libs/gi/dm-localization/assets/locales/en/char_durin_gen.json`
- Based on `char_skirk_gen.json` structure
- Include base stats (Lv90: HP 12430, ATK 346.81, DEF 822.35)
- Include skill multipliers for NA/E/Q

### 2. Rust Character Implementation
**Path**: `libs/gi/optimizer/src/characters/durin.rs` (new)
- Follow `skirk.rs` pattern
- Implement `Durin` struct with character data
- Implement `DurinData` for stat calculations

### 3. Target Function (TargetScorer)
**Key considerations**:
- **No energy system needed** - Durin generates energy on state switch
- **Pyro damage is primary** - Bonus crit DMG from ascension
- **Dual skill modes** - Purity (AoE) and Darkness (Single target)
- **Dragon summons** - Q skill summons follow character

**Weights** (preliminary):
```rust
atk_percentage: 1.0,   // Standard ATK scaling
atk_fixed: 0.5,       // Lower value than percentage
crit_rate: 1.0,       // Standard crit
crit_dmg: 1.0,        // Standard crit
bonus_pyro: 1.5,      // Pyro bonus important
recharge: 0.3,        // Minimal ER needs
```

### 4. Export in mod.rs
**Path**: `libs/gi/optimizer/src/characters/mod.rs`
- Add: `pub mod durin;`
- Add: `Durin => durin::Durin,` in character dispatch

## Implementation Details

### Skill Multipliers (from HHW)

#### Normal Attack (Radiant Wingslash)
- 1-Hit: 45.65% - 122.09%
- 2-Hit: 41% - 109.66%
- 3-Hit: 29.16% + 29.16% - 77.99% + 77.99%
- 4-Hit: 71.15% - 190.29%
- Charged: 113.43% - 303.37%

#### Elemental Skill (Binary Form)
**Confirmation of Purity**:
- DMG: 105.6% - 250.8%
- CD: 12s
- Energy regen: 6-48

**Denial of Darkness**:
- DMG: 72.24% + 53.2% + 64.64% - 171.57% + 126.35% + 153.52%
- 3 consecutive hits

#### Elemental Burst
**Principle of Purity**:
- Burst DMG: 118.96% + 96.4% + 111.84% - 282.53% + 228.95% + 265.62%
- Dragon of White Flame: 94.64% - 224.77%
- Duration: 20s
- Cost: 70

**Principle of Darkness**:
- Burst DMG: 125.44% + 101.76% + 111.84% - 297.92% + 241.68% + 265.62%
- Dragon of Dark Decay: 129.84% - 308.37%
- Duration: 20s
- Cost: 70

## Target Function Logic

### Basic Formula
```rust
let base_damage = base_atk * (1.0 + atk_bonus) + flat_atk;
let crit_damage = base_damage * (1.0 + crit_rate * crit_dmg);
let pyro_bonus = 1.0 + pyro_dmg_bonus;
let final_damage = base_damage * crit_damage * pyro_bonus;
```

### Special Considerations
1. **State switching** - E skill can switch between states
2. **Dragon summon** - Q skill summons companion
3. **No shield mechanic** - Unlike some characters
4. **High base HP** - 12430 at Lv90

## Files to Modify
1. `libs/gi/dm-localization/assets/locales/en/char_durin_gen.json`
2. `libs/gi/dm-localization/assets/locales/chs/char_durin_gen.json`
3. `libs/gi/optimizer/src/characters/durin.rs` (new)
4. `libs/gi/optimizer/src/characters/mod.rs`

## Testing Plan
1. Verify character loads in optimizer
2. Test stat calculations match HHW
3. Verify target function scores artifacts correctly
4. Test with various weapon/artifact combinations
