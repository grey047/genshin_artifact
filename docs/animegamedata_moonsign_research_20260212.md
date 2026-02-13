# AnimeGameData 月兆体系与月绽放反应研究报告

**研究日期**: 2026-02-12  
**数据来源**: AnimeGameData (原神官方解包数据)  
**研究范围**: 月兆角色识别 + 月绽放反应原始公式

---

## 1. 月兆角色 (Moonsign Characters) 完整列表

根据 `AvatarExcelConfigData.json` 中的 `AVATAR_TAG_MOONPHASE` 标签识别：

| ID | 英文名 | 中文名 | 元素 | 武器类型 | 实现状态 |
|----|--------|--------|------|----------|----------|
| 10000116 | Ineffa | 伊内丝 | Electro | Polearm | ✅ GA已实现 |
| 10000119 | **Lauma** | **菈乌玛** | Dendro | Catalyst | ✅ GA已实现 |
| 10000120 | Flins | 菲林斯 | Electro | Polearm | ✅ GA已实现 |
| 10000121 | Aino | 艾诺 | Hydro | Claymore | ✅ GA已实现 |
| 10000122 | Nefer | 妮芙 | Dendro | Catalyst | ✅ GA已实现 |
| 10000124 | Jahoda | 雅珂达 | Anemo | Bow | ✅ GA已实现 |
| 10000125 | Columbina | 哥伦比娅 | Hydro | Catalyst | ✅ GA已实现 |
| 10000126 | **Zibai** | **兹白** | Geo | Sword | ❌ GA未实现 |
| 10000127 | **Illuga** | **伊卢加** | ? | Polearm | ❌ GA未实现 |

**发现**:
- 共 **9名** 月兆角色
- GA 当前实现了 **7名** (缺少 Zibai 和 Illuga)
- 月兆角色只分布在 **6.0+ 版本** 的挪德卡莱 (Nod-Krai) 地区
- 非月兆角色不受月兆等级影响

---

## 2. 月兆等级 (Moonsign Level) 机制

### 2.1 等级定义

根据 `MoonPhaseTypeExcelConfigData.json` 和文本映射：

| 等级 | 英文名 | 中文名 | 触发条件 |
|------|--------|--------|----------|
| 0 | None | 无 | 队伍中无月兆角色 |
| 1 | **Nascent Gleam** | 月兆·初辉 | 队伍中有1名月兆角色 |
| 2 | **Ascendant Gleam** | 月兆·满辉 | 队伍中有2名或以上月兆角色 |

