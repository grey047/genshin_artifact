# Columbina Implementation Plan

## Character Info
| Property | Value |
|----------|-------|
| Name | Columbina |
| Element | Hydro |
| Weapon | Catalyst |
| Rarity | 5★ |

## Key Mechanics
1. **HP-scaling** - All damage scales with Max HP
2. **Lunar Reactions** - New v5.6/6.3 system (LC, LB, LCrys)
3. **Gravity Ripple** - E skill field that follows active character
4. **Verdant Dew** - Resource for charged attacks
5. **Moonsign Passive** - Converts EC/Bloom/Crystallize to Lunar
6. **Ascendant Gleam** - Special team bonus

## Implementation Checklist

### 1. Damage Enum (aino.rs pattern)
```rust
#[derive(Copy, Clone, FromPrimitive, Eq, PartialEq, EnumString, EnumCountMacro)]
pub enum ColumbinaDamageEnum {
    Normal1, Normal2, Normal3,
    Charged1,
    Plunging1, Plunging2, Plunging3,
    E1, // Skill initial (30.10% MaxHP)
    E2, // Gravity Ripple tick (16.85% MaxHP)
    E3_LC,  // Gravity Interference LC (8.47% MaxHP)
    E3_LB,  // Gravity Interference LB (2.53% MaxHP x5)
    E3_LCrys, // Gravity Interference LCrys (15.88% MaxHP)
    Q1, // Burst (58.03% MaxHP)
}

### 2. Character Common Data
- HP: [1469..., etc.] (15 values)
- ATK: [514..., etc.] (15 values)
- Sub Stat: HP% (need to verify)
- Weapon: Catalyst
- Star: 5

### 3. Skill Multipliers (Lv1-15)
From HHW data, extract:
- Normal attack multipliers
- Charged attack multipliers
- Plunge multipliers
- E skill multipliers (Skill DMG, Ripple DMG, Interference DMG)
- Q skill multipliers

### 4. Passives Implementation

#### A1: Lunacy's Lore
- `has_talent2` flag
- On Gravity Interference: +5% CRIT Rate for 10s, max 3 stacks

#### A4: Law of the New Moon
- Team effect within Lunar Domain
- Implementation: Add buff handlers for LC/LB/LCrys specific effects

#### Moonsign Benediction
- Convert EC → LC, Bloom → LB, Hydro Crystallize → LCrys
- Lunar DMG +0.2% per 1000 Max HP (max 7%)

### 5. Target Function

For off-field support:
- **ER** is critical (1.5) — priority #1
- **CRIT Rate** is important (1.2)
- **HP%** is important (1.2)
- **EM** moderate value (0.3)

```rust
fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
    TargetFunctionOptConfig {
        recharge: 1.5,              // ER until requirement
        critical: 1.2,              // CRIT Rate
        hp_percentage: 1.2,         // HP% scaling
        critical_damage: 1.0,        // CRIT DMG
        elemental_mastery: 0.3,       // EM moderate value
        // ... rest of config
    }
}
```

### 6. HP-Scaling Implementation

Unlike aino.rs which uses `builder.add_atk_ratio()`, Columbina needs HP-scaling:

```rust
// For E skill (30.10% MaxHP at Lv10)
damage_builder.add_hp_ratio("E", 0.3010);

// For Q burst (58.03% MaxHP at Lv10)
damage_builder.add_hp_ratio("Q", 0.5803);
```

### 6. Files to Modify/Create

#### New Files
- `mona_core/src/character/characters/hydro/columbina.rs`
- `mona_core/src/target_functions/target_functions/hydro/columbina_default.rs`

#### Modified Files
- `mona_core/src/character/characters/hydro/mod.rs` - Export Columbina
- `mona_core/src/target_functions/target_functions/hydro/mod.rs` - Export target function

## Notes

### HP-scaling Characters Pattern
- Use `MaxHP` instead of base ATK for damage calculations
- Passives often reference `has_talent2`, `has_talent4`, `has_c1`, etc.

### Lunar Reactions Special Handling
- Unlike regular reactions, Lunar Reactions can CRIT
- Need to track Gravity accumulation per reaction type
- Multi-target damage considerations

## Verification Checklist
- [x] Star rating correct (5★)
- [x] Weapon type correct (Catalyst)
- [x] HP-scaling properly implemented (add_hp_ratio)
- [x] Target function weights: ER=1.5, HP%=1.2
- [x] E3 data complete (LC: 8.47%, LB: 2.53%x5, LCrys: 15.88%)
- [ ] A1 passive grants CRIT Rate stacks
- [ ] A4 passive team effects
- [ ] Moonsign passive conversion
- [ ] Compiles without errors
