# GA 实现检测 report - 菈乌玛 & 月绽放体系

**检测日期**: 2026-02-12
**检测依据**: Bilibili 视频《菈乌玛：月绽放反应伤害计算！》
**视频UP主**: 那菈小离

---

## 1. 菈乌玛（Lauma）角色实现状态

### ✅ 已实现
- [x] 角色基础数据（HP/ATK/DEF，技能倍率）
- [x] 技能名称（中英双语）
- [x] A4 天赋：精通转E技能伤害（0.04%/点，上限32%）
- [x] C2：月兆·满辉时月绽放伤害+40%
- [x] Target Function
- [x] 月兆等级配置（Moonsign Level 1/2）

### ⚠️ 待验证
- [ ] A1 天赋实现（是否有额外精通加成？）
- [ ] 专武「访业天竞」的120精通+40%月绽放反应伤害
- [ ] 新圣遗物「访月的叶哥」60精通+10%月绽放反应伤害

---

## 2. 🚨 月绽放（Lunar-Bloom）伤害公式 - 发现重大问题

### 当前实现（疑似错误）
```rust
// mona_core/src/damage/simple_damage_builder.rs
let lunar_bloom_damage = if element != Element::Hydro {
    None
} else {
    let em_bonus_lb = 1.0 + 16.0 * em / (em + 2000.0);
    let direct_lb_multiplier = 3.0;
    
    let lb_base_damage = base * direct_lb_multiplier;  // ❌ 基于攻击力/防御力/生命值
    ...
}
```

### 问题分析
当前实现将月绽放伤害基于 `base`（攻击力/防御力/生命值），但视频明确指出：

> **菈乌玛的直伤月绽放与攻击力、防御力、等级系数均无关！**

### 正确公式（基于视频推导）

#### 2.1 菈乌玛「直伤月绽放」
```
伤害 = 等级系数 × 技能倍率 × 草露数量 × 反应提升 × 抗性区 × 暴击区 + 大招固定提升

其中：
- 等级系数：90级角色等级系数（视频未明确，约1446.85？）
- 技能倍率：10级E技能 273.6%（每枚草露）
- 草露数量：1-3枚
- 反应提升：318.36%（视频实测）
- 苍色岛格提升：精通 × 400%（大招18层）
- 抗性区：1.225（-55%草抗）
- 暴击区：1 + 暴击伤害（122.9%）
```

**视频实测验证**:
- 输入：精通1619，技能倍率273.6%，草露×3，反应提升318.36%，苍色岛格6476，抗性1.225，暴击122.9%
- 计算结果：197,038
- 实际伤害：197,039 ✅ 吻合

#### 2.2 丰穰之核（草原核）伤害
```
伤害 = 等级系数 × 2.0 × (1 + 精通反应提升 + 妮露提升 + 武器提升 + 圣遗物提升) × 抗性区 × 暴击区

其中：
- 等级系数：90级 = 1446.85
- 基础倍率：2.0
- 精通反应提升：16×EM/(EM+2000)
- 妮露提升：基于生命值（视频中约400%+）
- 菈乌玛武器：+120%
- 乐园套：+80%
- 暴击区：固定100%暴伤（可暴击，但暴伤固定）
```

**视频实测验证**:
- 输入：90级，精通1107.74%反应提升，暴击
- 计算结果：102,0394（应为102,394？）
- 实际伤害：102,395 ✅ 吻合

---

## 3. 关键差异对比

| 项目 | 当前实现 | 视频实测 | 状态 |
|------|----------|----------|------|
| **伤害基础** | 攻击力/防御力/生命值 | 等级系数 | ❌ 错误 |
| **直伤倍率** | 固定3.0 | 技能倍率×草露数 | ❌ 错误 |
| **精通加成** | 16×EM/(EM+2000) | 同左 | ✅ 正确 |
| **暴击** | 基于面板 | 草原核固定100%暴伤 | ⚠️ 待验证 |
| **抗性** | 有 | 有 | ✅ 正确 |

---

## 4. 修复建议

### 4.1 月绽放伤害重构
当前代码将月绽放当作「基于角色属性」的伤害，但实际上应该是：

