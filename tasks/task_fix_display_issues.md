# Task: Fix Character & Weapon Display Issues

**Created**: 2026-02-07
**Branch**: feature/lunar-reactions-76
**Priority**: P0-P2

---

## Summary

The frontend has multiple display issues affecting new characters and weapons:
1. Character avatars not loading (CDN URLs broken for newer characters)
2. Some character names showing in English instead of Chinese
3. Weapon icons not displaying for some new weapons
4. Azurelight (苍耀) sword not appearing at all

---

## Issue 1: Character Avatars Not Loading [P0]

### Root Cause

The character meta template (`mona_generate/templates/character_meta_template.js`) currently:
- **Comments out** local avatar imports (line 4): `// import {{ c.name }}_avatar from "@image/characters/{{ c.name }}_avatar"`
- Uses CDN URLs instead (lines 24-28), via `getName()` or `getHash()`
- `getName()` points to `upload-bbs.mihoyo.com` legacy CDN which no longer serves icons for newer characters
- `getHash()` works but new characters don't have hash entries in `icon_hashmap.rs`

### Affected Characters

ALL characters use CDN URLs. The following new characters fail because CDN doesn't have their assets:

| Character | Local Avatar File | CDN Status |
|-----------|-------------------|------------|
| Aino | `Aino_avatar.webp` EXISTS | CDN FAIL |
| Columbina | `Columbina_avatar.webp` EXISTS | CDN FAIL |
| Dahlia | `Dahlia_avatar.webp` EXISTS | CDN FAIL |
| Durin | `Durin_avatar.webp` EXISTS | CDN FAIL |
| **Escoffier** | **MISSING** (no avatar file) | CDN FAIL |
| Flins | `Flins_avatar.webp` EXISTS | CDN FAIL |
| Ifa | `Ifa_avatar.webp` EXISTS | CDN FAIL |
| Ineffa | `Ineffa_avatar.webp` EXISTS | CDN FAIL |
| Jahoda | `Jahoda_avatar.webp` EXISTS | CDN FAIL |
| Lauma | `Lauma_avatar.webp` EXISTS | CDN FAIL |
| Nefer | `Nefer_avatar.webp` EXISTS | CDN FAIL |
| Skirk | `Skirk_avatar.webp` EXISTS | CDN FAIL |

### Fix Plan

**Step 1**: Modify template to use local avatar imports instead of CDN.

File: `mona_generate/templates/character_meta_template.js`

Change line 4 from:
```javascript
// import {{ c.name }}_avatar from "@image/characters/{{ c.name }}_avatar"
```
to:
```javascript
import {{ c.name }}_avatar from "@image/characters/{{ c.name }}_avatar"
```

Change lines 22-28 from:
```javascript
// avatar: {{ c.name }}_avatar,
{% if c.icon_hash == "" -%}
avatar: getName("{{ c.internal_name }}"),
{% else -%}
avatar: getHash("{{ c.icon_hash }}"),
{%- endif %}
```
to:
```javascript
avatar: {{ c.name }}_avatar,
```

**Step 2**: Add missing Escoffier avatar image.

Create placeholder `src/images/characters/Escoffier_avatar.webp` (copy from another character as temp).

**Step 3**: Regenerate character meta.

```powershell
npm run gen_meta
```

This will regenerate `src/assets/_gen_character.js` with local avatar imports.

### Alternative Approach (if we want to keep CDN for old characters)

Instead of modifying the template, directly edit `src/assets/_gen_character.js`:
- Uncomment the avatar imports for ALL characters
- Replace `avatar: getName(...)` / `avatar: getHash(...)` with `avatar: Xxx_avatar`

This avoids re-running gen_meta but is fragile (gets overwritten on next gen_meta run).

**Recommendation**: Modify the template (Step 1) so future gen_meta runs produce correct output.

---

## Issue 2: Missing Chinese Character Names [P1]

### Root Cause

5 characters have English names in their `zh_cn` locale field (placeholder values never updated):

| Character | File | Current zh_cn | Needs |
|-----------|------|---------------|-------|
| Columbina | `mona_core/src/character/characters/hydro/columbina.rs` | `"Columbina"` | Official Chinese name |
| Ifa | `mona_core/src/character/characters/anemo/ifa.rs` | `"Ifa"` | Official Chinese name |
| Ineffa | `mona_core/src/character/characters/electro/ineffa.rs` | `"Ineffa"` | Official Chinese name |
| Jahoda | `mona_core/src/character/characters/anemo/jahoda.rs` | `"Jahoda"` | Official Chinese name |
| Lauma | `mona_core/src/character/characters/dendro/lauma.rs` | `"Lauma"` | Official Chinese name |
| Nefer | `mona_core/src/character/characters/dendro/nefer.rs` | `"Nefer"` | Official Chinese name |

Characters with CORRECT Chinese names (no action needed):
- Aino: 爱诺
- Dahlia: 达利亚
- Durin: 杜林
- Escoffier: 埃斯科菲耶
- Flins: 菲林斯
- Skirk: 丝柯克

### Fix Plan

1. Look up official Chinese names for these 6 characters
2. Update `name_locale` in each `.rs` file
3. Run `npm run gen_meta` to regenerate i18n files

---

## Issue 3: Weapon Icons Not Displaying [P1]

### Root Cause

New weapons have no entries in `mona_generate/src/utils/icon_hashmap.rs`, so they fall back to the legacy CDN URL (`upload-bbs.mihoyo.com`) which may not serve their icons.

