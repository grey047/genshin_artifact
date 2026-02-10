use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder, ConfigRate};
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::cryo::skirk::Skirk;
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

pub struct SkirkDefaultTargetFunction {
    pub recharge_demand: f64,
    pub use_seven_phase: bool,
    pub use_charged_attack: f64,  // 重击使用比例
    pub use_burst: f64,          // 大招使用比例
}

impl SkirkDefaultTargetFunction {
    pub fn new(config: &TargetFunctionConfig) -> Self {
        match *config {
            TargetFunctionConfig::SkirkDefault {
                recharge_demand,
                use_seven_phase,
                use_charged_attack,
                use_burst,
            } => Self {
                recharge_demand,
                use_seven_phase,
                use_charged_attack,
                use_burst,
            },
            _ => Self {
                recharge_demand: 1.0,
                use_seven_phase: true,
                use_charged_attack: 0.3,
                use_burst: 0.3,
            }
        }
    }
}

impl TargetFunctionMetaTrait for SkirkDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::SkirkDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "丝柯克-七相斩",
            en: "Skirk-Seven-Phase Slash"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "冰系主C，依赖七相模式和普通攻击/重击循环",
            en: "Cryo main DPS, relies on Seven-Phase mode and normal/charged attack combos"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Skirk),
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
            name: "use_seven_phase",
            title: locale!(
                zh_cn: "使用七相模式",
                en: "Use Seven-Phase Mode"
            ),
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "use_charged_attack",
            title: locale!(
                zh_cn: "重击使用比例",
                en: "Charged Attack Ratio"
            ),
            config: ItemConfigType::Float { default: 0.3, min: 0.0, max: 1.0 }
        },
        ItemConfig {
            name: "use_burst",
            title: locale!(
                zh_cn: "大招使用比例",
                en: "Burst Usage Ratio"
            ),
            config: ItemConfigType::Float { default: 0.3, min: 0.0, max: 1.0 }
        },
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(SkirkDefaultTargetFunction::new(config))
    }
}

impl TargetFunction for SkirkDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        TargetFunctionOptConfig {
            atk_fixed: 0.0,       // 固定攻击只有 ~36% 价值（17固 vs 5% ≈ 48攻）
            atk_percentage: 1.0,   // 标准权重
            hp_fixed: 0.0,
            hp_percentage: 0.0,
            def_fixed: 0.0,
            def_percentage: 0.0,
            recharge: 0.0,        // Skirk 不需要充能（无能量系统）
            elemental_mastery: 0.0,  // 不吃元素反应
            critical: 1.0,        // 标准权重
            critical_damage: 1.0, // 标准权重
            healing_bonus: 0.0,
            bonus_cryo: 1.5,      // 冰伤最重要（冰套提供暴击）
            bonus_pyro: 0.0,
            bonus_hydro: 0.0,
            bonus_anemo: 0.0,
            bonus_geo: 0.0,
            bonus_dendro: 0.0,
            bonus_physical: 0.0,  // 不打物理
            bonus_electro: 0.0,
            sand_main_stats: vec![
                StatName::ATKPercentage,
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
                ArtifactSetName::BlizzardStrayer,
                ArtifactSetName::GladiatorsFinale,
                ArtifactSetName::ShimenawasReminiscence,
            ]),
            very_critical_set_names: None,
            normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
            critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
            very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD
        }
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfigBuilder::new()
            .blizzard_strayer(0.3)
            .shimenawas_reminiscence(0.3)
            .build()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], _enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute,
            enemy: _enemy,
        };

        // 七相模式配置
        let s_config = CharacterSkillConfig::Skirk {
            in_seven_phase: self.use_seven_phase,
            death_stacks: 2,
            serpent_points: 50.0,
            c2_active: false,
            void_realm_active: false,
            has_hydro_cryo_team: false,
            extinction_hit_count: 5,
        };

        type S = <Skirk as CharacterTrait>::DamageEnumType;

        // 普通攻击伤害 (以 Normal5 为代表)
        let dmg_normal = Skirk::damage::<SimpleDamageBuilder>(&context, S::Normal5, &s_config, None).normal.expectation;
        
        // 重击伤害
        let dmg_charged = Skirk::damage::<SimpleDamageBuilder>(&context, S::Charged, &s_config, None).normal.expectation;

        // 大招伤害
        let dmg_burst = Skirk::damage::<SimpleDamageBuilder>(&context, S::Q2, &s_config, None).normal.expectation;

        // 总伤害 = 各部分伤害 * 使用比例
        let total_dmg = dmg_normal * (1.0 - self.use_charged_attack - self.use_burst)
                       + dmg_charged * self.use_charged_attack
                       + dmg_burst * self.use_burst;

        total_dmg
    }
}
