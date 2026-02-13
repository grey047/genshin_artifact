# Task: 月兆系统 (Moonsign System) 架构设计与实现

**任务编号**: TASK-2026-02-12-MOONSIGN-SYSTEM  
**优先级**: P1 (基础设施)  
**依赖**: 无  
**阻塞**: 所有挪德卡莱角色实现  

---

## 1. 背景与目标

### 1.1 什么是月兆系统

月兆 (Moonsign) 是原神 6.0 版本挪德卡莱地区引入的全新团队机制：
- **月兆等级** 基于队伍中月兆角色的数量动态计算
- 影响所有月兆角色的技能效果、天赋加成
- 影响月反应 (Lunar Reactions) 的伤害计算

### 1.2 当前问题

GA 当前实现：
- ❌ 每个角色独立配置 `moonsign_level`，不检查队伍构成
- ❌ 月兆等级不会随队伍变化而动态更新
- ❌ 非月兆角色无法享受月兆加成

### 1.3 目标

建立完整的月兆系统架构：
1. 角色标签系统 (识别月兆角色)
2. 队伍级月兆等级计算
3. 动态属性更新机制
4. 与非月兆角色的交互

---

## 2. 系统设计

### 2.1 月兆角色清单 (基于 AnimeGameData)

| 角色ID | 英文名 | 中文名 | 元素 | 武器 | GA状态 |
|--------|--------|--------|------|------|--------|
| 10000116 | Ineffa | 伊内丝 | Electro | Polearm | ✅ 已实现 |
| 10000119 | Lauma | 菈乌玛 | Dendro | Catalyst | ✅ 已实现 |
| 10000120 | Flins | 菲林斯 | Electro | Polearm | ✅ 已实现 |
| 10000121 | Aino | 艾诺 | Hydro | Claymore | ✅ 已实现 |
| 10000122 | Nefer | 妮芙 | Dendro | Catalyst | ✅ 已实现 |
| 10000124 | Jahoda | 雅珂达 | Anemo | Bow | ✅ 已实现 |
| 10000125 | Columbina | 哥伦比娅 | Hydro | Catalyst | ✅ 已实现 |
| 10000126 | **Zibai** | **兹白** | Geo | Sword | ❌ **待实现** |
| 10000127 | **Illuga** | **伊卢加** | ? | Polearm | ❌ **待实现** |

**数据来源**: `AvatarExcelConfigData.json` 中的 `AVATAR_TAG_MOONPHASE` 标签

### 2.2 月兆等级定义

```rust
pub enum MoonsignLevel {
    None = 0,           // 无月兆角色
    NascentGleam = 1,   // 月兆·初辉 (1名月兆角色)
    AscendantGleam = 2, // 月兆·满辉 (2名或以上月兆角色)
}
```

