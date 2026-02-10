# Genshin Impact Local Database Design

> 原神本地数据库设计文档  
> 版本: 1.0  
> 日期: 2026-02-07

---

## 1. 数据库概述

### 1.1 设计目标
- **完整性**: 存储所有原神游戏数据
- **规范性**: 统一的命名和结构
- **可扩展性**: 易于添加新数据
- **查询效率**: 优化常用查询模式
- **多语言支持**: 内置中英文本地化

### 1.2 数据库选型
```yaml
# 建议使用 SQLite + JSON 扩展
# 原因: 
# - 单文件，便于分发
# - 支持 JSON 类型，灵活存储复杂数据
# - 零配置，开箱即用
```

---

## 2. 数据表设计

### 2.1 characters - 角色基础信息
```sql
CREATE TABLE characters (
  id                    INTEGER PRIMARY KEY AUTOINCREMENT,
  key                   TEXT NOT NULL UNIQUE,        -- 'columbina'
  slug                  TEXT NOT NULL UNIQUE,         -- 'Columbina'
  name_zh               TEXT NOT NULL,                -- '哥伦比娅'
  name_en               TEXT NOT NULL,                -- 'Columbina'
  title_zh              TEXT,                         -- '空月归乡'
  title_en              TEXT,                         -- 'Welkin Moon's Homecoming'
  
  -- 基础属性
  element               TEXT NOT NULL,               -- 'Hydro', 'Pyro', 'Anemo', etc.
  weapon_type           TEXT NOT NULL,               -- 'Catalyst', 'Sword', 'Bow', etc.
  rarity                INTEGER NOT NULL,             -- 4, 5
  
  -- 基础数值 (Lv.1)
  base_hp               REAL NOT NULL,
  base_atk              REAL NOT NULL,
  base_def              REAL NOT NULL,
  
  -- 生日
  birthday_day         INTEGER,
  birthday_month        INTEGER,
  
  -- 所在组织/地区
  association           TEXT,                        -- 'NODKRAI'
  region                TEXT,                         -- 'Snezhnaya', 'Inazuma', etc.
  
  -- 命之座
  constellation_zh      TEXT,                         -- '御月鸽座'
  constellation_en     TEXT,                         -- 'Columbina Hyposelenia'
  
  -- 语音
  voice_actor_zh       TEXT,                         -- '杨梦露'
  voice_actor_en       TEXT,                         -- 'Emi Lo'
  
  -- 描述
  description_zh       TEXT,
  description_en       TEXT,
  
  -- 版本信息
  version_introduced   TEXT,                         -- '6.3'
  version_discovered   TEXT,                         -- 首次发现命之座的版本
  
  created_at            DATETIME DEFAULT CURRENT_TIMESTAMP,
  updated_at            DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_characters_element ON characters(element);
CREATE INDEX idx_characters_weapon ON characters(weapon_type);
CREATE INDEX idx_characters_rarity ON characters(rarity);
```

### 2.2 character_stats - 角色等级属性
```sql
CREATE TABLE character_stats (
  id                INTEGER PRIMARY KEY,
  character_id     INTEGER NOT NULL REFERENCES characters(id),
  level             INTEGER NOT NULL,                -- 1, 20, 40, ..., 90, 90+
  ascension_phase   INTEGER DEFAULT 0,               -- 0-6
  
  hp                REAL NOT NULL,
  atk               REAL NOT NULL,
  def               REAL NOT NULL,
  
  crit_rate_base    REAL DEFAULT 0.05,              -- 基础暴击率
  crit_dmg_base     REAL DEFAULT 0.50,               -- 基础暴击伤害
  
  bonus_stats       TEXT,                           -- JSON: {"AnemoBonus": 0.192}
  
  PRIMARY KEY (character_id, level),
  FOREIGN KEY (character_id) REFERENCES characters(id)
);
```

