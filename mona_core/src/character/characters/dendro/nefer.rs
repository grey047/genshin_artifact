use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::macros::{damage_enum, damage_ratio, skill_map, skill_type};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, StatName, WeaponType};
use crate::common::i18n::{locale, hit_n_dmg, plunging_dmg, charged_dmg};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::target_functions::target_functions::dendro::NeferDefaultTargetFunction;
use crate::target_functions::TargetFunctionConfig;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

// Auto-generated from Genshin Fandom Wiki
// Character: Nefer (Dendro / Catalyst)

pub struct NeferSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],
    pub e_dmg1: [f64; 15],
    pub q_dmg1: [f64; 15],
}

pub const NEFER_SKILL: NeferSkillType = NeferSkillType {
    normal_dmg1: [0.648, 0.700, 0.752, 0.827, 0.879, 0.931, 1.005, 1.079, 1.153, 1.227, 1.301, 1.375, 1.449, 1.523, 1.597],
    normal_dmg2: [0.576, 0.622, 0.668, 0.734, 0.780, 0.826, 0.892, 0.958, 1.024, 1.090, 1.156, 1.222, 1.288, 1.354, 1.420],
    normal_dmg3: [0.792, 0.855, 0.918, 1.009, 1.072, 1.135, 1.226, 1.317, 1.408, 1.499, 1.590, 1.681, 1.772, 1.863, 1.954],
    normal_dmg4: [0.896, 0.968, 1.040, 1.144, 1.216, 1.288, 1.392, 1.496, 1.600, 1.704, 1.808, 1.912, 2.016, 2.120, 2.224],
    charged_dmg1: [1.472, 1.590, 1.708, 1.878, 1.996, 2.114, 2.284, 2.454, 2.624, 2.846, 3.068, 3.290, 3.512, 3.734, 3.956],
    plunging_dmg1: [0.6393, 0.6902, 0.7411, 0.8137, 0.8646, 0.9155, 0.9881, 1.0607, 1.1333, 1.2275, 1.3218, 1.4160, 1.5102, 1.6044, 1.7098],
    plunging_dmg2: [1.2784, 1.3801, 1.4818, 1.6270, 1.7287, 1.8304, 1.9756, 2.1208, 2.2660, 2.4547, 2.6434, 2.8321, 3.0208, 3.2095, 3.4189],
    plunging_dmg3: [1.5968, 1.7248, 1.8528, 2.0336, 2.1616, 2.2896, 2.4704, 2.6512, 2.8320, 3.0678, 3.3036, 3.5394, 3.7752, 4.0110, 4.2704],
    e_dmg1: [3.072, 3.318, 3.564, 3.918, 4.164, 4.410, 4.764, 5.118, 5.472, 5.934, 6.396, 6.858, 7.320, 7.782, 8.244],
    q_dmg1: [4.992, 5.396, 5.800, 6.380, 6.784, 7.188, 7.768, 8.348, 8.928, 9.684, 10.440, 11.196, 11.952, 12.708, 13.464],
};

damage_enum!(
    NeferDamageEnum
    Normal1
    Normal2
    Normal3
    Normal4
    Charged
    Plunging1
    Plunging2
    Plunging3
    E1
    Q1
);

impl NeferDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use NeferDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst
        }
    }
}

pub struct NeferEffect;

impl<A: Attribute> ChangeAttribute<A> for NeferEffect {
    fn change_attribute(&self, _attribute: &mut A) {
        // TODO: Add constellation/talent effects from Fandom data
    }
}

pub struct Nefer;

impl CharacterTrait for Nefer {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Nefer,
        internal_name: "Nefer",
        name_locale: locale!(
            zh_cn: "奈芙尔",
            en: "Nefer",
        ),
        element: Element::Dendro,
        hp: [989, 2481, 2843, 3848, 4216, 4932, 5314, 6106, 6501, 7326, 7734, 8632, 9053, 9738, 10430],
        atk: [27, 71, 79, 108, 116, 137, 146, 170, 179, 205, 214, 242, 251, 270, 289],
        def: [62, 156, 173, 236, 255, 300, 320, 374, 394, 453, 472, 536, 556, 598, 641],
        sub_stat: CharacterSubStatFamily::CriticalDamage384,
        weapon_type: WeaponType::Catalyst,
        star: 5,
        skill_name1: locale!(
            zh_cn: "打击之蛇",
            en: "Striking Serpent",
        ),
        skill_name2: locale!(
            zh_cn: "塞内特策略：千夜之舞",
            en: "Senet Strategy: Dance of a Thousand Nights",
        ),
        skill_name3: locale!(
            zh_cn: "圣誓：真实之眼幻影",
            en: "Sacred Vow: True Eye's Phantasm",
        )
    };
    type SkillType = NeferSkillType;
    const SKILL: Self::SkillType = NEFER_SKILL;
    type DamageEnumType = NeferDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            NeferDamageEnum
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
            NeferDamageEnum
            E1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
        ),
        skill3: skill_map!(
            NeferDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
        )
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

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, _config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let skill: NeferDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, _s2, _s3) = context.character_common_data.get_3_skill();
        let builder = D::new();
        builder.damage(
            &context.attribute,
            &context.enemy,
            Element::Dendro,
            skill.get_skill_type(),
            context.character_common_data.level,
            fumo
        )
    }

    fn new_effect<A: Attribute>(_common_data: &CharacterCommonData, _config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        Some(Box::new(NeferEffect))
    }

    fn get_target_function_by_role(_role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        Box::new(NeferDefaultTargetFunction::new(&TargetFunctionConfig::NeferDefault {
            recharge_demand: 1.0,
            use_skill: 0.5,
            use_burst: 0.5,
            moonsign_level: 2,
        }))
    }
}
