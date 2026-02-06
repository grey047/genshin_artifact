use num_derive::FromPrimitive;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::macros::{damage_enum, damage_ratio, skill_map, skill_type};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::common::i18n::{locale, hit_n_dmg, plunging_dmg, charged_dmg};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use strum_macros::{EnumString, EnumCount as EnumCountMacro};

// ============================================================================
// Skill Data - from HHW (Lv1-15)
// ============================================================================

pub struct SkirkSkillType {
    // Normal Attack (普通状态)
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3_1: [f64; 15],
    pub normal_dmg3_2: [f64; 15],
    pub normal_dmg4_1: [f64; 15],
    pub normal_dmg4_2: [f64; 15],
    pub normal_dmg5: [f64; 15],
    pub normal_charged_dmg: [f64; 15],
    pub normal_charged_stamina: [f64; 15],
    pub normal_plunging_dmg1: [f64; 15],
    pub normal_plunging_dmg2: [f64; 15],
    pub normal_plunging_dmg3: [f64; 15],

    // Seven-Phase Flash Mode (七相一闪模式)
    // 普通攻击在七相模式下使用这些倍率
    pub seven_phase_dmg1: [f64; 15],
    pub seven_phase_dmg2: [f64; 15],
    pub seven_phase_dmg3_1: [f64; 15],
    pub seven_phase_dmg3_2: [f64; 15],
    pub seven_phase_dmg4_1: [f64; 15],
    pub seven_phase_dmg4_2: [f64; 15],
    pub seven_phase_dmg5: [f64; 15],
    pub seven_phase_charged_dmg: [f64; 15],  // 3段
    pub seven_phase_charged_stamina: [f64; 15],
    pub seven_phase_plunging_dmg1: [f64; 15],
    pub seven_phase_plunging_dmg2: [f64; 15],
    pub seven_phase_plunging_dmg3: [f64; 15],

    // Elemental Skill (极恶技·闪 / Havoc: Warp)
    // 七相模式下倍率更高
    pub e_dmg1: [f64; 15],  // 一段
    pub e_dmg2: [f64; 15],  // 二段
    pub e_dmg3_1: [f64; 15], // 三段 (1)
    pub e_dmg3_2: [f64; 15], // 三段 (2)
    pub e_dmg4_1: [f64; 15], // 四段 (1)
    pub e_dmg4_2: [f64; 15], // 四段 (2)
    pub e_dmg5: [f64; 15],  // 五段
    pub e_charged_dmg: [f64; 15], // 重击 (3段)
    pub e_charged_stamina: [f64; 15],
    pub e_duration: [f64; 15],
    pub e_serpent_max: [f64; 15],
    pub e_cd: [f64; 15],

    // Elemental Burst (极恶技·灭 / Havoc: Ruin)
    // 消耗 Serpent's Subtlety
    pub q_slash: [f64; 15],  // 斩击伤害 (5次)
    pub q_final: [f64; 15],  // 最终段伤害
    pub q_serpent_bonus: [f64; 15],  // 每点 SS 提供的 ATK%
    pub q_void_bonus: [f64; 15],  // 虚境裂隙加成
    pub q_cd: [f64; 15],

    // Passive: Death's Crossing (死河渡断)
    // A4: 层数对普通攻击和大招的加成
    pub passive_death_normal: [f64; 3],  // [1.10, 1.20, 1.70] - 层数1-3
    pub passive_death_burst: [f64; 3],   // [1.05, 1.15, 1.60] - 层数1-3

    // Constellation Effects
    pub c1_dmg: [f64; 15],  // 水晶刃伤害
    pub c2_atk: f64,        // C2 ATK加成 (70%)
    pub c4_atk: [f64; 3],   // C4 ATK加成 (10%, 20%, 40%)
    pub c6_sever: [f64; 15], // C6 Sever 协同攻击
}

