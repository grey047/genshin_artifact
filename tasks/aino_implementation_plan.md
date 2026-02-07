# Aino Implementation Plan

## 基本信息

| 字段 | 值 |
|------|-----|
| **角色名** | Aino |
| **元素** | Hydro |
| **武器** | Claymore |
| **稀有度** | 4★ |
| **数据源** | KQM, HHW |
| **Research 目录** | `.research_info/aino/` |

## 1. 数据准备

### 1.1 GO 数据检查

```bash
# 检查 GO 是否有 Aino 数据
ls libs/gi/dm-localization/assets/locales/en/char_aino*.json
ls libs/gi/dm-localization/assets/locales/chs/char_aino*.json
```

如果 GO 有数据，直接使用；否则从 HHW 抓取。

### 1.2 图片下载

```bash
# 从 HHW/KQM 下载
wget https://gensh.honeyhunterworld.com/img/characters/aino_tn.png
wget https://kqm-uploads.keqingmains.com/characters/aino_card.png
```

存储到 `.research_info/aino/character/`

### 1.3 武器图片下载

从 KQM/HHW 下载推荐武器图标：
- Favonius Greatsword
- Makhaira Aquamarine
- Forest Regalia
- Sacrificial Greatsword
- Flame-Forged Insight
- Master Key

存储到 `.research_info/aino/weapons/`

## 2. 代码结构

### 2.1 新建文件

```
mona_core/src/character/characters/hydro/aino.rs
mona_core/src/character/characters/hydro/mod.rs  # 添加 mod aino
```

### 2.2 角色结构体

```rust
pub struct Aino {
    pub char: CharacterMeta,
    pub weapon: WeaponMeta,
}
```

### 2.3 需实现的 Trait

- `CharacterFunction`
- `TargetFunction` (用于圣遗物评分)

### 2.4 核心方法

```rust
impl Aino {
    // 基础属性 (Lv90)
    pub fn base_atk(&self) -> f64 { /* 基础攻击 */ }
    pub fn base_hp(&self) -> f64 { /* 基础生命 */ }
    pub fn base_def(&self) -> f64 { /* 基础防御 */ }
    
    // 伤害计算
    pub fn normal_damage(&self, ctx: &CalculationContext) -> f64 { }
    pub fn skill_damage(&self, ctx: &CalculationContext) -> f64 { }
    pub fn burst_damage(&self, ctx: &CalculationContext) -> f64 { }
    
    // 元素战技 (Musecatcher)
    pub fn skill_cooldown(&self) -> f64 { 10.0 }  // 10s CD
    pub fn skill_cost(&self) -> f64 { 0.0 }  // 无能量消耗
    
    // 元素爆发 (Precision Hydronic Cooler)
    pub fn burst_cooldown(&self) -> f64 { 13.5 }  // 13.5s CD
    pub fn burst_energy(&self) -> f64 { 50.0 }  // 50 能量
    
    // 被动技能
    pub fn a1_bonus(&self) -> f64 { /* Moonsign 加成 */ }
    pub fn a4_bonus(&self) -> f64 { /* EM 50% 增伤 */ }
}
```

## 3. Target Function 设计

### 3.1 伤害公式

Aino 主要伤害来源：
- **Off-field Hydro** — 不造成主要伤害
- **Transformative Reactions** — Bloom, Hyperbloom
- **Lunar-Charged** — 与 Nod-Krai 角色配合

### 3.2 权重配置

```rust
impl TargetFunction for Aino {
    fn target(&self) -> CharacterTarget {
        // Aino 主要堆叠：
        // 1. Energy Recharge (直到满足充能需求: 190-250%)
        // 2. Elemental Mastery (反应伤害)
        // 3. CRIT Rate (Favonius 触发)
        CharacterTarget {
            atk_percentage: 0.5,  // 中等
            atk_fixed: 0.1,       // 较低
            hp_percentage: 0.0,
            hp_fixed: 0.0,
            def_percentage: 0.0,
            def_fixed: 0.0,
            elemental_mastery: 1.5,  // 高优先级
            recharge: 1.2,          // 充能优先
            crit_rate: 1.0,         // Favonius 需要
            crit_damage: 0.8,
            bonus_hydro: 1.0,
            bonus_physical: 0.0,
        }
    }
}
```

### 3.3 注意事项

- **Hydro 反应伤害** = EM 决定
- **不需要暴击** (除非触发反应)
- **充能是第一位** — 没法开大 = 没用

## 4. 任务列表

- [ ] 4.1.1 检查 GO 数据是否完整
- [ ] 4.1.2 从 HHW 抓取缺失数据
- [ ] 4.1.3 下载角色图片
- [ ] 4.1.4 下载武器图片
- [ ] 4.1.5 实现 `aino.rs` 角色结构
- [ ] 4.1.6 实现 `mod.rs` 导出
- [ ] 4.1.7 实现 Target Function
- [ ] 4.1.8 编译测试
- [ ] 4.1.9 Claude Code Review
- [ ] 4.1.10 合并代码

## 5. 依赖

- `mona_core::character::CharacterMeta`
- `mona_core::weapon::WeaponMeta`
- `mona_core::target_functions::CharacterTarget`
- `crate::config::CharacterFunctionConfig`

## 6. 测试用例

```rust
#[test]
fn test_aino_base_stats() {
    let aino = Aino::new();
    assert!aino.base_atk() > 200.0);  // 4★ 基础攻击应该在 200+ 范围
}

#[test]
fn test_aino_skill_cooldown() {
    let aino = Aino::new();
    assert_eq!(aino.skill_cooldown(), 10.0);
}

#[test]
fn test_aino_burst_energy() {
    let aino = Aino::new();
    assert_eq!(aino.burst_energy(), 50.0);
}
```

## 7. 参考代码

- `mona_core/src/character/characters/cryo/skirk.rs` (Cryo/Sword)
- `mona_core/src/character/characters/electro/flins.rs` (Electro/Catalyst)
- `mona_core/src/character/characters/cryo/escoffier.rs` (Cryo/Polearm)

## 8. 注意事项

1. **Aino 是 4★ 角色**，不是 5★
2. **Moonsign 系统** — 需要与 Nod-Krai 角色配合
3. **Off-field 定位** — 不要设计成主 C
4. **EM 收益高** — 伤害主要来自反应
