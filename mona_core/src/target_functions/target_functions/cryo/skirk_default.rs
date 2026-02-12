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
    pub death_stacks: usize,     // 死河渡断层数 (0-3)
    pub void_realm_cracks: usize, // 虚境裂隙数量 (0-3)
    pub extinction_active: bool,  // 极恶技·尽 凋尽状态
    pub has_all_hydro_cryo_team: bool, // A3: 全水/冰队
}

impl SkirkDefaultTargetFunction {
    pub fn new(config: &TargetFunctionConfig) -> Self {
        match *config {
            TargetFunctionConfig::SkirkDefault {
                death_stacks,
                void_realm_cracks,
                extinction_active,
                has_all_hydro_cryo_team,
                ..
            } => Self {
                death_stacks,
                void_realm_cracks,
                extinction_active,
                has_all_hydro_cryo_team,
            },
            _ => Self {
                death_stacks: 3,
                void_realm_cracks: 3,
                extinction_active: true,
                has_all_hydro_cryo_team: true,
            }
        }
    }
}

impl TargetFunctionMetaTrait for SkirkDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::SkirkDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "丝柯克-平A流",
            en: "Skirk-Normal Attack DPS"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "点按E进入七相模式，以普攻/重击为主输出，配合凋尽状态加成",
            en: "Tap E into Seven-Phase, main DPS from normal/charged attacks with Extinction buff"
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
            name: "void_realm_cracks",
            title: locale!(
                zh_cn: "虚境裂隙数量",
                en: "Void Realm Cracks"
            ),
            config: ItemConfigType::Int { min: 0, max: 3, default: 3 }
        },
        ItemConfig {
            name: "extinction_active",
            title: locale!(
                zh_cn: "极恶技·尽 凋尽状态",
                en: "Havoc: Extinction Active"
            ),
            config: ItemConfigType::Bool { default: true }
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

        // 平A流配置: 点按E → 七相模式普攻/重击
        let s_config = CharacterSkillConfig::Skirk {
            in_seven_phase: true,  // 平A流始终在七相模式
            death_stacks: self.death_stacks,
            serpent_points: 0.0,  // 平A流不使用极恶技·灭, SS无影响
            void_realm_cracks: self.void_realm_cracks,
            extinction_active: self.extinction_active,
            has_all_hydro_cryo_team: self.has_all_hydro_cryo_team,
        };

        type S = <Skirk as CharacterTrait>::DamageEnumType;

        // 平A流循环: 5NQ + (5NZ)×3
        // Q = 极恶技·尽 (无伤害, 仅激活凋尽状态buff普攻, 已通过extinction_active配置体现)
        // = 4×(N1+N2+N3_1+N3_2+N4_1+N4_2+N5) + 3×Charged

        // 普通攻击链 (七相模式: N1-N5, 含N3双段/N4双段)
        let dmg_n1 = Skirk::damage::<SimpleDamageBuilder>(&context, S::Normal1, &s_config, None).normal.expectation;
        let dmg_n2 = Skirk::damage::<SimpleDamageBuilder>(&context, S::Normal2, &s_config, None).normal.expectation;
        let dmg_n3_1 = Skirk::damage::<SimpleDamageBuilder>(&context, S::Normal3_1, &s_config, None).normal.expectation;
        let dmg_n3_2 = Skirk::damage::<SimpleDamageBuilder>(&context, S::Normal3_2, &s_config, None).normal.expectation;
        let dmg_n4_1 = Skirk::damage::<SimpleDamageBuilder>(&context, S::Normal4_1, &s_config, None).normal.expectation;
        let dmg_n4_2 = Skirk::damage::<SimpleDamageBuilder>(&context, S::Normal4_2, &s_config, None).normal.expectation;
        let dmg_n5 = Skirk::damage::<SimpleDamageBuilder>(&context, S::Normal5, &s_config, None).normal.expectation;
        let dmg_charged = Skirk::damage::<SimpleDamageBuilder>(&context, S::Charged, &s_config, None).normal.expectation;

        // 一条完整普攻链伤害
        let normal_chain = dmg_n1 + dmg_n2 + dmg_n3_1 + dmg_n3_2 + dmg_n4_1 + dmg_n4_2 + dmg_n5;

        // 总循环伤害: 4条普攻链 + 3次重击 (极恶技·尽无伤害)
        let total_dmg = normal_chain * 4.0
                       + dmg_charged * 3.0;

        total_dmg
    }
}