pub const SKIRK_SKILL: SkirkSkillType = SkirkSkillType {
    // Normal Attack (普通状态) - from HHW
    normal_dmg1: [0.5452, 0.5896, 0.6340, 0.6974, 0.7418, 0.7925, 0.8622, 0.9320, 1.0017, 1.0778, 1.1539, 1.2300, 1.3060, 1.3821, 1.4582],
    normal_dmg2: [0.4979, 0.5385, 0.5790, 0.6369, 0.6774, 0.7238, 0.7874, 0.8511, 0.9148, 0.9843, 1.0538, 1.1233, 1.1927, 1.2622, 1.3317],
    normal_dmg3_1: [0.3242, 0.3506, 0.3770, 0.4147, 0.4411, 0.4713, 0.5127, 0.5542, 0.5957, 0.6409, 0.6861, 0.7314, 0.7766, 0.8219, 0.8671],
    normal_dmg3_2: [0.3242, 0.3506, 0.3770, 0.4147, 0.4411, 0.4713, 0.5127, 0.5542, 0.5957, 0.6409, 0.6861, 0.7314, 0.7766, 0.8219, 0.8671],
    normal_dmg4_1: [0.6080, 0.6575, 0.7070, 0.7777, 0.8272, 0.8838, 0.9615, 1.0393, 1.1171, 1.2019, 1.2867, 1.3716, 1.4564, 1.5413, 1.6261],
    normal_dmg4_2: [0.6080, 0.6575, 0.7070, 0.7777, 0.8272, 0.8838, 0.9615, 1.0393, 1.1171, 1.2019, 1.2867, 1.3716, 1.4564, 1.5413, 1.6261],
    normal_dmg5: [0.8290, 0.8965, 0.9640, 1.0604, 1.1279, 1.2050, 1.3110, 1.4171, 1.5231, 1.6388, 1.7545, 1.8702, 1.9858, 2.1015, 2.2172],
    normal_charged_dmg: [0.6682, 0.7226, 0.7770, 0.8547, 0.9091, 0.9713, 1.0567, 1.1422, 1.2277, 1.3209, 1.4141, 1.5074, 1.6006, 1.6939, 1.7871],
    normal_charged_stamina: [20.0; 15],
    normal_plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530, 1.4422, 1.5314, 1.6206, 1.7098],
    normal_plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    normal_plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.6020, 3.8248, 4.0476, 4.2704],

    // Seven-Phase Flash Mode (七相一闪模式) - from HHW
    seven_phase_dmg1: [1.3282, 1.4364, 1.5445, 1.6989, 1.8071, 1.9306, 2.1005, 2.2704, 2.4403, 2.6256, 2.8109, 2.9963, 3.1816, 3.3669, 3.5523],
    seven_phase_dmg2: [1.1980, 1.2955, 1.3930, 1.5323, 1.6298, 1.7413, 1.8945, 2.0477, 2.2010, 2.3681, 2.5353, 2.7025, 2.8696, 3.0368, 3.2039],
    seven_phase_dmg3_1: [0.7572, 0.8189, 0.8805, 0.9686, 1.0302, 1.1006, 1.1975, 1.2943, 1.3912, 1.4969, 1.6025, 1.7082, 1.8138, 1.9195, 2.0252],
    seven_phase_dmg3_2: [0.7572, 0.8189, 0.8805, 0.9686, 1.0302, 1.1006, 1.1975, 1.2943, 1.3912, 1.4969, 1.6025, 1.7082, 1.8138, 1.9195, 2.0252],
    seven_phase_dmg4_1: [0.8054, 0.8709, 0.9365, 1.0302, 1.0957, 1.1706, 1.2736, 1.3767, 1.4797, 1.5921, 1.7044, 1.8168, 1.9292, 2.0416, 2.1540],
    seven_phase_dmg4_2: [0.8054, 0.8709, 0.9365, 1.0302, 1.0957, 1.1706, 1.2736, 1.3767, 1.4797, 1.5921, 1.7044, 1.8168, 1.9292, 2.0416, 2.1540],
    seven_phase_dmg5: [1.9662, 2.1263, 2.2863, 2.5150, 2.6750, 2.8579, 3.1094, 3.3609, 3.6124, 3.8868, 4.1611, 4.4355, 4.7098, 4.9842, 5.2586],
    seven_phase_charged_dmg: [0.4455, 0.4817, 0.5180, 0.5698, 0.6061, 0.6475, 0.7045, 0.7615, 0.8184, 0.8806, 0.9428, 1.0049, 1.0671, 1.1292, 1.1914],
    seven_phase_charged_stamina: [20.0; 15],
    seven_phase_plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530, 1.4422, 1.5314, 1.6206, 1.7098],
    seven_phase_plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    seven_phase_plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.6020, 3.8248, 4.0476, 4.2704],

    // Elemental Skill
    // TODO: 验证 E skill 数据 - +10/+9 per level 的线性增长看起来像占位符
    e_dmg1: [150.00, 160.00, 170.00, 180.00, 190.00, 200.00, 210.00, 220.00, 230.00, 240.00, 250.00, 260.00, 270.00, 280.00, 290.00],  // +10/level
    e_dmg2: [135.00, 144.00, 153.00, 162.00, 171.00, 180.00, 189.00, 198.00, 207.00, 216.00, 225.00, 234.00, 243.00, 252.00, 261.00],  // +9/level
    e_dmg3_1: [85.00, 91.00, 97.00, 103.00, 109.00, 115.00, 121.00, 127.00, 133.00, 139.00, 145.00, 151.00, 157.00, 163.00, 169.00],
    e_dmg3_2: [85.00, 91.00, 97.00, 103.00, 109.00, 115.00, 121.00, 127.00, 133.00, 139.00, 145.00, 151.00, 157.00, 163.00, 169.00],
    e_dmg4_1: [90.00, 96.00, 102.00, 108.00, 114.00, 120.00, 126.00, 132.00, 138.00, 144.00, 150.00, 156.00, 162.00, 168.00, 174.00],
    e_dmg4_2: [90.00, 96.00, 102.00, 108.00, 114.00, 120.00, 126.00, 132.00, 138.00, 144.00, 150.00, 156.00, 162.00, 168.00, 174.00],
    e_dmg5: [220.00, 236.00, 252.00, 268.00, 284.00, 300.00, 316.00, 332.00, 348.00, 364.00, 380.00, 396.00, 412.00, 428.00, 444.00],
    e_charged_dmg: [50.00, 54.00, 58.00, 62.00, 66.00, 70.00, 74.00, 78.00, 82.00, 86.00, 90.00, 94.00, 98.00, 102.00, 106.00],
    e_charged_stamina: [20.0; 15],
    e_duration: [12.5; 15],
    e_serpent_max: [100.0; 15],
    e_cd: [8.0; 15],

    // Elemental Burst
    q_slash: [122.76, 131.97, 141.17, 153.45, 162.66, 171.86, 184.14, 196.42, 208.69, 220.97, 233.24, 245.52, 260.87, 276.21, 291.56],
    q_final: [204.60, 219.95, 235.29, 255.75, 271.10, 286.44, 306.90, 327.36, 347.82, 368.28, 388.74, 409.20, 434.78, 460.35, 485.93],
    q_serpent_bonus: [0.1932, 0.2077, 0.2222, 0.2415, 0.2560, 0.2705, 0.2899, 0.3092, 0.3285, 0.3478, 0.3671, 0.3865, 0.4106, 0.4348, 0.4589],
    q_void_bonus: [0.035, 0.040, 0.045, 0.050, 0.055, 0.060, 0.065, 0.070, 0.075, 0.080, 0.085, 0.090, 0.095, 0.100, 0.105],
    q_cd: [15.0; 15],

    // Passive: Death's Crossing (A4)
    // 效果: 普通攻击 110%/120%/170%, 大招 105%/115%/160%
    passive_death_normal: [1.10, 1.20, 1.70],  // index 0-2 = 层数1-3
    passive_death_burst: [1.05, 1.15, 1.60],

    // Constellations
    c1_dmg: [500.00; 15],  // 水晶刃 500%
    c2_atk: 0.70,          // +70% ATK
    c4_atk: [0.10, 0.20, 0.40],  // 10%/20%/40% ATK
    c6_sever: [180.00; 15], // Sever 协同攻击 180%
};