### 2.3 character_skills - 角色技能
```sql
CREATE TABLE character_skills (
  id                INTEGER PRIMARY KEY AUTOINCREMENT,
  character_id     INTEGER NOT NULL REFERENCES characters(id),
  
  skill_type        TEXT NOT NULL,                  -- 'NormalAttack', 'ElementalSkill', 'ElementalBurst'
  skill_key         TEXT NOT NULL,                  -- 'Moondew Cascade'
  
  -- 名称
  name_zh           TEXT NOT NULL,                  -- '月露泼降'
  name_en           TEXT NOT NULL,                  -- 'Moondew Cascade'
  
  -- 描述
  description_zh    TEXT,
  description_en    TEXT,
  
  -- 类型标识
  is_talent        BOOLEAN DEFAULT FALSE,          -- 是否是天赋技能
  is_passive       BOOLEAN DEFAULT FALSE,          -- 是否是被动技能
  
  -- 冷却/能耗
  cooldown          REAL,                          -- 秒
  cost              REAL,                           -- 元素能量
  cost_type         TEXT,                          -- 'Energy', 'Stamina'
  
  -- 优先级（用于排序）
  display_order     INTEGER DEFAULT 0,
  
  FOREIGN KEY (character_id) REFERENCES characters(id),
  UNIQUE (character_id, skill_key)
);
```

### 2.4 skill_levels - 技能等级数据
```sql
CREATE TABLE skill_levels (
  id                INTEGER PRIMARY KEY,
  skill_id          INTEGER NOT NULL REFERENCES character_skills(id),
  level             INTEGER NOT NULL,              -- 1-15
  
  -- 伤害/效果值 (JSON 存储)
  multipliers       TEXT NOT NULL,                 -- JSON: {"1-Hit": 46.79, "2-Hit": 36.63}
  effects           TEXT,                          -- JSON: {"CD": 17, "Duration": 25}
  
  PRIMARY KEY (skill_id, level),
  FOREIGN KEY (skill_id) REFERENCES character_skills(id)
);
```

### 2.5 character_constellations - 命之座
```sql
CREATE TABLE character_constellations (
  id                INTEGER PRIMARY KEY AUTOINCREMENT,
  character_id      INTEGER NOT NULL REFERENCES characters(id),
  constellation_no  INTEGER NOT NULL,              -- 1-6
  
  -- 名称
  name_zh           TEXT NOT NULL,                  -- '遍照花海，隐入群山'
  name_en           TEXT NOT NULL,                  -- 'Radiance Over Blossoms and Peaks'
  
  -- 描述
  description_zh    TEXT,
  description_en    TEXT,
  
  -- 效果 (JSON 存储复杂效果)
  effect            TEXT NOT NULL,                  -- JSON 结构化效果
  
  -- 等级需求
  level_required    INTEGER DEFAULT 1,
  
  FOREIGN KEY (character_id) REFERENCES characters(id),
  UNIQUE (character_id, constellation_no)
);
```

### 2.6 character_passives - 角色被动天赋
```sql
CREATE TABLE character_passives (
  id                INTEGER PRIMARY KEY AUTOINCREMENT,
  character_id      INTEGER NOT NULL REFERENCES characters(id),
  
  passive_no        INTEGER,                        -- 1-4 (解锁顺序)
  
  -- 名称
  name_zh           TEXT NOT NULL,
  name_en           TEXT NOT NULL,
  
  -- 描述
  description_zh    TEXT,
  description_en    TEXT,
  
  -- 解锁条件
  unlock_condition  TEXT,                          -- 'Ascension Phase 1', 'C4'
  
  -- 效果 (JSON)
  effect            TEXT,
  
  FOREIGN KEY (character_id) REFERENCES characters(id)
);
```

### 2.7 character_talents - 技能升级材料
```sql
CREATE TABLE character_talents (
  id                INTEGER PRIMARY KEY,
  character_id      INTEGER NOT NULL REFERENCES characters(id),
  skill_type        TEXT NOT NULL,                  -- 'NormalAttack', 'ElementalSkill', 'ElementalBurst'
  
  -- 材料需求 (JSON)
  materials         TEXT NOT NULL,                  -- JSON: {"Lv2": {...}, "Lv3": {...}}
  
  FOREIGN KEY (character_id) REFERENCES characters(id),
  UNIQUE (character_id, skill_type)
);
```

