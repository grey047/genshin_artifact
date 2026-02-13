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
use crate::target_functions::target_functions::hydro::DahliaDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

type SKILL = [f64; 15];

pub struct DahliaSkillType {
    pub normal_dmg1: SKILL,
    pub normal_dmg2: SKILL,
    pub normal_dmg3: SKILL,
    pub normal_dmg4: SKILL,
    pub charged_dmg1: SKILL,
    pub charged_dmg2: SKILL,
    pub plunging_dmg1: SKILL,
    pub plunging_dmg2: SKILL,
    pub plunging_dmg3: SKILL,

    pub skill_sanctuary_dmg: SKILL,
    pub skill_crimson_blossom_dmg: SKILL,

    pub burst_exequy_dmg: SKILL,
}

pub const DAHLIA_SKILL: DahliaSkillType = DahliaSkillType {
    normal_dmg1: [
        0.546, 0.590, 0.634, 0.697, 0.741, 0.785, 0.848, 0.911, 0.974, 1.037, 1.100, 1.163, 1.226,
        1.289, 1.352,
    ],
    normal_dmg2: [
        0.499, 0.540, 0.581, 0.639, 0.680, 0.721, 0.778, 0.836, 0.893, 0.951, 1.008, 1.066,
        1.123, 1.181, 1.238,
    ],
    normal_dmg3: [
        0.637, 0.688, 0.739, 0.813, 0.864, 0.915, 0.988, 1.061, 1.134, 1.207, 1.280, 1.353,
        1.426, 1.499, 1.572,
    ],
    normal_dmg4: [
        0.728, 0.786, 0.845, 0.929, 0.988, 1.047, 1.130, 1.214, 1.297, 1.381, 1.464, 1.548,
        1.631, 1.715, 1.798,
    ],
    charged_dmg1: [
        1.304, 1.410, 1.516, 1.668, 1.774, 1.880, 2.032, 2.184, 2.336, 2.488, 2.640, 2.792,
        2.944, 3.096, 3.248,
    ],
    charged_dmg2: [
        0.896, 0.968, 1.040, 1.144, 1.216, 1.288, 1.392, 1.496, 1.600, 1.704, 1.808, 1.912,
        2.016, 2.120, 2.224,
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

    skill_sanctuary_dmg: [
        1.512, 1.632, 1.752, 1.920, 2.040, 2.160, 2.328, 2.496, 2.664, 2.880, 3.096, 3.312,
        3.528, 3.744, 3.960,
    ],
    skill_crimson_blossom_dmg: [
        0.240, 0.240, 0.240, 0.240, 0.240, 0.240, 0.240, 0.240, 0.240, 0.240, 0.240, 0.240,
        0.240, 0.240, 0.240,
    ],

    burst_exequy_dmg: [
        5.520, 5.960, 6.400, 7.040, 7.480, 7.920, 8.560, 9.200, 9.840, 10.640, 11.440, 12.240,
        13.040, 13.840, 14.640,
    ],
};

const DAHLIA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Dahlia,
    internal_name: "Dahlia",
    name_locale: locale!(
        zh_cn: "达利亚",
        en: "Dahlia"
    ),
    element: Element::Hydro,
    hp: [
        1049, 2695, 3477, 5211, 5767, 6632, 7376, 8243, 8800, 9666, 10222, 11089, 11645,
        12510, 12510,
    ],
    atk: [
        16, 41, 53, 80, 88, 101, 113, 126, 134, 147, 156, 169, 178, 191, 191,
    ],
    def: [
        47, 121, 156, 234, 258, 297, 331, 369, 394, 433, 458, 497, 522, 561, 561,
    ],
    sub_stat: CharacterSubStatFamily::HP240,
    weapon_type: WeaponType::Sword,
    star: 4,
    skill_name1: locale!(
        zh_cn: "西风剑术·仪式",
        en: "Favonius Bladework - Ritual",
    ),
    skill_name2: locale!(
        zh_cn: "沉浸领域",
        en: "Immersive Ordinance",
    ),
    skill_name3: locale!(
        zh_cn: "光启诗篇",
        en: "Radiant Psalter",
    ),
};

