"""
Character research script for Genshin Impact.
Extracts character data from AnimeGameData and outputs structured research files.

Usage:
    python research_character.py <character_name_or_id> [--output-dir DIR]

Examples:
    python research_character.py Nahida
    python research_character.py 10000073
    python research_character.py Columbina --output-dir .research_info/columbina
"""

import argparse
import sys
import re
from datetime import date
from pathlib import Path

from utils import (
    load_excel, text_en, text_chs, build_index, build_group_index,
    get_ascension_data, get_ascension_stat_bonus, get_ascension_stat_type,
    get_curve_multiplier, get_proud_skill_multipliers, extract_param_list,
    FIGHT_PROP_NAMES, FIGHT_PROP_SHORT, WEAPON_TYPE_NAMES, QUALITY_TO_RARITY,
    RESEARCH_DIR, write_json, write_text, fmt_percent, fmt_multiplier, round_list,
)


def find_character(name_or_id: str) -> dict | None:
    """Find a character by English name, Chinese name, or numeric ID."""
    avatars = load_excel("AvatarExcelConfigData.json")
    # Try numeric ID
    try:
        target_id = int(name_or_id)
        for av in avatars:
            if av.get("id") == target_id:
                return av
    except ValueError:
        pass

    # Try name match (EN or CHS)
    name_lower = name_or_id.lower().strip()
    for av in avatars:
        en = text_en(av.get("nameTextMapHash", 0)).lower()
        chs = text_chs(av.get("nameTextMapHash", 0)).lower()
        if en == name_lower or chs == name_lower:
            return av
    # Partial match fallback
    for av in avatars:
        en = text_en(av.get("nameTextMapHash", 0)).lower()
        if name_lower in en:
            return av
    return None


def get_element_from_depot(depot: dict) -> str:
    """Derive element from skill depot's energy skill."""
    skill_data = load_excel("AvatarSkillExcelConfigData.json")
    skill_idx = build_index(skill_data)
    energy_skill_id = depot.get("energySkill", 0)
    if energy_skill_id and energy_skill_id in skill_idx:
        cost_type = skill_idx[energy_skill_id].get("costElemType", "")
        from utils import ELEMENT_MAP
        return ELEMENT_MAP.get(cost_type, cost_type)
    return "Unknown"


