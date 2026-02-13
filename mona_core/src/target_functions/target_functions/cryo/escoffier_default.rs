use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder};
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::cryo::escoffier::Escoffier;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::StatName;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct EscoffierDefaultTargetFunction {
    pub recharge_requirement: f64,
    pub res_shred_active: bool,
    pub skill_damage_weight: f64,
    pub burst_damage_weight: f64,
}

impl EscoffierDefaultTargetFunction {
    pub fn new(config: &TargetFunctionConfig) -> Self {
        match *config {
            TargetFunctionConfig::EscoffierDefault {
                recharge_requirement,
                res_shred_active,
                skill_damage_weight,
                burst_damage_weight,
            } => Self {
                recharge_requirement,
                res_shred_active,
                skill_damage_weight,
                burst_damage_weight,
            },
            _ => Self {
                recharge_requirement: 1.3,
                res_shred_active: true,
                skill_damage_weight: 0.85,
                burst_damage_weight: 0.15,
            }
        }
    }
}

impl TargetFunctionMetaTrait for EscoffierDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::EscoffierDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "埃斯科菲耶-盛宴之礼",
            en: "Escoffier-Feast of Glory"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "冰系后台副C，兼顾减抗和治疗，主要依赖E技能伤害",
            en: "Cryo off-field DPS with RES Shred and healing, primarily skill-based damage"
        ),
        tags: "输出,辅助",
        four: TargetFunctionFor::SomeWho(CharacterName::Escoffier),
        image: TargetFunctionMetaImage::Avatar,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "recharge_requirement",
            title: ItemConfig::DEFAULT_RECHARGE_TITLE,
            config: ItemConfigType::Float { min: 1.0, max: 3.0, default: 1.3 },
        },
        ItemConfig {
            name: "res_shred_active",
            title: locale!(
                zh_cn: "减抗生效",
                en: "RES Shred Active",
            ),
            config: ItemConfigType::Bool { default: true },
        },
        ItemConfig {
            name: "skill_damage_weight",
            title: locale!(
                zh_cn: "技能伤害权重",
                en: "Skill Damage Weight",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.85 },
        },
        ItemConfig {
            name: "burst_damage_weight",
            title: locale!(
                zh_cn: "爆发伤害权重",
                en: "Burst Damage Weight",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.15 },
        },
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(EscoffierDefaultTargetFunction::new(config))
    }
}

impl TargetFunction for EscoffierDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        TargetFunctionOptConfig {
            atk_fixed: 0.1,
            atk_percentage: 1.0,
            hp_fixed: 0.0,
            hp_percentage: 0.0,
            def_fixed: 0.0,
            def_percentage: 0.0,
            recharge: 0.15,
            elemental_mastery: 0.0,
            critical: 1.0,
            critical_damage: 1.0,
            healing_bonus: 0.0,
            bonus_cryo: 1.0,
            bonus_pyro: 0.0,
            bonus_hydro: 0.0,
            bonus_anemo: 0.0,
            bonus_geo: 0.0,
            bonus_dendro: 0.0,
            bonus_physical: 0.0,
            bonus_electro: 0.0,
            sand_main_stats: vec![
                StatName::ATKPercentage,
                StatName::Recharge,
            ],
            goblet_main_stats: vec![
                StatName::CryoBonus,
                StatName::ATKPercentage,
            ],
            head_main_stats: vec![
                StatName::CriticalRate,
                StatName::CriticalDamage,
                StatName::ATKPercentage,
            ],
            set_names: Some(vec![
                ArtifactSetName::GoldenTroupe,
                ArtifactSetName::BlizzardStrayer,
                ArtifactSetName::TenacityOfTheMillelith,
                ArtifactSetName::NoblesseOblige,
            ]),
            very_critical_set_names: None,
            normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
            critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
            very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD
        }
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfigBuilder::new()
            .golden_troupe(1.0)
            .blizzard_strayer(0.4)
            .tenacity_of_the_millelith(0.3)
            .noblesse_oblige(0.3)
            .build()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], _enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute,
            enemy: _enemy,
        };

        type S = <Escoffier as CharacterTrait>::DamageEnumType;

        // E技能伤害 (主要伤害来源)
        let dmg_skill = Escoffier::damage::<SimpleDamageBuilder>(&context, S::E1, &CharacterSkillConfig::NoConfig, None);
        
        // Q技能伤害 (次要)
        let dmg_burst = Escoffier::damage::<SimpleDamageBuilder>(&context, S::Q1, &CharacterSkillConfig::NoConfig, None);

        // 充能需求
        let recharge = attribute.get_value(AttributeName::Recharge);
        let recharge_ratio = recharge.min(self.recharge_requirement);

        // 总伤害
        let total_dmg = dmg_skill.normal.expectation * self.skill_damage_weight 
                     + dmg_burst.normal.expectation * self.burst_damage_weight;

        recharge_ratio * total_dmg
    }
}
