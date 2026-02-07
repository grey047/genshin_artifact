use num_derive::FromPrimitive;
use num::FromPrimitive;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};

use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, StatName, WeaponType};
use crate::artifacts::ArtifactSetName;
use crate::common::i18n::{locale, hit_n_dmg, charged_dmg, plunging_dmg};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::character::macros::{skill_type, damage_enum, skill_map, damage_ratio};

// Auto-generated from Genshin Fandom Wiki and GO Data
// Character: Aino (Hydro / Claymore)

pub struct AinoSkillType {
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
    pub e_dmg2: [f64; 15],

    pub q_dmg1: [f64; 15],
}

pub const AINO_SKILL: AinoSkillType = AinoSkillType {
    normal_dmg1: [0.9592, 1.0246, 1.09, 1.1772, 1.2426, 1.3189, 1.417, 1.5151, 1.6132, 1.7113, 1.8094, 1.9075, 2.0056, 2.1037, 2.2018],
    normal_dmg2: [0.8263, 0.8827, 0.939, 1.0141, 1.0705, 1.1362, 1.2207, 1.3052, 1.3897, 1.4742, 1.5587, 1.6433, 1.7278, 1.8123, 1.8968],
    normal_dmg3: [1.0331, 1.1036, 1.174, 1.2679, 1.3384, 1.4205, 1.5262, 1.6319, 1.7375, 1.8432, 1.9488, 2.0545, 2.1602, 2.2658, 2.3715],
    normal_dmg4: [1.3605, 1.4532, 1.546, 1.6697, 1.7624, 1.8707, 2.0098, 2.1489, 2.2881, 2.4272, 2.5664, 2.7055, 2.8446, 2.9838, 3.1229],
    charged_dmg1: [0.6254, 0.6763, 0.7272, 0.7999, 0.8508, 0.909, 0.989, 1.069, 1.149, 1.2362, 1.3235, 1.4108, 1.498, 1.5853, 1.6726],
    charged_dmg2: [1.1309, 1.223, 1.315, 1.4465, 1.5386, 1.6438, 1.7884, 1.9331, 2.0777, 2.2355, 2.3933, 2.5511, 2.7089, 2.8667, 3.0245],
    plunging_dmg1: [0.8205, 0.8872, 0.954, 1.0494, 1.1162, 1.1925, 1.2975, 1.4024, 1.5074, 1.6219, 1.7363, 1.8508, 1.9653, 2.0798, 2.1943],
    plunging_dmg2: [1.6406, 1.7741, 1.9077, 2.0984, 2.232, 2.3846, 2.5944, 2.8043, 3.0141, 3.243, 3.4719, 3.7009, 3.9298, 4.1587, 4.3876],
    plunging_dmg3: [2.0492, 2.216, 2.3828, 2.621, 2.7878, 2.9785, 3.2406, 3.5027, 3.7648, 4.0507, 4.3366, 4.6226, 4.9085, 5.1944, 5.4804],
    e_dmg1: [2.5, 2.6875, 2.875, 3.125, 3.3125, 3.5, 3.75, 4.0, 4.25, 4.5, 4.75, 5.0, 5.25, 5.5, 5.75],
    e_dmg2: [3.5, 3.7625, 4.025, 4.375, 4.6375, 4.9, 5.25, 5.6, 5.95, 6.3, 6.65, 7.0, 7.35, 7.7, 8.05],
    q_dmg1: [2.0, 2.15, 2.3, 2.5, 2.65, 2.8, 3.0, 3.2, 3.4, 3.6, 3.8, 4.0, 4.2, 4.4, 4.6],
};

#[derive(Copy, Clone, FromPrimitive, Eq, PartialEq, EnumString, EnumCountMacro)]
pub enum AinoDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Normal4,
    Charged1,
    Charged2,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    E2,
    Q1,
}

impl AinoDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use AinoDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged1 | Charged2 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst,
        }
    }

    pub fn get_element(&self) -> Element {
        use AinoDamageEnum::*;
        match *self {
            E1 | E2 | Q1 => Element::Hydro,
            _ => Element::Physical
        }
    }
}

impl Into<usize> for AinoDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

pub struct AinoEffect {
    pub has_talent2: bool,
}

impl AinoEffect {
    pub fn new(common_data: &CharacterCommonData) -> Self {
        AinoEffect {
            has_talent2: common_data.has_talent2,
        }
    }
}

impl<A: Attribute> ChangeAttribute<A> for AinoEffect {
    fn change_attribute(&self, attribute: &mut A) {
        // A4 Passive: Burst DMG +50% of EM
        if self.has_talent2 {
            let em = attribute.get_value(AttributeName::ElementalMastery);
            attribute.set_value_by(AttributeName::BonusHydro, "爱诺天赋：结构化功率提升", em * 0.5);
        }
    }
}

pub struct Aino;

