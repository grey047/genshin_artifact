# Task 93: Moonsign Level Dropdown in Frontend

**Task ID**: 93
**Status**: In Progress
**Priority**: High
**Related**: Task 77 (Lunar Reactions + Moonsign)

## Overview

Add UI dropdown selector for Moonsign Level (初辉/满辉) in character config panel.
- Level 1: Nascent Gleam (初辉)
- Level 2: Ascendant Gleam (满辉)

## Implementation

### Phase 1: Update Generated Config

**File**: `src/assets/_gen_character.js`

For each character with Moonsign (Flins, Ineffa, Columbina, Lauma, Nefer, Aino):
```js
configSkill: [
    {"default":1,"max":2,"min":1,"name":"moonsign_level","title":"月兆等级/Moonsign Level","type":"int"},
]
```

### Phase 2: Add Frontend UI

**File**: `src/pages/CharacterPage/Components/*Config.vue`

Add dropdown in character config panel:
```vue
<el-form-item :label="t('moonsign.title')">
    <el-select v-model="config.moonsign_level">
        <el-option :label="t('moonsign.nascent')" :value="1" />
        <el-option :label="t('moonsign.ascendant')" :value="2" />
    </el-select>
</el-form-item>
```

### Phase 3: Add i18n

**Files**: `src/i18n/locales/*.js`

```js
// zh-cn.js
moonsign: {
    title: "月兆等级",
    nascent: "初辉 (Lv1)",
    ascendant: "满辉 (Lv2)",
}

// en.js
moonsign: {
    title: "Moonsign Level",
    nascent: "Nascent Gleam (Lv1)",
    ascendant: "Ascendant Gleam (Lv2)",
}
```

## Affected Characters

| Character | Element | Already has configSkill.moonsign_level |
|-----------|---------|---------------------------------------|
| Flins | Electro | ✅ |
| Ineffa | Electro | ✅ |
| Columbina | Hydro | ✅ |
| Lauma | Dendro | ✅ |
| Nefer | Dendro | ✅ |
| Aino | Hydro | ✅ |

## Files to Modify

| File | Change |
|------|--------|
| `src/i18n/locales/zh-cn.js` | Add moonsign translations |
| `src/i18n/locales/en.js` | Add moonsign translations |
| `src/pages/CharacterPage/Components/*Config.vue` | Add moonsign dropdown |

## Branch

`feature/moonsign-dropdown-93`

## Success Criteria

1. ✅ Dropdown appears in character config panel
2. ✅ Options: 初辉 (Lv1), 满辉 (Lv2)
3. ✅ Default value: 1
4. ✅ Value passed to DSL via `get_moonsign_level()`
5. ✅ i18n works for both languages

## Related

- Task 77: DSL `get_moonsign_level()` function
- Generated config already has moonsign_level field
