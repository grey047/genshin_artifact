use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder};
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::electro::flins::Flins;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::StatName;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::damage::damage_result::SimpleDamageResult;
use crate::enemies::Enemy;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct FlinsDefaultTargetFunction {
    pub recharge_requirement: f64,
    pub manifest_flame: bool,           // 是否在幽焰显迹模式
    pub moonsign_level: usize,          // 月兆等级 (1或2)
    pub lunar_charged_rate: f64,       // 月感电反应伤害占比
    pub normal_attack_rate: f64,        // 普通攻击占比（在E技能期间）
    pub burst_rate: f64,               // 大招占比
}

impl FlinsDefaultTargetFunction {
    pub fn new(config: &TargetFunctionConfig) -> Self {
        match *config {
            TargetFunctionConfig::FlinsDefault {
                recharge_requirement,
                manifest_flame,
                moonsign_level,
                lunar_charged_rate,
                normal_attack_rate,
                burst_rate,
            } => Self {
                recharge_requirement,
                manifest_flame,
                moonsign_level,
                lunar_charged_rate,
                normal_attack_rate,
                burst_rate,
            },
            _ => Self {
                recharge_requirement: 1.2,
                manifest_flame: true,
                moonsign_level: 2,
                lunar_charged_rate: 0.4,
                normal_attack_rate: 0.3,
                burst_rate: 0.3,
            }
        }
    }
}

impl TargetFunctionMetaTrait for FlinsDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::FlinsDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "菲林斯-永岩之诗",
            en: "Flins-Eternal Rock Poem"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "基于E技能期间普攻和Q技能伤害计算，月感电反应可暴击",
            en: "Calculates damage based on Normal Attacks during E skill and Q burst damage, Lunar-Charged can crit"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Flins),
        image: TargetFunctionMetaImage::Avatar,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "recharge_requirement",
            title: ItemConfig::DEFAULT_RECHARGE_TITLE,
            config: ItemConfigType::Float { min: 1.0, max: 3.0, default: 1.2 },
        },
        ItemConfig {
            name: "manifest_flame",
            title: locale!(
                zh_cn: "幽焰显迹模式",
                en: "Manifest Flame Mode",
            ),
            config: ItemConfigType::Bool { default: true },
        },
        ItemConfig {
            name: "moonsign_level",
            title: locale!(
                zh_cn: "月兆等级",
                en: "Moonsign Level",
            ),
            config: ItemConfigType::Int { min: 1, max: 2, default: 2 },
        },
        ItemConfig {
            name: "lunar_charged_rate",
            title: locale!(
                zh_cn: "月感电反应伤害占比",
                en: "Lunar-Charged Reaction DMG Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.4 },
        },
        ItemConfig {
            name: "normal_attack_rate",
            title: locale!(
                zh_cn: "普通攻击占比(E期间)",
                en: "Normal Attack Ratio (during E)",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.3 },
        },
        ItemConfig {
            name: "burst_rate",
            title: locale!(
                zh_cn: "大招伤害占比",
                en: "Burst DMG Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.3 },
        },
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(FlinsDefaultTargetFunction::new(config))
    }
}

impl TargetFunction for FlinsDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        TargetFunctionOptConfig {
            atk_fixed: 0.1,
            atk_percentage: 1.0,
            hp_fixed: 0.0,
            hp_percentage: 0.0,
            def_fixed: 0.0,
            def_percentage: 0.0,
            recharge: 0.1,
            elemental_mastery: 0.5,
            critical: 1.0,
            critical_damage: 1.0,
            healing_bonus: 0.0,
            bonus_electro: 1.0,
            bonus_pyro: 0.0,
            bonus_hydro: 0.0,
            bonus_anemo: 0.0,
            bonus_cryo: 0.0,
            bonus_geo: 0.0,
            bonus_dendro: 0.0,
            bonus_physical: 0.0,
            sand_main_stats: vec![
                StatName::ATKPercentage,
                StatName::ElementalMastery,
                StatName::Recharge,
            ],
            goblet_main_stats: vec![
                StatName::ElectroBonus,
                StatName::ATKPercentage,
                StatName::ElementalMastery,
            ],
            head_main_stats: vec![
                StatName::CriticalRate,
                StatName::CriticalDamage,
                StatName::ATKPercentage,
            ],
            set_names: Some(vec![
                ArtifactSetName::GildedDreams,
                ArtifactSetName::ThunderingFury,
                ArtifactSetName::GladiatorsFinale,
                ArtifactSetName::ShimenawasReminiscence,
                ArtifactSetName::EchoesOfAnOffering,
                ArtifactSetName::TenacityOfTheMillelith,
            ]),
            very_critical_set_names: None,
            normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
            critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
            very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD
        }
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfigBuilder::new()
            .shimenawas_reminiscence(0.35)
            .thundersoother(0.4)
            .echoes_of_an_offering_avg()
            .gilded_dreams(1, 2, 1.0)
            .tenacity_of_the_millelith(0.3)
            .build()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute,
            enemy,
        };

        type S = <Flins as CharacterTrait>::DamageEnumType;

        // E技能期间的普攻伤害
        let config_e = CharacterSkillConfig::Flins {
            in_manifest_flame: self.manifest_flame,
            moonsign_level: self.moonsign_level,
        };

        let dmg_e1 = Flins::damage::<SimpleDamageBuilder>(&context, S::E1, &config_e, None);
        let dmg_e2 = Flins::damage::<SimpleDamageBuilder>(&context, S::E2, &config_e, None);
        let dmg_e3 = Flins::damage::<SimpleDamageBuilder>(&context, S::E3, &config_e, None);
        let dmg_e5 = Flins::damage::<SimpleDamageBuilder>(&context, S::E5, &config_e, None);
        let dmg_e_charged = Flins::damage::<SimpleDamageBuilder>(&context, S::ECharged, &config_e, None);

        // E技能期间的一套普攻伤害 (假设4-5段连招)
        let normal_combo_dmg = dmg_e1.normal.expectation + dmg_e2.normal.expectation + dmg_e3.normal.expectation + dmg_e5.normal.expectation * 2.0;

        // 大招伤害 (Lunar-Charged)
        let dmg_q_initial = Flins::damage::<SimpleDamageBuilder>(&context, S::QInitial, &config_e, None);
        let dmg_q_middle = Flins::damage::<SimpleDamageBuilder>(&context, S::QMiddle_1, &config_e, None);
        let dmg_q_final = Flins::damage::<SimpleDamageBuilder>(&context, S::QFinal, &config_e, None);

        let burst_dmg = dmg_q_initial.normal.expectation + dmg_q_middle.normal.expectation * 2.0 + dmg_q_final.normal.expectation;

        // 月感电反应伤害 (Lunar-Charged)
        // 注意：Lunar-Charged 可以暴击，所以使用 normal expectation
        let lunar_charged_dmg = burst_dmg * self.lunar_charged_rate;

        // 充能需求
        let recharge = attribute.get_value(AttributeName::Recharge);
        let recharge_ratio = recharge.min(self.recharge_requirement);

        // 总伤害
        let total_dmg = normal_combo_dmg * self.normal_attack_rate + lunar_charged_dmg + burst_dmg * self.burst_rate;

        recharge_ratio * total_dmg
    }
}