def extract_character_data(avatar: dict) -> dict:
    """Extract all character data from AnimeGameData sources."""
    av_id = avatar["id"]
    name_en = text_en(avatar.get("nameTextMapHash", 0))
    name_chs = text_chs(avatar.get("nameTextMapHash", 0))
    rarity = QUALITY_TO_RARITY.get(avatar.get("qualityType", ""), 4)

    # ── Skill depot ────────────────────────────────────────────────────────
    depot_data = load_excel("AvatarSkillDepotExcelConfigData.json")
    depot_idx = build_index(depot_data)
    skill_depot_id = avatar.get("skillDepotId", 0)
    depot = depot_idx.get(skill_depot_id, {})

    element = get_element_from_depot(depot)
    weapon_type = WEAPON_TYPE_NAMES.get(avatar.get("weaponType", ""), "Unknown")

    # ── Base stats ─────────────────────────────────────────────────────────
    hp_base = avatar.get("hpBase", 0)
    atk_base = avatar.get("attackBase", 0)
    def_base = avatar.get("defenseBase", 0)
    crit_rate = avatar.get("critical", 0.05)
    crit_dmg = avatar.get("criticalHurt", 0.5)
    em = avatar.get("elementMastery", 0)
    er = avatar.get("chargeEfficiency", 1.0)

    # Growth curves
    grow_curves = {}
    for gc in avatar.get("propGrowCurves", []):
        grow_curves[gc["type"]] = gc["growCurve"]

    # ── Ascension data ─────────────────────────────────────────────────────
    promote_data = load_excel("AvatarPromoteExcelConfigData.json")
    promote_id = avatar.get("avatarPromoteId", 0)
    phases = get_ascension_data(promote_data, promote_id)
    ascension_stat = get_ascension_stat_type(phases)

    # Compute Lv90 stats
    curve_data = load_excel("AvatarCurveExcelConfigData.json")
    hp_90 = hp_base * get_curve_multiplier(curve_data, grow_curves.get("FIGHT_PROP_BASE_HP", ""), 90)
    atk_90 = atk_base * get_curve_multiplier(curve_data, grow_curves.get("FIGHT_PROP_BASE_ATTACK", ""), 90)
    def_90 = def_base * get_curve_multiplier(curve_data, grow_curves.get("FIGHT_PROP_BASE_DEFENSE", ""), 90)

    # Add ascension bonuses at phase 6
    asc_hp_bonus = 0
    asc_atk_bonus = 0
    asc_def_bonus = 0
    asc_stat_values = []
    for phase in phases:
        pl = phase.get("promoteLevel", 0)
        hp_add = get_ascension_stat_bonus(phase, "FIGHT_PROP_BASE_HP")
        atk_add = get_ascension_stat_bonus(phase, "FIGHT_PROP_BASE_ATTACK")
        def_add = get_ascension_stat_bonus(phase, "FIGHT_PROP_BASE_DEFENSE")
        asc_bonus = get_ascension_stat_bonus(phase, ascension_stat) if ascension_stat else 0

        if pl == 6:
            asc_hp_bonus = hp_add
            asc_atk_bonus = atk_add
            asc_def_bonus = def_add

        asc_stat_values.append({
            "phase": pl,
            "max_level": phase.get("unlockMaxLevel", 20),
            "hp_add": round(hp_add, 2),
            "atk_add": round(atk_add, 2),
            "def_add": round(def_add, 2),
            "bonus_stat": round(asc_bonus, 4) if asc_bonus else 0,
            "cost_items": phase.get("costItems", []),
            "mora_cost": phase.get("scoinCost", 0),
        })

    hp_90_total = round(hp_90 + asc_hp_bonus, 2)
    atk_90_total = round(atk_90 + asc_atk_bonus, 2)
    def_90_total = round(def_90 + asc_def_bonus, 2)
    asc_bonus_at_6 = asc_stat_values[-1]["bonus_stat"] if asc_stat_values else 0

    # ── Skills ─────────────────────────────────────────────────────────────
    skill_data = load_excel("AvatarSkillExcelConfigData.json")
    skill_idx = build_index(skill_data)
    proud_data = load_excel("ProudSkillExcelConfigData.json")

    skills_info = {}
    skill_ids = [s for s in depot.get("skills", []) if s != 0]
    energy_skill_id = depot.get("energySkill", 0)

    # Build skill order: normal_attack, elemental_skill(s), elemental_burst
    all_skill_ids = skill_ids.copy()
    if energy_skill_id:
        all_skill_ids.append(energy_skill_id)

    skill_categories = ["normal_attack"]
    for i in range(1, len(skill_ids)):
        skill_categories.append(f"elemental_skill{'_' + str(i) if i > 1 else ''}")
    if energy_skill_id:
        skill_categories.append("elemental_burst")

    for i, sid in enumerate(all_skill_ids):
        if sid not in skill_idx:
            continue
        sk = skill_idx[sid]
        category = skill_categories[i] if i < len(skill_categories) else f"skill_{i}"
        proud_group = sk.get("proudSkillGroupId", 0)

        skill_entry = {
            "id": sid,
            "name_en": text_en(sk.get("nameTextMapHash", 0)),
            "name_chs": text_chs(sk.get("nameTextMapHash", 0)),
            "cd": sk.get("cdTime", 0),
            "cost_type": sk.get("costElemType", "None"),
            "cost_val": sk.get("costElemVal", 0),
            "max_charges": sk.get("maxChargeNum", 1),
        }

        # Extract multipliers
        if proud_group:
            entries = get_proud_skill_multipliers(proud_data, proud_group)
            if entries:
                param_lists = extract_param_list(entries)
                skill_entry["multipliers"] = [round_list(pl) for pl in param_lists]
                # Get param descriptions from first entry
                desc_hashes = entries[0].get("paramDescList", [])
                param_descs = []
                for h in desc_hashes:
                    t = text_en(h)
                    if t and not t.startswith("[hash:"):
                        param_descs.append(t)
                skill_entry["param_descriptions"] = param_descs
                skill_entry["proud_skill_group_id"] = proud_group

        skills_info[category] = skill_entry

    # ── Passive talents (from proudSkill via inherentProudSkillList or depot)────
    # Passives are stored in the depot's extraAbilities and the GFFGFBCGBDH field
    passive_talents = []
    # Try the obfuscated field for inherent proud skills
    for field_name in ["GFFGFBCGBDH", "LOAMPGAFLMA", "inherentProudSkillOpens"]:
        inherent = depot.get(field_name, [])
        if inherent:
            for entry in inherent:
                psg = entry.get("proudSkillGroupId", 0)
                if psg:
                    entries = get_proud_skill_multipliers(proud_data, psg)
                    if entries:
                        first = entries[0]
                        passive_talents.append({
                            "proud_skill_group_id": psg,
                            "name_en": text_en(first.get("nameTextMapHash", 0)),
                            "name_chs": text_chs(first.get("nameTextMapHash", 0)),
                            "description": text_en(first.get("descTextMapHash", 0)),
                            "param_list": round_list(first.get("paramList", [])),
                        })

    # ── Constellations ─────────────────────────────────────────────────────
    talent_data = load_excel("AvatarTalentExcelConfigData.json")
    constellation_ids = [t for t in depot.get("talents", []) if t != 0]

    talent_idx = build_index(talent_data, "talentId")
    constellations = []
    for cid in constellation_ids:
        tal = talent_idx.get(cid, {})
        if not tal:
            continue
        constellations.append({
            "level": len(constellations) + 1,
            "talent_id": cid,
            "name_en": text_en(tal.get("nameTextMapHash", 0)),
            "name_chs": text_chs(tal.get("nameTextMapHash", 0)),
            "description": text_en(tal.get("descTextMapHash", 0)),
            "param_list": round_list(tal.get("paramList", [])),
            "open_config": tal.get("openConfig", ""),
        })

    # ── Assemble result ────────────────────────────────────────────────────
    return {
        "id": av_id,
        "key": name_en.lower().replace(" ", "_").replace("'", ""),
        "name_en": name_en,
        "name_chs": name_chs,
        "element": element,
        "weapon_type": weapon_type,
        "rarity": rarity,
        "body_type": avatar.get("bodyType", ""),
        "skill_depot_id": skill_depot_id,
        "base_stats": {
            "hp_base": round(hp_base, 2),
            "atk_base": round(atk_base, 2),
            "def_base": round(def_base, 2),
            "crit_rate": crit_rate,
            "crit_dmg": crit_dmg,
            "elemental_mastery": em,
            "energy_recharge": er,
        },
        "growth_curves": grow_curves,
        "lv90_stats": {
            "hp": hp_90_total,
            "atk": atk_90_total,
            "def": def_90_total,
        },
        "ascension": {
            "promote_id": promote_id,
            "bonus_stat_type": ascension_stat,
            "bonus_stat_name": FIGHT_PROP_NAMES.get(ascension_stat, ascension_stat or "None"),
            "phases": asc_stat_values,
        },
        "skills": skills_info,
        "passive_talents": passive_talents,
        "constellations": constellations,
    }


