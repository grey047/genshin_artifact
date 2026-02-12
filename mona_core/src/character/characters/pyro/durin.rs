use num_derive::FromPrimitive;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};

use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::pyro::DurinDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

type SKILL = [f64; 15];

pub struct DurinSkillType {
    pub normal_dmg1: SKILL,
    pub normal_dmg2: SKILL,
    pub normal_dmg3: SKILL,
    pub normal_dmg4: SKILL,
    pub charged_dmg1: SKILL,
    pub charged_dmg2: SKILL,
    pub plunging_dmg1: SKILL,
    pub plunging_dmg2: SKILL,
    pub plunging_dmg3: SKILL,

    pub skill_purity_dmg: SKILL,
    pub skill_darkness_dmg: SKILL,

    pub burst_initial1: SKILL,
    pub burst_initial2: SKILL,
    pub burst_initial3: SKILL,
    pub burst_dragon_white: SKILL,
    pub burst_dragon_dark: SKILL,
}

pub const DURIN_SKILL: DurinSkillType = DurinSkillType {
    normal_dmg1: [
        0.7955, 0.8596, 0.9237, 1.0161, 1.0802, 1.1537, 1.2552, 1.3568, 1.4583, 1.5684, 1.6785,
        1.7886, 1.8987, 2.0088, 2.1189,
    ],
    normal_dmg2: [
        0.6226, 0.6731, 0.7236, 0.7959, 0.8464, 0.9042, 0.9838, 1.0634, 1.143, 1.2296, 1.3162,
        1.4028, 1.4893, 1.5759, 1.6625,
    ],
    normal_dmg3: [
        0.9942, 1.0745, 1.1548, 1.2703, 1.3506, 1.4427, 1.5695, 1.6964, 1.8232, 1.9609, 2.0986,
        2.2363, 2.374, 2.5117, 2.6494,
    ],
    normal_dmg4: [
        1.2817, 1.3854, 1.4891, 1.638, 1.7417, 1.8603, 2.0243, 2.1883, 2.3523, 2.5302, 2.7081,
        2.886, 3.0639, 3.2418, 3.4197,
    ],
    charged_dmg1: [
        1.6378, 1.7707, 1.9035, 2.0939, 2.2267, 2.3786, 2.588, 2.7974, 3.0068, 3.2349, 3.463,
        3.6911, 3.9192, 4.1473, 4.3754,
    ],
    charged_dmg2: [
        0.9719, 1.0507, 1.1294, 1.2423, 1.3211, 1.4114, 1.5358, 1.6602, 1.7846, 1.9193, 2.054,
        2.1887, 2.3234, 2.4581, 2.5928,
    ],
    plunging_dmg1: [
        0.7459, 0.8066, 0.8673, 0.954, 1.0147, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5785,
        1.6826, 1.7866, 1.8907, 1.9948,
    ],
    plunging_dmg2: [
        1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563,
        3.3644, 3.5725, 3.7806, 3.9887,
    ],
    plunging_dmg3: [
        1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.946, 3.1842, 3.4225, 3.6825, 3.9424,
        4.2023, 4.4623, 4.7222, 4.9821,
    ],

    skill_purity_dmg: [
        0.301, 0.3241, 0.3472, 0.379, 0.4021, 0.4252, 0.457, 0.4888, 0.5206, 0.5524, 0.5842, 0.616,
        0.659, 0.702, 0.745,
    ],
    skill_darkness_dmg: [
        0.168, 0.1811, 0.1942, 0.212, 0.2251, 0.2382, 0.256, 0.2738, 0.2916, 0.3094, 0.3272, 0.345,
        0.369, 0.393, 0.417,
    ],

    burst_initial1: [
        0.96, 1.0352, 1.1104, 1.212, 1.2872, 1.3624, 1.464, 1.5656, 1.6672, 1.7688, 1.8704, 1.972,
        2.096, 2.22, 2.344,
    ],
    burst_initial2: [
        0.96, 1.0352, 1.1104, 1.212, 1.2872, 1.3624, 1.464, 1.5656, 1.6672, 1.7688, 1.8704, 1.972,
        2.096, 2.22, 2.344,
    ],
    burst_initial3: [
        1.20, 1.294, 1.388, 1.515, 1.609, 1.703, 1.83, 1.957, 2.084, 2.211, 2.338, 2.465, 2.62,
        2.775, 2.93,
    ],
    burst_dragon_white: [
        1.36, 1.4664, 1.5728, 1.716, 1.8224, 1.9288, 2.072, 2.2152, 2.3584, 2.5016, 2.6448, 2.788,
        2.964, 3.14, 3.316,
    ],
    burst_dragon_dark: [
        1.70, 1.833, 1.966, 2.145, 2.278, 2.411, 2.59, 2.769, 2.948, 3.127, 3.306, 3.485, 3.701,
        3.917, 4.133,
    ],
};

