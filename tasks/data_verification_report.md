# Data Verification Report

Date: 2026-02-11
Source: Cross-referenced against `AnimeGameData/ExcelBinOutput/` (AvatarExcelConfigData, AvatarPromoteExcelConfigData, WeaponExcelConfigData, WeaponPromoteExcelConfigData, EquipAffixExcelConfigData, ReliquarySetExcelConfigData)

---

## Characters

### Skirk — LOW severity

| Field | Code | Expected | Notes |
|---|---|---|---|
| atk[0] | 27 | 28 (27.93) | Rounding |
| def[0] | 62 | 63 (62.76) | Rounding |

### Escoffier — HIGH severity

| Field | Code | Expected |
|---|---|---|
| hp[0] | 1030 | 1039 |
| def[0] | 63 | 57 |
| weapon_type | Catalyst | **Polearm** |
| sub_stat | ATK288 | **CriticalRate192** |

### Lauma — CRITICAL severity

| Field | Code | Expected |
|---|---|---|
| hp[0] | 10264 | 829 |
| atk[0] | 572 | 20 |
| def[0] | 576 | 52 |
| sub_stat | CriticalRate192 | **ElementalMastery115** |

All stat arrays appear to be fabricated/max-level data from another source.

### Nefer — HIGH severity

| Field | Code | Expected |
|---|---|---|
| hp[0] | 1016 | 989 |
| def[0] | 63 | 62 |
| sub_stat | CriticalRate192 | **CriticalDamage384** |

Tail entries in stat arrays (positions 13-14) contain suspicious values (hp jumps to 10264, atk jumps to 572).

### Ifa — CRITICAL severity

| Field | Code | Expected |
|---|---|---|
| hp[0] | 800 | 845 |
| atk[0] | 45 | 15 |
| def[0] | 45 | 51 |
| star | 5 | **4** |
| sub_stat | CriticalRate192 | **ElementalMastery96** |

### Jahoda — CRITICAL severity

| Field | Code | Expected |
|---|---|---|
| hp[0] | 10264 (constant x15) | 809 |
| atk[0] | 572 (constant x15) | 19 |
| def[0] | 576 (constant x15) | 49 |
| star | 5 | **4** |
| sub_stat | CriticalRate192 | **HealingBonus222** |

All stat arrays are a single constant repeated 15 times (pure placeholder).

### Flins — CRITICAL severity

| Field | Code | Expected |
|---|---|---|
| hp[0] | 9724 | 972 (~10x inflated) |
| atk[0] | 2737 | 27 (~100x inflated) |
| def[0] | 6294 | 63 (~100x inflated) |

Element, weapon, rarity, and sub_stat are correct. Base stat values are scaled up by 10-100x.

### Ineffa — HIGH severity

| Field | Code | Expected |
|---|---|---|
| hp[0] | 1009 | 982 |
| def[0] | 66 | 64 |
| sub_stat | ATK288 | **CriticalRate192** |

Growth curve arrays have repeated/broken values. Skill multiplier data may contain stat base values instead of actual multipliers.

### Aino — HIGH severity

| Field | Code | Expected |
|---|---|---|
| hp[0] | 1003 | 939 |
| def[0] | 63 | 51 |
| sub_stat | ATK240 | **ElementalMastery96** |

### Columbina — CRITICAL severity

| Field | Code | Expected |
|---|---|---|
| hp[0] | 1469 | 1144 |
| atk[0] | 514 | **7** (7.45) |
| def[0] | 62 | 40 |
| sub_stat | HP288 | **CriticalRate192** |

Columbina's real base ATK is 7.45 (one of the lowest in the game, HP-scaling design). Code has 514.

### Dahlia — CRITICAL severity

| Field | Code | Expected |
|---|---|---|
| hp[0] | 10456 (constant x15) | 1049 |
| atk[0] | 587 (constant x15) | 16 |
| def[0] | 630 (constant x15) | 47 |
| star | 5 | **4** |
| weapon_type | Polearm | **Sword** |
| sub_stat | CriticalRate192 | **HP240** |

All stat arrays are placeholder constants. Every field except element is wrong.

### Durin — CRITICAL severity

| Field | Code | Expected |
|---|---|---|
| hp[0] | 14695 (constant x15) | 968 |
| atk[0] | 337 (constant x15) | 27 |
| def[0] | 630 (constant x15) | 64 |
| sub_stat | CriticalRate192 | **CriticalDamage384** |

All stat arrays are placeholder constants.

---

## Weapons

### Azurelight — MEDIUM severity

| Field | Code | Expected |
|---|---|---|
| internal_name | "Sword_Regalis" | **"Sword_OuterSword"** |
| Base ATK family | ATK674 | ATK674 |
| Sub stat | CritRate 4.8% | CritRate 4.8% |