def generate_markdown(data: dict) -> str:
    """Generate a markdown research document from extracted data."""
    lines = []
    a = lines.append

    a(f"# {data['name_en']} Research\n")
    a(f"> Auto-generated from AnimeGameData on {date.today().isoformat()}")
    a(f"> Manual verification required before implementation.\n")

    # Basic info
    a("## Basic Info")
    a("| Property | Value |")
    a("|----------|-------|")
    a(f"| Name (EN) | {data['name_en']} |")
    a(f"| Name (CHS) | {data['name_chs']} |")
    a(f"| Element | {data['element']} |")
    a(f"| Weapon | {data['weapon_type']} |")
    a(f"| Rarity | {data['rarity']}\u2605 |")
    a(f"| ID | {data['id']} |")
    a(f"| Skill Depot ID | {data['skill_depot_id']} |")
    a("")

    # Base stats
    bs = data["base_stats"]
    lv90 = data["lv90_stats"]
    a("## Base Stats")
    a("| Stat | Base (Lv1) | Lv90 (with ascension) | Growth Curve |")
    a("|------|------------|----------------------|--------------|")
    gc = data["growth_curves"]
    a(f"| HP | {bs['hp_base']} | {lv90['hp']} | {gc.get('FIGHT_PROP_BASE_HP', 'N/A')} |")
    a(f"| ATK | {bs['atk_base']} | {lv90['atk']} | {gc.get('FIGHT_PROP_BASE_ATTACK', 'N/A')} |")
    a(f"| DEF | {bs['def_base']} | {lv90['def']} | {gc.get('FIGHT_PROP_BASE_DEFENSE', 'N/A')} |")
    a(f"| CRIT Rate | {fmt_percent(bs['crit_rate'])} | - | - |")
    a(f"| CRIT DMG | {fmt_percent(bs['crit_dmg'])} | - | - |")
    if bs["elemental_mastery"]:
        a(f"| Elemental Mastery | {bs['elemental_mastery']} | - | - |")
    if bs["energy_recharge"] != 1.0:
        a(f"| Energy Recharge | {fmt_percent(bs['energy_recharge'])} | - | - |")
    a("")

    # Ascension
    asc = data["ascension"]
    a(f"## Ascension (Bonus: {asc['bonus_stat_name']})")
    a("| Phase | Max Level | HP Add | ATK Add | DEF Add | Bonus Stat | Mora |")
    a("|-------|-----------|--------|---------|---------|------------|------|")
    for ph in asc["phases"]:
        bonus = fmt_percent(ph["bonus_stat"]) if ph["bonus_stat"] and "PERCENT" in str(asc.get("bonus_stat_type", "")) or "CRITICAL" in str(asc.get("bonus_stat_type", "")) else str(ph["bonus_stat"])
        a(f"| {ph['phase']} | {ph['max_level']} | {ph['hp_add']} | {ph['atk_add']} | {ph['def_add']} | {bonus} | {ph['mora_cost']:,} |")
    a("")

    # Skills
    a("## Skills\n")
    for cat, sk in data["skills"].items():
        title = cat.replace("_", " ").title()
        a(f"### {title}: {sk['name_en']} ({sk['name_chs']})")
        a(f"- **Skill ID**: {sk['id']}")
        if sk.get("cd"):
            a(f"- **CD**: {sk['cd']}s")
        if sk.get("cost_val") and sk["cost_val"] > 0:
            a(f"- **Energy Cost**: {sk['cost_val']}")
        if sk.get("max_charges", 1) > 1:
            a(f"- **Max Charges**: {sk['max_charges']}")
        a("")

        # Param descriptions + multipliers
        descs = sk.get("param_descriptions", [])
        mults = sk.get("multipliers", [])
        if mults:
            a("#### Multipliers (Lv1 → Lv10 → Lv15)")
            a("")
            # Show a table with descriptions
            for i, mult_row in enumerate(mults):
                desc = descs[i] if i < len(descs) else f"Param {i+1}"
                # Clean HTML tags from descriptions
                desc = re.sub(r'<[^>]+>', '', desc)
                # Truncate long descriptions
                if len(desc) > 60:
                    desc = desc[:57] + "..."
                lv1 = fmt_multiplier(mult_row[0]) if mult_row else "?"
                lv10 = fmt_multiplier(mult_row[9]) if len(mult_row) > 9 else "?"
                lv15 = fmt_multiplier(mult_row[-1]) if mult_row else "?"
                a(f"| {desc} | {lv1} | {lv10} | {lv15} |")
            a("")
    a("")

    # Passive talents
    if data["passive_talents"]:
        a("## Passive Talents\n")
        for pt in data["passive_talents"]:
            a(f"### {pt['name_en']} ({pt['name_chs']})")
            desc = re.sub(r'<[^>]+>', '', pt.get("description", ""))
            a(f"{desc}\n")
            if any(v != 0 for v in pt.get("param_list", [])):
                params = [v for v in pt["param_list"] if v != 0]
                a(f"**Parameters**: {params}\n")
    a("")

    # Constellations
    if data["constellations"]:
        a("## Constellations\n")
        for c in data["constellations"]:
            a(f"### C{c['level']}: {c['name_en']} ({c['name_chs']})")
            desc = re.sub(r'<[^>]+>', '', c.get("description", ""))
            a(f"{desc}\n")
            if any(v != 0 for v in c.get("param_list", [])):
                params = [v for v in c["param_list"] if v != 0]
                a(f"**Parameters**: {params}\n")
    a("")

    # Rust hints
    a("## Rust Implementation Hints\n")
    a("```rust")
    a(f"// Element: {data['element']}")
    a(f"// Weapon: {data['weapon_type']}")
    a(f"// Ascension Stat: {asc['bonus_stat_name']}")
    a(f"// Skill Depot ID: {data['skill_depot_id']}")
    a("```\n")

    a("## References\n")
    a(f"- **AnimeGameData ID**: {data['id']}")
    a(f"- **Skill Depot**: {data['skill_depot_id']}")
    a("")

    return "\n".join(lines)


