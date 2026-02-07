# Columbina Implementation Plan

## Basic Info
- **Name**: Columbina
- **Element**: Hydro
- **Weapon**: Catalyst
- **Rarity**: 5★
- **Research**: `/mnt/e/Moltbot/workspace/genshin_artifact/.research_info/columbina/data.md`

## Implementation Files
- **Character**: `mona_core/src/character/characters/hydro/columbina.rs`
- **Target Function**: `mona_core/src/target_functions/target_functions/hydro/columbina_default.rs`
- **Export**: Update `hydro/mod.rs` and target functions export

## Key Mechanics

### Damage Scaling
- All damage scales with **Max HP** (not ATK)
- Skill DMG: ~30-60% Max HP per hit
- Burst DMG: ~58% Max HP

### Lunar Reactions
- LC (Lunar-Charged): Electro trigger
- LB (Lunar-Bloom): Dendro trigger
- LCrys (Lunar-Crystallize): Geo trigger
- All can crit (unlike transformative reactions)

### Energy Requirements
- Burst cost: 60 Energy
- CD: 15s (Skill) / 17s (Skill trigger)
- ER requirement: 165-300% depending on team

### Stat Priority
```
ER (until req) > CRIT = HP% > EM > Flat HP
```

## Implementation Notes

### Character Config
- `columbina_role`: Off-field Support vs On-field DPS
- `skill_timing`: Skill usage timing for rotation
- `lunar_reaction_type`: LC/LB/LCrys preference

### Target Function
- HP% weighting > CRIT > ER
- EM has lower priority than traditional Hydro
- Support role: ER > HP% > CRIT

## Reference Implementation
- Similar HP-scaling characters: Yelan, Kokomi
- Multi-form reactions: Similar to current Lunar Reactions system

## Task Checklist
- [ ] Verify existing columbina.rs implementation
- [ ] Check if target function matches HP-scaling mechanics
- [ ] Add proper HP-based damage calculations
- [ ] Handle Lunar Reaction state configuration
- [ ] Export to hydro/mod.rs if needed
- [ ] Update target_function_config.rs
- [ ] Self-review
- [ ] Claude Code review
