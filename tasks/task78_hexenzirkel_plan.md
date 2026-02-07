# Task 78: Implement Hexenzirkel Buffs System

**Task ID**: 78
**Status**: Pending
**Priority**: Medium
**Related**: Task 77 (Lunar Reactions + Moonsign)

## Overview

Hexenzirkel (魔女会) is a Version 6.2 faction system that provides powerful buffs when 2+ Hexenzirkel characters are in the party.

**Characters with Hexenzirkel buffs:**
- Venti (Anemo/Archon)
- Klee (Pyro/Catalyst)
- Albedo (Geo/Catalyst)
- Mona (Hydro/Catalyst)
- Fischl (Electro/Bow)
- Razor (Electro/Claymore)
- Sucrose (Anemo/Catalyst)

## How Hexenzirkel Works

1. **Trait Gained**: After completing "Witch's Homework" quest
2. **Resonance**: Having 2+ Hexenzirkel characters activates unique buffs
3. **Character-Specific**: Each character gets different enhancements

## Character Buffs (from KQM)

### Venti
- Anemo arrows from normal attacks
- Burst vortex increases on-field character damage (after Swirl)

### Klee
- "Boom Boost" stacks from skills/bursts/attacks
- Empowers Charged Attacks

### Albedo
- Skill creates extra "Transmuted Argentum"
- Nearby allies gain damage bonus based on Albedo's DEF

### Mona
- Enhanced Illusory Torrent (dash)
- Bonus damage amplification

### Fischl
- Oz attack frequency increased
- C6 grants extra Sigils and CRIT boosts

### Razor
- Constellation rework
- C6 grants extra Sigils and CRIT boosts

### Sucrose
- Elemental damage bonuses when triggering reactions
- Skill/Burst enhanced

## Implementation Requirements

### Phase 1: Team Configuration

**Need to extend DSL to support:**
1. Team definition (multiple characters)
2. Hexenzirkel membership tracking
3. Count of Hexenzirkel characters in team

### Phase 2: UI Extensions

**For each Hexenzirkel character in frontend:**
- Add "Hexenzirkel Member" checkbox in character config panel
- Only shown for characters that can be Hexenzirkel members
- Checkbox must be checked to enable Hexenzirkel buffs

**Example UI:**
```
Character Config Panel
├── Weapon Config
├── Constellation Level
└── Hexenzirkel
    └── ☐ Hexenzirkel Member (checkbox)
        └── Tooltip: "Requires completing 'Witch's Homework' quest"
```

### Phase 3: Character Buffs

**For each Hexenzirkel character:**
- Define their specific buff
- Add to DSL builtin functions
- Apply in damage calculations

### Phase 3: Integration

**DSL Usage Example:**
```mona
// Check if team has 2+ Hexenzirkel
if team_hexenzirkel_count() >= 2 {
    // Get character-specific buffs
    let albedo_def_bonus = get_hexenzirkel_buff("Albedo")  // Based on DEF
    let mona_amp_bonus = get_hexenzirkel_buff("Mona")      // Damage amp
    let venti_dmg_bonus = get_hexenzirkel_buff("Venti")    // Anemo bonus
    
    dmg = base_dmg * (1 + albedo_def_bonus + mona_amp_bonus + venti_dmg_bonus)
}
```

## Files to Modify

| File | Change |
|------|--------|
| `mona_dsl/src/vm/env.rs` | Add team context, Hexenzirkel tracking |
| `mona_dsl/src/builtin/global_function.rs` | Add `team_hexenzirkel_count()`, `get_hexenzirkel_buff()` |
| `mona_dsl/src/object/character.rs` | Add Hexenzirkel buff definitions |
| `mona_wasm/src/applications/dsl/dsl_interface.rs` | Pass team data to DSL |
| `genshin_artifact/src/pages/*` (frontend) | Add Hexenzirkel checkbox to character config |
| `genshin_artifact/src/assets/_gen_character.js` | Add `is_hexenzirkel` field for affected characters |

## UI Components

### Frontend Checkbox (for 7 characters)
**Location**: Character config panel (where constellation/level are configured)

**File pattern**: `src/pages/CharacterPage/Components/*Config.vue`

**Implementation**:
```vue
<el-form-item :label="t('hexenzirkel.title')">
    <el-checkbox v-model="characterConfig.isHexenzirkel">
        {{ t('hexenzirkel.member') }}
    </el-checkbox>
    <el-tooltip :content="t('hexenzirkel.tooltip')">
        <i class="el-icon-info"></i>
    </el-tooltip>
</el-form-item>
```

### Generated Config
**File**: `src/assets/_gen_character.js`

**For Venti, Klee, Albedo, Mona, Fischl, Razor, Sucrose:**
```js
configSkill: [
    // ... existing
    {"default":false,"name":"is_hexenzirkel","title":"Hexenzirkel Member","type":"bool"},
]

## Success Criteria

1. ✅ DSL can define team composition
2. ✅ Hexenzirkel characters are marked
3. ✅ Team can check Hexenzirkel count
4. ✅ Character-specific buffs apply correctly
5. ✅ Frontend shows Hexenzirkel checkbox for 7 characters
6. ✅ Checkbox state passed correctly to DSL

## References

- KQM Hexenzirkel Guide
- GO Character Sheets for affected characters
- Team optimization in `mona_wasm/src/applications/team_optimize/`

## Notes

- This is a **team-level** system, not single character
- Requires extending beyond single-character DamageContext
- May need new DSL types for team configuration
- Lower priority than core damage calculations

## Branch

`feature/hexenzirkel-buffs-78`
