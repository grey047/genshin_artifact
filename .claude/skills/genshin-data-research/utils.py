"""
Shared utilities for Genshin Impact data research scripts.
Loads and caches AnimeGameData JSON files, resolves text hashes, computes stats.
"""

import json
import os
import sys
from pathlib import Path
from functools import lru_cache

# ── Paths ──────────────────────────────────────────────────────────────────────

ANIME_GAME_DATA = Path(os.environ.get(
    "ANIME_GAME_DATA",
    r"E:\Moltbot\workspace\AnimeGameData"
))
EXCEL_DIR = ANIME_GAME_DATA / "ExcelBinOutput"
TEXTMAP_DIR = ANIME_GAME_DATA / "TextMap"

PROJECT_ROOT = Path(__file__).resolve().parents[3]  # up from .claude/skills/genshin-data-research
RESEARCH_DIR = PROJECT_ROOT / ".research_info"

# ── Property type mappings ─────────────────────────────────────────────────────

FIGHT_PROP_NAMES = {
    "FIGHT_PROP_BASE_HP": "Base HP",
    "FIGHT_PROP_HP": "HP",
    "FIGHT_PROP_HP_PERCENT": "HP%",
    "FIGHT_PROP_BASE_ATTACK": "Base ATK",
    "FIGHT_PROP_ATTACK": "ATK",
    "FIGHT_PROP_ATTACK_PERCENT": "ATK%",
    "FIGHT_PROP_BASE_DEFENSE": "Base DEF",
    "FIGHT_PROP_DEFENSE": "DEF",
    "FIGHT_PROP_DEFENSE_PERCENT": "DEF%",
    "FIGHT_PROP_CRITICAL": "CRIT Rate",
    "FIGHT_PROP_CRITICAL_HURT": "CRIT DMG",
    "FIGHT_PROP_CHARGE_EFFICIENCY": "Energy Recharge",
    "FIGHT_PROP_HEAL_ADD": "Healing Bonus",
    "FIGHT_PROP_ELEMENT_MASTERY": "Elemental Mastery",
    "FIGHT_PROP_PHYSICAL_ADD_HURT": "Physical DMG Bonus",
    "FIGHT_PROP_FIRE_ADD_HURT": "Pyro DMG Bonus",
    "FIGHT_PROP_WATER_ADD_HURT": "Hydro DMG Bonus",
    "FIGHT_PROP_ELEC_ADD_HURT": "Electro DMG Bonus",
    "FIGHT_PROP_ICE_ADD_HURT": "Cryo DMG Bonus",
    "FIGHT_PROP_WIND_ADD_HURT": "Anemo DMG Bonus",
    "FIGHT_PROP_ROCK_ADD_HURT": "Geo DMG Bonus",
    "FIGHT_PROP_GRASS_ADD_HURT": "Dendro DMG Bonus",
}

FIGHT_PROP_SHORT = {
    "FIGHT_PROP_BASE_HP": "hp",
    "FIGHT_PROP_HP": "hp_flat",
    "FIGHT_PROP_HP_PERCENT": "hp_percent",
    "FIGHT_PROP_BASE_ATTACK": "atk",
    "FIGHT_PROP_ATTACK": "atk_flat",
    "FIGHT_PROP_ATTACK_PERCENT": "atk_percent",
    "FIGHT_PROP_BASE_DEFENSE": "def",
    "FIGHT_PROP_DEFENSE": "def_flat",
    "FIGHT_PROP_DEFENSE_PERCENT": "def_percent",
    "FIGHT_PROP_CRITICAL": "crit_rate",
    "FIGHT_PROP_CRITICAL_HURT": "crit_dmg",
    "FIGHT_PROP_CHARGE_EFFICIENCY": "energy_recharge",
    "FIGHT_PROP_HEAL_ADD": "healing_bonus",
    "FIGHT_PROP_ELEMENT_MASTERY": "elemental_mastery",
    "FIGHT_PROP_PHYSICAL_ADD_HURT": "physical_dmg_bonus",
    "FIGHT_PROP_FIRE_ADD_HURT": "pyro_dmg_bonus",
    "FIGHT_PROP_WATER_ADD_HURT": "hydro_dmg_bonus",
    "FIGHT_PROP_ELEC_ADD_HURT": "electro_dmg_bonus",
    "FIGHT_PROP_ICE_ADD_HURT": "cryo_dmg_bonus",
    "FIGHT_PROP_WIND_ADD_HURT": "anemo_dmg_bonus",
    "FIGHT_PROP_ROCK_ADD_HURT": "geo_dmg_bonus",
    "FIGHT_PROP_GRASS_ADD_HURT": "dendro_dmg_bonus",
}