### 2.8 weapons - 武器基础信息
```sql
CREATE TABLE weapons (
  id                INTEGER PRIMARY KEY AUTOINCREMENT,
  key               TEXT NOT NULL UNIQUE,           -- 'AquilaFavonia'
  slug              TEXT NOT NULL UNIQUE,           -- 'Aquila Favonia'
  
  name_zh           TEXT NOT NULL,
  name_en           TEXT NOT NULL,
  
  weapon_type       TEXT NOT NULL,                  -- 'Sword', 'Claymore', etc.
  rarity            INTEGER NOT NULL,                -- 1-5
  
  -- 基础属性 (Lv.1)
  base_atk          REAL NOT NULL,
  sub_stat_type     TEXT,                           -- 'CRIT Rate', 'ATK%', etc.
  sub_stat_value    REAL,                           -- 基础副词条值
  
  -- 满级属性 (Lv.90)
  max_atk           REAL,
  max_sub_stat      REAL,
  
  -- 被动技能
  passive_name_zh   TEXT,
  passive_name_en    TEXT,
  passive_desc_zh   TEXT,
  passive_desc_en   TEXT,
  
  -- 材料
  ascend_material   TEXT,                           -- 突破材料
  
  version           TEXT,                           -- 所属版本
  
  created_at        DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_weapons_type ON weapons(weapon_type);
CREATE INDEX idx_weapons_rarity ON weapons(rarity);
```

### 2.9 weapon_levels - 武器等级属性
```sql
CREATE TABLE weapon_levels (
  id                INTEGER PRIMARY KEY,
  weapon_id         INTEGER NOT NULL REFERENCES weapons(id),
  level             INTEGER NOT NULL,
  
  atk               REAL NOT NULL,
  sub_stat          REAL,                           -- 副词条值
  
  ascension_phase   INTEGER DEFAULT 0,
  
  PRIMARY KEY (weapon_id, level),
  FOREIGN KEY (weapon_id) REFERENCES weapons(id)
);
```

### 2.10 artifacts - 圣遗物基础信息
```sql
CREATE TABLE artifacts (
  id                INTEGER PRIMARY KEY AUTOINCREMENT,
  key               TEXT NOT NULL UNIQUE,          -- 'HeartOfDepth'
  slug              TEXT NOT NULL UNIQUE,           -- 'Heart of Depth'
  
  name_zh           TEXT NOT NULL,
  name_en           TEXT NOT NULL,
  
  -- 类型
  set_type          TEXT NOT NULL,                  -- '4-Piece', '2-Piece'
  family            TEXT NOT NULL,                   -- 所属圣遗物系列
  
  -- 部位
  part              TEXT NOT NULL,                  -- 'Flower', 'Feather', 'Sand', 'Goblet', 'Head'
  
  -- 基础主词条
  main_stat_type    TEXT,                           -- 'HP', 'ATK', 'CRIT Rate', etc.
  main_stat_values  TEXT NOT NULL,                  -- JSON: {"Lv1": 717, "Lv20": 1050}
  
  -- 副词条池
  sub_stat_pool     TEXT NOT NULL,                  -- JSON: ["HP", "ATK", "ATK%", "CRIT Rate", ...]
  
  -- 套装效果 (JSON)
  set_effects       TEXT NOT NULL,                  -- JSON 结构
  
  -- 适配角色
  recommended_chars TEXT,                           -- JSON: ["Yoimiya", "Hu Tao"]
  
  version           TEXT,                           -- 所属版本
  
  created_at        DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_artifacts_family ON artifacts(family);
CREATE INDEX idx_artifacts_part ON artifacts(part);
```

### 2.11 element_reactions - 元素反应
```sql
CREATE TABLE element_reactions (
  id                INTEGER PRIMARY KEY AUTOINCREMENT,
  
  -- 反应标识
  reaction_key      TEXT NOT NULL UNIQUE,          -- ' Vaporize', 'Melt', 'Bloom'
  
  -- 名称
  name_zh           TEXT NOT NULL,
  name_en           TEXT NOT NULL,
  
  -- 参与元素
  elements          TEXT NOT NULL,                  -- JSON: ["Pyro", "Hydro"]
  
  -- 反应类型
  reaction_type     TEXT NOT NULL,                  -- 'Transformative', 'Amplifying'
  
  -- 基础倍率/公式
  base_formula      TEXT NOT NULL,                  -- 参考公式
  
  -- 是否有增强版本
  enhanced_version  TEXT,                           -- 'Pyro+Vaporize' vs 'Hydro+Vaporize'
  
  -- 描述
  description_zh    TEXT,
  description_en   TEXT
);
```

