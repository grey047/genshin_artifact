"""
Weapon research script for Genshin Impact.
Extracts weapon data from AnimeGameData and outputs structured research files.

Usage:
    python research_weapon.py <weapon_name_or_id> [--output-dir DIR]
    python research_weapon.py --list-type Catalyst
    python research_weapon.py --list-rarity 5

Examples:
    python research_weapon.py "Staff of Homa"
    python research_weapon.py 13501
    python research_weapon.py --list-type Catalyst --min-rarity 4
"""

import argparse
import sys
import re
from datetime import date
from pathlib import Path

from utils import (
    load_excel, text_en, text_chs, build_index, build_group_index,
    get_curve_multiplier, get_ascension_data, get_ascension_stat_bonus,
    FIGHT_PROP_NAMES, FIGHT_PROP_SHORT, WEAPON_TYPE_NAMES,
    RESEARCH_DIR, write_json, write_text, fmt_percent, fmt_multiplier, round_list,
)


def get_weapon_rarity(weapon: dict) -> int:
    """Get weapon rarity from rankLevel field."""
    return weapon.get("rankLevel", 0)


def find_weapon(name_or_id: str) -> dict | None:
    """Find a weapon by English name, Chinese name, or numeric ID."""
    weapons = load_excel("WeaponExcelConfigData.json")

    try:
        target_id = int(name_or_id)
        for w in weapons:
            if w.get("id") == target_id:
                return w
    except ValueError:
        pass

    name_lower = name_or_id.lower().strip()
    for w in weapons:
        en = text_en(w.get("nameTextMapHash", 0)).lower()
        chs = text_chs(w.get("nameTextMapHash", 0)).lower()
        if en == name_lower or chs == name_lower:
            return w

    # Partial match
    for w in weapons:
        en = text_en(w.get("nameTextMapHash", 0)).lower()
        if name_lower in en and en:
            return w
    return None


def list_weapons(weapon_type: str = None, min_rarity: int = 1) -> list[dict]:
    """List weapons, optionally filtered by type and rarity."""
    weapons = load_excel("WeaponExcelConfigData.json")
    results = []
    for w in weapons:
        rarity = get_weapon_rarity(w)
        if rarity < min_rarity:
            continue
        wtype = WEAPON_TYPE_NAMES.get(w.get("weaponType", ""), "")
        if weapon_type and wtype.lower() != weapon_type.lower():
            continue
        name = text_en(w.get("nameTextMapHash", 0))
        if not name or name.startswith("[hash:"):
            continue
        results.append({
            "id": w["id"],
            "name": name,
            "type": wtype,
            "rarity": rarity,
        })
    results.sort(key=lambda x: (-x["rarity"], x["name"]))
    return results