WEAPON_TYPE_NAMES = {
    "WEAPON_SWORD_ONE_HAND": "Sword",
    "WEAPON_CLAYMORE": "Claymore",
    "WEAPON_POLE": "Polearm",
    "WEAPON_CATALYST": "Catalyst",
    "WEAPON_BOW": "Bow",
}

ELEMENT_MAP = {
    "Fire": "Pyro",
    "Water": "Hydro",
    "Electric": "Electro",
    "Ice": "Cryo",
    "Wind": "Anemo",
    "Rock": "Geo",
    "Grass": "Dendro",
}

QUALITY_TO_RARITY = {
    "QUALITY_ORANGE": 5,
    "QUALITY_ORANGE_SP": 5,
    "QUALITY_PURPLE": 4,
    "QUALITY_BLUE": 3,
    "QUALITY_GREEN": 2,
    "QUALITY_GRAY": 1,
}

EQUIP_TYPE_NAMES = {
    "EQUIP_BRACER": "Flower of Life",
    "EQUIP_NECKLACE": "Plume of Death",
    "EQUIP_SHOES": "Sands of Eon",
    "EQUIP_RING": "Goblet of Eonothem",
    "EQUIP_DRESS": "Circlet of Logos",
}


# ── JSON loading with cache ───────────────────────────────────────────────────

_json_cache: dict[str, any] = {}


def load_excel(filename: str) -> list[dict]:
    """Load a JSON file from ExcelBinOutput, with caching."""
    if filename not in _json_cache:
        path = EXCEL_DIR / filename
        if not path.exists():
            print(f"[WARN] File not found: {path}", file=sys.stderr)
            return []
        with open(path, "r", encoding="utf-8") as f:
            _json_cache[filename] = json.load(f)
    return _json_cache[filename]


def load_textmap(lang: str = "EN") -> dict[str, str]:
    """Load TextMap for given language. Returns {hash_str: text}."""
    key = f"TextMap{lang}"
    if key not in _json_cache:
        path = TEXTMAP_DIR / f"TextMap{lang}.json"
        if not path.exists():
            # some languages are split into _0 and _1
            merged = {}
            for suffix in ["_0", "_1"]:
                p = TEXTMAP_DIR / f"TextMap{lang}{suffix}.json"
                if p.exists():
                    with open(p, "r", encoding="utf-8") as f:
                        merged.update(json.load(f))
            _json_cache[key] = merged
        else:
            with open(path, "r", encoding="utf-8") as f:
                _json_cache[key] = json.load(f)
    return _json_cache[key]


def text(hash_val, lang: str = "EN") -> str:
    """Resolve a nameTextMapHash to its localized string."""
    if hash_val is None or hash_val == 0:
        return ""
    tm = load_textmap(lang)
    return tm.get(str(hash_val), f"[hash:{hash_val}]")


def text_en(hash_val) -> str:
    return text(hash_val, "EN")


def text_chs(hash_val) -> str:
    return text(hash_val, "CHS")


# ── Index builders ─────────────────────────────────────────────────────────────

def build_index(data: list[dict], key: str = "id") -> dict:
    """Build a dict index from a list of objects by a key field."""
    return {item[key]: item for item in data if key in item}


def build_group_index(data: list[dict], key: str) -> dict[int, list[dict]]:
    """Group items by a key into lists."""
    groups: dict[int, list[dict]] = {}
    for item in data:
        k = item.get(key)
        if k is not None:
            groups.setdefault(k, []).append(item)
    return groups


# ── Growth curve computation ──────────────────────────────────────────────────

