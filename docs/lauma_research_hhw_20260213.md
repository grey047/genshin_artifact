# Lauma Research Report - HHW Verification

**Date**: 2026-02-13  
**Source**: Honey Hunter World (https://gensh.honeyhunterworld.com/lauma_119/?lang=EN)  

---

## 1. 基础属性 (Lv90)

| 属性 | 数值 |
|------|------|
| HP | 10,654 |
| ATK | 254.96 |
| DEF | 668.64 |
| EM | 115.2 (突破加成) |

---

## 2. 技能倍率验证

### E Skill (Runo: Dawnless Rest of Karsikko)

**Hold - 2-Hit DMG (月绽放伤害)**:
| 等级 | 倍率 |
|------|------|
| Lv1 | 152% EM × 草露数 |
| Lv10 | 273.6% EM × 草露数 |
| Lv13 | 323% EM × 草露数 |
| Lv15 | 361% EM × 草露数 |

**Frostgrove Sanctuary 攻击伤害**:
| 等级 | 倍率 |
|------|------|
| Lv1 | 96% ATK + 192% EM |
| Lv10 | 172.8% ATK + 345.6% EM |
| Lv13 | 204% ATK + 408% EM |
| Lv15 | 228% ATK + 456% EM |

**元素抗性降低**: 2.5% - 40% (随等级)

---

## 3. 天赋验证

### A1 - Light for the Frosty Night (月兆)

| 月兆等级 | 效果 |
|----------|------|
| **初辉 (Nascent Gleam)** | 绽放/超绽放/烈绽放可暴击，固定15%暴击率/100%暴伤 |
| **满辉 (Ascendant Gleam)** | 月绽放暴击率+10%，暴伤+20% |

**实现状态**: ✅ 已实现 (`LunarBloomCritRate`/`LunarBloomCritDMG`)

### A3 - Moonsign Benediction (月兆祝福)

- 每点精通增加月绽放基础伤害 0.0175%，上限14%
- 队伍月兆等级+1

**实现状态**: ✅ 已实现 (`LunarBloomBaseDmg`)

### A4 - Cleansing for the Spring

- 每点精通增加E技能伤害 0.04%，上限32%
- 每点精通减少重击冷却 0.02%，上限20%

**实现状态**: ✅ 已实现

---

## 4. 命座验证

### C1 - O Lips, Weave Me Songs and Psalms

- 使用E或Q后获得「生命之线」20秒
- 附近角色触发月绽放时，为附近角色恢复生命值 (500% EM)
- 灵使形态体力消耗-40%，持续时间+5秒

**实现状态**: ❌ 未实现 (治疗机制)

### C2 - Twine Warnings and Tales From the North

- 苍色岛格效果增强：
  - 绽放/超绽放/烈绽放伤害额外增加 (500% EM)
  - 月绽放伤害额外增加 (400% EM)
- **月兆·满辉**: 月绽放伤害+40%

**实现状态**: ⚠️ 部分实现 (只实现了40%加成，Burst的Pale Hymn加成未实现)

### C6 - I Offer Blood and Tears to the Moonlight

**霜林圣殿攻击时**:
- 额外造成1次范围草元素伤害，伤害值为菈乌玛精通的 **185%**
- 该伤害视为**月绽放伤害**
- 该伤害不消耗「苍色岛格」层数
- 提供2层「苍色岛格」并刷新持续时间
- 每次霜林圣殿最多触发8次

**普通攻击时** (有苍色岛格层数):
- 消耗1层，将伤害转为草元素伤害，伤害值为精通的 **150%**
- 该伤害视为**月绽放伤害**

**月兆·满辉**: 附近所有角色月绽放伤害 **elevated** (擢升) +25%

**实现状态**: ⚠️ 部分实现
- ✅ C6 召唤物 185% EM - **已实现**
- ✅ C6 普攻 150% EM - **已实现**
- ✅ Elevate 25% - **已实现**
- ❌ Pale Hymn Burst 机制 - **未实现**

---

## 5. 与当前实现对比

### 已实现 ✅

| 机制 | 实现文件 | 状态 |
|------|----------|------|
| 月兆系统 (A1) | `lauma.rs:147-150` | ✅ 暴击率/暴伤加成 |
| A3 精通转基础伤害 | `lauma.rs:154-156` | ✅ 0.0175% EM, max 14% |
| A4 E技能精通加成 | `lauma.rs:159-161` | ✅ 0.04% EM, max 32% |
| C2 月绽放+40% | `lauma.rs:164-166` | ✅ EnhanceLunarBloom |
| C6 召唤物 185% | `lauma.rs:285-288` | ✅ 刚修复 |
| C6 普攻 150% | `lauma.rs:309-311` | ✅ 已实现 |
| C6 Elevate 25% | `lauma.rs:169-171` | ✅ ElevateLunarBloom |
| 月绽放种子伤害 | `simple_damage_builder.rs` | ✅ 等级系数×2.0 |

### 未实现/问题 ❌

| 机制 | 问题 | 优先级 |
|------|------|--------|
| Pale Hymn (Q) | Burst的层数机制和伤害加成未实现 | P2 |
| C1 治疗 | 500% EM 治疗 | P3 |
| C2 Pale Hymn 加成 | +400% EM 月绽放, +500% EM 其他绽放 | P2 |

---

## 6. 关键发现与问题

### E2 伤害计算 - 已修复 ✅

**HHW 数据**: "2-Hit Hold DMG = 152% Elemental Mastery Per Verdant Dew"

**AnimeGameData 验证**:
```
E Skill Multipliers (Hold - 2-Hit Hold DMG - EM per Verdant Dew):
Lv1: 152.0%, Lv2: 163.4%, Lv3: 174.8%, Lv4: 190.0%, Lv5: 201.4%
Lv6: 212.8%, Lv7: 228.0%, Lv8: 243.2%, Lv9: 258.4%, Lv10: 273.6%
Lv11: 288.8%, Lv12: 304.0%, Lv13: 323.0%, Lv14: 342.0%, Lv15: 361.0%
```

**问题**: 之前错误地将 C6 的 185% 用于 E2

**修正** (lauma.rs:281-289):
```rust
LaumaDamageEnum::E2 => {
    // E2 Hold: EM × skill_ratio × spirit_count
    // Lv1: 152%, Lv10: 273.6%, Lv13: 323%, Lv15: 361%
    let skill_ratio = LAUMA_SKILL.e_dmg2[s2];
    let em_ratio = skill_ratio * spirit_count;
    builder.add_em_ratio("E2 Lunar Bloom Hold", em_ratio);
}
```

### C6 185% 伤害 - 需要单独实现 ⚠️

**HHW 描述**: "Frostgrove Sanctuary 攻击时额外造成 185% 精通伤害"

**与 E2 的区别**:
| 伤害类型 | 来源 | 倍率 | 触发条件 |
|----------|------|------|----------|
| E2 Hold | 元素战技长按 | 152%-361% EM × 草露数 | 消耗草露 |
| C6 额外 | Frostgrove Sanctuary 攻击 | 固定 185% EM | 每次攻击最多8次 |

**状态**: C6 185% 需要作为单独技能实现 (E4 或额外伤害)

---

## 7. 结论

1. **核心机制已实现**: A1/A3/A4/C2/C6的关键加成
2. **C6 185% 已修正**: 召唤物伤害现在正确
3. **E2 需要重新审视**: 当前实现与HHW数据不完全匹配
4. **Pale Hymn (Q)**: Burst的完整机制较复杂，可以延后实现

---

**Research By**: Compass  
**Source**: HHW (Honey Hunter World)  
**Date**: 2026-02-13