pub struct DahliaEffect {
    pub constellation: i32,
    pub is_burning_enemy: bool,
}

impl<A: Attribute> ChangeAttribute<A> for DahliaEffect {
    fn change_attribute(&self, attribute: &mut A) {
        // A4 Passive - Against burning enemies, Pyro DMG Bonus +15%
        if self.is_burning_enemy {
            attribute.set_value_by(AttributeName::BonusPyro, "Dahlia A4", 0.15);
        }

        // C6 - While Burst active, party members gain 20% ATK and 15% CRIT Rate against burning enemies
        // NOTE: Requires team context - not implemented in ChangeAttribute
        // TODO: Implement in damage calculation with team quantification
    }
}

#[derive(Copy, Clone, FromPrimitive, EnumString, EnumCountMacro)]
pub enum DahliaDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal4,
    Charged1,
    Charged2,
    Plunging1,
    Plunging2,
    Plunging3,
    ESanctuary,
    ECrimsonBlossom,
    QExequy,
}

impl DahliaDamageEnum {
    pub fn get_element(&self) -> Element {
        Element::Hydro
    }

    pub fn get_skill_type(&self) -> SkillType {
        use DahliaDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged1 | Charged2 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            ESanctuary | ECrimsonBlossom => SkillType::ElementalSkill,
            QExequy => SkillType::ElementalBurst,
        }
    }
}

impl Into<usize> for DahliaDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum DahliaRoleEnum {
    SubDPS, // Support/Sub-DPS with Burn focus
}

pub struct Dahlia;