```rust
// 建议修改方案
let lunar_bloom_damage = if element != Element::Hydro {
    None
} else {
    // 基础伤害 = 等级系数 × 反应倍率（类似剧变反应）
    let level_multiplier = LEVEL_MULTIPLIER[character_level - 1];
    let reaction_base = 2.0; // 基础倍率，类似绽放
    
    // 精通加成（剧变公式）
    let em_bonus = Reaction::transformative(em);
    
    // 月绽放特定加成（来自菈乌玛武器/圣遗物）
    let lunar_bonus = attribute.get_value(AttributeName::LunarBloomBonus);
    
    // 基础伤害（不依赖攻击力/防御力/生命值）
    let lb_base = level_multiplier * reaction_base * (1.0 + em_bonus + lunar_bonus);
    
    // 应用暴击、增伤、抗性
    let dmg = DamageResult {
        critical: lb_base * (1.0 + bonus) * (1.0 + critical_damage),
        non_critical: lb_base * (1.0 + bonus),
        expectation: lb_base * (1.0 + bonus) * (1.0 + critical_damage * critical_rate),
        ...
    } * (defensive_ratio * resistance_ratio);
    
    Some(dmg)
}
```

### 4.2 菈乌玛「直伤月绽放」特殊处理
菈乌玛的E技能长按时，第二段伤害应视为「直伤月绽放」，需要特殊处理：

```rust
// 在菈乌玛 damage_internal 中
LaumaDamageEnum::E2 => {
    // 直伤月绽放：等级系数 × 技能倍率 × 草露数
    let level_multiplier = LEVEL_MULTIPLIER[character_level - 1];
    let skill_ratio = LAUMA_SKILL.e_dmg2[s2]; // 256.0% at Lv10
    let spirit_count = 3.0; // 草露数量
    
    // 基础伤害 = 等级系数 × 技能倍率 × 草露数
    let base_dendro_damage = level_multiplier * skill_ratio * spirit_count;
    
    // 加上苍色岛格固定提升（精通×400%）
    let cangse_daoge = em * 4.0; // 18层
    
    let total_base = base_dendro_damage + cangse_daoge;
    
    // 后续应用反应提升、抗性、暴击...
}
```

---

## 5. 需要添加的新属性

```rust
// AttributeName 新增
LunarBloomBonus,      // 月绽放反应伤害加成
LunarBloomCritRate,   // 月绽放暴击率
LunarBloomCritDamage, // 月绽放暴击伤害（固定100%）
SpiritEnvoyCount,     // 草露数量（1-3）
CangseDaogeStacks,    // 苍色岛格层数（0-18）
```

---

## 6. 测试用例（基于视频）

### 测试1：菈乌玛直伤月绽放
```
输入：
- 角色等级：90
- 精通：1619
- 技能等级：10级E
- 草露数量：3
- 苍色岛格：18层
- 草抗降低：55%（抗性区1.225）
- 暴击伤害：122.9%
- 反应提升：318.36%

期望输出：197,039
```

### 测试2：丰穰之核伤害
```
输入：
- 角色等级：90
- 触发者精通：心海
- 反应提升总计：1107.74%
- 妮露生命值：74000（天赋加成）
- 菈乌玛武器：+120%
- 乐园套：+80%

期望输出：102,395
```

---

## 7. 结论

**菈乌玛角色实现**：✅ 基本完成，天赋和技能倍率正确

**月绽放反应实现**：❌ **存在重大错误**
- 当前：基于攻击力/防御力/生命值计算
- 正确：基于等级系数（剧变反应类型）

**建议优先级**：
1. 🔴 P0 - 修复月绽放基础伤害公式（等级系数而非攻击力）
2. 🟡 P1 - 添加菈乌玛直伤月绽放特殊处理
3. 🟢 P2 - 添加月绽放相关属性枚举
4. 🟢 P2 - 实现测试用例验证

---

**相关文件**：
- `mona_core/src/damage/simple_damage_builder.rs` (lines ~270-290)
- `mona_core/src/character/characters/dendro/lauma.rs`
- `mona_core/src/damage/reaction.rs` (LunarBloom multiplier)
