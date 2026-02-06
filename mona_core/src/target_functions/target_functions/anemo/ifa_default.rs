use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder, ConfigRate};
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::anemo::ifa::Ifa;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::{Element, StatName};
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct IfaDefaultTargetFunction {
    pub recharge_demand: f64,
    pub use_skill: f64,
    pub use_burst: f64,
}

impl IfaDefaultTargetFunction {
    pub fn new(config: &TargetFunctionConfig) -> Self {
        match *config {
            TargetFunctionConfig::IfaDefault {
                recharge_demand,
                use_skill,
                use_burst,
            } => Self {
                recharge_demand,
                use_skill,
                use_burst,
            },
            _ => Self {
                recharge_demand: 1.0,
                use_skill: 0.5,
                use_burst: 0.5,
            }
        }
    }
}

impl TargetFunctionMetaTrait for IfaDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::IfaDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "Ifa-默认",
            en: "Ifa-Default"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "风系输出角色",
            en: "Anemo DPS character"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Ifa),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "recharge_demand",
            title: locale!(
                zh_cn: "充能需求",
                en: "Recharge Requirement"
            ),
            config: ItemConfigType::Float { default: 1.0, min: 1.0, max: 3.0 }
        },
        ItemConfig {
            name: "use_skill",
            title: locale!(
                zh_cn: "E技能使用比例",
                en: "Skill Usage Ratio"
            ),
            config: ItemConfigType::Float { default: 0.5, min: 0.0, max: 1.0 }
        },
        ItemConfig {
            name: "use_burst",
            title: locale!(
                zh_cn: "大招使用比例",
                en: "Burst Usage Ratio"
            ),
            config: ItemConfigType::Float { default: 0.5, min: 0.0, max: 1.0 }
        },
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(IfaDefaultTargetFunction::new(config))
    }
}

impl TargetFunction for IfaDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        TargetFunctionOptConfig {
            atk_fixed: 0.1,
            atk_percentage: 1.0,
            hp_fixed: 0.0,
            hp_percentage: 0.0,
            def_fixed: 0.0,
            def_percentage: 0.0,
            recharge: 0.3,
            elemental_mastery: 0.0,
            critical: 1.0,
            critical_damage: 1.0,
            healing_bonus: 0.0,
            bonus_anemo: 1.0,
            bonus_pyro: 0.0,
            bonus_hydro: 0.0,
            bonus_cryo: 0.0,
            bonus_geo: 0.0,
            bonus_dendro: 0.0,
            bonus_physical: 0.0,
            bonus_electro: 0.0,
            sand_main_stats: vec![
                StatName::ATKPercentage,
                StatName::AnemoBonus,
                StatName::Recharge,
            ],
            goblet_main_stats: vec![
                StatName::AnemoBonus,
                StatName::ATKPercentage,
            ],
            head_main_stats: vec![
                StatName::CriticalRate,
                StatName::CriticalDamage,
                StatName::ATKPercentage,
            ],
            set_names: Some(vec![
                ArtifactSetName::ViridescentVenerer,
                ArtifactSetName::GladiatorsFinale,
                ArtifactSetName::ShimenawasReminiscence,
                ArtifactSetName::EmblemOfSeveredFate,
            ]),
            very_critical_set_names: None,
            normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
            critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
            very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD
        }
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], _enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute,
            enemy: _enemy,
        };

        let s_config = CharacterSkillConfig::NoConfig;

        type S = <Ifa as CharacterTrait>::DamageEnumType;

        let dmg_normal = Ifa::damage::<SimpleDamageBuilder>(&context, S::Normal1, &s_config, None).normal.expectation;
        let dmg_skill = Ifa::damage::<SimpleDamageBuilder>(&context, S::E1, &s_config, None).normal.expectation;
        let dmg_burst = Ifa::damage::<SimpleDamageBuilder>(&context, S::Q1, &s_config, None).normal.expectation;

        let recharge = attribute.get_value(AttributeName::Recharge);
        let r = recharge.min(self.recharge_demand);

        let total_dmg = (dmg_normal * (1.0 - self.use_skill * 0.5) 
                       + dmg_skill * self.use_skill 
                       + dmg_burst * self.use_burst) * r;

        total_dmg
    }
}