def generate_rust_hints(data: dict) -> str:
    """Generate Rust code hints for skill multipliers."""
    lines = []
    a = lines.append

    a(f"# {data['name_en']} Rust Data Hints\n")
    a(f"**Source**: AnimeGameData (auto-extracted {date.today().isoformat()})")
    a(f"**Verified**: Manual verification required\n")

    a("## Basic Info")
    a(f"- **Element**: {data['element']}")
    a(f"- **Weapon**: {data['weapon_type']}")
    a(f"- **Star**: {data['rarity']}")
    asc = data["ascension"]
    a(f"- **Ascension Stat**: {asc['bonus_stat_name']}")
    a("")

    a("## Name Locale")
    a("```rust")
    a("name_locale: locale!(")
    a(f'    zh_cn: "{data["name_chs"]}",')
    a(f'    en: "{data["name_en"]}",')
    a(")")
    a("```\n")

    a("## Skill Names")
    a("```rust")
    for cat, sk in data["skills"].items():
        idx = list(data["skills"].keys()).index(cat) + 1
        a(f"skill_name{idx}: locale!(")
        a(f'    zh_cn: "{sk["name_chs"]}",')
        a(f'    en: "{sk["name_en"]}",')
        a("),")
    a("```\n")

    # Multiplier arrays
    a("## Skill Multipliers\n")
    for cat, sk in data["skills"].items():
        title = cat.replace("_", " ").title()
        a(f"### {title} ({sk['name_en']})")
        a("```rust")
        descs = sk.get("param_descriptions", [])
        mults = sk.get("multipliers", [])
        prefix_map = {
            "normal_attack": "normal_dmg",
            "elemental_skill": "e",
            "elemental_burst": "q",
        }
        prefix = prefix_map.get(cat, cat[0])

        for i, mult_row in enumerate(mults):
            desc = descs[i] if i < len(descs) else f"Param {i+1}"
            desc = re.sub(r'<[^>]+>', '', desc).strip()
            if len(desc) > 80:
                desc = desc[:77] + "..."

            var_name = f"{prefix}{i+1}" if cat != "normal_attack" else f"normal_dmg{i+1}"
            rounded = [round(v, 4) for v in mult_row]
            a(f"{var_name}: {rounded},  // {desc}")
        a("```\n")

        if sk.get("cd"):
            a(f"// CD: {sk['cd']}s")
        if sk.get("cost_val") and sk["cost_val"] > 0:
            a(f"// Energy: {sk['cost_val']}")
        a("")

    # Damage enum suggestion
    a("## Damage Enum (suggested)")
    a("```rust")
    enum_name = data["name_en"].replace(" ", "").replace("'", "") + "DamageEnum"
    a(f"damage_enum!(")
    a(f"    {enum_name}")
    for cat, sk in data["skills"].items():
        mults = sk.get("multipliers", [])
        prefix_map = {
            "normal_attack": "Normal",
            "elemental_skill": "E",
            "elemental_burst": "Q",
        }
        prefix = prefix_map.get(cat, "S")
        for i in range(len(mults)):
            a(f"    {prefix}{i+1}")
    a(")")
    a("```\n")

    return "\n".join(lines)


