# GO vs GA 菈乌玛 & 月绽放实现对比分析

**分析日期**: 2026-02-12
**分析依据**: 
- Bilibili 视频《菈乌玛：月绽放反应伤害计算！》(UP: 那菈小离)
- GO (genshin-optimizer) 实现
- GA (genshin_artifact) 当前实现

---

## 1. 核心发现：三种不同的计算模型

| 模型 | 基础属性 | 倍率来源 | 适用场景 |
|------|----------|----------|----------|
| **视频实测** | 等级系数 + 精通 | 技能倍率 × 草露数 | 菈乌玛直伤月绽放 |
| **GO实现** | 精通 | 技能倍率 | 所有月绽放伤害 |
| **GA当前** | 攻击力/防御力/生命 | 固定3.0倍率 | ❌ 错误实现 |

---

## 2. GO 实现详解

### 2.1 关键函数：`lunarDmg()`

```typescript
// libs/gi/wr/src/reaction.ts
export function lunarDmg(
  multiplier: NumNode,        // 伤害倍率
  base: 'reaction' | MainStatKey | SubstatKey,  // 基础属性
  variant: LunarReactionKey,  // 'lunarbloom' | 'lunarcharged' | 'lunarcrystallize'
  additional: Data = {},
  specialMultiplier?: NumNode
)
```

**伤害公式**（GO）：
```
伤害 = [
  倍率 × 基础值 × 剧变加成 × (1 + 基础伤害加成%) 
  + 固定伤害加成
] × 暴击区 × 特殊伤害加成 × 防御 × 抗性

其中：
- 倍率：技能倍率（如 273.6%）
- 基础值：当 base='eleMas' 时，为精通值
- 剧变加成：1 + 16×EM/(EM+2000) + 月绽放伤害加成%
- 暴击区：1 + (基础暴击率 + 月绽放暴击率) × (基础暴伤 + 月绽放暴伤)
```

### 2.2 菈乌玛 E技能长按伤害（GO）

```typescript
// libs/gi/sheets/src/Characters/Lauma/index.tsx
hold2Dmg: lunarDmg(
  prod(
    skillVerdantDew,  // 草露数量（1-3）
    subscript(input.total.skillIndex, dm.skill.hold2Dmg, { unit: '%' })
  ),
  'eleMas',  // 基于精通！
  'lunarbloom'
),
```

**关键差异**：
- GO 使用 `lunarDmg` 函数
- 基础属性是 `'eleMas'`（精通）
- 倍率包含草露数量（1-3）

### 2.3 菈乌玛 A3（被动3）基础伤害加成

```typescript
const a0_lunarbloom_baseDmg_ = min(
  prod(percent(dm.passive3.base_lunarBloom_dmg_), input.total.eleMas),
  percent(dm.passive3.maxBase_lunarBloom_dmg_)
)
```

**作用**：为月绽放提供基础伤害加成（精通 × 14%，上限约200%）

### 2.4 菈乌玛大招加成（GO）

```typescript
burstPaleHymn_lunarbloom_dmgInc: equal(
  condBurstPaleHymn,
  'on',
  prod(
    subscript(input.total.burstIndex, dm.burst.lunarBloomDmgInc, { unit: '%' }),
    input.total.eleMas  // 基于精通！
  )
)
```

**加成值**：精通 × 400%（10级大招）

### 2.5 GO Team Buff 汇总

```typescript
teamBuff: {
  premod: {
    // 固定伤害加成（精通 × 400%）
    lunarbloom_dmgInc: sum(burstPaleHymn_lunarbloom_dmgInc, c2PaleHymn_lunarbloom_dmgInc),
    
    // 暴击率加成（月兆时）
    lunarbloom_critRate_: a1AfterSkill_lunarBloom_critRate_,  // 15%
    
    // 暴击伤害加成（月兆时）
    lunarbloom_critDMG_: a1AfterSkill_lunarBloom_critDMG_,    // 20%
    
    // 基础伤害加成（精通 × 14%）
    lunarbloom_baseDmg_: a0_lunarbloom_baseDmg_,
    
    // C2 伤害加成
    lunarbloom_dmg_: c2Ascendant_lunarbloom_dmg_,  // 40%
  }
}
```