const DURIN_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Durin,
    internal_name: "Durin",
    name_locale: locale!(
        zh_cn: "杜林",
        en: "Durin"
    ),
    element: Element::Pyro,
    hp: [
        968, 2511, 3341, 4998, 5587, 6429, 7214, 8063, 8653, 9512, 10100, 10967, 11556,
        12432, 13315,
    ],
    atk: [
        27, 70, 93, 139, 156, 179, 201, 225, 241, 265, 282, 306, 322, 347, 371,
    ],
    def: [
        64, 166, 221, 330, 369, 425, 477, 533, 572, 629, 668, 725, 764, 822, 880,
    ],
    sub_stat: CharacterSubStatFamily::CriticalDamage384,
    weapon_type: WeaponType::Sword,
    star: 5,
    skill_name1: locale!(
        zh_cn: "辉彩灵刃",
        en: "Radiant Wingslash",
    ),
    skill_name2: locale!(
        zh_cn: "二元形体·聚合与分裂",
        en: "Binary Form: Convergence and Division"
    ),
    skill_name3: locale!(
        zh_cn: "纯净之理：光随心动",
        en: "Principle of Purity: As the Light Shifts"
    ),
};

pub struct DurinEffect {
    pub constellation: i32,
    pub transmutation_state: usize,
    pub has_hexerei_teammate: bool,
}

impl<A: Attribute> ChangeAttribute<A> for DurinEffect {
    fn change_attribute(&self, attribute: &mut A) {
        // A1 Passive - depends on state
        if self.transmutation_state == 0 {
            // Purity state - RES Shred
            let res_shred = if self.has_hexerei_teammate {
                0.35
            } else {
                0.20
            };
            attribute.set_value_by(
                AttributeName::ResMinusPyro,
                "杜林A1：神圣计算之光的显形",
                res_shred,
            );
        } else {
            // Darkness state - Melt/Vape DMG Bonus
            // This is handled in damage calculation
        }

        // C2 - Elemental DMG Bonus after reactions
        if self.constellation >= 2 {
            attribute.set_value_by(AttributeName::BonusPyro, "杜林C2：无根愿景", 0.50);
        }
    }
}

#[derive(Copy, Clone, FromPrimitive, EnumString, EnumCountMacro)]
pub enum DurinDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal4,
    Charged1,
    Charged2,
    Plunging1,
    Plunging2,
    Plunging3,
    EPurity,
    EDarkness1,
    EDarkness2,
    EDarkness3,
    QInitial1,
    QInitial2,
    QInitial3,
    QDragonWhite,
    QDragonDark,
}

impl DurinDamageEnum {
    pub fn get_element(&self) -> Element {
        Element::Pyro
    }

    pub fn get_skill_type(&self) -> SkillType {
        use DurinDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged1 | Charged2 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            EPurity | EDarkness1 | EDarkness2 | EDarkness3 => SkillType::ElementalSkill,
            QInitial1 | QInitial2 | QInitial3 | QDragonWhite | QDragonDark => {
                SkillType::ElementalBurst
            }
        }
    }
}

impl Into<usize> for DurinDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum DurinRoleEnum {
    BurstSupport, // Purity form
    BurstDPS,     // Darkness form
}

pub struct Durin;

