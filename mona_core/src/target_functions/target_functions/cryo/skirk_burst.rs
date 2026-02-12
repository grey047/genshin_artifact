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

pub struct SkirkBurstTargetFunction {
    pub death_stacks: usize,     // 死河渡断层数 (0-3)
    pub serpent_points: f64,     // 蛇之狡谋点数
    pub has_all_hydro_cryo_team: bool, // A3: 全水/冰队
}

impl SkirkBurstTargetFunction {
    pub fn new(config: &TargetFunctionConfig) -> Self {
        match *config {
            TargetFunctionConfig::SkirkBurst {
                death_stacks,
                serpent_points,
                has_all_hydro_cryo_team,
            } => Self {
                death_stacks,
                serpent_points,
                has_all_hydro_cryo_team,
            },
            _ => Self {
                death_stacks: 3,
                serpent_points: 62.0,
                has_all_hydro_cryo_team: true,
            }
        }
    }
}

impl TargetFunctionMetaTrait for SkirkBurstTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::SkirkBurst,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "丝柯克-爆发流",
            en: "Skirk-Burst DPS"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "长按E进入七相模式后直接Q，以大招斩击/终段为主输出，不打普攻",
            en: "Hold E into Seven-Phase then Burst, main DPS from Q slash/final hit, no normal attacks"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Skirk),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "death_stacks",
            title: locale!(
                zh_cn: "死河渡断层数",
                en: "Death's Crossing Stacks"
            ),
            config: ItemConfigType::Int { min: 0, max: 3, default: 3 }
        },
        ItemConfig {
            name: "serpent_points",
            title: locale!(
                zh_cn: "蛇之狡谋点数",
                en: "Serpent's Subtlety Points"
            ),
            config: ItemConfigType::Int { min: 0, max: 100, default: 62 }
        },
        ItemConfig {
            name: "has_all_hydro_cryo_team",
            title: locale!(
                zh_cn: "A3: 队伍全水/冰 (E+1)",
                en: "A3: All Hydro/Cryo Team (E+1)"
            ),
            config: ItemConfigType::Bool { default: true }
        },
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(SkirkBurstTargetFunction::new(config))
    }
}

impl TargetFunction for SkirkBurstTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        TargetFunctionOptConfig {
            atk_fixed: 0.0,
            atk_percentage: 1.0,
            hp_fixed: 0.0,
            hp_percentage: 0.0,
            def_fixed: 0.0,
            def_percentage: 0.0,
            recharge: 0.0,           // Skirk 无能量系统
            elemental_mastery: 0.0,  // 不吃元素反应
            critical: 1.0,
            critical_damage: 1.0,
            healing_bonus: 0.0,
            bonus_cryo: 2.0,        // 爆发流冰伤权重更高 (全部输出为冰)
            bonus_pyro: 0.0,
            bonus_hydro: 0.0,
            bonus_anemo: 0.0,
            bonus_geo: 0.0,
            bonus_dendro: 0.0,
            bonus_physical: 0.0,
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
            ],
            set_names: Some(vec![
                ArtifactSetName::BlizzardStrayer,
                ArtifactSetName::GladiatorsFinale,
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
            .build()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], _enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute,
            enemy: _enemy,
        };

        // 爆发流配置: 长按E → Q, 不打普攻
        let s_config = CharacterSkillConfig::Skirk {
            in_seven_phase: true,  // 长按E也进入七相模式
            death_stacks: self.death_stacks,
            serpent_points: self.serpent_points,
            void_realm_cracks: 0,       // 爆发流不依赖裂隙 (不打普攻)
            extinction_active: false,    // 不打普攻, 凋尽无意义
            has_all_hydro_cryo_team: self.has_all_hydro_cryo_team,
        };

        type S = <Skirk as CharacterTrait>::DamageEnumType;

        // 大招斩击 (×5) + 最终段 (×1) = 总爆发伤害
        let dmg_slash = Skirk::damage::<SimpleDamageBuilder>(&context, S::Q1, &s_config, None).normal.expectation;
        let dmg_final = Skirk::damage::<SimpleDamageBuilder>(&context, S::Q2, &s_config, None).normal.expectation;

        let total_dmg = dmg_slash * 5.0 + dmg_final;

        total_dmg
    }
}
