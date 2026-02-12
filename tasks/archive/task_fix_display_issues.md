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
| **Escoffier** | `Escoffier_avatar.webp` EXISTS | CDN FAIL |
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
- Dahlia: 塔利雅
- Durin: 杜林
- Escoffier: 爱可菲
- Flins: 菲林斯
- Skirk: 丝柯克

### Fix Plan

1. Look up official Chinese names for these 6 characters
2. Update `name_locale` in each `.rs` file
3. Run `npm run gen_meta` to regenerate i18n files

---

## Issue 3: Weapon Icons Not Displaying [P1]

### Root Cause

The weapon meta template (`mona_generate/templates/weapon_meta_template.js`) has local image imports **commented out** (lines 2-4):
```javascript
// {% for weapon in weapons %}
// import {{ weapon.name }}_tn from "@image/weapons/{{ weapon.name }}_tn"
// {% endfor %}
```

All 207 weapons use CDN URLs instead of local images:
- 170 weapons use old CDN (`upload-bbs.mihoyo.com`) with internal_name
- 37 weapons use new CDN (`act-webstatic.mihoyo.com`) with MD5 hash

Local images exist at `src/images/weapons/` (135+ files) but are completely unused.

New weapons without CDN coverage fail to display icons.

### Affected Weapons

| Weapon | In Enum | Exported | In gen_weapon.js | Local Image |
|--------|---------|----------|------------------|-------------|
| Peakbreaker | YES (polearm) | YES | **NO** (needs gen_meta) | Need to verify |
| FangOfTheMountainKing | YES (claymore) | YES | YES (old CDN fails) | Need to verify |
| FruitfulHook | YES (claymore) | YES | YES (old CDN fails) | Need to verify |
| AThousandBlazingSuns | YES (claymore) | YES | YES (old CDN fails) | Need to verify |

### Can We Get Hashes from Honey Hunter World?

**No.** Honey Hunter World uses its own image hosting with numeric IDs (e.g., `/img/i_n11101.webp`), completely unrelated to the miHoYo CDN MD5 hashes this project uses.

### Fix Plan — Hybrid Approach (CDN + Local Fallback)

We have ~207 weapons but only ~135 local images, so a full switch to local is not possible without first filling the gaps. Instead, use a **hybrid** approach: keep working CDN URLs, replace only broken ones with local images.

#### Step 1: Write a diagnostic script to identify broken CDN links

Create `script/check_weapon_icons.js`:
```javascript
// Parse _gen_weapon.js to extract all weapon CDN URLs
// HTTP HEAD request each URL to check if it returns 200
// Cross-reference with local images in src/images/weapons/
// Output report:
//   - WORKING: CDN URL returns 200 → keep as-is
//   - BROKEN + LOCAL: CDN fails but local image exists → replace with local
//   - BROKEN + MISSING: CDN fails and no local image → need to download from HHW
```

Can be run by an agent with browser capabilities to automate the full check.

#### Step 2: Modify template to support hybrid mode

File: `mona_generate/templates/weapon_meta_template.js`

Add local imports only for weapons that need them:
```javascript
{% for weapon in weapons %}
{% if weapon.use_local_image -%}
import {{ weapon.name }}_tn from "@image/weapons/{{ weapon.name }}_tn"
{%- endif %}
{% endfor %}
```

Use local or CDN based on a flag:
```javascript
{% if weapon.use_local_image -%}
url: {{ weapon.name }}_tn,
{% elif weapon.icon_hash != "" -%}
url: newImageUrl("{{ weapon.icon_hash }}"),
{% else -%}
url: imageUrl("{{ weapon.internal_name }}"),
{%- endif %}
```

This requires adding a `use_local_image: bool` field to the weapon meta generation in `mona_generate/src/gen_meta/gen_weapon_meta.rs`, determined by checking whether the weapon's CDN URL is known to be broken or whether a local image file exists.

#### Step 3: Download missing weapon images from HHW

For weapons flagged as BROKEN + MISSING, download from Honey Hunter World (the original source for local weapon images):

```
https://gensh.honeyhunterworld.com/img/i_n{weapon_id}.webp
```

HHW weapon ID format: `{type}{serial}` where type = 1(sword), 2(claymore), 3(polearm), 4(catalyst), 5(bow).
Save as `src/images/weapons/{WeaponName}_tn.webp`.

**Automation**: Use an agent with browser capabilities to:
1. Run the diagnostic script to get the BROKEN + MISSING list
2. For each missing weapon, search HHW to find the weapon page and its `i_n{id}` identifier
3. Download the icon from `https://gensh.honeyhunterworld.com/img/i_n{id}.webp`
4. Save to `src/images/weapons/{WeaponName}_tn.webp`

This is the same workflow originally used to populate the existing 135+ weapon images.

#### Step 4: Regenerate

```powershell
npm run gen_meta
```

#### Simpler Alternative (if template changes are too complex)

Instead of modifying the Rust generation pipeline, directly patch `src/assets/_gen_weapon.js` after generation:
1. Run diagnostic script to find broken URLs
2. For each broken weapon with a local image, manually add its import and swap `url:` to use the local import
3. Downside: gets overwritten on next `gen_meta` run, but is quick to implement

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
