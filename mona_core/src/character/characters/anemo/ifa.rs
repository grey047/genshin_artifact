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
use crate::target_functions::target_functions::anemo::IfaDefaultTargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

// Auto-generated from Genshin Fandom Wiki
// Character: Ifa (Anemo / Catalyst)

pub struct IfaSkillType {
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
    pub q_dmg2: [f64; 15],
}

pub const IFA_SKILL: IfaSkillType = IfaSkillType {
    normal_dmg1: [0.432, 0.467, 0.502, 0.552, 0.587, 0.622, 0.672, 0.722, 0.772, 0.822, 0.872, 0.922, 0.972, 1.022, 1.072],
    normal_dmg2: [0.384, 0.415, 0.446, 0.491, 0.522, 0.553, 0.598, 0.643, 0.688, 0.733, 0.778, 0.823, 0.868, 0.913, 0.958],
    normal_dmg3: [0.528, 0.570, 0.612, 0.673, 0.715, 0.757, 0.819, 0.881, 0.943, 1.005, 1.067, 1.129, 1.191, 1.253, 1.315],
    normal_dmg4: [0.0; 15],
    charged_dmg1: [0.0; 15],
    plunging_dmg1: [0.5683; 15],
    plunging_dmg2: [1.1363; 15],
    plunging_dmg3: [1.4193; 15],
    e_dmg1: [1.280, 1.382, 1.484, 1.634, 1.736, 1.838, 1.988, 2.138, 2.288, 2.486, 2.684, 2.882, 3.080, 3.278, 3.476],
    q_dmg1: [4.160, 4.496, 4.832, 5.312, 5.648, 5.984, 6.464, 6.944, 7.424, 8.048, 8.672, 9.296, 9.920, 10.544, 11.168],
    q_dmg2: [1.664, 1.798, 1.932, 2.125, 2.259, 2.393, 2.586, 2.779, 2.972, 3.224, 3.476, 3.728, 3.980, 4.232, 4.484],
};

damage_enum!(
    IfaDamageEnum
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
    Q2
);

impl IfaDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use IfaDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 => SkillType::ElementalSkill,
            Q1 | Q2 => SkillType::ElementalBurst
        }
    }
}

pub struct IfaEffect;

impl<A: Attribute> ChangeAttribute<A> for IfaEffect {
    fn change_attribute(&self, _attribute: &mut A) {
        // TODO: Add constellation/talent effects from Fandom data
    }
}

pub struct Ifa;

impl CharacterTrait for Ifa {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Ifa,
        internal_name: "Ifa",
        name_locale: locale!(
            zh_cn: "伊法",
            en: "Ifa",
        ),
        element: Element::Anemo,
        hp: [800, 2081, 2774, 4151, 4632, 5325, 5968, 6670, 7151, 7854, 8334, 9047, 9528, 10264, 10264],
        atk: [45, 118, 157, 235, 262, 302, 338, 378, 405, 446, 473, 513, 541, 572, 572],
        def: [45, 117, 156, 233, 260, 299, 336, 376, 403, 443, 470, 510, 538, 576, 576],
        sub_stat: CharacterSubStatFamily::CriticalRate192,
        weapon_type: WeaponType::Catalyst,
        star: 5,
        skill_name1: locale!(
            zh_cn: "驱风仪式",
            en: "Rite of Dispelling Winds",
        ),
        skill_name2: locale!(
            zh_cn: "空气疾病预防",
            en: "Airborne Disease Prevention",
        ),
        skill_name3: locale!(
            zh_cn: "复合镇静领域",
            en: "Compound Sedation Field",
        )
    };
    type SkillType = IfaSkillType;
    const SKILL: Self::SkillType = IFA_SKILL;
    type DamageEnumType = IfaDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            IfaDamageEnum
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
            IfaDamageEnum
            E1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
        ),
        skill3: skill_map!(
            IfaDamageEnum
            Q1 locale!(zh_cn: "爆发伤害", en: "Burst DMG")
            Q2 locale!(zh_cn: "标记伤害", en: "Sedation Mark DMG")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = None;

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, _config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let skill: IfaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, _s2, s3) = context.character_common_data.get_3_skill();
        
        let ratio = match skill {
            IfaDamageEnum::Normal1 => IFA_SKILL.normal_dmg1[s1],
            IfaDamageEnum::Normal2 => IFA_SKILL.normal_dmg2[s1],
            IfaDamageEnum::Normal3 => IFA_SKILL.normal_dmg3[s1],
            IfaDamageEnum::Normal4 => IFA_SKILL.normal_dmg4[s1],
            IfaDamageEnum::Charged => IFA_SKILL.charged_dmg1[s1],
            IfaDamageEnum::Plunging1 => IFA_SKILL.plunging_dmg1[s1],
            IfaDamageEnum::Plunging2 => IFA_SKILL.plunging_dmg2[s1],
            IfaDamageEnum::Plunging3 => IFA_SKILL.plunging_dmg3[s1],
            IfaDamageEnum::E1 => IFA_SKILL.e_dmg1[s1],
            IfaDamageEnum::Q1 => IFA_SKILL.q_dmg1[s3],
            IfaDamageEnum::Q2 => IFA_SKILL.q_dmg2[s3],
        };
        
        let mut builder = D::new();
        builder.add_atk_ratio("Skill Ratio", ratio);
        
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
        Some(Box::new(IfaEffect))
    }

    fn get_target_function_by_role(_role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        Box::new(IfaDefaultTargetFunction::new(&crate::target_functions::TargetFunctionConfig::IfaDefault {
            recharge_demand: 1.0,
            use_skill: 0.5,
            use_burst: 0.5,
        }))
    }
}