### 2.12 lunar_reactions - 月之反应 (v5.6+)
```sql
CREATE TABLE lunar_reactions (
  id                INTEGER PRIMARY KEY AUTOINCREMENT,
  
  reaction_key      TEXT NOT NULL UNIQUE,          -- 'Lunar-Charged', 'Lunar-Bloom', 'Lunar-Crystallize'
  
  name_zh           TEXT NOT NULL,
  name_en           TEXT NOT NULL,
  
  -- 参与元素
  base_element       TEXT NOT NULL,                 -- 'Electro', 'Dendro', 'Hydro'
  lunar_element     TEXT NOT NULL,                  -- 'Lunar' (统一月元素)
  
  -- 是否可暴击
  can_crit          BOOLEAN NOT NULL DEFAULT TRUE,
  
  -- 基础倍率公式
  base_multiplier   TEXT NOT NULL,                  -- DSL 公式
  
  -- 与剧变反应的区别
  differences       TEXT,                           -- JSON 描述差异
  
  description_zh    TEXT,
  description_en   TEXT
);
```

### 2.13 materials - 材料表
```sql
CREATE TABLE materials (
  id                INTEGER PRIMARY KEY AUTOINCREMENT,
  key               TEXT NOT NULL UNIQUE,
  
  name_zh           TEXT NOT NULL,
  name_en           TEXT NOT NULL,
  
  material_type     TEXT NOT NULL,                  -- 'CharacterAscension', 'WeaponAscension', 'Talent', 'Food', 'Common'
  
  -- 基础属性加成
  stats             TEXT,                          -- JSON: {"ATK": 8.0, "DEF": 8.0}
  
  -- 来源
  source            TEXT,                           -- 'Mondstadt', 'Ley Line', 'Crafting'
  
  -- 稀有度
  rarity            INTEGER,                        -- 1-4
  
  -- 描述
  description_zh   TEXT,
  description_en   TEXT
);

CREATE INDEX idx_materials_type ON materials(material_type);
```

### 2.14 enemies - 敌人/BOSS
```sql
CREATE TABLE enemies (
  id                INTEGER PRIMARY KEY AUTOINCREMENT,
  key               TEXT NOT NULL UNIQUE,
  
  name_zh           TEXT NOT NULL,
  name_en           TEXT NOT NULL,
  
  enemy_type        TEXT NOT NULL,                  -- 'Normal', 'Elite', 'Boss', 'Weekly Boss'
  
  -- 属性 (Lv.90/100)
  hp                REAL,
  atk               REAL,
  def               REAL,
  
  -- 抗性
  resistance        TEXT NOT NULL,                  -- JSON: {"Anemo": 0.1, "Pyro": -0.1}
  
  -- 可获奖杯
  drops             TEXT,                           -- JSON: 材料掉落列表
  
  -- 所在区域
  region            TEXT,
  
  version           TEXT
);
```

### 2.15 regions - 地区/区域
```sql
CREATE TABLE regions (
  id                INTEGER PRIMARY KEY AUTOINCREMENT,
  
  key               TEXT NOT NULL UNIQUE,          -- 'Mondstadt', 'Inazuma'
  
  name_zh           TEXT NOT NULL,
  name_en           TEXT NOT NULL,
  
  -- 元素加成
  element_bonus     TEXT,                           -- JSON: {"Anemo": 0.1}
  
  -- 包含的敌人
  enemies           TEXT,                           -- JSON: 敌人ID列表
  
  -- 描述
  description_zh   TEXT,
  description_en   TEXT
);
```

### 2.16 constellations_data - 命之座系统
```sql
CREATE TABLE constellations_data (
  id                INTEGER PRIMARY KEY AUTOINCREMENT,
  
  -- 命座编号
  constellation_no  INTEGER NOT NULL,                -- 1-6
  
  -- 效果名称
  name_zh           TEXT NOT NULL,
  name_en           TEXT NOT NULL,
  
  -- 效果描述
  description_zh   TEXT NOT NULL,
  description_en   TEXT NOT NULL,
  
  -- 效果类型
  effect_type       TEXT NOT NULL,                  -- 'Buff', 'Mechanic', 'Damage'
  
  -- 效果参数 (JSON)
  parameters        TEXT,                           -- JSON 结构化参数
  
  -- 影响的技能
  affects_skills    TEXT,                           -- JSON: ["ElementalBurst"]
  
  -- 条件
  condition         TEXT                            -- 触发条件描述
);
```

### 2.17 mooncrescent_system - 月相系统
```sql
CREATE TABLE mooncrescent_system (
  id                INTEGER PRIMARY KEY AUTOINCREMENT,
  
  level             INTEGER NOT NULL UNIQUE,       -- 1-12
  
  name_zh           TEXT NOT NULL,
  name_en           TEXT NOT NULL,
  
  -- 月相等级加成
  bonus             TEXT NOT NULL,                  -- JSON: {"CRIT Rate": 0.1, "CRIT DMG": 0.2}
  
  -- 特殊效果
  special_effect    TEXT,                           -- JSON 特殊效果
  
  -- 满辉效果
  full_bonus        TEXT,                           -- JSON
  
  description_zh   TEXT,
  description_en   TEXT
);
```

