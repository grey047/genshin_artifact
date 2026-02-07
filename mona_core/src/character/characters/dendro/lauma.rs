use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::macros::{damage_enum, damage_ratio, skill_map, skill_type};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::{ChangeAttribute, Element, SkillType, StatName, WeaponType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::{TargetFunction, TargetFunctionConfig};
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

// Auto-generated from Research Data
// Character: Lauma (Dendro / Catalyst)

pub struct LaumaSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],
    pub e_dmg1: [f64; 15],
    pub e_dmg2: [f64; 15],
    pub e_dmg3: [f64; 15],
    pub q_dmg1: [f64; 15],
}

pub const LAUMA_SKILL: LaumaSkillType = LaumaSkillType {
    // Normal Attack Multipliers (Peregrination of Linnunrata)
    normal_dmg1: [
        0.432, 0.467, 0.502, 0.552, 0.587, 0.622, 0.672, 0.722, 0.772, 0.822, 0.872, 0.922, 0.972,
        1.022, 1.072,
    ],
    normal_dmg2: [
        0.384, 0.415, 0.446, 0.491, 0.522, 0.553, 0.598, 0.643, 0.688, 0.733, 0.778, 0.823, 0.868,
        0.913, 0.958,
    ],
    normal_dmg3: [
        0.528, 0.570, 0.612, 0.673, 0.715, 0.757, 0.819, 0.881, 0.943, 1.005, 1.067, 1.129, 1.191,
        1.253, 1.315,
    ],
    normal_dmg4: [
        0.528, 0.570, 0.612, 0.673, 0.715, 0.757, 0.819, 0.881, 0.943, 1.005, 1.067, 1.129, 1.191,
        1.253, 1.315,
    ],
    charged_dmg: [
        0.528, 0.570, 0.612, 0.673, 0.715, 0.757, 0.819, 0.881, 0.943, 1.005, 1.067, 1.129, 1.191,
        1.253, 1.315,
    ],
    // Plunging Attack Multipliers (standard Catalyst values)
    plunging_dmg1: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
    plunging_dmg2: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
    plunging_dmg3: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.602, 3.8248, 4.0476, 4.2704,
    ],
    // Elemental Skill Multipliers (Runo: Dawnless Rest of Karsikko)
    e_dmg1: [
        2.048, 2.214, 2.380, 2.618, 2.784, 2.950, 3.188, 3.426, 3.664, 3.974, 4.284, 4.594, 4.904,
        5.214, 5.524,
    ],
    e_dmg2: [
        2.560, 2.768, 2.976, 3.272, 3.480, 3.688, 3.984, 4.280, 4.576, 4.968, 5.360, 5.752, 6.144,
        6.536, 6.928,
    ],
    e_dmg3: [
        1.280, 1.382, 1.484, 1.634, 1.736, 1.838, 1.988, 2.138, 2.288, 2.486, 2.684, 2.882, 3.080,
        3.278, 3.476,
    ],
    // Elemental Burst Multipliers (Runo: All Hearts Become the Beating Moon)
    q_dmg1: [
        4.160, 4.496, 4.832, 5.312, 5.648, 5.984, 6.464, 6.944, 7.424, 8.048, 8.672, 9.296, 9.920,
        10.544, 11.168,
    ],
};

damage_enum!(
    LaumaDamageEnum
    Normal1
    Normal2
    Normal3
    Normal4
    Charged
    Plunging1
    Plunging2
    Plunging3
    E1
    E2
    E3
    Q1
);

impl LaumaDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use LaumaDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 | E3 => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst,
        }
    }
}

pub struct LaumaEffect;

impl<A: Attribute> ChangeAttribute<A> for LaumaEffect {
    fn change_attribute(&self, _attribute: &mut A) {
        // TODO: Add constellation/talent effects from research data
    }
}

pub struct Lauma;

