use crate::artifacts::Artifact;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Durin;
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

pub struct DurinDefaultTargetFunction {
    pub recharge_demand: f64,
    pub use_skill: f64,
    pub use_burst: f64,
    pub transmutation_state: i32,
}

impl TargetFunctionMetaTrait for DurinDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::DurinDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "杜林-默认",
            en: "Durin-Default"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "普通输出杜林",
            en: "DPS Durin"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Durin),
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
            config: ItemConfigType::Float { min: 1.0, max: 3.0, default: 1.3 }
        },
        ItemConfig {
            name: "use_skill",
            title: crate::common::i18n::locale!(
                zh_cn: "技能伤害权重",
                en: "Skill Damage Weight",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.2 }
        },
        ItemConfig {
            name: "use_burst",
            title: crate::common::i18n::locale!(
                zh_cn: "爆发伤害权重",
                en: "Burst Damage Weight",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.8 }
        },
        ItemConfig {
            name: "transmutation_state",
            title: crate::common::i18n::locale!(
                zh_cn: "蜕变状态",
                en: "Transmutation State",
            ),
            config: ItemConfigType::Int { min: 0, max: 1, default: 1 }
        },
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let (recharge_demand, use_skill, use_burst, transmutation_state) = match *config {
            TargetFunctionConfig::DurinDefault { recharge_demand, use_skill, use_burst, transmutation_state } => (recharge_demand, use_skill, use_burst, transmutation_state),
            _ => (1.3, 0.2, 0.8, 1)
        };
        Box::new(DurinDefaultTargetFunction {
            recharge_demand,
            use_skill,
            use_burst,
            transmutation_state,
        })
    }
}

impl TargetFunction for DurinDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unimplemented!()
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute,
            enemy
        };

        let config = CharacterSkillConfig::Durin { transmutation_state: self.transmutation_state as usize };
        type S = <Durin as CharacterTrait>::DamageEnumType;

        let skill_dmg = if self.transmutation_state == 1 {
            Durin::damage::<SimpleDamageBuilder>(
                &context, S::EDarkness1, &config, None
            ).normal.expectation
        } else {
            Durin::damage::<SimpleDamageBuilder>(
                &context, S::EPurity, &config, None
            ).normal.expectation
        };

        let burst_dmg = if self.transmutation_state == 1 {
            Durin::damage::<SimpleDamageBuilder>(
                &context, S::QDragonDark, &config, None
            ).normal.expectation
        } else {
            Durin::damage::<SimpleDamageBuilder>(
                &context, S::QDragonWhite, &config, None
            ).normal.expectation
        };

        let recharge = attribute.get_value(AttributeName::Recharge);
        let recharge_ratio = recharge.min(self.recharge_demand);

        recharge_ratio * (skill_dmg * self.use_skill + burst_dmg * self.use_burst)
    }
}