def generate_character_json(data: dict) -> dict:
    """Generate the character JSON in the .research_info format."""
    # Build multipliers in the compact format matching existing files
    skills_compact = {}
    for cat, sk in data["skills"].items():
        entry = {
            "name": sk["name_en"],
        }
        if sk.get("name_chs"):
            entry["name_chs"] = sk["name_chs"]
        if sk.get("cd"):
            entry["cd"] = sk["cd"]
        if sk.get("cost_val") and sk["cost_val"] > 0:
            entry["energy_cost"] = sk["cost_val"]
        if sk.get("max_charges", 1) > 1:
            entry["charges"] = sk["max_charges"]
        if sk.get("multipliers"):
            entry["multipliers"] = sk["multipliers"]
        if sk.get("param_descriptions"):
            # Clean HTML from descriptions
            entry["param_descriptions"] = [
                re.sub(r'<[^>]+>', '', d) for d in sk["param_descriptions"]
            ]
        skills_compact[cat] = entry

    # Build constellations compact
    constellations = {}
    for c in data["constellations"]:
        desc = re.sub(r'<[^>]+>', '', c.get("description", ""))
        constellations[f"c{c['level']}"] = {
            "name_en": c["name_en"],
            "name_chs": c["name_chs"],
            "description": desc,
            "params": [v for v in c.get("param_list", []) if v != 0],
        }

    # Build passives compact
    passives = {}
    for i, pt in enumerate(data["passive_talents"]):
        desc = re.sub(r'<[^>]+>', '', pt.get("description", ""))
        key = f"passive_{i+1}"
        passives[key] = {
            "name_en": pt["name_en"],
            "name_chs": pt["name_chs"],
            "description": desc,
            "params": [v for v in pt.get("param_list", []) if v != 0],
        }

    asc = data["ascension"]

    return {
        "id": data["id"],
        "key": data["key"],
        "name_en": data["name_en"],
        "name_chs": data["name_chs"],
        "element": data["element"],
        "weapon_type": data["weapon_type"],
        "rarity": data["rarity"],
        "base_stats": {
            "hp_base": data["base_stats"]["hp_base"],
            "atk_base": data["base_stats"]["atk_base"],
            "def_base": data["base_stats"]["def_base"],
            "crit_rate": data["base_stats"]["crit_rate"],
            "crit_dmg": data["base_stats"]["crit_dmg"],
            "elemental_mastery": data["base_stats"]["elemental_mastery"],
            "energy_recharge": data["base_stats"]["energy_recharge"],
        },
        "growth_curves": data["growth_curves"],
        "lv90_stats": data["lv90_stats"],
        "ascension": {
            "bonus_stat_type": asc["bonus_stat_type"],
            "bonus_stat_name": asc["bonus_stat_name"],
            "values": [ph["bonus_stat"] for ph in asc["phases"]],
        },
        "skills": skills_compact,
        "passives": passives,
        "constellations": constellations,
    }