impl CharacterTrait for Lauma {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Lauma,
        internal_name: "Lauma",
        name_locale: locale!(
            zh_cn: "Lauma",
            en: "Lauma",
        ),
        element: Element::Dendro,
        // Base HP: 10264 (Lv1), ATK: 572 (Lv1), DEF: 576 (Lv1)
        // Using simplified pattern matching other 5-star characters
        hp: [
            10264, 13256, 14530, 17246, 18897, 21695, 23773, 27317, 29896, 34299, 37581, 43176,
            47317, 52152, 54913,
        ],
        atk: [
            572, 739, 811, 962, 1054, 1211, 1326, 1524, 1669, 1916, 2101, 2412, 2644, 2910, 3062,
        ],
        def: [
            576, 744, 816, 968, 1061, 1220, 1337, 1536, 1681, 1929, 2115, 2428, 2660, 2920, 3072,
        ],
        sub_stat: CharacterSubStatFamily::CriticalRate192,
        weapon_type: WeaponType::Catalyst,
        star: 5,
        skill_name1: locale!(
            zh_cn: "普通攻击",
            en: "Normal Attack",
        ),
        skill_name2: locale!(
            zh_cn: "元素战技",
            en: "Elemental Skill",
        ),
        skill_name3: locale!(
            zh_cn: "元素爆发",
            en: "Elemental Burst",
        ),
    };
    type SkillType = LaumaSkillType;
    const SKILL: Self::SkillType = LAUMA_SKILL;
    type DamageEnumType = LaumaDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            LaumaDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 hit_n_dmg!(3)
            Normal4 hit_n_dmg!(4)
            Charged charged_dmg!()
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            LaumaDamageEnum
            E1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
            E2 locale!(zh_cn: "强化伤害", en: "Enhanced DMG")
            E3 locale!(zh_cn: "灵菇伤害", en: "Spirit Envoy DMG")
        ),
        skill3: skill_map!(
            LaumaDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = None;

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "moonsign_level",
            title: locale!(
                zh_cn: "月兆等级",
                en: "Moonsign Level"
            ),
            config: ItemConfigType::Int { min: 1, max: 2, default: 2 },
        },
    ]);

    fn damage_internal<D: DamageBuilder>(
        context: &DamageContext<'_, D::AttributeType>,
        s: usize,
        _config: &CharacterSkillConfig,
        fumo: Option<Element>,
    ) -> D::Result {
        let skill: LaumaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let ratio = match skill {
            LaumaDamageEnum::Normal1 => LAUMA_SKILL.normal_dmg1[s1],
            LaumaDamageEnum::Normal2 => LAUMA_SKILL.normal_dmg2[s1],
            LaumaDamageEnum::Normal3 => LAUMA_SKILL.normal_dmg3[s1],
            LaumaDamageEnum::Normal4 => LAUMA_SKILL.normal_dmg4[s1],
            LaumaDamageEnum::Charged => LAUMA_SKILL.charged_dmg[s1],
            LaumaDamageEnum::Plunging1 => LAUMA_SKILL.plunging_dmg1[s1],
            LaumaDamageEnum::Plunging2 => LAUMA_SKILL.plunging_dmg2[s1],
            LaumaDamageEnum::Plunging3 => LAUMA_SKILL.plunging_dmg3[s1],
            LaumaDamageEnum::E1 => LAUMA_SKILL.e_dmg1[s2],
            LaumaDamageEnum::E2 => LAUMA_SKILL.e_dmg2[s2],
            LaumaDamageEnum::E3 => LAUMA_SKILL.e_dmg3[s2],
            LaumaDamageEnum::Q1 => LAUMA_SKILL.q_dmg1[s3],
        };

        let mut builder = D::new();
        builder.add_atk_ratio("Skill Ratio", ratio);

        builder.damage(
            &context.attribute,
            &context.enemy,
            Element::Dendro,
            skill.get_skill_type(),
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(
        _common_data: &CharacterCommonData,
        _config: &CharacterConfig,
    ) -> Option<Box<dyn ChangeAttribute<A>>> {
        Some(Box::new(LaumaEffect))
    }

    fn get_target_function_by_role(
        _role_index: usize,
        _team: &TeamQuantization,
        _c: &CharacterCommonData,
        _w: &WeaponCommonData,
    ) -> Box<dyn TargetFunction> {
        use crate::target_functions::target_function_config::TargetFunctionConfig;
        Box::new(crate::target_functions::target_functions::dendro::lauma_default::LaumaDefaultTargetFunction::new(&TargetFunctionConfig::LaumaDefault {
            recharge_demand: 1.0,
            use_skill: 0.5,
            use_burst: 0.5,
            moonsign_level: 2,
        }))
    }
}