### Affected Weapons

| Weapon | In Enum | Exported | In gen_weapon.js | Hash in icon_hashmap |
|--------|---------|----------|------------------|---------------------|
| Peakbreaker | YES (polearm, line 141) | YES | **NO** (needs gen_meta) | NO |
| FangOfTheMountainKing | YES (claymore) | YES | YES (old CDN) | NO |
| FruitfulHook | YES (claymore) | YES | YES (old CDN) | NO |
| AThousandBlazingSuns | YES (claymore) | YES | YES (old CDN) | NO |

### Can We Get Hashes from Honey Hunter World?

**No.** Honey Hunter World (`gensh.honeyhunterworld.com`) uses its own image hosting with numeric IDs (e.g., `/img/i_n11101.webp`), not miHoYo CDN hashes. The two systems are unrelated:

| Source | URL Format | Example |
|--------|-----------|---------|
| Honey Hunter | `/img/i_n{id}.webp` | `gensh.honeyhunterworld.com/img/i_n11101.webp` |
| This Project | `{md5_hash}.png` on miHoYo CDN | `act-webstatic.mihoyo.com/.../f5336c01b0f1e19833a0f9e8bd04c107.png` |

### Fix Plan (Option A - Use Local Images)

Similar to the character avatar fix, switch the weapon template to use local images:

File: `mona_generate/templates/weapon_meta_template.js`

Uncomment weapon image imports (line 3):
```javascript
import {{ weapon.name }}_tn from "@image/weapons/{{ weapon.name }}_tn"
```

Replace CDN URL with local reference (lines 18-22):
```javascript
url: {{ weapon.name }}_tn,
```

Then add any missing weapon thumbnail images to `src/images/weapons/`.

### Fix Plan (Option B - Add Hash Values)

Find hashes from miHoYo's API (HoYoLab calculator API or game data) and add entries to `mona_generate/src/utils/icon_hashmap.rs`:

```rust
("Peakbreaker", "xxx..."),
("FangOfTheMountainKing", "xxx..."),
("FruitfulHook", "xxx..."),
("AThousandBlazingSuns", "xxx..."),
```

Then run `npm run gen_meta`.

### Fix Plan (Option C - Peakbreaker Quick Fix)

Peakbreaker is fully registered in Rust but missing from the generated JS. Simply running `npm run gen_meta` will add it to `_gen_weapon.js` (though still with old CDN URL).

---

## Issue 4: Azurelight (苍耀) Not Showing At All [P0]

### Root Cause

Azurelight has a complete implementation at `mona_core/src/weapon/weapons/swords/azurelight.rs` and a config entry in `weapon_config.rs` (line 39), but is **NOT registered** in two critical locations:

1. **NOT in WeaponName enum** (`mona_core/src/weapon/weapon_name.rs`) - missing after line 62 (CalamityOfEshu)
2. **NOT exported in swords module** (`mona_core/src/weapon/weapons/swords/mod.rs`) - missing `pub use` and `mod` declarations

### Fix Plan

**Step 1**: Add to WeaponName enum.

File: `mona_core/src/weapon/weapon_name.rs`, after line 62:
```rust
    CalamityOfEshu,
    Azurelight,       // <-- ADD
```

**Step 2**: Export in swords module.

File: `mona_core/src/weapon/weapons/swords/mod.rs`:
```rust
// Add with other pub use statements:
pub use azurelight::Azurelight;

// Add with other mod declarations:
mod azurelight;
```

**Step 3**: Rebuild WASM and regenerate meta.

```powershell
npm run build:wasm
npm run gen_meta
```

The `#[derive(WeaponData)]` macro on the WeaponName enum will auto-generate all trait implementations once the weapon is in the enum.

---

## Execution Order

1. **Fix Azurelight registration** (Rust changes - Issue 4)
2. **Fix character Chinese names** (Rust changes - Issue 2)
3. **Modify templates for local images** (character avatar + weapon icon - Issues 1 & 3)
4. **Add missing Escoffier avatar** image placeholder
5. **Rebuild**: `npm run build:wasm` (for Rust changes)
6. **Regenerate**: `npm run gen_meta` (for all frontend data)
7. **Verify**: `npm run serve` and check display

---

## Files to Modify

### Rust (requires WASM rebuild)
- `mona_core/src/weapon/weapon_name.rs` — Add Azurelight to enum
- `mona_core/src/weapon/weapons/swords/mod.rs` — Export azurelight module
- `mona_core/src/character/characters/hydro/columbina.rs` — Fix zh_cn name
- `mona_core/src/character/characters/anemo/ifa.rs` — Fix zh_cn name
- `mona_core/src/character/characters/electro/ineffa.rs` — Fix zh_cn name
- `mona_core/src/character/characters/anemo/jahoda.rs` — Fix zh_cn name
- `mona_core/src/character/characters/dendro/lauma.rs` — Fix zh_cn name
- `mona_core/src/character/characters/dendro/nefer.rs` — Fix zh_cn name

### Templates (requires gen_meta rerun)
- `mona_generate/templates/character_meta_template.js` — Use local avatar imports
- `mona_generate/templates/weapon_meta_template.js` — Use local weapon image imports (if choosing Option A)

### Assets
- `src/images/characters/Escoffier_avatar.webp` — Add missing file
- Possibly add missing weapon thumbnails to `src/images/weapons/`
