# 菈乌玛 & 月绽放体系修复计划

**计划编号**: PLAN-2026-02-12-LAUMA
**优先级**: P0（核心机制错误）
**预估工期**: 3-5 天
**依赖**: 需等待 Claude Code Review

---

## 1. 问题概述

GA 中月绽放（Lunar-Bloom）伤害计算存在**核心机制错误**：
- ❌ 当前：基于攻击力/防御力/生命值计算
- ✅ 正确：基于等级系数 + 精通计算（类似剧变反应）

---

## 2. 具体改动清单

### Phase 1: 月绽放基础公式修复（P0）

#### 2.1 修改文件: `mona_core/src/damage/simple_damage_builder.rs`

**位置**: 约 line 290-310

**当前代码**:
```rust
let lunar_bloom_damage = if element != Element::Hydro {
    None
} else {
    let em_bonus_lb = 1.0 + 16.0 * em / (em + 2000.0);
    let direct_lb_multiplier = 3.0;
    
    let lb_base_damage = base * direct_lb_multiplier;  // ❌ 错误：基于攻击力
    
    let dmg = DamageResult {
        critical: lb_base_damage * (1.0 + bonus) * (1.0 + critical_damage),
        non_critical: lb_base_damage * (1.0 + bonus),
        expectation: lb_base_damage * (1.0 + bonus) * (1.0 + critical_damage * critical_rate),
        is_heal: false,
        is_shield: false
    } * (defensive_ratio * resistance_ratio);
    Some(dmg)
}
```

**修改为**:
```rust
let lunar_bloom_damage = if element != Element::Hydro {
    None
} else {
    // 月绽放基础伤害 = 等级系数 × 反应倍率（类似绽放反应）
    let level_multiplier = LEVEL_MULTIPLIER[character_level - 1];
    let reaction_multiplier = 2.0; // 同绽放反应倍率
    
    // 精通加成（剧变公式：16×EM/(EM+2000)）
    let em_bonus = Reaction::transformative(em);
    
    // 月绽放专属加成
    let lunar_dmg_inc = attribute.get_value(AttributeName::LunarBloomDmgInc); // 固定伤害加成
    let lunar_base_dmg_bonus = attribute.get_value(AttributeName::LunarBloomBaseDmg); // 基础伤害加成%
    let lunar_crit_rate_bonus = attribute.get_value(AttributeName::LunarBloomCritRate);
    let lunar_crit_dmg_bonus = attribute.get_value(AttributeName::LunarBloomCritDMG);
    
    // 基础伤害计算（剧变反应类型）
    let lb_base = level_multiplier * reaction_multiplier * (1.0 + em_bonus + lunar_base_dmg_bonus);
    let lb_base = lb_base + lunar_dmg_inc; // 加上固定伤害加成
    
    // 暴击计算（月绽放可暴击）
    let total_crit_rate = (critical_rate + lunar_crit_rate_bonus).clamp(0.0, 1.0);
    let total_crit_dmg = critical_damage + lunar_crit_dmg_bonus;
    
    let dmg = DamageResult {
        critical: lb_base * (1.0 + bonus) * (1.0 + total_crit_dmg),
        non_critical: lb_base * (1.0 + bonus),
        expectation: lb_base * (1.0 + bonus) * (1.0 + total_crit_dmg * total_crit_rate),
        is_heal: false,
        is_shield: false
    } * (defensive_ratio * resistance_ratio);
    
    Some(dmg)
}
```

---

### Phase 2: 添加月绽放属性枚举（P0）

#### 2.2 修改文件: `mona_core/src/common/attribute_name.rs`

**添加以下枚举值**:
```rust
// 月绽放相关属性
LunarBloomDmgInc,      // 固定伤害加成（如：精通 × 400%）
LunarBloomBaseDmg,     // 基础伤害加成百分比（如：精通 × 14%）
LunarBloomCritRate,    // 暴击率加成
LunarBloomCritDMG,     // 暴击伤害加成
```

#### 2.3 修改文件: `mona_core/src/attribute/attribute.rs`

**在 `AttributeName` trait 中添加默认值处理**:
```rust
// 月绽放属性默认值均为0
AttributeName::LunarBloomDmgInc => 0.0,
AttributeName::LunarBloomBaseDmg => 0.0,
AttributeName::LunarBloomCritRate => 0.0,
AttributeName::LunarBloomCritDMG => 0.0,
```

---