// ============================================================================
// Damage Enum
// ============================================================================

#[derive(Copy, Clone, FromPrimitive, Eq, PartialEq)]
#[derive(EnumString, EnumCountMacro)]
pub enum SkirkDamageEnum {
    // Normal Attack (七相模式)
    Normal1,
    Normal2,
    Normal3_1,
    Normal3_2,
    Normal4_1,
    Normal4_2,
    Normal5,
    Charged,
    Plunging1,
    Plunging2,
    Plunging3,

    // Elemental Skill (极恶技·闪)
    E1,        // 点按 - 一段
    E2,        // 点按 - 二段
    E3_1,      // 点按 - 三段 (1)
    E3_2,      // 点按 - 三段 (2)
    E4_1,      // 点按 - 四段 (1)
    E4_2,      // 点按 - 四段 (2)
    E5,        // 点按 - 五段
    ECharged,  // 重击 (3段)

    // Elemental Burst (极恶技·灭 / 极恶技·尽)
    Q1,        // 斩击 (5次)
    Q2,        // 最终段
    QC1,       // C1 水晶刃

    // C6 Sever (协同攻击)
    C6Sever,
}

impl SkirkDamageEnum {
    pub fn get_element(&self) -> Element {
        Element::Cryo
    }

    pub fn get_skill_type(&self) -> SkillType {
        use SkirkDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3_1 | Normal3_2 | Normal4_1 | Normal4_2 | Normal5 => SkillType::NormalAttack,
            Charged | ECharged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 | E3_1 | E3_2 | E4_1 | E4_2 | E5 => SkillType::ElementalSkill,
            Q1 | Q2 | QC1 | C6Sever => SkillType::ElementalBurst,
        }
    }
}