def extract_weapon_data(weapon: dict) -> dict:
    """Extract all weapon data from AnimeGameData sources."""
    w_id = weapon["id"]
    name_en = text_en(weapon.get("nameTextMapHash", 0))
    name_chs = text_chs(weapon.get("nameTextMapHash", 0))
    desc_en = text_en(weapon.get("descTextMapHash", 0))
    rarity = get_weapon_rarity(weapon)
    weapon_type = WEAPON_TYPE_NAMES.get(weapon.get("weaponType", ""), "Unknown")

    # ── Base stats from weaponProp ─────────────────────────────────────────
    base_atk = 0
    base_atk_curve = ""
    sub_stat_type = None
    sub_stat_value = 0
    sub_stat_curve = ""

    for prop in weapon.get("weaponProp", []):
        pt = prop.get("propType", "")
        init_val = prop.get("initValue", 0)
        curve = prop.get("type", "")
        if pt == "FIGHT_PROP_BASE_ATTACK":
            base_atk = init_val
            base_atk_curve = curve
        elif pt and init_val > 0:
            sub_stat_type = pt
            sub_stat_value = init_val
            sub_stat_curve = curve

    # Compute Lv90 base ATK
    curve_data = load_excel("WeaponCurveExcelConfigData.json")
    atk_90_base = base_atk * get_curve_multiplier(curve_data, base_atk_curve, 90) if base_atk_curve else base_atk

    sub_90_base = 0
    if sub_stat_curve and sub_stat_value:
        sub_90_base = sub_stat_value * get_curve_multiplier(curve_data, sub_stat_curve, 90)

    # ── Ascension data ─────────────────────────────────────────────────────
    promote_data = load_excel("WeaponPromoteExcelConfigData.json")
    promote_id = weapon.get("weaponPromoteId", 0)
    phases = [p for p in promote_data if p.get("weaponPromoteId") == promote_id]
    phases.sort(key=lambda x: x.get("promoteLevel", 0))

    atk_asc_bonus = 0
    sub_asc_bonus = 0
    ascension_phases = []
    for phase in phases:
        pl = phase.get("promoteLevel", 0)
        atk_add = get_ascension_stat_bonus(phase, "FIGHT_PROP_BASE_ATTACK")
        sub_add = get_ascension_stat_bonus(phase, sub_stat_type) if sub_stat_type else 0

        if pl == 6:
            atk_asc_bonus = atk_add
            sub_asc_bonus = sub_add

        ascension_phases.append({
            "phase": pl,
            "max_level": phase.get("unlockMaxLevel", 20),
            "atk_add": round(atk_add, 2),
            "sub_add": round(sub_add, 4),
            "cost_items": phase.get("costItems", []),
            "mora_cost": phase.get("coinCost", 0),
        })

    atk_90_total = round(atk_90_base + atk_asc_bonus, 2)
    sub_90_total = round(sub_90_base + sub_asc_bonus, 4)

    # ── Passive effect from EquipAffixExcelConfigData ──────────────────────
    affix_data = load_excel("EquipAffixExcelConfigData.json")
    skill_affixes = weapon.get("skillAffix", [])
    passive_effects = []

    for affix_id in skill_affixes:
        if affix_id == 0:
            continue
        # Find all refinement levels for this affix
        affix_entries = [a for a in affix_data if a.get("id") == affix_id]
        affix_entries.sort(key=lambda x: x.get("level", 0))

        for entry in affix_entries:
            passive_effects.append({
                "affix_id": entry.get("affixId", 0),
                "refinement": entry.get("level", 0) + 1,
                "name_en": text_en(entry.get("nameTextMapHash", 0)),
                "name_chs": text_chs(entry.get("nameTextMapHash", 0)),
                "description": re.sub(r'<[^>]+>', '', text_en(entry.get("descTextMapHash", 0))),
                "param_list": round_list(entry.get("paramList", [])),
                "add_props": [
                    {
                        "type": p.get("propType", ""),
                        "name": FIGHT_PROP_NAMES.get(p.get("propType", ""), p.get("propType", "")),
                        "value": p.get("value", 0),
                    }
                    for p in entry.get("addProps", [])
                    if p.get("value", 0) != 0
                ],
            })

    return {
        "id": w_id,
        "key": name_en.lower().replace(" ", "_").replace("'", "").replace('"', ""),
        "name_en": name_en,
        "name_chs": name_chs,
        "description": re.sub(r'<[^>]+>', '', desc_en),
        "weapon_type": weapon_type,
        "rarity": rarity,
        "base_atk": round(base_atk, 2),
        "base_atk_curve": base_atk_curve,
        "sub_stat": {
            "type": sub_stat_type,
            "name": FIGHT_PROP_NAMES.get(sub_stat_type, sub_stat_type or "None"),
            "base_value": round(sub_stat_value, 4),
            "curve": sub_stat_curve,
        } if sub_stat_type else None,
        "lv90_stats": {
            "atk": atk_90_total,
            "sub_stat_value": sub_90_total if sub_stat_type else 0,
        },
        "ascension": {
            "promote_id": promote_id,
            "phases": ascension_phases,
        },
        "passive_effects": passive_effects,
    }


def generate_weapon_json(data: dict) -> dict:
    """Generate compact weapon JSON for .research_info format."""
    sub = data.get("sub_stat")
    result = {
        "name": data["name_en"],
        "name_chs": data["name_chs"],
        "rarity": data["rarity"],
        "type": data["weapon_type"],
        "base_atk": data["lv90_stats"]["atk"],
        "sub_stat": sub["name"] if sub else "None",
        "sub_value": data["lv90_stats"]["sub_stat_value"] if sub else 0,
    }

    # Add passive from R1
    r1_effects = [e for e in data["passive_effects"] if e["refinement"] == 1]
    if r1_effects:
        result["passive_name"] = r1_effects[0]["name_en"]
        result["passive"] = r1_effects[0]["description"]
        result["passive_params_r1"] = r1_effects[0]["param_list"]

    # Add all refinement params
    all_params = {}
    for e in data["passive_effects"]:
        all_params[f"r{e['refinement']}"] = e["param_list"]
    if all_params:
        result["refinement_params"] = all_params

    return result


