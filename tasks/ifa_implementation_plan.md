# Ifa Implementation Plan

## Task Info
- Task ID: 86
- Character: Ifa (Anemo/Catalyst) - NOTE: MEMORY.md incorrectly lists as Dendro
- Research: `/mnt/e/Moltbot/workspace/genshin_artifact/.research_info/ifa/`

## Implementation Files
- Character: `mona_core/src/character/characters/anemo/ifa.rs` (EXISTS - has CORRUPTED data)
- Target Function: `mona_core/src/target_functions/target_functions/anemo/ifa_default.rs` (EXISTS - complete)

## Issues Found

### 1. ifa.rs has CORRUPTED skill multipliers
- Current: `[50.76, 38.43, 130.40...]` (garbage data)
- Research data: `[0.432, 0.467, 0.502...]` (percentages as decimals)

### 2. get_target_function_by_role returns unimplemented!()

## Implementation Checklist
- [ ] Rewrite ifa.rs with correct skill multipliers from research data
- [ ] Implement get_target_function_by_role returning IfaDefaultTargetFunction
- [ ] Verify character data (HP, ATK, DEF from research)
- [ ] Self-review
- [ ] Claude Code review
- [ ] Compilation verification
- [ ] Mark Task 86 complete

## Correct Multipliers from Research
| Skill | Lv1 | Lv9 | Lv10 |
|-------|-----|-----|------|
| Normal 1 | 0.432 (43.2%) | 0.772 (77.2%) | 1.072 (107.2%) |
| E Skill | 1.280 (128%) | 2.288 (228.8%) | 3.476 (347.6%) |
| Q Burst | 4.160 (416%) | 7.424 (742.4%) | 11.168 (1116.8%) |
