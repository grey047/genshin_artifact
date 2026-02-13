# 菈乌玛 & 月绽放体系修复计划 (整合版)

**计划编号**: PLAN-2026-02-12-LAUMA-REVISED
**优先级**: P0（核心机制错误）
**预估工期**: 5-7 天
**状态**: 已整合 Claude Code Review + AnimeGameData 研究

---

## 研究数据汇总

### 已确认数据 (AnimeGameData + 玉衡杯)

| 数据项 | 数值 | 来源 |
|--------|------|------|
| 90级等级系数 | **1446.85** | 玉衡杯公式大全 |
| A3 精通转化率 | **0.0175%** (上限14%) | AnimeGameData ProudSkill 11923 |
| A4 精通转化率 | **0.04%** (上限32%) | AnimeGameData ProudSkill 11922 |
| C6 额外伤害 | **精通×185%** | AnimeGameData Talent 1196 |
| C6 elevated | **25%** | AnimeGameData Talent 1196 param[5] |

### Claude Code Review 关键意见

1. **使用 `Reaction::lunar_em_bonus()`** 而非 `transformative()`
2. **属性命名**遵循 `Enhance*` 模式
3. **C2 实现**从 `BonusDendro` 改为 `EnhanceLunarBloom`
4. **添加 `spirit_envoy_count`** 到 `LaumaEffect`

---

## 核心概念澄清

### 月绽放体系构成

```
月兆角色在场时，草+水 → 月绽放
                    ↓
        ┌───────────┼───────────┐
        ▼           ▼           ▼
    产生草露    普通种子爆炸   被火/雷触发
    (资源)      (2.0倍率)    (烈/超绽放 3.0)
        │
        └───────────┐
                    ▼
              直伤月绽放
        (Lauma, Nefer技能)
```

### 属性定义

| 属性 | 作用对象 | 说明 |
|------|----------|------|
| `EnhanceLunarBloom` | 种子伤害 | 类似妮露丰穰之核加成 |
| `LunarBloomBaseDmg` | 种子+直伤 | A3天赋 (精通×0.0175%) |
| `LunarBloomCritRate` | 种子 | A1天赋暴击率 |
| `LunarBloomCritDMG` | 种子 | A1天赋暴击伤害 |
| `ElevateLunarBloom` | 直伤 | C6等独立乘区 |

---

## 实施阶段

### Phase 1: 基础架构 (Day 1)

#### 1.1 修改 `attribute_name.rs`

```rust
// 添加到 AttributeName 枚举
EnhanceLunarBloom,        // 月绽放反应伤害加成 (种子)
EnhanceLunarCharged,      // 月感电伤害加成
EnhanceLunarCrystallize,  // 月结晶伤害加成
LunarBloomBaseDmg,        // 月绽放基础伤害加成 (A3)
LunarBloomCritRate,       // 月绽放暴击率 (A1)
LunarBloomCritDMG,        // 月绽放暴击伤害 (A1)
ElevateLunarBloom,        // 直伤月绽放擢升加成 (C6)
```

#### 1.2 修改 `attribute.rs`

为新增属性添加默认值 0.0。

#### 1.3 修改 `reaction.rs`

确认 `lunar_em_bonus()` 函数存在且正确。

---

### Phase 2: 月绽放伤害计算修复 (Day 2)

#### 2.1 修改 `simple_damage_builder.rs`

**种子伤害计算**:
```rust
// 月绽放种子伤害 (普通草原核被月兆强化)
let lunar_bloom_seed_damage = if element != Element::Hydro {
    None
} else {
    let level_multiplier = LEVEL_MULTIPLIER[character_level - 1]; // 1446.85 at 90
    let reaction_multiplier = 2.0; // 普通草原核倍率
    
    // 精通加成: 1.0 + 16*EM/(EM+2000)
    let em_bonus = Reaction::lunar_em_bonus(em);
    
    // 基础伤害加成 (A3天赋)
    let base_dmg_bonus = attribute.get_value(AttributeName::LunarBloomBaseDmg);
    
    // 固定伤害加成 (Pale Hymn等)
    let dmg_inc = attribute.get_value(AttributeName::EnhanceLunarBloom); // 需要确认
    
    // 基础伤害
    let base = level_multiplier * reaction_multiplier * (1.0 + em_bonus - 1.0 + base_dmg_bonus);
    let base = base + dmg_inc;
    
    // 暴击 (A1天赋提供的种子暴击)
    let lb_crit_rate = attribute.get_value(AttributeName::LunarBloomCritRate);
    let lb_crit_dmg = attribute.get_value(AttributeName::LunarBloomCritDMG);
    let total_crit_rate = (critical_rate + lb_crit_rate).clamp(0.0, 1.0);
    let total_crit_dmg = critical_damage + lb_crit_dmg;
    
    // 计算最终伤害
    let dmg = DamageResult {
        critical: base * (1.0 + bonus) * (1.0 + total_crit_dmg),
        non_critical: base * (1.0 + bonus),
        expectation: base * (1.0 + bonus) * (1.0 + total_crit_dmg * total_crit_rate),
        is_heal: false,
        is_shield: false
    } * (defensive_ratio * resistance_ratio);
    
    Some(dmg)
}
```

