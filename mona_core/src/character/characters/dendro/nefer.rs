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
    normal_dmg1: [26.81, 62.22, 100.00, 69.55, 161.41, 92.54, 214.76, 100.00, 138.47, 321.35, 15.00, 154.80, 359.26, 100.00, 0.0],
    normal_dmg2: [92.54, 214.76, 100.00, 138.47, 321.35, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    normal_dmg3: [15.00, 154.80, 359.26, 100.00, 178.10, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    normal_dmg4: [413.33, 20.00, 199.88, 463.87, 100.00, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    charged_dmg1: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    plunging_dmg1: [0.5683; 15],
    plunging_dmg2: [1.1363; 15],
    plunging_dmg3: [1.4193; 15],
    e_dmg1: [150.00, 160.00, 170.00, 180.00, 190.00, 200.00, 210.00, 220.00, 230.00, 240.00, 250.00, 260.00, 270.00, 280.00, 0.0],
    q_dmg1: [200.00, 215.00, 230.00, 245.00, 260.00, 275.00, 290.00, 305.00, 320.00, 335.00, 350.00, 365.00, 380.00, 395.00, 0.0],
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
            zh_cn: "Nefer",
            en: "Nefer",
        ),
        element: Element::Dendro,
        hp: [1016, 2921, 2921, 5716, 5716, 5716, 5716, 7240, 7240, 7240, 7240, 8765, 8765, 8765, 10162],
        atk: [27, 79, 79, 154, 154, 154, 154, 196, 196, 196, 196, 237, 237, 237, 275],
        def: [63, 183, 183, 359, 359, 359, 359, 455, 455, 455, 455, 551, 551, 551, 639],
        sub_stat: CharacterSubStatFamily::ATK288,
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
        unimplemented!()
    }
}