impl CharacterTrait for Durin {
    const STATIC_DATA: CharacterStaticData = DURIN_STATIC_DATA;
    type SkillType = DurinSkillType;
    const SKILL: Self::SkillType = DURIN_SKILL;
    type DamageEnumType = DurinDamageEnum;
    type RoleEnum = DurinRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem {
                index: DurinDamageEnum::Normal1 as usize,
                text: hit_n_dmg!(1),
            },
            CharacterSkillMapItem {
                index: DurinDamageEnum::Normal2 as usize,
                text: hit_n_dmg!(2),
            },
            CharacterSkillMapItem {
                index: DurinDamageEnum::Normal3 as usize,
                text: hit_n_dmg!(3),
            },
            CharacterSkillMapItem {
                index: DurinDamageEnum::Normal4 as usize,
                text: hit_n_dmg!(4),
            },
            CharacterSkillMapItem {
                index: DurinDamageEnum::Charged1 as usize,
                text: charged_dmg!(),
            },
            CharacterSkillMapItem {
                index: DurinDamageEnum::Charged2 as usize,
                text: charged_dmg!(1),
            },
            CharacterSkillMapItem {
                index: DurinDamageEnum::Plunging1 as usize,
                text: plunging_dmg!(1),
            },
            CharacterSkillMapItem {
                index: DurinDamageEnum::Plunging2 as usize,
                text: plunging_dmg!(2),
            },
            CharacterSkillMapItem {
                index: DurinDamageEnum::Plunging3 as usize,
                text: plunging_dmg!(3),
            },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem {
                index: DurinDamageEnum::EPurity as usize,
                text: locale!(zh_cn: "纯净确认伤害", en: "Confirmation of Purity DMG"),
            },
            CharacterSkillMapItem {
                index: DurinDamageEnum::EDarkness1 as usize,
                text: locale!(zh_cn: "暗黑否定伤害1", en: "Denial of Darkness DMG 1"),
            },
            CharacterSkillMapItem {
                index: DurinDamageEnum::EDarkness2 as usize,
                text: locale!(zh_cn: "暗黑否定伤害2", en: "Denial of Darkness DMG 2"),
            },
            CharacterSkillMapItem {
                index: DurinDamageEnum::EDarkness3 as usize,
                text: locale!(zh_cn: "暗黑否定伤害3", en: "Denial of Darkness DMG 3"),
            },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem {
                index: DurinDamageEnum::QInitial1 as usize,
                text: locale!(zh_cn: "爆发初始伤害1", en: "Burst Initial DMG 1"),
            },
            CharacterSkillMapItem {
                index: DurinDamageEnum::QInitial2 as usize,
                text: locale!(zh_cn: "爆发初始伤害2", en: "Burst Initial DMG 2"),
            },
            CharacterSkillMapItem {
                index: DurinDamageEnum::QInitial3 as usize,
                text: locale!(zh_cn: "爆发初始伤害3", en: "Burst Initial DMG 3"),
            },
            CharacterSkillMapItem {
                index: DurinDamageEnum::QDragonWhite as usize,
                text: locale!(zh_cn: "白龙伤害", en: "Dragon of White Flame DMG"),
            },
            CharacterSkillMapItem {
                index: DurinDamageEnum::QDragonDark as usize,
                text: locale!(zh_cn: "暗黑龙伤害", en: "Dragon of Dark Decay DMG"),
            },
        ]),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "transmutation_state",
            title: locale!(
                zh_cn: "变形状态 (0=纯净, 1=黑暗)",
                en: "Transmutation State (0=Purity, 1=Darkness)"
            ),
            config: ItemConfigType::Int {
                min: 0,
                max: 1,
                default: 0,
            },
        },
        ItemConfig {
            name: "hexerei_teammate",
            title: locale!(
                zh_cn: "是否有其他Hexerei队友",
                en: "Has Hexerei Teammate"
            ),
            config: ItemConfigType::Bool { default: false },
        },
        ItemConfig {
            name: "primordial_fusion_stacks",
            title: locale!(
                zh_cn: "原初聚合之形层数",
                en: "Primordial Fusion Stacks"
            ),
            config: ItemConfigType::Float {
                min: 0.0,
                max: 10.0,
                default: 10.0,
            },
        },
    ]);

    fn damage_internal<D: DamageBuilder>(
        context: &DamageContext<'_, D::AttributeType>,
        s: usize,
        config: &CharacterSkillConfig,
        _fumo: Option<Element>,
    ) -> D::Result {
        let s: DurinDamageEnum = num::FromPrimitive::from_usize(s).expect("wrong skill index");
        let (s1, s2, s3) = context.character_common_data.get_3_skill();
        let constellation = context.character_common_data.constellation;

        let (transmutation_state, _has_hexerei, primordial_stacks) = match *config {
            CharacterSkillConfig::Durin {
                transmutation_state,
            } => (transmutation_state, false, 10.0),
            _ => (0, false, 10.0),
        };

        let ratio = match s {
            DurinDamageEnum::Normal1 => DURIN_SKILL.normal_dmg1[s1],
            DurinDamageEnum::Normal2 => DURIN_SKILL.normal_dmg2[s1],
            DurinDamageEnum::Normal3 => DURIN_SKILL.normal_dmg3[s1],
            DurinDamageEnum::Normal4 => DURIN_SKILL.normal_dmg4[s1],
            DurinDamageEnum::Charged1 => DURIN_SKILL.charged_dmg1[s1],
            DurinDamageEnum::Charged2 => DURIN_SKILL.charged_dmg2[s1],
            DurinDamageEnum::Plunging1 => DURIN_SKILL.plunging_dmg1[s1],
            DurinDamageEnum::Plunging2 => DURIN_SKILL.plunging_dmg2[s1],
            DurinDamageEnum::Plunging3 => DURIN_SKILL.plunging_dmg3[s1],
            DurinDamageEnum::EPurity => DURIN_SKILL.skill_purity_dmg[s2],
            DurinDamageEnum::EDarkness1 => DURIN_SKILL.skill_darkness_dmg[s2],
            DurinDamageEnum::EDarkness2 => DURIN_SKILL.skill_darkness_dmg[s2],
            DurinDamageEnum::EDarkness3 => DURIN_SKILL.skill_darkness_dmg[s2],
            DurinDamageEnum::QInitial1 => DURIN_SKILL.burst_initial1[s3],
            DurinDamageEnum::QInitial2 => DURIN_SKILL.burst_initial2[s3],
            DurinDamageEnum::QInitial3 => DURIN_SKILL.burst_initial3[s3],
            DurinDamageEnum::QDragonWhite => DURIN_SKILL.burst_dragon_white[s3],
            DurinDamageEnum::QDragonDark => DURIN_SKILL.burst_dragon_dark[s3],
        };

        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);

        // A4 Passive - Primordial Fusion
        if matches!(
            s,
            DurinDamageEnum::QDragonWhite | DurinDamageEnum::QDragonDark
        ) {
            let stacks_f64: f64 = primordial_stacks as f64;
            let bonus_ratio = 3.0 * stacks_f64.min(10.0) / 100.0;
            builder.add_extra_bonus("杜林A4：混沌化夜", bonus_ratio);
        }

        // C6 - Additional bonus in Darkness form
        if constellation >= 6 && transmutation_state == 1 {
            if matches!(s, DurinDamageEnum::QDragonDark) {
                builder.add_extra_bonus("杜林C6：双生", 0.40);
            }
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(),
            s.get_skill_type(),
            context.character_common_data.level,
            None,
        )
    }

    fn new_effect<A: Attribute>(
        common_data: &CharacterCommonData,
        config: &CharacterConfig,
    ) -> Option<Box<dyn ChangeAttribute<A>>> {
        let transmutation_state = match *config {
            CharacterConfig::Durin {
                transmutation_state,
            } => transmutation_state,
            _ => 0,
        };

        Some(Box::new(DurinEffect {
            constellation: common_data.constellation,
            transmutation_state,
            has_hexerei_teammate: false,
        }))
    }

    fn get_target_function_by_role(
        role_index: usize,
        _team: &TeamQuantization,
        _c: &CharacterCommonData,
        _w: &WeaponCommonData,
    ) -> Box<dyn TargetFunction> {
        let role: DurinRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            DurinRoleEnum::BurstSupport | DurinRoleEnum::BurstDPS => {
                Box::new(DurinDefaultTargetFunction {
                    recharge_demand: 1.3,
                    use_skill: 0.2,
                    use_burst: 0.8,
                    transmutation_state: 1,
                })
            }
        }
    }
}