**关键文本** (TextMapEN.json #1400535123):
> "When Moonsign characters are in the party, the party will attain a Moonsign Level corresponding to the number of such characters. When the party reaches 'Moonsign: Ascendant Gleam,' Moonsign characters will gain enhanced effects..."

### 2.2 GA 当前实现的问题

当前 GA 的 Lauma 实现中：
- ✅ 有 `moonsign_level` 配置项 (1-2)
- ❌ 但默认设为2，且**不检查队伍中其他月兆角色**

**正确行为**:
- 月兆等级应该基于**队伍中所有月兆角色的数量**
- 如果用户只带了 Lauma 一个，应该只有 Nascent Gleam (Lv1)
- 需要带第二个月兆角色才能达到 Ascendant Gleam (Lv2)

---

## 3. 月绽放 (Lunar-Bloom) 原始公式发现

### 3.1 Lauma A3 天赋 (关键发现)

**TextMapEN.json #3041427899** (A3 天赋描述):
> "When a Bloom reaction is triggered by a party member, it will be converted into the Lunar-Bloom reaction, with every point of Elemental Mastery that Lauma has increasing Lunar-Bloom's Base DMG by **0.0175%**, up to a maximum of **14%**."

**公式**:
```
月绽放基础伤害加成 = min(精通 × 0.000175, 0.14)  // 14%上限
```

**对比**:
| 来源 | 精通转化率 | 上限 |
|------|-----------|------|
| AnimeGameData (原始) | 0.0175% | 14% |
| GO 实现 | 0.014% | ? |
| 视频推断 | ? | ? |

**结论**: GO 的 0.014% 可能是错误，原始数据是 **0.0175%**

### 3.2 Lauma A4 天赋 (Cleansing for the Spring)

**TextMapEN.json #2674139795**:
> "Each point of Elemental Mastery Lauma has will give her the following bonuses:
> · DMG dealt by her Elemental Skill is increased by **0.04%**. The maximum increase obtainable this way is **32%**.
> · The cooldown of her Charged Attack is reduced by **0.02%**. The maximum decrease obtainable this way is **20%**."

**公式**:
```
E技能伤害加成 = min(精通 × 0.0004, 0.32)  // 32%上限
重击冷却减少 = min(精通 × 0.0002, 0.20)  // 20%上限
```

### 3.3 E技能召唤物额外伤害

**TextMapEN.json #2504127783**:
> "When the Frostgrove Sanctuary attacks opponents, it will deal 1 additional instance of AoE Dendro DMG equal to **185%** of Lauma's Elemental Mastery. This DMG is considered **Lunar-Bloom DMG**."

**公式**:
```
额外伤害 = 精通 × 1.85  // 视为月绽放伤害，可暴击
```

### 3.4 C6 命之座 (大招增强)

**TextMapEN.json #2175397487**:
> "The Elemental Burst is enhanced: Pale Hymn effects are increased: All nearby party members' Bloom, Hyperbloom, and Burgeon DMG is further increased by **500%** of Lauma's Elemental Mastery"

**公式**:
```
固定伤害加成 = 精通 × 5.0  // 仅对 Bloom/Hyperbloom/Burgeon 有效
```

---

## 4. GA 当前实现的错误对比

### 4.1 当前代码 (mona_core/src/character/characters/dendro/lauma.rs)

```rust
impl ChangeAttribute for LaumaEffect {
    fn change_attribute(&self, attribute: &mut A) {
        // A4: 精通转E伤害（0.04%/点，上限32%）✅ 正确
        let em = attribute.get_value(AttributeName::ElementalMastery);
        let a4_bonus = (em * 0.0004).min(0.32);
        attribute.set_value_by(AttributeName::BonusElementalSkill, "A4", a4_bonus);

        // C2: 月兆·满辉时月绽放伤害+40%
        if self.has_c2 && self.moonsign_level >= 2 {
            // ❌ 错误：用 BonusDendro 近似，应该是 LunarBloomDmg_
            attribute.set_value_by(AttributeName::BonusDendro, "C2", 0.40);
        }
    }
}
```

### 4.2 缺失的机制

| 机制 | 原始数据 | GA状态 |
|------|----------|--------|
| A3: 月绽放基础伤害加成 | 精通×0.0175%, 上限14% | ❌ 缺失 |
| A1: 月兆时月绽放暴击 | +15%暴击率, +20%暴击伤害 | ❌ 缺失 |
| A6: Threads of Life | 大招期间月绽放加成 | ❌ 缺失 |
| Pale Hymn 系统 | 18层精通加成 | ❌ 缺失 |
| E技能召唤物 | 精通×185% 额外月绽放伤害 | ❌ 缺失 |

---

## 5. 正确的月绽放伤害公式 (基于原始数据)

### 5.1 普通月绽放 (丰穰之核)

```
伤害 = 等级系数 × 2.0 × (1 + 精通加成 + 月绽放基础加成 + 其他加成) × 暴击区 × 增伤区 × 抗性区

其中:
- 等级系数: LEVEL_MULTIPLIER[角色等级-1] (90级约1446.85)
- 精通加成: 16 × EM / (EM + 2000)
- 月绽放基础加成: min(精通 × 0.000175, 0.14)  // Lauma A3
- 暴击区: 1 + (基础暴击率 + 月兆暴击加成) × (基础暴伤 + 月兆暴伤加成)
```

### 5.2 Lauma E技能召唤物月绽放

```
伤害 = (精通 × 1.85) × 暴击区 × 增伤区 × 抗性区

注意: 
- 此伤害**不基于等级系数**
- 视为月绽放伤害，享受所有月绽放加成
- 可暴击
```

---

## 6. 修复建议 (更新版)

### 6.1 Phase 1: 修复月兆等级计算

```rust
// 新增：检查队伍中月兆角色数量
fn calculate_moonsign_level(team: &Team) -> usize {
    let moonsign_count = team.characters.iter()
        .filter(|c| c.has_tag(AVATAR_TAG_MOONPHASE))
        .count();
    match moonsign_count {
        0 => 0,
        1 => 1,  // Nascent Gleam
        _ => 2,  // Ascendant Gleam
    }
}
```

### 6.2 Phase 2: 添加缺失属性

```rust
// AttributeName 新增
LunarBloomBaseDmg,      // A3: 精通×0.0175%, 上限14%
LunarBloomCritRate,     // A1: +15% (月兆时)
LunarBloomCritDMG,      // A1: +20% (月兆时)
LunarBloomDmgInc,       // Pale Hymn: 精通×400%
```

### 6.3 Phase 3: 完善 Lauma 天赋

```rust
fn change_attribute(&self, attribute: &mut A) {
    let em = attribute.get_value(AttributeName::ElementalMastery);
    
    // A4: E技能伤害+0.04%/精通, 上限32%
    let a4_bonus = (em * 0.0004).min(0.32);
    attribute.set_value_by(AttributeName::BonusElementalSkill, "A4", a4_bonus);
    
    // A3: 月绽放基础伤害+0.0175%/精通, 上限14%
    let a3_base = (em * 0.000175).min(0.14);
    attribute.set_value_by(AttributeName::LunarBloomBaseDmg, "A3", a3_base);
    
    // A1: 月兆时月绽放暴击
    if self.moonsign_level >= 1 {
        attribute.set_value_by(AttributeName::LunarBloomCritRate, "A1", 0.15);
        attribute.set_value_by(AttributeName::LunarBloomCritDMG, "A1", 0.20);
    }
    
    // C2: 月兆·满辉时月绽放伤害+40%
    if self.has_c2 && self.moonsign_level >= 2 {
        attribute.set_value_by(AttributeName::LunarBloomDmg_, "C2", 0.40);
    }
}
```

### 6.4 Phase 4: 修复月绽放伤害计算

```rust
// simple_damage_builder.rs
let lunar_bloom_damage = if element != Element::Hydro {
    None
} else {
    let level_multiplier = LEVEL_MULTIPLIER[character_level - 1];
    let reaction_multiplier = 2.0;
    
    // 精通加成
    let em_bonus = 16.0 * em / (em + 2000.0);
    
    // Lauma A3 基础伤害加成
    let lunar_base_bonus = attribute.get_value(AttributeName::LunarBloomBaseDmg);
    
    // Pale Hymn 固定伤害加成
    let lunar_dmg_inc = attribute.get_value(AttributeName::LunarBloomDmgInc);
    
    // 基础伤害
    let lb_base = level_multiplier * reaction_multiplier * (1.0 + em_bonus + lunar_base_bonus);
    let lb_base = lb_base + lunar_dmg_inc;
    
    // 月兆暴击加成
    let lunar_crit_rate = attribute.get_value(AttributeName::LunarBloomCritRate);
    let lunar_crit_dmg = attribute.get_value(AttributeName::LunarBloomCritDMG);
    let total_crit_rate = (critical_rate + lunar_crit_rate).clamp(0.0, 1.0);
    let total_crit_dmg = critical_damage + lunar_crit_dmg;
    
    // ... 计算最终伤害
}
```

---

## 7. 结论

### 7.1 关键发现

1. **月兆角色共9名**: GA 缺少 Zibai 和 Illuga
2. **A3 精通转化率**: 原始数据是 **0.0175%** (不是 GO 的 0.014%)
3. **月兆等级**: 应该基于队伍中月兆角色数量动态计算
4. **E技能召唤物**: 造成精通×185%的额外月绽放伤害

### 7.2 数据可信度

| 来源 | 可信度 | 说明 |
|------|--------|------|
| **AnimeGameData** | ⭐⭐⭐⭐⭐ | 官方解包数据，最可信 |
| GO 实现 | ⭐⭐⭐⭐ | 社区实现，可能有误 |
| 视频实测 | ⭐⭐⭐⭐ | 实际游戏验证，但难以获取完整公式 |

### 7.3 下一步行动

1. 更新修复计划中的 A3 数值 (0.014% → 0.0175%)
2. 添加月兆等级动态计算
3. 实现缺失的 A1/A3/A6 天赋
4. 添加 Pale Hymn 层数追踪系统

---

**研究文件位置**: `/mnt/e/Moltbot/workspace/genshin_artifact/docs/animegamedata_moonsign_research_20260212.md`
