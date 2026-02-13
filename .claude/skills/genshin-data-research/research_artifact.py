"""
Artifact set research script for Genshin Impact.
Extracts artifact set data from AnimeGameData and outputs structured research files.

Usage:
    python research_artifact.py <set_name_or_id> [--output-dir DIR]
    python research_artifact.py --list
    python research_artifact.py --list --min-rarity 4

Examples:
    python research_artifact.py "Crimson Witch of Flames"
    python research_artifact.py 15006
    python research_artifact.py --list --min-rarity 5
"""

import argparse
import sys
import re
from datetime import date
from pathlib import Path

from utils import (
    load_excel, text_en, text_chs, build_index,
    FIGHT_PROP_NAMES, EQUIP_TYPE_NAMES,
    RESEARCH_DIR, write_json, write_text, fmt_percent, round_list,
)


def find_artifact_set(name_or_id: str) -> dict | None:
    """Find an artifact set by name or set ID."""
    sets = load_excel("ReliquarySetExcelConfigData.json")

    # Try numeric ID
    try:
        target_id = int(name_or_id)
        for s in sets:
            if s.get("setId") == target_id:
                return s
    except ValueError:
        pass

    # We need to resolve set names from EquipAffix
    affix_data = load_excel("EquipAffixExcelConfigData.json")
    name_lower = name_or_id.lower().strip()

    for s in sets:
        affix_id = s.get("equipAffixId", 0)
        if not affix_id:
            continue
        # Find the affix entry (level 0 = 2pc bonus)
        for a in affix_data:
            if a.get("id") == affix_id and a.get("level", 0) == 0:
                en = text_en(a.get("nameTextMapHash", 0)).lower()
                chs = text_chs(a.get("nameTextMapHash", 0)).lower()
                if en == name_lower or chs == name_lower:
                    return s
                if name_lower in en and en:
                    return s
                break
    return None


def get_set_name(artifact_set: dict) -> tuple[str, str]:
    """Get EN and CHS names for an artifact set from its affix data."""
    affix_data = load_excel("EquipAffixExcelConfigData.json")
    affix_id = artifact_set.get("equipAffixId", 0)
    for a in affix_data:
        if a.get("id") == affix_id and a.get("level", 0) == 0:
            return text_en(a.get("nameTextMapHash", 0)), text_chs(a.get("nameTextMapHash", 0))
    return "Unknown", "Unknown"


def list_artifact_sets(min_rarity: int = 1) -> list[dict]:
    """List all artifact sets, optionally filtered by rarity."""
    sets = load_excel("ReliquarySetExcelConfigData.json")
    reliquary_data = load_excel("ReliquaryExcelConfigData.json")
    affix_data = load_excel("EquipAffixExcelConfigData.json")

    # Build a quick lookup for max rarity per set
    set_rarity = {}
    for r in reliquary_data:
        sid = r.get("setId", 0)
        rl = r.get("rankLevel", 0)
        if sid and rl:
            set_rarity[sid] = max(set_rarity.get(sid, 0), rl)

    results = []
    for s in sets:
        set_id = s.get("setId", 0)
        rarity = set_rarity.get(set_id, 0)
        if rarity < min_rarity:
            continue

        name_en, name_chs = get_set_name(s)
        if not name_en or name_en == "Unknown" or name_en.startswith("[hash:"):
            continue

        set_nums = s.get("setNeedNum", [])
        results.append({
            "set_id": set_id,
            "name_en": name_en,
            "name_chs": name_chs,
            "rarity": rarity,
            "bonuses": f"{'/'.join(str(n) + 'pc' for n in set_nums)}" if set_nums else "N/A",
        })

    results.sort(key=lambda x: (-x["rarity"], x["name_en"]))
    return results


