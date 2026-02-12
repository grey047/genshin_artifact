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
use crate::target_functions::target_functions::anemo::JahodaDefaultTargetFunction;
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
    pub charged_dmg2: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],
    pub e_dmg1: [f64; 15],
    pub q_dmg1: [f64; 15],
}

pub const JAHODA_SKILL: JahodaSkillType = JahodaSkillType {
    normal_dmg1: [0.3740, 0.4038, 0.4336, 0.4769, 0.5067, 0.5365, 0.5798, 0.6231, 0.6664, 0.7097, 0.7530, 0.7963, 0.8396, 0.8829, 0.9262],
    normal_dmg2: [0.3724, 0.4020, 0.4316, 0.4748, 0.5044, 0.5340, 0.5772, 0.6204, 0.6636, 0.7068, 0.7500, 0.7932, 0.8364, 0.8796, 0.9228],
    normal_dmg3: [0.4939, 0.5333, 0.5727, 0.6300, 0.6694, 0.7088, 0.7661, 0.8234, 0.8807, 0.9380, 0.9953, 1.0526, 1.1099, 1.1672, 1.2245],
    normal_dmg4: [0.0; 15],
    charged_dmg1: [0.4400, 0.4752, 0.5104, 0.5614, 0.5966, 0.6318, 0.6828, 0.7338, 0.7848, 0.8466, 0.9084, 0.9702, 1.0320, 1.0938, 1.1556],
    charged_dmg2: [1.2400, 1.3392, 1.4384, 1.5822, 1.6814, 1.7806, 1.9244, 2.0682, 2.2120, 2.3874, 2.5628, 2.7382, 2.9136, 3.0890, 3.2644],
    plunging_dmg1: [0.5683; 15],
    plunging_dmg2: [1.1363; 15],
    plunging_dmg3: [1.4193; 15],
    e_dmg1: [1.1520, 1.2442, 1.3364, 1.4699, 1.5621, 1.6543, 1.7878, 1.9213, 2.0548, 2.2296, 2.4044, 2.5792, 2.7540, 2.9288, 3.1036],
    q_dmg1: [1.7280, 1.8662, 2.0044, 2.2049, 2.3431, 2.4813, 2.6818, 2.8823, 3.0828, 3.3444, 3.6060, 3.8676, 4.1292, 4.3908, 4.6524],
};

damage_enum!(
    JahodaDamageEnum
    Normal1
    Normal2
    Normal3
    Normal4
    Charged1
    Charged2
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
            Charged1 | Charged2 => SkillType::ChargedAttack,
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
            zh_cn: "雅珂达",
            en: "Jahoda",
        ),
        element: Element::Anemo,
        hp: [809, 2078, 2683, 4019, 4448, 5116, 5688, 6355, 6787, 7454, 7883, 8552, 8981, 9648, 9648],
        atk: [19, 49, 63, 94, 105, 120, 134, 149, 159, 175, 185, 201, 211, 227, 227],
        def: [49, 126, 163, 243, 269, 310, 345, 385, 411, 452, 478, 518, 544, 584, 584],
        sub_stat: CharacterSubStatFamily::HealingBonus222,
        weapon_type: WeaponType::Bow,
        star: 4,
        skill_name1: locale!(
            zh_cn: "趁热射箭",
            en: "Strike While the Arrow's Hot",
        ),
        skill_name2: locale!(
            zh_cn: "明智策略：分配战利品",
            en: "Savvy Strategy: Splitting the Spoils",
        ),
        skill_name3: locale!(
            zh_cn: "隐藏王牌：猎人七工具",
            en: "Hidden Aces: Seven Tools of the Hunter",
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
            Charged1 charged_dmg!()
            Charged2 locale!(zh_cn: "满蓄力瞄准射击", en: "Fully-Charged Aimed Shot")
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
        
        let ratio = match skill {
            JahodaDamageEnum::Normal1 => JAHODA_SKILL.normal_dmg1[s1],
            JahodaDamageEnum::Normal2 => JAHODA_SKILL.normal_dmg2[s1],
            JahodaDamageEnum::Normal3 => JAHODA_SKILL.normal_dmg3[s1],
            JahodaDamageEnum::Normal4 => JAHODA_SKILL.normal_dmg4[s1],
            JahodaDamageEnum::Charged1 => JAHODA_SKILL.charged_dmg1[s1],
            JahodaDamageEnum::Charged2 => JAHODA_SKILL.charged_dmg2[s1],
            JahodaDamageEnum::Plunging1 => JAHODA_SKILL.plunging_dmg1[s1],
            JahodaDamageEnum::Plunging2 => JAHODA_SKILL.plunging_dmg2[s1],
            JahodaDamageEnum::Plunging3 => JAHODA_SKILL.plunging_dmg3[s1],
            JahodaDamageEnum::E1 => JAHODA_SKILL.e_dmg1[s1],
            JahodaDamageEnum::Q1 => JAHODA_SKILL.q_dmg1[s1],
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
        Some(Box::new(JahodaEffect))
    }

    fn get_target_function_by_role(_role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        Box::new(JahodaDefaultTargetFunction::new(&crate::target_functions::TargetFunctionConfig::JahodaDefault {
            recharge_demand: 1.0,
            use_skill: 0.5,
            use_burst: 0.5,
        }))
    }
}
