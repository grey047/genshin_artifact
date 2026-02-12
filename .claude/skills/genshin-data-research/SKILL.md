---
name: genshin-data-research
description: Research and extract Genshin Impact game data (characters, weapons, artifact sets) from AnimeGameData into structured research files.
---

# Genshin Data Research Skill

Extracts game data from the local AnimeGameData repository and generates structured research files for use in the Mona artifact optimizer project.

## Data Source

- **AnimeGameData**: `E:\Moltbot\workspace\AnimeGameData`
  - `ExcelBinOutput/` — JSON game data tables (characters, weapons, artifacts, skills, curves)
  - `TextMap/` — Localization strings (EN, CHS, JP, etc.) keyed by hash integers

## Output Location

All research output goes to `.research_info/` at the project root:

```
.research_info/
├── <character_key>/           # Per-character folders
│   ├── data/
│   │   └── character_<key>.json
│   ├── auto_data.json         # Full extracted data
│   ├── research.md            # Human-readable research doc
│   └── hhw_data.rs            # Rust code hints for implementation
├── weapons/
│   └── <weapon_key>/
│       ├── weapon_<key>.json
│       ├── auto_data.json
│       └── research.md
└── artifact_sets/
    └── <set_key>/
        ├── artifact_<key>.json
        ├── auto_data.json
        └── research.md
```

## Python Scripts

All scripts are in `.claude/skills/genshin-data-research/`:

### 1. Character Research

```bash
python .claude/skills/genshin-data-research/research_character.py <name_or_id>
```

**Arguments:**
- `<name_or_id>` — English name (e.g. `Nahida`), Chinese name, or numeric ID (e.g. `10000073`)
- `--output-dir DIR` — Override output directory
- `--json-only` — Skip markdown/Rust output

**Extracts:**
- Base stats (HP, ATK, DEF, CRIT Rate/DMG, EM, ER)
- Growth curves and Lv90 computed stats
- Ascension phases (materials, stat bonuses per phase)
- All skills (normal attack, elemental skill, elemental burst)
- Skill multipliers across 15 levels from ProudSkillExcelConfigData
- Param descriptions for each multiplier row
- Passive talents with descriptions and parameters
- Constellations (C1-C6) with descriptions and parameters
- EN + CHS localized names for everything

**Outputs:**
- `data/character_<key>.json` — Compact character data
- `auto_data.json` — Full extracted data with all fields
- `research.md` — Markdown research document
- `hhw_data.rs` — Rust code hints (locale macros, multiplier arrays, damage enum)

### 2. Weapon Research

```bash
python .claude/skills/genshin-data-research/research_weapon.py <name_or_id>
python .claude/skills/genshin-data-research/research_weapon.py --list-type Catalyst --min-rarity 4
```

**Arguments:**
- `<name_or_id>` — English name, Chinese name, or numeric ID
- `--list-type TYPE` — List weapons of a type (Sword, Claymore, Polearm, Catalyst, Bow)
- `--min-rarity N` — Minimum rarity filter (default: 1)
- `--output-dir DIR` — Override output directory
- `--json-only` — Skip markdown output

**Extracts:**
- Base ATK and sub stat (type, base value, growth curve)
- Lv90 computed stats with ascension bonuses
- Ascension phases with material costs
- Passive effects from EquipAffixExcelConfigData
- All refinement levels (R1-R5) with parameter arrays
- EN + CHS localized names

**Outputs to** `.research_info/weapons/<weapon_key>/`:
- `weapon_<key>.json` — Compact weapon data
- `auto_data.json` — Full extracted data
- `research.md` — Markdown research document

### 3. Artifact Set Research

```bash
python .claude/skills/genshin-data-research/research_artifact.py <name_or_id>
python .claude/skills/genshin-data-research/research_artifact.py --list --min-rarity 5
```

**Arguments:**
- `<name_or_id>` — English set name, Chinese name, or set ID
- `--list` — List all artifact sets
- `--min-rarity N` — Minimum rarity filter (default: 1)
- `--output-dir DIR` — Override output directory
- `--json-only` — Skip markdown output

**Extracts:**
- Set bonuses (2pc, 4pc) with descriptions and parameter arrays
- Stat bonuses (addProps) for each set bonus
- Open config names (for effect implementation)
- Individual pieces (names, slots, equip types)
- EN + CHS localized names

**Outputs to** `.research_info/artifact_sets/<set_key>/`:
- `artifact_<key>.json` — Compact artifact set data
- `auto_data.json` — Full extracted data
- `research.md` — Markdown research document