def extract_artifact_set_data(artifact_set: dict) -> dict:
    """Extract complete artifact set data."""
    set_id = artifact_set["setId"]
    name_en, name_chs = get_set_name(artifact_set)

    # ── Set bonuses from EquipAffixExcelConfigData ─────────────────────────
    affix_data = load_excel("EquipAffixExcelConfigData.json")
    affix_id = artifact_set.get("equipAffixId", 0)
    set_need_nums = artifact_set.get("setNeedNum", [])

    bonuses = []
    affix_entries = [a for a in affix_data if a.get("id") == affix_id]
    affix_entries.sort(key=lambda x: x.get("level", 0))

    for i, entry in enumerate(affix_entries):
        piece_count = set_need_nums[i] if i < len(set_need_nums) else (i + 1) * 2
        desc = text_en(entry.get("descTextMapHash", 0))
        desc_chs = text_chs(entry.get("descTextMapHash", 0))

        add_props = []
        for p in entry.get("addProps", []):
            pt = p.get("propType", "")
            val = p.get("value", 0)
            if pt and val != 0:
                add_props.append({
                    "type": pt,
                    "name": FIGHT_PROP_NAMES.get(pt, pt),
                    "value": round(val, 4),
                })

        bonuses.append({
            "pieces": piece_count,
            "description": re.sub(r'<[^>]+>', '', desc),
            "description_chs": re.sub(r'<[^>]+>', '', desc_chs),
            "param_list": round_list(entry.get("paramList", [])),
            "add_props": add_props,
            "open_config": entry.get("openConfig", ""),
        })

    # ── Individual pieces ──────────────────────────────────────────────────
    # Find the highest-rarity pieces for this set by querying all reliquaries with this setId
    reliquary_data = load_excel("ReliquaryExcelConfigData.json")

    # Group all reliquaries belonging to this set by equip type, keeping highest rarity
    set_pieces_by_type: dict[str, dict] = {}
    max_rarity = 0
    for r in reliquary_data:
        if r.get("setId") != set_id:
            continue
        rl = r.get("rankLevel", 0)
        et = r.get("equipType", "")
        max_rarity = max(max_rarity, rl)
        # Keep the base piece (lowest level variant) at the highest rarity
        existing = set_pieces_by_type.get(et)
        if not existing or rl > existing.get("rankLevel", 0):
            set_pieces_by_type[et] = r
        elif rl == existing.get("rankLevel", 0) and r.get("maxLevel", 1) >= existing.get("maxLevel", 1):
            # Prefer the one with higher maxLevel (base piece, level 0)
            set_pieces_by_type[et] = r

    # If no reliquaries found by setId, fall back to containsList
    if not set_pieces_by_type:
        contains_list = artifact_set.get("containsList", [])
        for piece_id in contains_list:
            for r in reliquary_data:
                if r.get("id") == piece_id:
                    rl = r.get("rankLevel", 0)
                    max_rarity = max(max_rarity, rl)
                    et = r.get("equipType", "")
                    set_pieces_by_type[et] = r
                    break

    # Also try the obfuscated rarity field on the set itself
    for key, val in artifact_set.items():
        if isinstance(val, int) and 1 <= val <= 5 and key not in ("setId", "equipAffixId", "bagSortValue", "disableFilter"):
            if key not in ("setNeedNum",) and val >= max_rarity:
                max_rarity = max(max_rarity, val)
                break

    pieces = []
    equip_order = ["EQUIP_BRACER", "EQUIP_NECKLACE", "EQUIP_SHOES", "EQUIP_RING", "EQUIP_DRESS"]
    for et in equip_order:
        r = set_pieces_by_type.get(et)
        if r:
            pieces.append({
                "id": r.get("id", 0),
                "name_en": text_en(r.get("nameTextMapHash", 0)),
                "name_chs": text_chs(r.get("nameTextMapHash", 0)),
                "equip_type": et,
                "equip_name": EQUIP_TYPE_NAMES.get(et, ""),
                "rarity": r.get("rankLevel", 0),
                "max_level": r.get("maxLevel", 1),
            })

    return {
        "set_id": set_id,
        "key": name_en.lower().replace(" ", "_").replace("'", "").replace('"', ''),
        "name_en": name_en,
        "name_chs": name_chs,
        "rarity": max_rarity,
        "equip_affix_id": affix_id,
        "bonuses": bonuses,
        "pieces": pieces,
    }


def generate_artifact_json(data: dict) -> dict:
    """Generate compact artifact JSON for .research_info format."""
    pieces_dict = {}
    for bonus in data["bonuses"]:
        pieces_dict[f"{bonus['pieces']}pc"] = {
            "effect": bonus["description"],
            "effect_chs": bonus["description_chs"],
            "params": [v for v in bonus["param_list"] if v != 0],
            "add_props": bonus["add_props"],
        }

    piece_list = []
    for p in data["pieces"]:
        piece_list.append({
            "name": p["name_en"],
            "name_chs": p["name_chs"],
            "type": p["equip_name"],
            "slot": p["equip_type"],
        })

    return {
        "set_id": data["set_id"],
        "name": data["name_en"],
        "name_chs": data["name_chs"],
        "rarity": data["rarity"],
        "bonuses": pieces_dict,
        "pieces": piece_list,
    }