**关键发现** (TextMapEN.json #1400535123):
> "When Moonsign characters are in the party, the party will attain a Moonsign Level corresponding to the number of such characters."

### 2.3 核心架构

```
┌─────────────────────────────────────────────────────────────┐
│                    Team (队伍)                              │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  MoonsignManager (月兆管理器)                        │   │
│  │  - count_moonsign_chars() -> usize                  │   │
│  │  - get_moonsign_level() -> MoonsignLevel            │   │
│  │  - notify_level_change()                            │   │
│  └─────────────────────────────────────────────────────┘   │
│                          │                                  │
│              ┌───────────┼───────────┐                      │
│              ▼           ▼           ▼                      │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐        │
│  │  Character 1 │ │  Character 2 │ │  Character 3 │        │
│  │ (Moonsign)   │ │ (Moonsign)   │ │ (Non-Moon)   │        │
│  └──────────────┘ └──────────────┘ └──────────────┘        │
│         │                │                │                │
│         ▼                ▼                ▼                │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐        │
│  │ apply        │ │ apply        │ │ apply        │        │
│  │ _moonsign_   │ │ _moonsign_   │ │ _moonsign_   │        │
│  │ bonuses(Lv2) │ │ bonuses(Lv2) │ │ bonuses(Lv2) │        │
│  └──────────────┘ └──────────────┘ └──────────────┘        │
└─────────────────────────────────────────────────────────────┘
```

---

## 3. 实现计划

### Phase 1: 基础架构 (P1)

#### 3.1 添加角色标签系统

**文件**: `mona_core/src/character/character_common_data.rs`

```rust
pub struct CharacterCommonData {
    // ... existing fields
    
    /// 角色标签 (用于识别特殊机制角色)
    pub tags: Vec<CharacterTag>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterTag {
    Moonsign,       // 月兆角色
    Fatui,          // 愚人众
    // ... 其他标签
}
```

#### 3.2 标记月兆角色

**修改文件**: `mona_core/src/character/characters/*/mod.rs`

为以下角色添加 `CharacterTag::Moonsign`:
- Ineffa
- Lauma
- Flins
- Aino
- Nefer
- Jahoda
- Columbina

#### 3.3 创建月兆等级枚举

**文件**: `mona_core/src/common/moonsign.rs` (新建)

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MoonsignLevel {
    #[default]
    None = 0,
    NascentGleam = 1,
    AscendantGleam = 2,
}

impl MoonsignLevel {
    pub fn from_count(count: usize) -> Self {
        match count {
            0 => Self::None,
            1 => Self::NascentGleam,
            _ => Self::AscendantGleam,
        }
    }
    
    pub fn is_ascendant(&self) -> bool {
        matches!(self, Self::AscendantGleam)
    }
}
```

### Phase 2: 队伍级集成 (P1)

#### 3.4 队伍月兆管理器

**文件**: `mona_core/src/team/moonsign_manager.rs` (新建)

```rust
use crate::character::{CharacterCommonData, CharacterTag};
use crate::common::MoonsignLevel;

pub struct MoonsignManager {
    level: MoonsignLevel,
    character_indices: Vec<usize>, // 月兆角色在队伍中的索引
}

impl MoonsignManager {
    pub fn new(team: &[Option<CharacterCommonData>]) -> Self {
        let character_indices: Vec<usize> = team
            .iter()
            .enumerate()
            .filter_map(|(idx, char_opt)| {
                char_opt.as_ref().and_then(|c| {
                    if c.tags.contains(&CharacterTag::Moonsign) {
                        Some(idx)
                    } else {
                        None
                    }
                })
            })
            .collect();
        
        let level = MoonsignLevel::from_count(character_indices.len());
        
        Self {
            level,
            character_indices,
        }
    }
    
    pub fn get_level(&self) -> MoonsignLevel {
        self.level
    }
    
    pub fn is_character_moonsign(&self, index: usize) -> bool {
        self.character_indices.contains(&index)
    }
    
    pub fn get_moonsign_character_count(&self) -> usize {
        self.character_indices.len()
    }
}
```

#### 3.5 集成到 Team 结构

**文件**: `mona_core/src/team/team.rs`

```rust
use crate::team::moonsign_manager::MoonsignManager;

pub struct Team {
    pub characters: [Option<CharacterCommonData>; 4],
    pub moonsign_manager: MoonsignManager,
    // ...
}

impl Team {
    pub fn new(...) -> Self {
        let moonsign_manager = MoonsignManager::new(&characters);
        Self {
            characters,
            moonsign_manager,
            // ...
        }
    }
}
```

### Phase 3: 角色配置更新 (P2)

#### 3.6 移除独立的 moonsign_level 配置

**当前问题**: Lauma 等角色有独立的 `moonsign_level` 配置项

**修改方案**:
1. 保留配置项用于 **强制覆盖** (debug/测试用途)
2. 默认使用队伍计算的月兆等级
3. 如果配置为 `0` 或 `Auto`，使用队伍值

**修改文件**: `mona_core/src/character/characters/dendro/lauma.rs`

```rust
pub struct LaumaEffect {
    pub moonsign_level: MoonsignLevel,  // 从队伍获取，不再是配置项
    pub has_c2: bool,
    pub config_level_override: Option<usize>, // 仅用于强制覆盖
}

impl<A: Attribute> ChangeAttribute<A> for LaumaEffect {
    fn change_attribute(&self, attribute: &mut A) {
        let effective_level = self.config_level_override
            .map(|l| MoonsignLevel::from_count(l))
            .unwrap_or(self.moonsign_level);
        
        // 应用基于 effective_level 的加成...
    }
}
```

### Phase 4: 新角色实现 (P2)

#### 3.7 实现 Zibai (兹白)

**参考数据**:
- ID: 10000126
- 元素: Geo
- 武器: Sword
- 标签: AVATAR_TAG_MOONPHASE

**文件**:
- `mona_core/src/character/characters/geo/zibai.rs` (新建)
- `mona_core/src/target_functions/target_functions/geo/zibai_default.rs` (新建)

#### 3.8 实现 Illuga (伊卢加)

**参考数据**:
- ID: 10000127
- 武器: Polearm
- 标签: AVATAR_TAG_MOONPHASE
- 元素: 待确认

---

## 4. 与其他系统的关系

### 4.1 与月反应系统的关系

```
月兆等级 ──┬──► 月感电 (Lunar-Charged) 伤害加成
          ├──► 月绽放 (Lunar-Bloom) 伤害加成  
          ├──► 月结晶 (Lunar-Crystallize) 伤害加成
          └──► 角色天赋触发条件
```

### 4.2 与菈乌玛实现的关系

菈乌玛的以下效果依赖月兆等级：
- A1 天赋: 月兆·初辉/满辉提供不同暴击加成
- A3 天赋: 使队伍月兆等级+1 (TextMapEN.json #3041427899)
- C6 命座: 月兆·满辉时全队月绽放伤害 elevated +25%

**注意**: A3 使队伍月兆等级+1 的效果需要特别处理，可能使 Nascent→Ascendant

---

## 5. 验证清单

- [ ] 9名月兆角色全部标记正确
- [ ] 队伍中0/1/2名月兆角色时等级计算正确
- [ ] 月兆等级变化时属性正确更新
- [ ] 非月兆角色也能享受部分月兆加成 (如满辉时的团队Buff)
- [ ] Zibai 和 Illuga 角色实现完成

---

## 6. 参考文件

- AnimeGameData: `ExcelBinOutput/AvatarExcelConfigData.json` (AVATAR_TAG_MOONPHASE)
- AnimeGameData: `ExcelBinOutput/MoonPhaseTypeExcelConfigData.json`
- TextMap: `TextMapEN.json` #1400535123, #3041427899
- GA实现: `mona_core/src/character/characters/dendro/lauma.rs`

---

**创建时间**: 2026-02-12  
**状态**: 待实现