---

### Phase 3: Lauma 角色修复 (Day 3-4)

#### 3.1 修改 `lauma.rs`

**配置结构**:
```rust
pub struct LaumaEffect {
    pub moonsign_level: usize,
    pub has_c2: bool,
    pub spirit_envoy_count: usize, // 新增: 草露数量 (1-3)
    pub has_c6: bool,
}
```

**A3 天赋实现**:
```rust
// 精通 × 0.0175%, 上限 14%
let a3_bonus = (em * 0.000175).min(0.14);
attribute.set_value_by(AttributeName::LunarBloomBaseDmg, "A3", a3_bonus);
```

**A4 天赋实现** (已有，确认正确):
```rust
// E技能伤害: 精通 × 0.04%, 上限 32%
let a4_bonus = (em * 0.0004).min(0.32);
attribute.set_value_by(AttributeName::BonusElementalSkill, "A4", a4_bonus);
```

**A1 天赋实现**:
```rust
if self.moonsign_level >= 1 {
    // 月兆·初辉: Bloom/Hyperbloom/Burgeon可暴击
    // 这部分在伤害计算中处理
    
    // 月兆·满辉: 月绽放暴击率+10%, 暴伤+20%
    if self.moonsign_level >= 2 {
        attribute.set_value_by(AttributeName::LunarBloomCritRate, "A1", 0.10);
        attribute.set_value_by(AttributeName::LunarBloomCritDMG, "A1", 0.20);
    }
}
```

**C2 修正**:
```rust
// 原代码 (错误)
// attribute.set_value_by(AttributeName::BonusDendro, "C2", 0.40);

// 修正后
if self.has_c2 && self.moonsign_level >= 2 {
    attribute.set_value_by(AttributeName::EnhanceLunarBloom, "C2", 0.40);
}
```

**E2 直伤月绽放**:
```rust
LaumaDamageEnum::E2 => {
    let spirit_count = self.spirit_envoy_count as f64;
    let skill_ratio = LAUMA_SKILL.e_dmg2[s2]; // 技能倍率
    
    // 直伤月绽放 = 精通 × 技能倍率 × 草露数
    let em = attribute.get_value(AttributeName::ElementalMastery);
    let base_dmg = em * skill_ratio * spirit_count;
    
    // 加上等级系数部分? (需要确认)
    // 或者使用特殊的直伤计算...
    
    let mut builder = D::new();
    builder.add_extra_damage("Lunar Bloom Direct", base_dmg);
    builder.damage(...)
}
```

**C6 直伤**:
```rust
// 召唤物: 精通 × 185%
// 普攻: 精通 × 150%
// 需要新增伤害类型或修改 damage_internal
```

---

### Phase 4: 武器实现 (Day 5)

#### 4.1 新建 `nightweavers_looking_glass.rs`

**文件**: `mona_core/src/weapon/weapons/catalysts/nightweavers_looking_glass.rs`

**命名依据**: Nightweaver's Looking Glass (ID 14520)

**实现内容**:
- 基础属性 (待查 AnimeGameData)
- 被动效果 (月绽放相关)

---

### Phase 5: 测试验证 (Day 6-7)

#### 5.1 单元测试

```rust
#[test]
fn test_lauma_a3_calculation() {
    // EM = 1000
    // 预期: min(1000 * 0.000175, 0.14) = 0.175 -> capped to 0.14
    let em = 1000.0;
    let bonus = (em * 0.000175).min(0.14);
    assert!((bonus - 0.14).abs() < 0.001);
}

#[test]
fn test_lunar_bloom_seed_damage() {
    // Lv90, EM=500, no bonus
    // Expected: 1446.85 * 2.0 * (1 + 16*500/2500)
    // = 1446.85 * 2.0 * (1 + 3.2) = 1446.85 * 2.0 * 4.2
}
```

#### 5.2 与视频数据对比

| 场景 | 输入 | 期望输出 |
|------|------|----------|
| Lauma E2 直伤 | EM=1619, 草露×3 | ~197,039 |
| 种子伤害 | EM=500 | ~12,000 |

---

## 修改文件清单

**需修改**:
1. `mona_core/src/common/attribute_name.rs`
2. `mona_core/src/attribute/attribute.rs`
3. `mona_core/src/damage/simple_damage_builder.rs`
4. `mona_core/src/damage/reaction.rs` (确认)
5. `mona_core/src/character/characters/dendro/lauma.rs`

**需创建**:
6. `mona_core/src/weapon/weapons/catalysts/nightweavers_looking_glass.rs`
7. `mona_core/src/damage/tests/lunar_bloom_tests.rs`

---

## 风险与注意事项

1. **E2 直伤公式**: 需要确认是否基于精通+技能倍率，还是有其他机制
2. **EnhanceLunarBloom 作用范围**: 确认是否只影响种子，还是也影响直伤
3. **Pale Hymn 系统**: 需要实现层数追踪 (0-18层)
4. **Nefer 兼容性**: 确保修复后的系统也能支持 Nefer 的直伤月绽放

---

**创建时间**: 2026-02-12  
**更新时间**: 2026-02-12 (整合版)
**状态**: 准备执行