def generate_markdown(data: dict) -> str:
    """Generate artifact set research markdown."""
    lines = []
    a = lines.append

    a(f"# {data['name_en']} Research\n")
    a(f"> Auto-generated from AnimeGameData on {date.today().isoformat()}\n")

    a("## Basic Info")
    a("| Property | Value |")
    a("|----------|-------|")
    a(f"| Name (EN) | {data['name_en']} |")
    a(f"| Name (CHS) | {data['name_chs']} |")
    a(f"| Set ID | {data['set_id']} |")
    a(f"| Max Rarity | {data['rarity']}\u2605 |")
    a("")

    a("## Set Bonuses\n")
    for bonus in data["bonuses"]:
        a(f"### {bonus['pieces']}-Piece Bonus")
        a(f"{bonus['description']}\n")
        if bonus["add_props"]:
            a("**Stat Bonuses:**")
            for p in bonus["add_props"]:
                is_pct = "PERCENT" in p["type"] or "CRITICAL" in p["type"] or "CHARGE" in p["type"] or "ADD_HURT" in p["type"] or "HEAL" in p["type"]
                val = fmt_percent(p["value"]) if is_pct else str(round(p["value"], 1))
                a(f"- {p['name']}: +{val}")
            a("")
        params = [v for v in bonus["param_list"] if v != 0]
        if params:
            a(f"**Parameters**: {params}\n")
        if bonus["open_config"]:
            a(f"**Config**: `{bonus['open_config']}`\n")

    a("## Pieces\n")
    a("| Slot | Name (EN) | Name (CHS) |")
    a("|------|-----------|------------|")
    for p in data["pieces"]:
        a(f"| {p['equip_name']} | {p['name_en']} | {p['name_chs']} |")
    a("")

    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(description="Research a Genshin Impact artifact set from AnimeGameData")
    parser.add_argument("artifact", nargs="?", help="Artifact set name (EN/CHS) or set ID")
    parser.add_argument("--output-dir", "-o", help="Override output directory")
    parser.add_argument("--list", action="store_true", help="List all artifact sets")
    parser.add_argument("--min-rarity", type=int, default=1, help="Minimum rarity filter (for --list)")
    parser.add_argument("--json-only", action="store_true", help="Only output JSON")
    args = parser.parse_args()

    if args.list:
        print(f"[*] Listing artifact sets (rarity >= {args.min_rarity})")
        results = list_artifact_sets(args.min_rarity)
        print(f"\n{'Set ID':<10} {'Rarity':<8} {'Bonuses':<12} {'Name'}")
        print("-" * 70)
        for r in results:
            print(f"{r['set_id']:<10} {r['rarity']}\u2605{'':>5} {r['bonuses']:<12} {r['name_en']}")
        print(f"\nTotal: {len(results)} sets")
        return

    if not args.artifact:
        parser.error("artifact set name/ID is required (or use --list)")

    print(f"[*] Searching for artifact set: {args.artifact}")
    artifact_set = find_artifact_set(args.artifact)
    if not artifact_set:
        print(f"[ERROR] Artifact set not found: {args.artifact}", file=sys.stderr)
        sys.exit(1)

    name_en, _ = get_set_name(artifact_set)
    print(f"[*] Found: {name_en} (Set ID: {artifact_set['setId']})")

    print("[*] Extracting artifact set data...")
    data = extract_artifact_set_data(artifact_set)

    # Output directory
    if args.output_dir:
        out_dir = Path(args.output_dir)
    else:
        set_key = data["key"]
        out_dir = RESEARCH_DIR / "artifact_sets" / set_key

    print(f"[*] Output directory: {out_dir}")

    # Write artifact JSON
    artifact_json = generate_artifact_json(data)
    write_json(out_dir / f"artifact_{data['key']}.json", artifact_json)

    # Write full auto_data
    write_json(out_dir / "auto_data.json", data)

    if not args.json_only:
        md = generate_markdown(data)
        write_text(out_dir / "research.md", md)

    print(f"\n[OK] Artifact set research complete for {name_en}")
    print(f"     Files written to: {out_dir}")


if __name__ == "__main__":
    main()