### 2.18 character_special_mechanics - 角色特殊机制
```sql
CREATE TABLE character_special_mechanics (
  id                INTEGER PRIMARY KEY AUTOINCREMENT,
  character_id      INTEGER NOT NULL REFERENCES characters(id),
  
  mechanic_type     TEXT NOT NULL,                  -- 'StanceChange', 'Energy', 'Shield', 'Heal'
  
  -- 机制名称
  name_zh           TEXT NOT NULL,
  name_en           TEXT NOT NULL,
  
  -- 详细描述
  description_zh    TEXT NOT NULL,
  description_en   TEXT NOT NULL,
  
  -- 参数 (JSON)
  parameters        TEXT NOT NULL,
  
  -- 触发条件
  trigger_condition TEXT,
  
  FOREIGN KEY (character_id) REFERENCES characters(id)
);
```

---

## 3. 索引优化

```sql
-- 常用查询索引
CREATE INDEX idx_artifacts_recommended ON artifacts(recommended_chars);
CREATE INDEX idx_enemies_region ON enemies(region);
CREATE INDEX idx_materials_rarity ON materials(rarity, material_type);
CREATE INDEX idx_char_skills_type ON character_skills(character_id, skill_type);
```

---

## 4. JSON Schema 示例

### 4.1 圣遗物套装效果
```json
{
  "2-Piece": {
    "effect": "Hydro DMG Bonus +15%",
    "effect_zh": "水元素伤害加成+15%"
  },
  "4-Piece": {
    "effect": "After using Elemental Skill, increase ATK by 18% for 10s",
    "effect_zh": "施放元素战技后，攻击力提升18%，持续10秒",
    "trigger": "ElementalSkill",
    "duration": 10,
    "atk_bonus": 0.18
  }
}
```

### 4.2 角色配置 (用于优化器)
```json
{
  "type": "Configuration",
  "data": {
    "skill2_rate": 0.65,
    "le_50": true,
    "c6_active": false
  }
}
```

### 4.3 月相系统
```json
{
  "level": 8,
  "name_zh": "盈凸月",
  "name_en": "Gibbous Moon",
  "bonus": {
    "CRIT Rate": 0.12,
    "CRIT DMG": 0.24
  },
  "special_effect": {
    "description": "Increases all party members' CRIT Rate by 12%",
    "affected_reactions": ["Vaporize", "Melt"]
  }
}
```

---

## 5. 数据来源

| 数据类型 | 来源 |
|---------|------|
| 角色/技能 | HHW (Honey Hunter World) |
| 本地化文本 | HOYO-MIYA / 游戏数据 |
| 伤害公式 | Genshin Optimizer Sheets |
| 敌人属性 | In-Game Data / Fandom Wiki |
| 月之反应 | v5.6/6.3 更新日志 |

---

## 6. 版本管理

```sql
CREATE TABLE data_version (
  id              INTEGER PRIMARY KEY,
  version         TEXT NOT NULL UNIQUE,
  release_date    DATETIME,
  changes         TEXT,                  -- JSON 更新内容
  created_at      DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

---

## 7. 备份策略

- 每次数据更新前自动备份
- 支持版本回滚
- 增量更新支持

---

## 8. 使用场景

### 8.1 圣遗物优化器
```rust
// 查询示例
SELECT * FROM characters 
WHERE key = 'Columbina' 
  AND element = 'Hydro';

// 获取技能倍率
SELECT * FROM skill_levels 
WHERE skill_id = (SELECT id FROM character_skills 
                   WHERE character_id = ? AND skill_type = 'ElementalSkill')
ORDER BY level;
```

### 8.2 伤害计算器
```rust
// 组合查询
SELECT c.base_atk, c.base_hp, w.max_atk, a.main_stat_type
FROM characters c
JOIN weapons w ON c.id = ? 
LEFT JOIN artifacts a ON a.key = 'HeartOfDepth';
```

### 8.3 队伍构建
```sql
-- 查找特定元素角色
SELECT * FROM characters 
WHERE element = 'Dendro' 
  AND rarity = 5;
```
