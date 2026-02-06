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
// Character: Columbina (Hydro / Catalyst)

pub struct ColumbinaSkillType {
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

pub const COLUMBINA_SKILL: ColumbinaSkillType = ColumbinaSkillType {
    normal_dmg1: [40.09, 19.32, 103.98, 25.71, 138.35, 38.46, 207.02, 15.00, 43.00, 231.44, 49.47, 266.28, 20.00, 55.52, 0.0],
    normal_dmg2: [38.46, 207.02, 15.00, 43.00, 231.44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    normal_dmg3: [49.47, 266.28, 20.00, 55.52, 298.84, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    normal_dmg4: [62.06, 334.04, 30.00, 18.00, 66.60, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    charged_dmg1: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    plunging_dmg1: [0.5683; 15],
    plunging_dmg2: [1.1363; 15],
    plunging_dmg3: [1.4193; 15],
    e_dmg1: [150.00, 160.00, 170.00, 180.00, 190.00, 200.00, 210.00, 220.00, 230.00, 240.00, 250.00, 260.00, 270.00, 280.00, 0.0],
    q_dmg1: [200.00, 215.00, 230.00, 245.00, 260.00, 275.00, 290.00, 305.00, 320.00, 335.00, 350.00, 365.00, 380.00, 395.00, 0.0],
};

damage_enum!(
    ColumbinaDamageEnum
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

impl ColumbinaDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use ColumbinaDamageEnum::*;
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

pub struct ColumbinaEffect;

impl<A: Attribute> ChangeAttribute<A> for ColumbinaEffect {
    fn change_attribute(&self, _attribute: &mut A) {
        // TODO: Add constellation/talent effects from Fandom data
    }
}

pub struct Columbina;

impl CharacterTrait for Columbina {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Columbina,
        internal_name: "Columbina",
        name_locale: locale!(
            zh_cn: "Columbina",
            en: "Columbina",
        ),
        element: Element::Hydro,
        hp: [1175, 3379, 3379, 6612, 6612, 6612, 6612, 8376, 8376, 8376, 8376, 10139, 10139, 10139, 11756],
        atk: [7, 21, 21, 42, 42, 42, 42, 54, 54, 54, 54, 65, 65, 65, 76],
        def: [41, 118, 118, 231, 231, 231, 231, 292, 292, 292, 292, 354, 354, 354, 411],
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
    type SkillType = ColumbinaSkillType;
    const SKILL: Self::SkillType = COLUMBINA_SKILL;
    type DamageEnumType = ColumbinaDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            ColumbinaDamageEnum
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
            ColumbinaDamageEnum
            E1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
        ),
        skill3: skill_map!(
            ColumbinaDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = None;

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, _config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let skill: ColumbinaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, _s2, _s3) = context.character_common_data.get_3_skill();
        let builder = D::new();
        builder.damage(
            &context.attribute,
            &context.enemy,
            Element::Hydro,
            skill.get_skill_type(),
            context.character_common_data.level,
            fumo
        )
    }

    fn new_effect<A: Attribute>(_common_data: &CharacterCommonData, _config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        Some(Box::new(ColumbinaEffect))
    }

    fn get_target_function_by_role(_role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
