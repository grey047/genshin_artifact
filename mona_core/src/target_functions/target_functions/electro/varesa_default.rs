use crate::artifacts::Artifact;
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::SimpleAttributeGraph2;
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Varesa;
use crate::character::prelude::CharacterTrait;
use crate::character::skill_config::CharacterSkillConfig;
use crate::common::i18n::locale;
use crate::common::StatName;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct VaresaDefaultTargetFunction;

impl TargetFunction for VaresaDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        TargetFunctionOptConfig {
            atk_fixed: 0.0,
            atk_percentage: 1.0,
            hp_fixed: 0.0,
            hp_percentage: 0.0,
            def_fixed: 0.0,
            def_percentage: 0.0,
            recharge: 0.5,
            elemental_mastery: 0.3,
            critical: 1.0,
            critical_damage: 1.0,
            healing_bonus: 0.0,
            bonus_electro: 1.0,
            bonus_pyro: 0.0,
            bonus_hydro: 0.0,
            bonus_cryo: 0.0,
            bonus_anemo: 0.0,
            bonus_geo: 0.0,
            bonus_dendro: 0.0,
            bonus_physical: 0.0,
            sand_main_stats: vec![
                StatName::ATKPercentage,
                StatName::ElectroBonus,
                StatName::Recharge,
            ],
            goblet_main_stats: vec![
                StatName::ElectroBonus,
                StatName::ATKPercentage,
            ],
            head_main_stats: vec![
                StatName::CriticalRate,
                StatName::CriticalDamage,
                StatName::ATKPercentage,
            ],
            set_names: Some(vec![
                ArtifactSetName::ThunderingFury,
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

    fn get_default_artifact_config(&self, team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, weapon: &Weapon<SimpleAttributeGraph2>, artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute,
            enemy
        };

        type S = <Varesa as CharacterTrait>::DamageEnumType;
        let dmg = Varesa::damage::<SimpleDamageBuilder>(
            &context, S::PassionPlunging3, &CharacterSkillConfig::NoConfig, None
        );

        dmg.normal.expectation
    }
}

impl TargetFunctionMetaTrait for VaresaDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::VaresaDefault,
        name_locale: locale!(
            zh_cn: "瓦雷莎-悠暇豪劲",
            en: "Varesa-Strength in Serenity"
        ),
        description: locale!(
            zh_cn: "最大化炽热激情状态伤害",
            en: "Maximize Fiery Passion DMG"
        ),
        tags: "",
        four: TargetFunctionFor::SomeWho(CharacterName::Varesa),
        image: TargetFunctionMetaImage::Avatar,
    };

    fn create(character: &CharacterCommonData, weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(VaresaDefaultTargetFunction)
    }
}
