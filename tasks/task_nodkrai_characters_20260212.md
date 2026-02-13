# Task: 挪德卡莱 (Nod-Krai) 角色补全计划

**任务编号**: TASK-2026-02-12-NODKRAI-CHARACTERS  
**优先级**: P2 (内容扩展)  
**依赖**: TASK-2026-02-12-MOONSIGN-SYSTEM  
**阻塞**: 无  

---

## 1. 背景

### 1.1 挪德卡莱地区角色

挪德卡莱 (Nod-Krai) 是原神 6.0 版本引入的新地区，其角色具有以下特点：
- 全部为 **月兆角色** (AVATAR_TAG_MOONPHASE)
- 与 **月反应系统** 深度绑定
- 具有独特的 **月兆等级** 机制

### 1.2 当前状态

GA 已实现 **7/9** 名挪德卡莱角色：
- ✅ Ineffa (伊内丝)
- ✅ Lauma (菈乌玛)
- ✅ Flins (菲林斯)
- ✅ Aino (艾诺)
- ✅ Nefer (妮芙)
- ✅ Jahoda (雅珂达)
- ✅ Columbina (哥伦比娅)
- ❌ **Zibai (兹白)** - 待实现
- ❌ **Illuga (伊卢加)** - 待实现

---

## 2. 缺失角色详情

### 2.1 Zibai (兹白)

**基础数据** (来自 AnimeGameData):

| 属性 | 数值 |
|------|------|
| ID | 10000126 |
| 英文名 | Zibai |
| 元素 | Geo |
| 武器 | Sword (单手剑) |
| 星级 | 5★ (推测) |
| 标签 | AVATAR_TAG_MOONPHASE |

**机制特点** (基于 TextMap 分析):
- 月结晶反应 (Lunar-Crystallize) 相关
- 具有 "Lunar Phase Shift" 模式切换
- 可以积累 "Phase Shift Radiance"

**TextMap 关键文本** (TextMapEN.json):
> "Summoning a shadow of her former powers, she switches to the Lunar Phase Shift mode. In this mode, Zibai's Normal Attacks and Charged Attacks will deal Geo DMG that cannot be overridden by other infusions, and she can accrue special Phase Shift Radiance through different methods."

> "While in the Lunar Phase Shift mode, when Zibai performs Normal Attacks, the fourth attack will deal an additional instance of Geo DMG, which is considered Lunar-Crystallize Reaction DMG."

#### 实现文件

1. **角色实现**: `mona_core/src/character/characters/geo/zibai.rs`
2. **目标函数**: `mona_core/src/target_functions/target_functions/geo/zibai_default.rs`
3. **导出**: `mona_core/src/character/characters/geo/mod.rs`

#### 预估工作量

- 基础角色数据: 2-3 小时
- 技能机制 (Lunar Phase Shift): 4-6 小时
- 月结晶反应集成: 3-4 小时
- 测试验证: 2-3 小时
- **总计**: 约 2-3 天

---

### 2.2 Illuga (伊卢加)

**基础数据**:

| 属性 | 数值 |
|------|------|
| ID | 10000127 |
| 英文名 | Illuga |
| 武器 | Polearm (长柄武器) |
| 标签 | AVATAR_TAG_MOONPHASE |
| 元素 | **待确认** |

**数据限制**:
- 当前 AnimeGameData 版本中信息较少
- 元素类型尚未确认
- 技能数据待补充

**建议**: 等待更多数据发布后再实现

---

## 3. 挪德卡莱角色通用机制

### 3.1 月反应支持

所有挪德卡莱角色都需要支持以下月反应：

| 反应 | 触发条件 | 角色示例 |
|------|----------|----------|
| **月感电** (Lunar-Charged) | Electro + Hydro + 月兆 | Flins |
| **月绽放** (Lunar-Bloom) | Dendro + Hydro + 月兆 | Lauma |
| **月结晶** (Lunar-Crystallize) | Geo + Hydro + 月兆 | Zibai |

### 3.2 月兆等级交互

每个挪德卡莱角色都需要实现：

```rust
pub struct NodKraiCharacterEffect {
    pub moonsign_level: MoonsignLevel,  // 从队伍获取
}

impl<A: Attribute> ChangeAttribute<A> for NodKraiCharacterEffect {
    fn change_attribute(&self, attribute: &mut A) {
        // 根据月兆等级应用不同加成
        match self.moonsign_level {
            MoonsignLevel::NascentGleam => {
                // 月兆·初辉效果
            }
            MoonsignLevel::AscendantGleam => {
                // 月兆·满辉效果
            }
            _ => {}
        }
    }
}
```

### 3.3 天赋命名参考

根据已有角色，挪德卡莱角色天赋命名模式：

| 天赋 | 命名模式 | 示例 |
|------|----------|------|
| A1 | 夜/光相关 | "Light for the Frosty Night" |
| A3 | 月兆祝福 | "Moonsign Benediction: ..." |
| A4 | 自然/净化相关 | "Cleansing for the Spring" |

---

## 4. 实现顺序建议

### Phase 1: Zibai 优先 (P2)

**原因**:
1. 元素已确认 (Geo)
2. 月结晶反应系统需要测试
3. 技能机制相对明确

**步骤**:
1. 研究 Zibai 完整技能数据
2. 实现基础角色框架
3. 实现 "Lunar Phase Shift" 模式切换
4. 集成月结晶反应
5. 编写目标函数
6. 测试验证

### Phase 2: Illuga 延后 (P3)

**原因**:
- 数据不完整
- 等待官方发布更多信息

---

## 5. 与现有工作的关系

```
月兆系统 (TASK-MOONSIGN-SYSTEM)
         │
         ├──► 菈乌玛修复 (PLAN-LAUMA) - 进行中
         │
         └──► 挪德卡莱角色补全 (本任务)
                  │
                  ├──► Zibai - 待开始
                  └──► Illuga - 待数据
```

---

## 6. 技术注意事项

### 6.1 元素枚举更新

确保 `Element` 枚举包含所有需要的元素类型：

```rust
pub enum Element {
    Pyro,
    Hydro,
    Electro,
    Cryo,
    Anemo,
    Geo,
    Dendro,
    Physical,
}
```

### 6.2 月反应伤害类型

需要确保伤害系统能正确处理月反应：

```rust
pub enum ReactionType {
    // ... 原有反应
    LunarCharged,
    LunarBloom,
    LunarCrystallize,
}
```

### 6.3 角色图标资源

需要准备的角色图标：
- `zibai_tn.png`
- `zibai_card.png`
- `zibai_splash.png`

---

## 7. 验证清单

### Zibai
- [ ] 基础属性正确 (HP/ATK/DEF)
- [ ] 技能倍率正确
- [ ] Lunar Phase Shift 模式切换正常
- [ ] 月结晶反应伤害计算正确
- [ ] 月兆等级加成正确应用
- [ ] 目标函数合理

### Illuga
- [ ] 等待元素确认
- [ ] 等待技能数据
- [ ] 实现基础框架

---

## 8. 参考文件

- AnimeGameData: `ExcelBinOutput/AvatarExcelConfigData.json`
- AnimeGameData: `TextMap/TextMapEN.json` (Zibai/Illuga 相关)
- GA实现: `mona_core/src/character/characters/dendro/lauma.rs` (参考)

---

**创建时间**: 2026-02-12  
**状态**: 规划中