impl Into<usize> for SkirkDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

// ============================================================================
// Effect Implementation
// ============================================================================

pub struct SkirkEffect {
    pub constellation: usize,
    pub ascend: bool,
    pub death_stacks: usize,      // 死河渡断层数 (0-3)，C4 也复用这个
    pub in_seven_phase: bool,     // 是否在七相一闪模式下
    pub serpent_points: f64,      // 蛇之狡谋点数
    pub c2_active: bool,          // C2 是否激活
    pub c2_atk_value: f64,        // C2 ATK加成值 (0.70)
    pub c4_atk_values: [f64; 3], // C4 ATK加成值 [0.10, 0.20, 0.40]
    pub void_realm_active: bool,  // 虚境裂隙是否生效
}

impl<A: Attribute> ChangeAttribute<A> for SkirkEffect {
    fn change_attribute(&self, attribute: &mut A) {
        // C2: 大招后 ATK +70% (持续18秒)
        if self.constellation >= 2 && self.c2_active {
            attribute.add_atk_percentage("命座「湮远」", self.c2_atk_value);
        }

        // C4: 冰水队 ATK+ (10%/20%/40% @ 1/2/3层)，复用 death_stacks
        if self.constellation >= 4 && self.death_stacks > 0 {
            let c4_idx = self.death_stacks.saturating_sub(1).min(2);
            let c4_bonus = self.c4_atk_values[c4_idx];
            attribute.add_atk_percentage("命座「万流归寂」", c4_bonus);
        }
    }
}

// ============================================================================
// Character Implementation
// ============================================================================

pub struct Skirk;