---

## 3. GA 当前实现问题

### 3.1 问题代码位置

```rust
// mona_core/src/damage/simple_damage_builder.rs (lines ~290-310)
let lunar_bloom_damage = if element != Element::Hydro {
    None
} else {
    let em_bonus_lb = 1.0 + 16.0 * em / (em + 2000.0);
    let direct_lb_multiplier = 3.0;
    
    // ❌ 错误：基于攻击力/防御力/生命值
    let lb_base_damage = base * direct_lb_multiplier;
    
    let dmg = DamageResult {
        critical: lb_base_damage * (1.0 + bonus) * (1.0 + critical_damage),
        ...
    } * (defensive_ratio * resistance_ratio);
    Some(dmg)
}
```

### 3.2 问题分析

| 问题 | 当前实现 | 正确实现 |
|------|----------|----------|
| **基础伤害** | `base` (攻击力/防御力/生命) | `eleMas` (精通) 或 等级系数 × 技能倍率 |
| **倍率** | 固定 3.0 | 技能倍率（如 273.6%）× 草露数 |
| **剧变加成** | 有 | 有（正确） |
| **固定加成** | 无 | `lunarbloom_dmgInc`（精通 × 400%） |
| **基础伤害加成** | 无 | `lunarbloom_baseDmg_`（精通 × 14%） |
| **暴击** | 基于面板 | 基础暴击 + 月绽放专属暴击 |

---

## 4. 视频实测 vs GO vs GA 对比

### 4.1 菈乌玛直伤月绽放（E技能长按第二段）

**输入参数**（来自视频）：
- 角色等级：90
- 精通：1619
- 技能等级：10级E
- 草露数量：3枚
- 苍色岛格：18层（精通 × 400%）
- 抗性：-55%草抗（1.225倍）
- 暴伤：122.9%
- 反应提升：318.36%

**计算结果对比**：

| 来源 | 计算方式 | 结果 | 视频实测 |
|------|----------|------|----------|
| **视频** | 等级系数 × 273.6% × 3 × 318.36% × 1.225 × 2.229 + 6476 | 197,038 | ✅ 197,039 |
| **GO** | lunarDmg(273.6% × 3, eleMas=1619, dmgInc=6476, ...) | ~197,000 | ✅ 吻合 |
| **GA当前** | base(攻击力) × 3.0 × ... | ❌ 错误 | ❌ 不吻合 |

### 4.2 丰穰之核（草原核）伤害

**输入参数**（来自视频）：
- 触发者：心海（乐园套）
- 反应提升总计：1107.74%
- 妮露天赋：基于生命值
- 菈乌玛武器：+120%
- 乐园套：+80%

**计算结果对比**：

| 来源 | 计算方式 | 结果 | 视频实测 |
|------|----------|------|----------|
| **视频** | 1446.85 × 2.0 × 1107.74% × 1.225 × 2.0 | 102,394 | ✅ 102,395 |
| **GO** | transformative(等级系数 × 2.0 × (1 + 精通加成 + 反应加成)) | ~102,000 | ✅ 吻合 |
| **GA当前** | base × 3.0 × ... | ❌ 错误 | ❌ 不吻合 |

---

## 5. 修复建议

### 5.1 短期修复（保持现有架构）

修改 `simple_damage_builder.rs`：

