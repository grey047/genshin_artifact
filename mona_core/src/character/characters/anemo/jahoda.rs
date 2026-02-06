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
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

// Auto-generated from Genshin Fandom Wiki
// Character: Jahoda (Anemo / Bow)

pub struct JahodaSkillType {
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

pub const JAHODA_SKILL: JahodaSkillType = JahodaSkillType {
    normal_dmg1: [18.70, 48.64, 48.04, 124.96, 62.01, 161.30, 92.88, 241.60, 15.00, 102.80, 267.42, 118.25, 307.60, 20.00, 0.0],
    normal_dmg2: [161.30, 92.88, 241.60, 15.00, 102.80, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    normal_dmg3: [267.42, 118.25, 307.60, 20.00, 131.48, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    normal_dmg4: [342.02, 146.93, 382.20, 30.00, 18.00, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    charged_dmg1: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    plunging_dmg1: [0.5683; 15],
    plunging_dmg2: [1.1363; 15],
    plunging_dmg3: [1.4193; 15],
    e_dmg1: [150.00, 160.00, 170.00, 180.00, 190.00, 200.00, 210.00, 220.00, 230.00, 240.00, 250.00, 260.00, 270.00, 280.00, 0.0],
    q_dmg1: [200.00, 215.00, 230.00, 245.00, 260.00, 275.00, 290.00, 305.00, 320.00, 335.00, 350.00, 365.00, 380.00, 395.00, 0.0],
};

damage_enum!(
    JahodaDamageEnum
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

impl JahodaDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use JahodaDamageEnum::*;
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

pub struct JahodaEffect;

impl<A: Attribute> ChangeAttribute<A> for JahodaEffect {
    fn change_attribute(&self, _attribute: &mut A) {
        // TODO: Add constellation/talent effects from Fandom data
    }
}

pub struct Jahoda;

impl CharacterTrait for Jahoda {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Jahoda,
        internal_name: "Jahoda",
        name_locale: locale!(
            zh_cn: "Jahoda",
            en: "Jahoda",
        ),
        element: Element::Anemo,
        hp: [771, 2218, 2218, 4340, 4340, 4340, 4340, 5498, 5498, 5498, 5498, 6655, 6655, 6655, 7716],
        atk: [17, 51, 51, 100, 100, 100, 100, 127, 127, 127, 127, 153, 153, 153, 178],
        def: [46, 133, 133, 261, 261, 261, 261, 330, 330, 330, 330, 400, 400, 400, 464],
        sub_stat: CharacterSubStatFamily::ATK288,
        weapon_type: WeaponType::Bow,
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
        )
    };
    type SkillType = JahodaSkillType;
    const SKILL: Self::SkillType = JAHODA_SKILL;
    type DamageEnumType = JahodaDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            JahodaDamageEnum
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
            JahodaDamageEnum
            E1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
        ),
        skill3: skill_map!(
            JahodaDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = None;

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, _config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let skill: JahodaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, _s2, _s3) = context.character_common_data.get_3_skill();
        let builder = D::new();
        builder.damage(
            &context.attribute,
            &context.enemy,
            Element::Anemo,
            skill.get_skill_type(),
            context.character_common_data.level,
            fumo
        )
    }

    fn new_effect<A: Attribute>(_common_data: &CharacterCommonData, _config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        Some(Box::new(JahodaEffect))
    }

    fn get_target_function_by_role(_role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
