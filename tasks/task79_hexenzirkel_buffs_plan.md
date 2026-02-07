# Task 79: Implement Hexenzirkel Character Buffs

**Task ID**: 79
**Status**: ✅ Completed
**Priority**: Medium
**Related**: Task 78 (Hexenzirkel System)

## Overview

Implement detailed Hexenzirkel buffs for each of the 7 characters. Each has a unique buff effect when Hexenzirkel is active.

## Character Buffs

### Venti
- **Normal Attacks**: Become Anemo arrows
- **Burst**: Vortex increases on-field character damage after Swirl
- **DSL**: `get_hexenzirkel_buff("Venti")` returns damage multiplier

### Klee
- **Boom Boost**: Stacks from skills/bursts/attacks
- **Charged Attacks**: Empowered when Boom Boost stacks active
- **DSL**: `get_hexenzirkel_buff("Klee")` returns charged attack multiplier

### Albedo
- **Transmuted Argentum**: Skill creates extra blossoms
- **Team Buff**: Nearby allies gain damage bonus based on Albedo's DEF
- **DSL**: `get_hexenzirkel_buff("Albedo")` returns DEF-based damage bonus

### Mona
- **Illusory Torrent**: Enhanced dash
- **Damage Amplification**: Bonus reaction damage amplification
- **DSL**: `get_hexenzirkel_buff("Mona")` returns amp multiplier

### Fischl
- **Oz**: Increased attack frequency
- **C6**: Grants extra Sigils and CRIT boosts
- **DSL**: `get_hexenzirkel_buff("Fischl")` returns attack frequency + crit bonus

### Razor
- **Wolf's Gravestone synergy**: Enhanced constellation effects
- **C6**: Extra Sigils and CRIT boosts
- **DSL**: `get_hexenzirkel_buff("Razor")` returns constellation bonus

### Sucrose
- **Reaction Bonus**: Elemental damage bonuses when triggering reactions
- **Skill/Burst**: Enhanced with Hexenzirkel trait
- **DSL**: `get_hexenzirkel_buff("Sucrose")` returns EM/elemental bonus

## DSL Usage

```mona
// Check Hexenzirkel activation
if team_hexenzirkel_count() >= 2 {
    // Apply character-specific buffs
    let venti_dmg = get_hexenzirkel_buff("Venti")
    let albedo_def_bonus = get_hexenzirkel_buff("Albedo")
    let mona_amp = get_hexenzirkel_buff("Mona")
    
    // Calculate final damage
    dmg = base_dmg * (1 + venti_dmg + albedo_def_bonus + mona_amp)
}
```

## Implementation

### Phase 1: Define Buff Values

**File**: `mona_dsl/src/object/hexenzirkel.rs` (create new)

```rust
pub struct HexenzirkelBuffs {
    pub venti_normal_bonus: f64,
    pub venti_burst_bonus: f64,
    pub klee_charged_bonus: f64,
    pub klee_boom_stacks: u8,
    pub albedo_def_bonus: f64,
    pub mona_amp_bonus: f64,
    pub fischl_oz_frequency: f64,
    pub fischl_crit_bonus: f64,
    pub razor_constellation_bonus: f64,
    pub sucrose_em_bonus: f64,
    pub sucrose_elemental_bonus: f64,
}
```

### Phase 2: Update DSL Functions

**File**: `mona_dsl/src/builtin/global_function.rs`

```rust
pub fn get_hexenzirkel_buff(character: &str, ctx: &mut UnsafeDamageContext) -> f64 {
    // Calculate and return character-specific buff
    // Based on character stats and constellation level
}
```

### Phase 3: Integrate with Damage Calculation

**File**: `mona_dsl/src/vm/env.rs` - `init_damage()`

Apply Hexenzirkel buffs when:
1. `team_hexenzirkel_count() >= 2`
2. Character has `is_hexenzirkel` flag

## Files to Modify

| File | Change |
|------|--------|
| `mona_dsl/src/object/hexenzirkel.rs` | Create - Define buff structures |
| `mona_dsl/src/builtin/global_function.rs` | Implement `get_hexenzirkel_buff()` |
| `mona_dsl/src/vm/env.rs` | Apply buffs in damage calculation |
| `mona_dsl/src/object/mod.rs` | Export new hexenzirkel module |

## Success Criteria

1. ✅ Each character has specific buff value
2. ✅ Buffs scale with constellation level
3. ✅ DSL function returns correct values
4. ✅ Damage calculation includes Hexenzirkel buffs
5. ✅ Works with `team_hexenzirkel_count() >= 2` condition

## References

- KQM Hexenzirkel Guide
- GO Character Sheets for constellation values
- `mona_core/src/character/characters/` for character data

## Branch

`feature/hexenzirkel-buffs-79`