```rust
// 月绽放伤害 - 基于精通（类似剧变反应）
let lunar_bloom_damage = if element != Element::Hydro {
    None
} else {
    // 基础伤害 = 等级系数 × 反应倍率（类似绽放）
    let level_multiplier = LEVEL_MULTIPLIER[character_level - 1];
    let reaction_base = 2.0; // 基础倍率，同绽放
    
    // 精通加成（剧变公式）
    let em_bonus = Reaction::transformative(em);
    
    // 月绽放特定加成
    let lunar_dmg_inc = attribute.get_value(AttributeName::LunarBloomDmgInc); // 精通 × 400%
    let lunar_base_dmg = attribute.get_value(AttributeName::LunarBloomBaseDmg); // 精通 × 14%
    
    // 基础伤害（等级系数 × 倍率 × 精通加成）
    let lb_base = level_multiplier * reaction_base * (1.0 + em_bonus) + lunar_dmg_inc;
    let lb_base = lb_base * (1.0 + lunar_base_dmg); // 基础伤害加成
    
    // 应用暴击、增伤、抗性
    let lunar_crit_rate = attribute.get_value(AttributeName::LunarBloomCritRate);
    let lunar_crit_dmg = attribute.get_value(AttributeName::LunarBloomCritDMG);
    let total_crit_rate = (critical_rate + lunar_crit_rate).clamp(0.0, 1.0);
    let total_crit_dmg = critical_damage + lunar_crit_dmg;
    
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

### 5.2 长期优化（参考 GO）

1. **创建 `lunar_dmg` 模块**
   - 类似 GO 的 `lunarDmg()` 函数
   - 支持三种月反应：LunarCharged / LunarBloom / LunarCrystallize

2. **添加菈乌玛专属伤害类型**
   - 区分「直伤月绽放」和「丰穰之核」
   - 直伤月绽放使用技能倍率 × 草露数

3. **完善属性系统**
   ```rust
   // AttributeName 新增
   LunarBloomDmgInc,      // 固定伤害加成（精通 × 400%）
   LunarBloomBaseDmg,     // 基础伤害加成（精通 × 14%）
   LunarBloomCritRate,    // 暴击率加成（15%）
   LunarBloomCritDMG,     // 暴击伤害加成（20%）
   SpiritEnvoyCount,      // 草露数量（1-3）
   ```

---

## 6. 结论

| 项目 | GO | GA当前 | 视频验证 |
|------|-----|--------|----------|
| **架构设计** | ✅ 完善的 `lunarDmg` 函数 | ❌ 临时修补 | - |
| **基础属性** | ✅ 精通 | ❌ 攻击力 | ✅ 等级系数+精通 |
| **倍率系统** | ✅ 技能倍率 | ❌ 固定3.0 | ✅ 技能倍率×草露 |
| **固定加成** | ✅ 精通×400% | ❌ 缺失 | ✅ 有 |
| **基础加成** | ✅ 精通×14% | ❌ 缺失 | ✅ 有 |
| **暴击系统** | ✅ 专属暴击率/暴伤 | ❌ 面板暴击 | ✅ 专属暴击 |
| **团队Buff** | ✅ 完整实现 | ❌ 缺失 | ✅ 有 |

**关键差距**：
1. GA 把月绽放当作「基于攻击力」的伤害类型
2. 实际上应该是「基于精通」的剧变反应类型
3. 菈乌玛的「直伤月绽放」需要额外处理技能倍率 × 草露数

**建议**：
- 🔴 **P0** - 参考 GO 的 `lunarDmg` 函数重构月绽放伤害
- 🟡 **P1** - 添加菈乌玛专属属性和Buff
- 🟢 **P2** - 完善测试用例（使用视频中的实测数据）

---

**参考文件**：
- GO: `libs/gi/wr/src/reaction.ts` (lunarDmg函数)
- GO: `libs/gi/sheets/src/Characters/Lauma/index.tsx` (菈乌玛实现)
- GA: `mona_core/src/damage/simple_damage_builder.rs` (需修复)
- GA: `mona_core/src/character/characters/dendro/lauma.rs` (菈乌玛实现)