impl CharacterTrait for Skirk {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Skirk,
        internal_name: "Skirk",
        name_locale: locale!(
            zh_cn: "丝柯克",
            en: "Skirk",
        ),
        element: Element::Cryo,
        // All level stats verified from HHW (Lv100 data added)
        hp: [967, 2508, 3336, 4992, 5581, 6421, 7206, 8055, 8644, 9501, 10089, 10956, 11544, 12417, 13300],
        atk: [27, 72, 96, 144, 161, 185, 208, 232, 249, 274, 291, 316, 333, 358, 439],
        def: [62, 162, 216, 324, 362, 416, 467, 522, 561, 616, 655, 711, 749, 806, 864],
    sub_stat: CharacterSubStatFamily::CriticalDamage384,
        weapon_type: WeaponType::Sword,
        star: 5,
        skill_name1: locale!(
            zh_cn: "极恶技·断",
            en: "Havoc: Sunder",
        ),
        skill_name2: locale!(
            zh_cn: "极恶技·闪",
            en: "Havoc: Warp",
        ),
        skill_name3: locale!(
            zh_cn: "极恶技·灭",
            en: "Havoc: Annihilation",
        ),
    };

    type SkillType = SkirkSkillType;
    const SKILL: Self::SkillType = SKIRK_SKILL;
    type DamageEnumType = SkirkDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            // Normal Attack (普通状态)
            CharacterSkillMapItem { index: SkirkDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: SkirkDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: SkirkDamageEnum::Normal3_1 as usize, text: hit_n_dmg!(3, 1) },
            CharacterSkillMapItem { index: SkirkDamageEnum::Normal3_2 as usize, text: hit_n_dmg!(3, 2) },
            CharacterSkillMapItem { index: SkirkDamageEnum::Normal4_1 as usize, text: hit_n_dmg!(4, 1) },
            CharacterSkillMapItem { index: SkirkDamageEnum::Normal4_2 as usize, text: hit_n_dmg!(4, 2) },
            CharacterSkillMapItem { index: SkirkDamageEnum::Normal5 as usize, text: hit_n_dmg!(5) },
            CharacterSkillMapItem { index: SkirkDamageEnum::Charged as usize, text: locale!(zh_cn: "重击·三段攻击", en: "Charged: 3-Hit Combo") },
            CharacterSkillMapItem { index: SkirkDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: SkirkDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: SkirkDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            // Elemental Skill (极恶技·闪)
            CharacterSkillMapItem { index: SkirkDamageEnum::E1 as usize, text: locale!(zh_cn: "一段伤害", en: "1st Hit DMG") },
            CharacterSkillMapItem { index: SkirkDamageEnum::E2 as usize, text: locale!(zh_cn: "二段伤害", en: "2nd Hit DMG") },
            CharacterSkillMapItem { index: SkirkDamageEnum::E3_1 as usize, text: locale!(zh_cn: "三段伤害(1)", en: "3rd Hit DMG (1)") },
            CharacterSkillMapItem { index: SkirkDamageEnum::E3_2 as usize, text: locale!(zh_cn: "三段伤害(2)", en: "3rd Hit DMG (2)") },
            CharacterSkillMapItem { index: SkirkDamageEnum::E4_1 as usize, text: locale!(zh_cn: "四段伤害(1)", en: "4th Hit DMG (1)") },
            CharacterSkillMapItem { index: SkirkDamageEnum::E4_2 as usize, text: locale!(zh_cn: "四段伤害(2)", en: "4th Hit DMG (2)") },
            CharacterSkillMapItem { index: SkirkDamageEnum::E5 as usize, text: locale!(zh_cn: "五段伤害", en: "5th Hit DMG") },
            CharacterSkillMapItem { index: SkirkDamageEnum::ECharged as usize, text: locale!(zh_cn: "重击伤害(3段)", en: "Charged DMG (3 hits)") },
        ]),
        skill3: Some(&[
            // Elemental Burst (极恶技·灭)
            CharacterSkillMapItem { index: SkirkDamageEnum::Q1 as usize, text: locale!(zh_cn: "斩击伤害", en: "Slash DMG") },
            CharacterSkillMapItem { index: SkirkDamageEnum::Q2 as usize, text: locale!(zh_cn: "最终段伤害", en: "Final Hit DMG") },
        ])
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        // 七相模式配置
        ItemConfig {
            name: "in_seven_phase",
            title: locale!(
                zh_cn: "七相一闪模式",
                en: "Seven-Phase Flash Mode",
            ),
            config: ItemConfigType::Bool { default: true }
        },
        // 死河渡断层数 (A4)
        ItemConfig {
            name: "death_stacks",
            title: locale!(
                zh_cn: "死河渡断层数",
                en: "Death's Crossing Stacks",
            ),
            config: ItemConfigType::Int { min: 0, max: 3, default: 0 }
        },
        // 蛇之狡谋点数
        ItemConfig {
            name: "serpent_points",
            title: locale!(
                zh_cn: "蛇之狡谋点数",
                en: "Serpent's Subtlety Points",
            ),
            config: ItemConfigType::Int { min: 0, max: 100, default: 50 }
        },
        // C2 激活状态
        ItemConfig {
            name: "c2_active",
            title: locale!(
                zh_cn: "C2: 施放后ATK+",
                en: "C2: Post-Burst ATK+",
            ),
            config: ItemConfigType::Bool { default: false }
        },
        // 虚境裂隙生效 (A1/C6 效果)
        ItemConfig {
            name: "void_realm_active",
            title: locale!(
                zh_cn: "虚境裂隙生效",
                en: "Void Realm Active",
            ),
            config: ItemConfigType::Bool { default: false }
        },
        // 注意: C4 层数复用 A4 的 death_stacks
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, _fumo: Option<Element>) -> D::Result {
        let s: SkirkDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();
        let constellation = context.character_common_data.constellation as usize;
        let ascend = context.character_common_data.ascend;

        // 解析配置 (C4 复用 death_stacks)
        let (in_seven_phase, death_stacks, serpent_points, c2_active, void_realm_active) = match *config {
            CharacterSkillConfig::Skirk { in_seven_phase, death_stacks, serpent_points, c2_active, void_realm_active } =>
                (in_seven_phase, death_stacks, serpent_points, c2_active, void_realm_active),
            _ => (true, 0, 50.0, false, false)
        };

        // 获取倍率
        let mut ratio: f64 = 0.0;
        let mut bonus: f64 = 0.0;  // 额外伤害加成

        use SkirkDamageEnum::*;

        // 根据是否在七相模式选择倍率
        let (norm_dmg1, norm_dmg2, norm_dmg3_1, norm_dmg3_2, norm_dmg4_1, norm_dmg4_2, norm_dmg5, norm_charged, norm_plunge1, norm_plunge2, norm_plunge3) = if in_seven_phase {
            (
                SKIRK_SKILL.seven_phase_dmg1,
                SKIRK_SKILL.seven_phase_dmg2,
                SKIRK_SKILL.seven_phase_dmg3_1,
                SKIRK_SKILL.seven_phase_dmg3_2,
                SKIRK_SKILL.seven_phase_dmg4_1,
                SKIRK_SKILL.seven_phase_dmg4_2,
                SKIRK_SKILL.seven_phase_dmg5,
                SKIRK_SKILL.seven_phase_charged_dmg,
                SKIRK_SKILL.seven_phase_plunging_dmg1,
                SKIRK_SKILL.seven_phase_plunging_dmg2,
                SKIRK_SKILL.seven_phase_plunging_dmg3,
            )
        } else {
            (
                SKIRK_SKILL.normal_dmg1,
                SKIRK_SKILL.normal_dmg2,
                SKIRK_SKILL.normal_dmg3_1,
                SKIRK_SKILL.normal_dmg3_2,
                SKIRK_SKILL.normal_dmg4_1,
                SKIRK_SKILL.normal_dmg4_2,
                SKIRK_SKILL.normal_dmg5,
                SKIRK_SKILL.normal_charged_dmg,
                SKIRK_SKILL.normal_plunging_dmg1,
                SKIRK_SKILL.normal_plunging_dmg2,
                SKIRK_SKILL.normal_plunging_dmg3,
            )
        };

        match s {
            // Normal Attack (根据模式选择倍率)
            Normal1 => ratio = norm_dmg1[s1],
            Normal2 => ratio = norm_dmg2[s1],
            Normal3_1 => ratio = norm_dmg3_1[s1],
            Normal3_2 => ratio = norm_dmg3_2[s1],
            Normal4_1 => ratio = norm_dmg4_1[s1],
            Normal4_2 => ratio = norm_dmg4_2[s1],
            Normal5 => ratio = norm_dmg5[s1],
            Charged => {
                ratio = norm_charged[s1];
                bonus = (norm_charged[s1] * 2.0) * 2.0;  // 3段伤害
            },
            Plunging1 => ratio = norm_plunge1[s1],
            Plunging2 => ratio = norm_plunge2[s1],
            Plunging3 => ratio = norm_plunge3[s1],

            // Elemental Skill
            E1 => ratio = SKIRK_SKILL.e_dmg1[s2],
            E2 => ratio = SKIRK_SKILL.e_dmg2[s2],
            E3_1 => ratio = SKIRK_SKILL.e_dmg3_1[s2],
            E3_2 => ratio = SKIRK_SKILL.e_dmg3_2[s2],
            E4_1 => ratio = SKIRK_SKILL.e_dmg4_1[s2],
            E4_2 => ratio = SKIRK_SKILL.e_dmg4_2[s2],
            E5 => ratio = SKIRK_SKILL.e_dmg5[s2],
            ECharged => {
                // GO 数据 skill[7] 已经是 ×3 (0.44548 = 基础值 × 3)
                // 不需要额外乘算
                ratio = SKIRK_SKILL.e_charged_dmg[s2];
            },

            // Elemental Burst (极恶技·灭)
            Q1 => {
                ratio = SKIRK_SKILL.q_slash[s3];
                // Serpent bonus: 超过50点的部分每点增加伤害
                let extra_ss = (serpent_points - 50.0).max(0.0);
                let max_ss_bonus = if constellation >= 2 { 22.0 } else { 12.0 };
                let effective_ss = extra_ss.min(max_ss_bonus);
                // GO 版本: bonus × serpentBonus × a4_burst_mult
                let a4_idx = death_stacks.saturating_sub(1).min(2);
                let a4_burst_mult = if death_stacks == 0 { 1.0 } else { SKIRK_SKILL.passive_death_burst[a4_idx] };
                bonus = effective_ss * SKIRK_SKILL.q_serpent_bonus[s3] * a4_burst_mult;
            },
            Q2 => {
                ratio = SKIRK_SKILL.q_final[s3];
                let extra_ss = (serpent_points - 50.0).max(0.0);
                let max_ss_bonus = if constellation >= 2 { 22.0 } else { 12.0 };
                let effective_ss = extra_ss.min(max_ss_bonus);
                let a4_idx = death_stacks.saturating_sub(1).min(2);
                let a4_burst_mult = if death_stacks == 0 { 1.0 } else { SKIRK_SKILL.passive_death_burst[a4_idx] };
                bonus = effective_ss * SKIRK_SKILL.q_serpent_bonus[s3] * a4_burst_mult;
            },

            // C1: 水晶刃
            QC1 => ratio = SKIRK_SKILL.c1_dmg[s3],

            // C6 Sever
            C6Sever => ratio = SKIRK_SKILL.c6_sever[s3],
        }

        let mut builder = D::new();

        // 添加基础倍率
        builder.add_atk_ratio("技能倍率", ratio);

        // 添加 Serpent bonus (Burst)
        if matches!(s, Q1 | Q2) && bonus > 0.0 {
            builder.add_atk_ratio("蛇之狡谋加成", bonus);
        }

        // A4: 死河渡断加成
        if ascend {
            let death_idx = death_stacks.saturating_sub(1).min(2);  // 0->0(未解锁用1.0), 1->0, 2->1, 3->2
            let death_multiplier = match s {
                Normal1 | Normal2 | Normal3_1 | Normal3_2 | Normal4_1 | Normal4_2 | Normal5 | Charged | ECharged =>
                    if death_stacks == 0 { 1.0 } else { SKIRK_SKILL.passive_death_normal[death_idx] },
                Q1 | Q2 =>
                    if death_stacks == 0 { 1.0 } else { SKIRK_SKILL.passive_death_burst[death_idx] },
                _ => 1.0
            };
            if death_multiplier > 1.0 {
                builder.add_atk_ratio("死河渡断", death_multiplier - 1.0);
            }
        }

        // C4: ATK+ (冰水队) - GO 版本复用 A4 层数
        if constellation >= 4 && death_stacks > 0 {
            // C4: 10%/20%/40% ATK (对应 A4 1/2/3 层)
            let c4_idx = death_stacks.saturating_sub(1).min(2);
            let c4_bonus = SKIRK_SKILL.c4_atk[c4_idx];
            builder.add_atk_ratio("命座「万流归寂」", c4_bonus);
        }

        // A1/C6: 虚境裂隙加成 (Q技能对处于虚境裂隙中的敌人造成的伤害提升)
        if void_realm_active && matches!(s, Q1 | Q2 | QC1 | C6Sever) {
            builder.add_atk_ratio("虚境裂隙", SKIRK_SKILL.q_void_bonus[s3]);
        }

        // C2: ATK+ (已在 ChangeAttribute 中处理)

        builder.damage(
            &context.attribute,
            &context.enemy,
            Element::Cryo,
            s.get_skill_type(),
            context.character_common_data.level,
            None,
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        let constellation = common_data.constellation as usize;
        let ascend = common_data.ascend;

        let (in_seven_phase, death_stacks, serpent_points, c2_active, void_realm_active) = match *config {
            CharacterConfig::Skirk { in_seven_phase, death_stacks, serpent_points, c2_active, void_realm_active } =>
                (in_seven_phase, death_stacks, serpent_points, c2_active, void_realm_active),
            _ => (true, 0, 50.0, false, false)
        };

        let c2_atk_value = if constellation >= 2 { SKIRK_SKILL.c2_atk } else { 0.0 };
        let c4_atk_values = SKIRK_SKILL.c4_atk;

        Some(Box::new(SkirkEffect {
            constellation,
            ascend,
            death_stacks,
            in_seven_phase,
            serpent_points,
            c2_active,
            c2_atk_value,
            c4_atk_values,
            void_realm_active,
        }))
    }

    fn get_target_function_by_role(_role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