### Phase 3: 菈乌玛角色实现完善（P1）

#### 2.4 修改文件: `mona_core/src/character/characters/dendro/lauma.rs`

**A. 添加新配置项**:
```rust
pub struct LaumaEffect {
    pub moonsign_level: usize,
    pub has_c2: bool,
    pub spirit_envoy_count: usize, // 草露数量（1-3）
}
```

**B. 修改 `change_attribute` 方法**:
```rust
impl<A: Attribute> ChangeAttribute<A> for LaumaEffect {
    fn change_attribute(&self, attribute: &mut A) {
        // A4: 奉向甘泉的沐濯（已存在，确认正确）
        let em = attribute.get_value(AttributeName::ElementalMastery);
        let a4_bonus = (em * 0.0004).min(0.32);
        attribute.set_value_by(AttributeName::BonusElementalSkill, "A4", a4_bonus);
        
        // A3: 固有天赋 - 月绽放基础伤害加成（精通 × 14%，上限200%）
        let a3_base_bonus = (em * 0.00014).min(2.0);
        attribute.set_value_by(AttributeName::LunarBloomBaseDmg, "A3", a3_base_bonus);
        
        // C2: 月兆·满辉时月绽放伤害+40%
        if self.has_c2 && self.moonsign_level >= 2 {
            attribute.set_value_by(AttributeName::LunarBloomDmg_, "C2", 0.40);
        }
    }
}
```

**C. 修改 Team Buff（大招加成）**:
```rust
fn get_target_function_by_role(...) -> Box<dyn TargetFunction> {
    // 大招期间为全队添加月绽放加成
    // lunarbloom_dmgInc = 精通 × 400%（10级）
    // 这部分在 target function 中实现
}
```

**D. 修改伤害计算（直伤月绽放）**:
```rust
fn damage_internal<D: DamageBuilder>(...) -> D::Result {
    let skill: LaumaDamageEnum = ...;
    let em = attribute.get_em_all();
    
    match skill {
        // 普通攻击 - 基于攻击力
        LaumaDamageEnum::Normal1 | ... => {
            let ratio = ...;
            let mut builder = D::new();
            builder.add_atk_ratio("Skill Ratio", ratio);
            builder.damage(...)
        }
        
        // E技能长按第二段 - 直伤月绽放（特殊处理）
        LaumaDamageEnum::E2 => {
            let spirit_count = config.spirit_envoy_count as f64; // 1-3
            let skill_ratio = LAUMA_SKILL.e_dmg2[s2]; // 273.6%
            
            // 直伤月绽放 = 等级系数 × 技能倍率 × 草露数 + 固定加成
            let level_mult = LEVEL_MULTIPLIER[character_common_data.level - 1];
            let base_dmg = level_mult * skill_ratio * spirit_count;
            
            // 加上苍色岛格固定提升（精通 × 400% × 18层 / 18）
            let cangse_bonus = em * 4.0; // 简化处理
            
            let total_base = base_dmg + cangse_bonus;
            
            // 使用标准伤害构建器
            let mut builder = D::new();
            builder.add_extra_damage("Lunar Bloom Base", total_base);
            builder.damage(...)
        }
        
        ...
    }
}
```

---

### Phase 4: 圣遗物和武器支持（P2）

#### 2.5 新圣遗物: `访月的夜歌` (Night of the Moon's Elegy)

**文件**: `mona_core/src/artifact/artifacts/night_of_moons_elegy.rs`

```rust
// 2件套: 元素精通 +80
// 4件套: 触发月绽放后，月绽放伤害提升10%，持续8秒，最多叠加3层

impl ArtifactTrait for NightOfMoonsElegy {
    fn set_effect_4(&self, data: &ArtifactSetData) -> Vec<Stat> {
        vec![
            Stat {
                name: AttributeName::LunarBloomDmg_,
                value: 0.10 * self.stacks as f64, // 10% × 层数
            }
        ]
    }
}
```

#### 2.6 新武器: `访业天竞` (Lauma's Signature)

**文件**: `mona_core/src/weapon/weapons/catalysts/fangye_tianjing.rs`

```rust
// 武器被动: 
// - +120 精通
// - 月绽放反应伤害提升40%

impl WeaponTrait for FangyeTianjing {
    fn get_passive(&self) -> Vec<Stat> {
        vec![
            Stat { name: AttributeName::ElementalMastery, value: 120.0 },
            Stat { name: AttributeName::LunarBloomDmg_, value: 0.40 },
        ]
    }
}
```