impl CharacterTrait for Dahlia {
    const STATIC_DATA: CharacterStaticData = DAHLIA_STATIC_DATA;
    type SkillType = DahliaSkillType;
    const SKILL: Self::SkillType = DAHLIA_SKILL;
    type DamageEnumType = DahliaDamageEnum;
    type RoleEnum = DahliaRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem {
                index: DahliaDamageEnum::Normal1 as usize,
                text: hit_n_dmg!(1),
            },
            CharacterSkillMapItem {
                index: DahliaDamageEnum::Normal2 as usize,
                text: hit_n_dmg!(2),
            },
            CharacterSkillMapItem {
                index: DahliaDamageEnum::Normal3 as usize,
                text: hit_n_dmg!(3),
            },
            CharacterSkillMapItem {
                index: DahliaDamageEnum::Normal4 as usize,
                text: hit_n_dmg!(4),
            },
            CharacterSkillMapItem {
                index: DahliaDamageEnum::Charged1 as usize,
                text: charged_dmg!(),
            },
            CharacterSkillMapItem {
                index: DahliaDamageEnum::Charged2 as usize,
                text: charged_dmg!(1),
            },
            CharacterSkillMapItem {
                index: DahliaDamageEnum::Plunging1 as usize,
                text: plunging_dmg!(1),
            },
            CharacterSkillMapItem {
                index: DahliaDamageEnum::Plunging2 as usize,
                text: plunging_dmg!(2),
            },
            CharacterSkillMapItem {
                index: DahliaDamageEnum::Plunging3 as usize,
                text: plunging_dmg!(3),
            },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem {
                index: DahliaDamageEnum::ESanctuary as usize,
                text: locale!(zh_cn: "绯红领域伤害", en: "Scarlet Sanctuary DMG"),
            },
            CharacterSkillMapItem {
                index: DahliaDamageEnum::ECrimsonBlossom as usize,
                text: locale!(zh_cn: "赤红之花伤害", en: "Crimson Blossom DMG"),
            },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem {
                index: DahliaDamageEnum::QExequy as usize,
                text: locale!(zh_cn: "绯红终曲伤害", en: "Crimson Exequy DMG"),
            },
        ]),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "is_burning_enemy",
            title: locale!(
                zh_cn: "敌人处于燃烧状态",
                en: "Enemy is Burning"
            ),
            config: ItemConfigType::Bool { default: false },
        },
    ]);

    fn damage_internal<D: DamageBuilder>(
        context: &DamageContext<'_, D::AttributeType>,
        s: usize,
        config: &CharacterSkillConfig,
        _fumo: Option<Element>,
    ) -> D::Result {
        let s: DahliaDamageEnum = num::FromPrimitive::from_usize(s).expect("wrong skill index");
        let (s1, s2, s3) = context.character_common_data.get_3_skill();
        let constellation = context.character_common_data.constellation;

        let (is_burning, _) = match *config {
            CharacterSkillConfig::Dahlia { is_burning } => (is_burning, false),
            _ => (false, false),
        };

        let ratio = match s {
            DahliaDamageEnum::Normal1 => DAHLIA_SKILL.normal_dmg1[s1],
            DahliaDamageEnum::Normal2 => DAHLIA_SKILL.normal_dmg2[s1],
            DahliaDamageEnum::Normal3 => DAHLIA_SKILL.normal_dmg3[s1],
            DahliaDamageEnum::Normal4 => DAHLIA_SKILL.normal_dmg4[s1],
            DahliaDamageEnum::Charged1 => DAHLIA_SKILL.charged_dmg1[s1],
            DahliaDamageEnum::Charged2 => DAHLIA_SKILL.charged_dmg2[s1],
            DahliaDamageEnum::Plunging1 => DAHLIA_SKILL.plunging_dmg1[s1],
            DahliaDamageEnum::Plunging2 => DAHLIA_SKILL.plunging_dmg2[s1],
            DahliaDamageEnum::Plunging3 => DAHLIA_SKILL.plunging_dmg3[s1],
            DahliaDamageEnum::ESanctuary => DAHLIA_SKILL.skill_sanctuary_dmg[s2],
            DahliaDamageEnum::ECrimsonBlossom => {
                // Crimson Blossom scales off enemy Max HP (2.4% per tick)
                // For simplicity, we'll return a percentage-based calculation
                DAHLIA_SKILL.skill_crimson_blossom_dmg[s2]
            }
            DahliaDamageEnum::QExequy => DAHLIA_SKILL.burst_exequy_dmg[s3],
        };

        let mut builder = D::new();
        builder.add_atk_ratio("Skill Ratio", ratio);

        // A1 Passive - Crimson Blossom chains to nearby enemies (not implemented in basic damage)

        // A4 Passive - Bonus against burning enemies
        if is_burning && matches!(s, DahliaDamageEnum::ESanctuary | DahliaDamageEnum::QExequy) {
            builder.add_extra_bonus("Dahlia A4", 0.15);
        }

        // C1 - Crimson Blossom damage increased by 50%
        if constellation >= 1 && matches!(s, DahliaDamageEnum::ECrimsonBlossom) {
            builder.add_extra_bonus("Dahlia C1", 0.50);
        }

        // C4 - When Burst hits burning enemies, generates 4 Energy
        // NOTE: Energy generation requires team context - not implemented in damage calculation

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
        let is_burning = match *config {
            CharacterConfig::Dahlia { is_burning } => is_burning,
            _ => false,
        };

        Some(Box::new(DahliaEffect {
            constellation: common_data.constellation,
            is_burning_enemy: is_burning,
        }))
    }

    fn get_target_function_by_role(
        role_index: usize,
        _team: &TeamQuantization,
        _c: &CharacterCommonData,
        _w: &WeaponCommonData,
    ) -> Box<dyn TargetFunction> {
        let role: DahliaRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap();
        match role {
            DahliaRoleEnum::SubDPS => Box::new(DahliaDefaultTargetFunction {
                recharge_demand: 1.0,
                use_skill: 0.4,
                use_burst: 0.6,
            }),
        }
    }
}