def generate_markdown(data: dict) -> str:
    """Generate weapon research markdown."""
    lines = []
    a = lines.append

    a(f"# {data['name_en']} Research\n")
    a(f"> Auto-generated from AnimeGameData on {date.today().isoformat()}\n")

    a("## Basic Info")
    a("| Property | Value |")
    a("|----------|-------|")
    a(f"| Name (EN) | {data['name_en']} |")
    a(f"| Name (CHS) | {data['name_chs']} |")
    a(f"| Type | {data['weapon_type']} |")
    a(f"| Rarity | {data['rarity']}\u2605 |")
    a(f"| ID | {data['id']} |")
    a("")

    a("## Stats")
    a("| Stat | Base (Lv1) | Lv90 |")
    a("|------|------------|------|")
    a(f"| Base ATK | {data['base_atk']} | {data['lv90_stats']['atk']} |")
    sub = data.get("sub_stat")
    if sub:
        is_pct = "PERCENT" in sub["type"] or "CRITICAL" in sub["type"] or "CHARGE" in sub["type"] or "ADD_HURT" in sub["type"] or "HEAL" in sub["type"]
        base_fmt = fmt_percent(sub["base_value"]) if is_pct else str(round(sub["base_value"], 1))
        lv90_fmt = fmt_percent(data["lv90_stats"]["sub_stat_value"]) if is_pct else str(round(data["lv90_stats"]["sub_stat_value"], 1))
        a(f"| {sub['name']} | {base_fmt} | {lv90_fmt} |")
    a("")

    if data["passive_effects"]:
        a("## Passive Effect\n")
        r1 = [e for e in data["passive_effects"] if e["refinement"] == 1]
        if r1:
            a(f"**{r1[0]['name_en']}** ({r1[0]['name_chs']})\n")
            a(f"{r1[0]['description']}\n")

        a("### Refinement Scaling")
        a("| R | Parameters |")
        a("|---|-----------|")
        for e in data["passive_effects"]:
            params = [v for v in e["param_list"] if v != 0]
            a(f"| R{e['refinement']} | {params} |")
        a("")

    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(description="Research a Genshin Impact weapon from AnimeGameData")
    parser.add_argument("weapon", nargs="?", help="Weapon name (EN/CHS) or numeric ID")
    parser.add_argument("--output-dir", "-o", help="Override output directory")
    parser.add_argument("--list-type", help="List weapons of a specific type")
    parser.add_argument("--min-rarity", type=int, default=1, help="Minimum rarity filter")
    parser.add_argument("--json-only", action="store_true", help="Only output JSON")
    args = parser.parse_args()

    if args.list_type:
        print(f"[*] Listing {args.list_type} weapons (rarity >= {args.min_rarity})")
        results = list_weapons(args.list_type, args.min_rarity)
        print(f"\n{'ID':<12} {'Rarity':<8} {'Name'}")
        print("-" * 50)
        for r in results:
            print(f"{r['id']:<12} {r['rarity']}\u2605{'':>5} {r['name']}")
        print(f"\nTotal: {len(results)} weapons")
        return

    if not args.weapon:
        parser.error("weapon name/ID is required (or use --list-type)")

    print(f"[*] Searching for weapon: {args.weapon}")
    weapon = find_weapon(args.weapon)
    if not weapon:
        print(f"[ERROR] Weapon not found: {args.weapon}", file=sys.stderr)
        sys.exit(1)

    name_en = text_en(weapon.get("nameTextMapHash", 0))
    print(f"[*] Found: {name_en} (ID: {weapon['id']})")

    print("[*] Extracting weapon data...")
    data = extract_weapon_data(weapon)

    # Output directory - goes under weapon subfolder
    if args.output_dir:
        out_dir = Path(args.output_dir)
    else:
        weapon_key = data["key"]
        out_dir = RESEARCH_DIR / "weapons" / weapon_key

    print(f"[*] Output directory: {out_dir}")

    # Write weapon JSON
    weapon_json = generate_weapon_json(data)
    write_json(out_dir / f"weapon_{data['key']}.json", weapon_json)

    # Write full auto_data
    write_json(out_dir / "auto_data.json", data)

    if not args.json_only:
        md = generate_markdown(data)
        write_text(out_dir / "research.md", md)

    print(f"\n[OK] Weapon research complete for {name_en}")
    print(f"     Files written to: {out_dir}")


if __name__ == "__main__":
    main()