---

## 3. 验证方式

### 3.1 单元测试

**文件**: `mona_core/src/damage/tests/lunar_bloom_tests.rs`

```rust
#[test]
fn test_lunar_bloom_base_damage() {
    let level = 90;
    let em = 1619.0;
    let enemy = Enemy::default();
    
    // 预期: 等级系数 × 2.0 × (1 + 精通加成)
    // 1446.85 × 2.0 × (1 + 16×1619/3619) ≈ 48000
    let dmg = calculate_lunar_bloom_damage(level, em, &enemy);
    
    assert!(dmg.normal.expectation > 45000.0 && dmg.normal.expectation < 50000.0);
}

#[test]
fn test_lauma_direct_lunar_bloom() {
    // 输入: 精通1619, 草露3枚, -55%草抗, 122.9%暴伤
    // 预期: ~197,000
    let config = LaumaConfig {
        level: 90,
        em: 1619,
        spirit_count: 3,
        res_shred: 0.55,
        crit_dmg: 1.229,
    };
    
    let dmg = calculate_lauma_e2_damage(&config);
    
    assert!((dmg - 197000.0).abs() < 5000.0); // 误差 ±5k
}
```

### 3.2 集成测试（基于视频数据）

| 测试场景 | 输入 | 期望输出 | 容差 |
|----------|------|----------|------|
| 菈乌玛直伤月绽放 | 精通1619, 草露3, 大招18层 | 197,039 | ±1% |
| 丰穰之核（心海触发） | 精通1107%, 乐园套 | 102,395 | ±1% |
| 基础月绽放 | 90级, 精通500 | ~30,000 | ±5% |

### 3.3 与 GO 对比验证

使用相同输入参数，对比 GO 和 GA 的输出：
```bash
# GO 计算结果（参考值）
curl -X POST https://gcsim.app/api/calc \
  -d '{"character":"Lauma","em":1619,...}'

# GA 计算结果（待验证）
cargo test --test lunar_bloom -- --nocapture
```

---

## 4. 实施顺序

```
Day 1: Phase 1 + Phase 2
  - 修改月绽放基础公式
  - 添加属性枚举
  - 编译通过

Day 2: Phase 3 前半
  - 完善菈乌玛角色实现
  - 添加 A3/A4/C2 效果

Day 3: Phase 3 后半
  - 直伤月绽放特殊处理
  - 草露数量配置

Day 4: Phase 4 + 测试
  - 圣遗物和武器（可选）
  - 编写单元测试

Day 5: 验证 + Review
  - 视频数据验证
  - Claude Code Review
  - Bug修复
```

---

## 5. 风险评估

| 风险 | 可能性 | 影响 | 缓解措施 |
|------|--------|------|----------|
| 公式理解错误 | 中 | 高 | 与视频和GO双重验证 |
| 属性冲突 | 低 | 中 | 使用独立枚举值 |
| 性能下降 | 低 | 低 | 计算复杂度不变 |
| 向后兼容 | 低 | 高 | 仅影响月绽放，用户少 |

---

## 6. 相关文件清单

**需修改**:
1. `mona_core/src/damage/simple_damage_builder.rs`
2. `mona_core/src/common/attribute_name.rs`
3. `mona_core/src/attribute/attribute.rs`
4. `mona_core/src/character/characters/dendro/lauma.rs`

**需创建**:
5. `mona_core/src/damage/tests/lunar_bloom_tests.rs`
6. `mona_core/src/artifact/artifacts/night_of_moons_elegy.rs` (P2)
7. `mona_core/src/weapon/weapons/catalysts/fangye_tianjing.rs` (P2)

**参考文档**:
- Bilibili视频数据分析（已保存）
- GO实现: `libs/gi/wr/src/reaction.ts`
- GO实现: `libs/gi/sheets/src/Characters/Lauma/index.tsx`

---

## 7. 待 Claude Code Review 确认

- [ ] 月绽放公式是否正确（等级系数 × 2.0 × 精通加成）
- [ ] 菈乌玛直伤月绽放处理方式
- [ ] 属性命名规范（LunarBloomDmgInc vs lunar_bloom_dmg_inc）
- [ ] 是否需要创建独立的 `lunar_damage.rs` 模块
- [ ] 测试用例覆盖范围

---

**计划创建时间**: 2026-02-12
**等待 Review**: ⏳