impl CharacterTrait for Aino {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Aino,
        internal_name: "Aino",
        element: Element::Hydro,
        hp: [1003, 2577, 3326, 4982, 5514, 6343, 7052, 7881, 8413, 9241, 9773, 10602, 11134, 11962, 11962],
        atk: [20, 50, 65, 97, 108, 124, 138, 154, 164, 180, 191, 207, 217, 234, 234],
        def: [63, 162, 209, 313, 346, 398, 443, 495, 528, 580, 613, 665, 699, 751, 751],
        sub_stat: CharacterSubStatFamily::ATK240,
        weapon_type: WeaponType::Claymore,
        star: 4,
        skill_name1: locale!(
            zh_cn: "普通攻击·敲打修理法",
            en: "Normal Attack: Hammering Repair Method",
        ),
        skill_name2: locale!(
            zh_cn: "妙思捕手",
            en: "Musecatcher",
        ),
        skill_name3: locale!(
            zh_cn: "精密水冷仪",
            en: "Precision Hydronic Cooler",
        ),
        name_locale: locale!(
            zh_cn: "爱诺",
            en: "Aino",
        )
    };
    type SkillType = AinoSkillType;
    const SKILL: Self::SkillType = AINO_SKILL;
    type DamageEnumType = AinoDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            AinoDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 hit_n_dmg!(3)
            Normal4 hit_n_dmg!(4)
            Charged1 charged_dmg!()
            Charged2 charged_dmg!(2)
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            AinoDamageEnum
            E1 locale!(zh_cn: "一段伤害", en: "First Hit DMG")
            E2 locale!(zh_cn: "二段伤害", en: "Second Hit DMG")
        ),
        skill3: skill_map!(
            AinoDamageEnum
            Q1 locale!(zh_cn: "水弹伤害", en: "Water Bomb DMG")
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
        ItemConfig {
            name: "has_moonsign_benediction",
            title: locale!(
                zh_cn: "月兆祝福",
                en: "Moonsign Benediction"
            ),
            config: ItemConfigType::Bool { default: true },
        },
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, _config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: AinoDamageEnum = FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let ratio = match s {
            AinoDamageEnum::Normal1 => AINO_SKILL.normal_dmg1[s1],
            AinoDamageEnum::Normal2 => AINO_SKILL.normal_dmg2[s1],
            AinoDamageEnum::Normal3 => AINO_SKILL.normal_dmg3[s1],
            AinoDamageEnum::Normal4 => AINO_SKILL.normal_dmg4[s1],
            AinoDamageEnum::Charged1 => AINO_SKILL.charged_dmg1[s1],
            AinoDamageEnum::Charged2 => AINO_SKILL.charged_dmg2[s1],
            AinoDamageEnum::Plunging1 => AINO_SKILL.plunging_dmg1[s1],
            AinoDamageEnum::Plunging2 => AINO_SKILL.plunging_dmg2[s1],
            AinoDamageEnum::Plunging3 => AINO_SKILL.plunging_dmg3[s1],
            AinoDamageEnum::E1 => AINO_SKILL.e_dmg1[s2],
            AinoDamageEnum::E2 => AINO_SKILL.e_dmg2[s2],
            AinoDamageEnum::Q1 => AINO_SKILL.q_dmg1[s3],
        };

        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);
        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(),
            s.get_skill_type(),
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, _config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        Some(Box::new(AinoEffect::new(common_data)))
    }

    fn get_target_function_by_role(_role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        Box::new(AinoDefaultTargetFunction)
    }
}

pub struct AinoDefaultTargetFunction;

impl TargetFunctionMetaTrait for AinoDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::AinoDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "爱诺-默认",
            en: "Aino-Default"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "元素精通输出爱诺",
            en: "EM-focused DPS Aino"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Aino),
        image: TargetFunctionMetaImage::Avatar
    };

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, _config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(AinoDefaultTargetFunction)
    }
}

impl TargetFunction for AinoDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        TargetFunctionOptConfig {
            atk_fixed: 0.1,
            atk_percentage: 0.5,
            hp_fixed: 0.0,
            hp_percentage: 0.0,
            def_fixed: 0.0,
            def_percentage: 0.0,
            recharge: 1.2,
            elemental_mastery: 1.5,
            critical: 1.0,
            critical_damage: 0.8,
            healing_bonus: 0.0,
            bonus_physical: 0.0,
            bonus_pyro: 0.0,
            bonus_hydro: 1.0,
            bonus_anemo: 0.0,
            bonus_cryo: 0.0,
            bonus_geo: 0.0,
            bonus_dendro: 0.0,
            bonus_electro: 0.0,
            sand_main_stats: vec![
                StatName::ATKPercentage,
                StatName::Recharge,
            ],
            goblet_main_stats: vec![
                StatName::HydroBonus,
                StatName::ATKPercentage,
            ],
            head_main_stats: vec![
                StatName::CriticalRate,
                StatName::CriticalDamage,
                StatName::ATKPercentage,
            ],
            set_names: Some(vec![
                crate::artifacts::ArtifactSetName::HeartOfDepth,
                crate::artifacts::ArtifactSetName::EmblemOfSeveredFate,
                crate::artifacts::ArtifactSetName::GladiatorsFinale,
                crate::artifacts::ArtifactSetName::ShimenawasReminiscence,
            ]),
            very_critical_set_names: None,
            normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
            critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
            very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD
        }
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> crate::artifacts::effect_config::ArtifactEffectConfig {
        crate::artifacts::effect_config::ArtifactEffectConfigBuilder::new()
            .build()
    }

    fn target(&self, _attribute: &crate::attribute::SimpleAttributeGraph2, _character: &crate::character::Character<crate::attribute::SimpleAttributeGraph2>, _weapon: &crate::weapon::Weapon<crate::attribute::SimpleAttributeGraph2>, _artifacts: &[&crate::artifacts::Artifact], _enemy: &crate::enemies::Enemy) -> f64 {
        0.0
    }
}