def get_curve_multiplier(curve_data: list[dict], curve_type: str, level: int) -> float:
    """Get growth curve multiplier for a given curve type at a given level."""
    for entry in curve_data:
        if entry.get("level") == level:
            for info in entry.get("curveInfos", []):
                if info.get("type") == curve_type:
                    return info.get("value", 1.0)
    return 1.0


def compute_stat_at_level(base: float, curve_data: list[dict], curve_type: str,
                          level: int, ascension_bonus: float = 0.0) -> float:
    """Compute a stat value at a given level including ascension bonus."""
    mult = get_curve_multiplier(curve_data, curve_type, level)
    return base * mult + ascension_bonus


# ── Ascension helpers ─────────────────────────────────────────────────────────

def get_ascension_data(promote_data: list[dict], promote_id: int) -> list[dict]:
    """Get all ascension phases for a given promote ID, sorted by level."""
    phases = [p for p in promote_data if p.get("avatarPromoteId") == promote_id
              or p.get("weaponPromoteId") == promote_id]
    phases.sort(key=lambda x: x.get("promoteLevel", 0))
    return phases


def get_ascension_stat_bonus(phase: dict, prop_type: str) -> float:
    """Get a specific stat bonus from an ascension phase."""
    for prop in phase.get("addProps", []):
        if prop.get("propType") == prop_type:
            return prop.get("value", 0.0)
    return 0.0


def get_ascension_stat_type(phases: list[dict]) -> str | None:
    """Determine the ascension bonus stat type (the one that increases across phases)."""
    if len(phases) < 3:
        return None
    # Phase 0 has base props, phase 1+ has the scaling bonus
    # Look at phase 2+ for the changing stat
    for prop in phases[2].get("addProps", []):
        pt = prop.get("propType", "")
        val = prop.get("value", 0)
        if val > 0 and pt not in ("FIGHT_PROP_BASE_HP", "FIGHT_PROP_BASE_ATTACK", "FIGHT_PROP_BASE_DEFENSE"):
            return pt
    return None


# ── Skill multiplier extraction ───────────────────────────────────────────────

def get_proud_skill_multipliers(proud_data: list[dict], group_id: int) -> list[dict]:
    """Get all level entries for a proud skill group, sorted by level."""
    entries = [e for e in proud_data if e.get("proudSkillGroupId") == group_id]
    entries.sort(key=lambda x: x.get("level", 0))
    return entries


def extract_param_list(entries: list[dict]) -> list[list[float]]:
    """Extract paramList arrays from proud skill entries, transposed.
    Returns list of param arrays, where each inner list contains values across all levels.
    e.g. param_lists[0] = [lv1_param0, lv2_param0, ..., lv15_param0]
    """
    if not entries:
        return []
    n_params = len(entries[0].get("paramList", []))
    result = []
    for i in range(n_params):
        row = []
        for entry in entries:
            params = entry.get("paramList", [])
            row.append(params[i] if i < len(params) else 0.0)
        # skip rows that are all zeros
        if any(v != 0 for v in row):
            result.append(row)
    return result


# ── Output helpers ────────────────────────────────────────────────────────────

def ensure_dir(path: Path):
    """Create directory if it doesn't exist."""
    path.mkdir(parents=True, exist_ok=True)


def write_json(path: Path, data: dict, indent: int = 2):
    """Write JSON data to file."""
    ensure_dir(path.parent)
    with open(path, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=indent, ensure_ascii=False)
    print(f"  Written: {path}")


def write_text(path: Path, content: str):
    """Write text content to file."""
    ensure_dir(path.parent)
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)
    print(f"  Written: {path}")


def fmt_percent(val: float, decimals: int = 2) -> str:
    """Format a decimal as percentage string."""
    return f"{val * 100:.{decimals}f}%"


def fmt_multiplier(val: float, decimals: int = 3) -> str:
    """Format a multiplier value."""
    return f"{val:.{decimals}f}"


def round_list(lst: list[float], decimals: int = 4) -> list[float]:
    """Round all values in a list."""
    return [round(v, decimals) for v in lst]
