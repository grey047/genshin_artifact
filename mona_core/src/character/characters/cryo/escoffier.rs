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

pub struct EscoffierSkillType {
    pub normal_dmg1: [f64; 15], pub normal_dmg2: [f64; 15], pub normal_dmg3: [f64; 15], pub normal_dmg4: [f64; 15],
    pub charged_dmg1: [f64; 15], pub plunging_dmg1: [f64; 15], pub plunging_dmg2: [f64; 15], pub plunging_dmg3: [f64; 15],
    pub e_dmg1: [f64; 15], pub q_dmg1: [f64; 15],
}

pub const ESCOFFIER_SKILL: EscoffierSkillType = EscoffierSkillType {
    normal_dmg1: [0.5155, 0.5575, 0.5994, 0.6594, 0.7013, 0.7493, 0.8152, 0.8812, 0.9471, 1.019, 1.091, 1.163, 1.235, 1.307, 1.379],
    normal_dmg2: [0.4759, 0.5147, 0.5534, 0.6088, 0.6475, 0.6918, 0.7526, 0.8135, 0.8744, 0.9408, 1.0072, 1.0736, 1.14, 1.2064, 1.2728],
    normal_dmg3: [0.3300, 0.3569, 0.3837, 0.4221, 0.4489, 0.4796, 0.5219, 0.5641, 0.6063, 0.6523, 0.6984, 0.7444, 0.7905, 0.8365, 0.8826],
    normal_dmg4: [0.4033, 0.4362, 0.4694, 0.5159, 0.5487, 0.5862, 0.6378, 0.6894, 0.741, 0.7973, 0.8536, 0.9099, 0.9662, 1.0225, 1.0788],
    charged_dmg1: [1.1541, 1.2481, 1.342, 1.4762, 1.5701, 1.6775, 1.8251, 1.9727, 2.1204, 2.2814, 2.4424, 2.6034, 2.7644, 2.9254, 3.0864],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    e_dmg1: [0.5045, 0.5418, 0.5796, 0.6366, 0.7056, 0.7568, 0.8064, 0.8568, 0.9072, 0.9576, 1.008, 1.0584, 1.1088, 1.1592, 1.2096],
    q_dmg1: [5.9286, 6.3726, 6.8172, 7.4172, 7.8546, 8.2992, 8.8929, 9.4848, 10.0776, 10.6704, 11.2632, 11.856, 12.4488, 13.0416, 13.6344],
};

damage_enum!(EscoffierDamageEnum Normal1 Normal2 Normal3 Normal4 Charged Plunging1 Plunging2 Plunging3 E1 Q1);

impl EscoffierDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use EscoffierDamageEnum::*;
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

pub struct EscoffierEffect;
impl<A: Attribute> ChangeAttribute<A> for EscoffierEffect { fn change_attribute(&self, _attribute: &mut A) {} }

pub struct Escoffier;

impl CharacterTrait for Escoffier {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Escoffier,
        internal_name: "Escoffier",
        name_locale: locale!(zh_cn: "埃斯科菲耶", en: "Escoffier"),
        element: Element::Cryo,
        // Base stats (Lv1-90 + 100级数据)
        // TODO: Verify actual data from KQM/HHW
        hp: [1030, 2659, 3538, 5295, 5914, 6807, 7641, 8533, 9159, 10072, 10700, 11621, 12248, 13175, 14112],
        atk: [27, 71, 95, 142, 159, 183, 205, 229, 246, 270, 287, 312, 329, 354, 379],
        def: [63, 164, 218, 327, 365, 420, 472, 527, 566, 622, 661, 718, 757, 815, 872],
        sub_stat: CharacterSubStatFamily::ATK288,
        weapon_type: WeaponType::Catalyst,
        star: 5,
        skill_name1: locale!(zh_cn: "烹饪技艺", en: "Kitchen Skills"),
        skill_name2: locale!(zh_cn: "低温烹饪", en: "Low-Temperature Cooking"),
        skill_name3: locale!(zh_cn: "评分切割", en: "Scoring Cuts")
    };
    type SkillType = EscoffierSkillType;
    const SKILL: Self::SkillType = ESCOFFIER_SKILL;
    type DamageEnumType = EscoffierDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(EscoffierDamageEnum Normal1 hit_n_dmg!(1) Normal2 hit_n_dmg!(2) Normal3 hit_n_dmg!(3) Normal4 hit_n_dmg!(4) Charged charged_dmg!() Plunging1 plunging_dmg!(1) Plunging2 plunging_dmg!(2) Plunging3 plunging_dmg!(3)),
        skill2: skill_map!(EscoffierDamageEnum E1 locale!(zh_cn: "技能伤害", en: "Skill DMG")),
        skill3: skill_map!(EscoffierDamageEnum Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG"))
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = None;

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, _config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let _s: EscoffierDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, _s2, _s3) = context.character_common_data.get_3_skill();
        let builder = D::new();
        builder.damage(
            &context.attribute,
            &context.enemy,
            Element::Cryo,
            SkillType::NormalAttack,
            context.character_common_data.level,
            fumo
        )
    }

    fn new_effect<A: Attribute>(_common_data: &CharacterCommonData, _config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> { None }
    fn get_target_function_by_role(_role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> { unimplemented!() }
}
