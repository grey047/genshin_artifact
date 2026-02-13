use crate::artifacts::Artifact;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::hydro::Dahlia;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::SkillType;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct DahliaDefaultTargetFunction {
    pub recharge_demand: f64,
    pub use_skill: f64,
    pub use_burst: f64,
}

impl TargetFunctionMetaTrait for DahliaDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::DahliaDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "达利亚-默认",
            en: "Dahlia-Default"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "达利亚-燃烧辅助/副C",
            en: "Dahlia - Burn Support/Sub-DPS"
        ),
        tags: "输出,辅助",
        four: TargetFunctionFor::SomeWho(CharacterName::Dahlia),
        image: TargetFunctionMetaImage::Avatar,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "recharge_demand",
            title: crate::common::i18n::locale!(
                zh_cn: "充能需求",
                en: "Recharge Requirement",
            ),
            config: ItemConfigType::Float { min: 1.0, max: 3.0, default: 1.0 }
        },
        ItemConfig {
            name: "use_skill",
            title: crate::common::i18n::locale!(
                zh_cn: "技能伤害权重",
                en: "Skill Damage Weight",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.4 }
        },
        ItemConfig {
            name: "use_burst",
            title: crate::common::i18n::locale!(
                zh_cn: "爆发伤害权重",
                en: "Burst Damage Weight",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.6 }
        },
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let (recharge_demand, use_skill, use_burst) = match *config {
            TargetFunctionConfig::DahliaDefault { recharge_demand, use_skill, use_burst } => (recharge_demand, use_skill, use_burst),
            _ => (1.0, 0.4, 0.6)
        };
        Box::new(DahliaDefaultTargetFunction {
            recharge_demand,
            use_skill,
            use_burst,
        })
    }
}

impl TargetFunction for DahliaDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unimplemented!()
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        // BiS Artifacts for Dahlia (Burn Support/Sub-DPS):
        // - SilkenMoonsSerenade
        // - AubadeOfMorningstarAndMoon
        // - CrimsonWitchOfFlames
        Default::default()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute,
            enemy
        };

        let config = CharacterSkillConfig::Dahlia { is_burning: true };
        type S = <Dahlia as CharacterTrait>::DamageEnumType;

        // Skill damage (Scarlet Sanctuary + Crimson Blossom)
        let skill_dmg = Dahlia::damage::<SimpleDamageBuilder>(
            &context, S::ESanctuary, &config, None
        ).normal.expectation;

        // Burst damage (Crimson Exequy)
        let burst_dmg = Dahlia::damage::<SimpleDamageBuilder>(
            &context, S::QExequy, &config, None
        ).normal.expectation;

        let recharge = attribute.get_value(AttributeName::Recharge);
        let recharge_ratio = recharge.min(self.recharge_demand);

        recharge_ratio * (skill_dmg * self.use_skill + burst_dmg * self.use_burst)
    }
}