def main():
    parser = argparse.ArgumentParser(description="Research a Genshin Impact character from AnimeGameData")
    parser.add_argument("character", help="Character name (EN/CHS) or numeric ID")
    parser.add_argument("--output-dir", "-o", help="Override output directory (default: .research_info/<name>)")
    parser.add_argument("--json-only", action="store_true", help="Only output JSON, skip markdown/rust")
    args = parser.parse_args()

    print(f"[*] Searching for character: {args.character}")
    avatar = find_character(args.character)
    if not avatar:
        print(f"[ERROR] Character not found: {args.character}", file=sys.stderr)
        print("  Try using the exact English name or numeric ID (e.g. 10000073)", file=sys.stderr)
        sys.exit(1)

    name_en = text_en(avatar.get("nameTextMapHash", 0))
    print(f"[*] Found: {name_en} (ID: {avatar['id']})")

    print("[*] Extracting character data...")
    data = extract_character_data(avatar)

    # Determine output directory
    if args.output_dir:
        out_dir = Path(args.output_dir)
    else:
        char_key = data["key"]
        out_dir = RESEARCH_DIR / char_key

    print(f"[*] Output directory: {out_dir}")

    # Write character JSON
    char_json = generate_character_json(data)
    write_json(out_dir / "data" / f"character_{data['key']}.json", char_json)

    # Write auto_data.json (full extracted data)
    write_json(out_dir / "auto_data.json", data)

    if not args.json_only:
        # Write markdown research
        md = generate_markdown(data)
        write_text(out_dir / "research.md", md)

        # Write Rust hints
        rust_hints = generate_rust_hints(data)
        write_text(out_dir / "hhw_data.rs", rust_hints)

    print(f"\n[OK] Character research complete for {name_en}")
    print(f"     Files written to: {out_dir}")


if __name__ == "__main__":
    main()
