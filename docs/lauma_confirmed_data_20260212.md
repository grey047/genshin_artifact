# Lauma 与月绽放系统 - 已确认数据记录

**记录日期**: 2026-02-12  
**数据来源**: AnimeGameData (原神官方解包数据)  
**用途**: 避免重复查找，确保数据一致性

---

## 1. 月兆角色 (已确认完整列表)

| ID | 英文名 | 中文名 | 元素 | 武器 | 标签 |
|----|--------|--------|------|------|------|
| 10000116 | Ineffa | 伊内丝 | Electro | Polearm | AVATAR_TAG_MOONPHASE |
| 10000119 | Lauma | 菈乌玛 | Dendro | Catalyst | AVATAR_TAG_MOONPHASE |
| 10000120 | Flins | 菲林斯 | Electro | Polearm | AVATAR_TAG_MOONPHASE |
| 10000121 | Aino | 艾诺 | Hydro | Claymore | AVATAR_TAG_MOONPHASE |
| 10000122 | Nefer | 妮芙 | Dendro | Catalyst | AVATAR_TAG_MOONPHASE |
| 10000124 | Jahoda | 雅珂达 | Anemo | Bow | AVATAR_TAG_MOONPHASE |
| 10000125 | Columbina | 哥伦比娅 | Hydro | Catalyst | AVATAR_TAG_MOONPHASE |
| 10000126 | Zibai | 兹白 | Geo | Sword | AVATAR_TAG_MOONPHASE |
| 10000127 | Illuga | 伊卢加 | ? | Polearm | AVATAR_TAG_MOONPHASE |

**数据来源**: `AvatarExcelConfigData.json` 中的 `tags` 字段

---

## 2. 菈乌玛天赋数据 (已确认)

### A1: Light for the Frosty Night (ProudSkill Group 11921)

| 参数 | 数值 | 说明 |
|------|------|------|
| param[0] | 0.15 | 月兆·初辉: 固定暴击率15% |
| param[1] | 1 | ? |
| param[2] | 0.1 | 月兆·满辉: 暴击率+10% |
| param[3] | 0.2 | 月兆·满辉: 暴击伤害+20% |
| param[4] | 20 | 持续时间20秒 |

**效果**: 
- 使用E技能后20秒内，根据月兆等级提供不同加成
- **月兆·初辉**: Bloom/Hyperbloom/Burgeon 可暴击，固定15%暴击率/100%暴击伤害
- **月兆·满辉**: 全队月绽放暴击率+10%，暴击伤害+20%

**数据来源**: `ProudSkillExcelConfigData.json` (Group 11921)

### A3: Moonsign Benediction: Nature's Chorus (Group 11923)

| 参数 | 数值 | 说明 |
|------|------|------|
| param[0] | 0.000175 | 精通转化率 **0.0175%** |
| param[1] | 0.14 | 上限 **14%** |

**效果**: 
- 队伍成员触发绽放转为月绽放
- 每点精通使月绽放基础伤害+0.0175%，上限14%
- 额外: Lauma在队伍中时，队伍月兆等级+1

**数据来源**: `ProudSkillExcelConfigData.json` (Group 11923)

### A4: Cleansing for the Spring (Group 11922)

| 参数 | 数值 | 说明 |
|------|------|------|
| param[0] | 0.0004 | E技能伤害加成 0.04%/精通 |
| param[1] | 0.32 | E技能加成上限 32% |
| param[2] | 0.0002 | 重击CD减少 0.02%/精通 |
| param[3] | 0.2 | CD减少上限 20% |

**数据来源**: `ProudSkillExcelConfigData.json` (Group 11922)

### C6: I Offer Blood and Tears to the Moonlight (Talent 1196)

| 参数 | 数值 | 说明 |
|------|------|------|
| param[0] | 1.85 | **精通×185%** 额外伤害 |
| param[1] | 2 | 获得2层 Pale Hymn |
| param[2] | 15 | 持续时间15秒 |
| param[3] | 8 | 每次召唤最多8次触发 |
| param[4] | 1.5 | 普攻转化倍率 精通×150% |
| param[5] | 0.25 | **月兆·满辉时月绽放伤害 elevated +25%** |

**效果**:
1. E技能召唤物攻击时，额外造成精通×185%的AoE草伤（视为月绽放伤害）
2. 有Pale Hymn时普攻，消耗1层转化为精通×150%草伤（视为月绽放）
3. **月兆·满辉**: 全队月绽放伤害 **elevated** (擢升) **25%**

**数据来源**: `AvatarTalentExcelConfigData.json` (Talent ID 1196)

---

## 3. 月绽放反应倍率 (待确认)

**当前状态**: ❌ 未在 AnimeGameData 中找到直接的倍率定义

**已知信息**:
- GA当前代码: `lunar_bloom_multiplier() = 3.0` (`reaction.rs:33`)
- 计划假设: 2.0 (同普通绽放)
- 需要进一步确认

**下一步**: 查找 `ReactionExcelConfigData` 或类似文件中的定义

---

## 4. 武器状态 (已确认)

### Lauma 专武 "Fangye Tianjing" (访业天竞)

**状态**: ❌ **未实现**

**检查目录**: `mona_core/src/weapon/weapons/catalysts/`
- 文件 `fangye_tianjing.rs` **不存在**
- 需要新建

**武器命名规范** (已确认):
- 使用英文小写+下划线
- 示例: `a_thousand_floating_dreams.rs`, `cashflow_supervision.rs`
- 所以应为: `fangye_tianjing.rs` ✅

---

## 5. 关键问题澄清

### Q1: EnhanceLunarBloom vs LunarBloom vs 直伤月绽放的区别

| 类型 | 定义 | 来源 |
|------|------|------|
| **EnhanceLunarBloom** | 属性枚举名，用于加成月绽放反应伤害 | GA代码规范 |
| **LunarBloom** | 反应类型，Dendro + Hydro + 月兆触发的反应 | 游戏机制 |
| **直伤月绽放** | Lauma特有的直接伤害类型，基于精通而非等级系数 | C6命座效果 |

**区别详解**:

1. **EnhanceLunarBloom** (属性)
   - 类似 `EnhanceBloom`, `EnhanceBurgeon`
   - 用于 `AttributeName` 枚举
   - 加成所有月绽放反应伤害

2. **普通月绽放** (反应)
   - 由 Dendro + Hydro + 月兆触发
   - 基于等级系数 × 精通加成
   - 可暴击 (通过A1天赋)

3. **直伤月绽放** (Lauma特有)
   - C6命座: 精通×185%，不消耗Pale Hymn
   - E2技能: 长按第二段直接造成月绽放伤害
   - **关键区别**: 基于精通而非等级系数

### Q2: "elevated" (擢升) 是什么？

**来源**: C6命座描述 "Lunar-Bloom DMG is elevated by 25%"

**理解**: 
- 是独立乘区，区别于普通增伤 (Bonus)
- 类似妮露的 "Transcendent" 加成
- 在GA中可能需要新建属性: `ElevateLunarBloom`

---

## 6. 待确认项清单

- [ ] 月绽放反应基础倍率 (2.0 vs 3.0)
- [ ] LEVEL_MULTIPLIER 数组定义位置
- [ ] Lauma 专武的具体数值

---

**更新记录**:
- 2026-02-12: 创建文件，整合 AnimeGameData 研究结果
