# Lauma Implementation Plan

## Task Info
- Task ID: 87
- Character: Lauma (Dendro/Catalyst)
- Research: `/mnt/e/Moltbot/workspace/genshin_artifact/.research_info/lauma/`

## Implementation Files
- Character: `mona_core/src/character/characters/dendro/lauma.rs` (EXISTS - has CORRUPTED data + unimplemented!())
- Target Function: `mona_core/src/target_functions/target_functions/dendro/lauma_default.rs` (EXISTS - complete)

## Issues Found

### 1. lauma.rs has CORRUPTED skill multipliers
- Current: `[19.85, 52.05, 200.00...]` (garbage data)
- Research data: `[0.432, 0.467, 0.502...]` (percentages as decimals)

### 2. get_target_function_by_role returns unimplemented!()

### 3. Complex mechanics not implemented
- Verdant Dew system
- Moon Song stacks
- Pale Hymn stacks (Lunar-Bloom conversion)
- Moonsign levels
- These require team context - may need TODO comments

## Implementation Checklist
- [ ] Rewrite lauma.rs with correct skill multipliers from research data
- [ ] Implement get_target_function_by_role returning LaumaDefaultTargetFunction
- [ ] Verify character data (HP, ATK, DEF from research)
- [ ] Add TODO comments for complex team-buffing mechanics
- [ ] Self-review
- [ ] Claude Code review
- [ ] Compilation verification
- [ ] Mark Task 87 complete

## Correct Multipliers from Research
| Skill | Values (Lv1-Lv15) |
|-------|-------------------|
| Normal 1 | `[0.432, 0.467, 0.502, 0.552, 0.587, 0.622, 0.672, 0.722, 0.772, 0.822, 0.872, 0.922, 0.972, 1.022, 1.072]` |
| Normal 2 | `[0.384, 0.415, 0.446, 0.491, 0.522, 0.553, 0.598, 0.643, 0.688, 0.733, 0.778, 0.823, 0.868, 0.913, 0.958]` |
| Normal 3 | `[0.528, 0.570, 0.612, 0.673, 0.715, 0.757, 0.819, 0.881, 0.943, 1.005, 1.067, 1.129, 1.191, 1.253, 1.315]` |
| E (Tap) | `[2.048, 2.214, 2.380, 2.618, 2.784, 2.950, 3.188, 3.426, 3.664, 3.974, 4.284, 4.594, 4.904, 5.214, 5.524]` |
| E (Hold 1) | `[2.560, 2.768, 2.976, 3.272, 3.480, 3.688, 3.984, 4.280, 4.576, 4.968, 5.360, 5.752, 6.144, 6.536, 6.928]` |
| E (Hold 2) | `[1.280, 1.382, 1.484, 1.634, 1.736, 1.838, 1.988, 2.138, 2.288, 2.486, 2.684, 2.882, 3.080, 3.278, 3.476]` |
| Q Burst | `[4.160, 4.496, 4.832, 5.312, 5.648, 5.984, 6.464, 6.944, 7.424, 8.048, 8.672, 9.296, 9.920, 10.544, 11.168]` |

## Correct Base Stats from Research
| Stat | Value |
|------|-------|
| HP (Lv90) | 10264 |
| ATK (Lv90) | 572 |
| DEF (Lv90) | 576 |
| Sub Stat | CriticalRate192 |
| Weapon | Catalyst |
| Element | Dendro |