### 4. Shared Utilities (`utils.py`)

Common functions used by all scripts:
- `load_excel(filename)` — Load JSON from ExcelBinOutput with caching
- `load_textmap(lang)` — Load TextMap for a language
- `text_en(hash)` / `text_chs(hash)` — Resolve text hashes
- `get_curve_multiplier(...)` — Compute growth curve values
- `get_ascension_data(...)` — Extract ascension phase data
- `get_proud_skill_multipliers(...)` — Extract skill multiplier tables
- `extract_param_list(...)` — Transpose param arrays across levels
- Property mappings: `FIGHT_PROP_NAMES`, `WEAPON_TYPE_NAMES`, `ELEMENT_MAP`, etc.

## AnimeGameData Key Files

| File | Contains |
|------|----------|
| `AvatarExcelConfigData.json` | Character base stats, IDs, growth curves |
| `AvatarSkillDepotExcelConfigData.json` | Links characters to skills, constellations, passives |
| `AvatarSkillExcelConfigData.json` | Skill metadata (CD, energy cost, proud group) |
| `AvatarTalentExcelConfigData.json` | Constellation data (C1-C6 params) |
| `AvatarPromoteExcelConfigData.json` | Ascension materials and stat bonuses |
| `AvatarCurveExcelConfigData.json` | Character growth curves (HP/ATK/DEF per level) |
| `ProudSkillExcelConfigData.json` | Skill multipliers across 15 levels |
| `WeaponExcelConfigData.json` | Weapon base stats, sub stats, affix links |
| `WeaponCurveExcelConfigData.json` | Weapon growth curves |
| `WeaponPromoteExcelConfigData.json` | Weapon ascension data |
| `ReliquarySetExcelConfigData.json` | Artifact set definitions and piece lists |
| `ReliquaryExcelConfigData.json` | Individual artifact piece data |
| `EquipAffixExcelConfigData.json` | Set bonuses AND weapon passives |
| `TextMapEN.json` / `TextMapCHS.json` | English/Chinese localization |

## Data Relationships

```
Avatar → skillDepotId → AvatarSkillDepot
  AvatarSkillDepot → skills[] → AvatarSkill → proudSkillGroupId → ProudSkill (multipliers)
  AvatarSkillDepot → energySkill → AvatarSkill (burst)
  AvatarSkillDepot → talents[] → AvatarTalent (constellations)
  AvatarSkillDepot → inherentProudSkills → ProudSkill (passives)
Avatar → avatarPromoteId → AvatarPromote (ascension)
Avatar → propGrowCurves → AvatarCurve (stat scaling)

Weapon → weaponProp → WeaponCurve (ATK/sub scaling)
Weapon → weaponPromoteId → WeaponPromote (ascension)
Weapon → skillAffix[] → EquipAffix (passive effect)

ReliquarySet → equipAffixId → EquipAffix (set bonuses)
ReliquarySet → containsList[] → Reliquary (individual pieces)
```

## Usage Examples

### Research a new character for implementation
```bash
# Extract all data for Nahida
python .claude/skills/genshin-data-research/research_character.py Nahida

# Check the output
cat .research_info/nahida/research.md
cat .research_info/nahida/hhw_data.rs
```

### Find weapons for a character
```bash
# List all 4★+ catalysts
python .claude/skills/genshin-data-research/research_weapon.py --list-type Catalyst --min-rarity 4

# Research a specific weapon
python .claude/skills/genshin-data-research/research_weapon.py "Sacrificial Jade"
```

### Research artifact sets
```bash
# List all 5★ artifact sets
python .claude/skills/genshin-data-research/research_artifact.py --list --min-rarity 5

# Research a specific set
python .claude/skills/genshin-data-research/research_artifact.py "Crimson Witch of Flames"
```

### Output to a specific character's research folder
```bash
# Put weapon data alongside character research
python .claude/skills/genshin-data-research/research_weapon.py "Staff of Homa" \
    --output-dir .research_info/hu_tao/weapons/staff_of_homa
```

## Notes

- All scripts use only Python standard library (no pip dependencies)
- TextMap files are large (~67MB each); first load may take a few seconds, subsequent lookups are cached
- Some newer game data fields are obfuscated (e.g. `GFFGFBCGBDH`); scripts handle known obfuscated fields
- Multiplier arrays contain 15 values corresponding to skill levels 1-15 (1-10 base + 3 from talent, +2 from constellations)
- Param descriptions may contain HTML tags (e.g. `<color=#FFD780FF>`) which are stripped in output
- Character IDs follow the pattern `10000XXX` where XXX is the character index