**Passive formula off-by-one:** `refine` is 1-indexed in this codebase. Current formulas produce R2 values at R1.

| Refinement | ATK% in Code | ATK% Expected | CritDMG% in Code | CritDMG% Expected |
|---|---|---|---|---|
| R1 | 30% | **24%** | 50% | **40%** |
| R2 | 36% | **30%** | 60% | **50%** |
| R3 | 42% | **36%** | 70% | **60%** |
| R4 | 48% | **42%** | 80% | **70%** |
| R5 | 54% | **48%** | 90% | **80%** |

Fix: change `0.24 + refine * 0.06` to `0.18 + refine * 0.06`, and `0.40 + refine * 0.10` to `0.30 + refine * 0.10`.

### Nocturne's Curtain Call — CRITICAL severity

| Field | Code | Expected |
|---|---|---|
| weapon_base | ATK608 (Lv1=46) | **ATK542 (Lv1=44)** |
| weapon_sub_stat | CritRate 6.8% | **CritDmg 19.2%** |
| Passive effect | HP% + Skill/Burst DMG scaling | **HP% + Lunar Reaction triggers + Energy recovery** |

The entire passive description and implementation are wrong. Actual passive:
- Static Max HP +10/12/14/16/18%
- Lunar Reaction triggers: +14/16/18/20/22% HP, +60/80/100/120/140% Lunar CRIT DMG, recover 14/15/16/17/18 Energy (every 18s)

### Reliquary of Truth — CRITICAL severity (passive only)

| Field | Code | Expected |
|---|---|---|
| weapon_base | ATK542 | ATK542 (OK) |
| weapon_sub_stat | CritDmg 19.2% | CritDmg 19.2% (OK) |
| Passive effect | HP% + ATK SPD + Spectrum stacks | **CritRate + EM + Lunar-Bloom CritDMG** |

Base stats are correct. The entire passive is wrong. Actual passive:
- Static CRIT Rate +8/10/12/14/16%
- Elemental Skill grants EM +80/100/120/140/160 for 12s
- Lunar-Bloom DMG grants CRIT DMG +24/30/36/42/48% for 4s
- When both effects active simultaneously, both are amplified by 50%

---

## Artifact Sets

### Long Night's Oath — OK

No errors found. 2pc (Plunging +25%) and 4pc (per-stack Plunging +15%, max 5) are correct.

### Finale of the Deep Galleries — OK

No errors found. 2pc (Cryo DMG +15%) and 4pc (Normal/Burst DMG +60%) are correct.

### Night of the Sky's Unveiling — CRITICAL severity

| Field | Code | Expected |
|---|---|---|
| CHS name | 天空之舞 | **穹境示现之夜** |
| internal_id | 99901 | **15041** |
| 2pc effect | CRIT Rate +8% | **Elemental Mastery +80** |
| 4pc effect | Lunar DMG +20% (placeholder) | **Gleaming Moon: Intent — CR +15/30% (Nascent/Ascendant Gleam) + Lunar DMG +10% per unique Gleaming Moon effect** |

### Silken Moon's Serenade — CRITICAL severity

| Field | Code | Expected |
|---|---|---|
| CHS name | 丝绸之月 | **纺月的夜歌** |
| internal_id | 99902 | **15042** |
| 2pc effect | EM +30 | **Energy Recharge +20%** |
| 4pc effect | Lunar DMG +20% (placeholder) | **Gleaming Moon: Devotion — EM +60/120 (Nascent/Ascendant Gleam) + Lunar DMG +10% per unique Gleaming Moon effect** |

### Aubade of Morningstar and Moon — CRITICAL severity

| Field | Code | Expected |
|---|---|---|
| CHS name | 晨星与月之歌 | **晨星与月的晓歌** |
| internal_id | 99904 | **15043** |
| 2pc effect | CRIT DMG +12% | **Elemental Mastery +80** |
| 4pc effect | Lunar DMG +32% (placeholder) | **Off-field Lunar Reaction DMG +20%; at Ascendant Gleam +40% more; removed 3s after going on-field** |

### A Day Carved From Rising Winds — CRITICAL severity (4pc only)

| Field | Code | Expected |
|---|---|---|
| CHS name | 风之时刻 | **风起之日** |
| internal_id | 99903 | **15044** |
| 2pc effect | ATK +18% | ATK +18% (OK) |
| 4pc effect | Plunging DMG +30% | **ATK +25% for 6s after hit; upgraded to also give CR +20% when Witch's Homework is completed** |

---

## Summary

| Category | Total | Correct | Errors |
|---|---|---|---|
| Characters | 12 | 1 (Skirk, minor rounding only) | 11 |
| Weapons | 3 | 0 | 3 |
| Artifacts | 6 | 2 | 4 |
