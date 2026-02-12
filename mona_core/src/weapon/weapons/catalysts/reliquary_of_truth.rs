use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};

struct ReliquaryOfTruthEffect {
    pub has_skill_buff: bool,
    pub has_lunar_bloom_buff: bool,
}

impl<A: Attribute> WeaponEffect<A> for ReliquaryOfTruthEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        // Static CRIT Rate: 8/10/12/14/16%
        attribute.set_value_by(
            AttributeName::CriticalBase,
            "Reliquary of Truth Passive",
            0.06 + 0.02 * refine,
        );

        let both_active = self.has_skill_buff && self.has_lunar_bloom_buff;
        let amplify = if both_active { 1.5 } else { 1.0 };

        // Elemental Skill grants EM: 80/100/120/140/160 for 12s
        if self.has_skill_buff {
            let em = (60.0 + 20.0 * refine) * amplify;
            attribute.set_value_by(
                AttributeName::ElementalMastery,
                "Reliquary of Truth Skill",
                em,
            );
        }

        // Lunar-Bloom DMG grants CRIT DMG: 24/30/36/42/48% for 4s
        if self.has_lunar_bloom_buff {
            let cd = (0.18 + 0.06 * refine) * amplify;
            attribute.set_value_by(
                AttributeName::CriticalDamageBase,
                "Reliquary of Truth Lunar-Bloom",
                cd,
            );
        }
    }
}

pub struct ReliquaryOfTruth;

impl WeaponTrait for ReliquaryOfTruth {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::ReliquaryOfTruth,
        internal_name: "Catalyst_Sistrum",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage192),
        weapon_base: WeaponBaseATKFamily::ATK542,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "暴击率提高<span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span>。施放元素战技后的12秒内，元素精通提高<span style=\"color: #409EFF;\">80-100-120-140-160</span>点。触发月华绽放伤害后的4秒内，暴击伤害提高<span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span>。两种效果同时存在时，二者的效果各提升50%。",
            en: "CRIT Rate increased by <span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span>. After using an Elemental Skill, Elemental Mastery is increased by <span style=\"color: #409EFF;\">80-100-120-140-160</span> for 12s. After Lunar-Bloom DMG is dealt, CRIT DMG is increased by <span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span> for 4s. When both effects are active simultaneously, both are amplified by 50%."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "真理的圣匣",
            en: "Reliquary of Truth"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "has_skill_buff",
            title: locale!(
                zh_cn: "元素战技增益",
                en: "Skill Buff Active"
            ),
            config: ItemConfigType::Bool { default: true },
        },
        ItemConfig {
            name: "has_lunar_bloom_buff",
            title: locale!(
                zh_cn: "月华绽放增益",
                en: "Lunar-Bloom Buff Active"
            ),
            config: ItemConfigType::Bool { default: false },
        },
    ]);

    fn get_effect<A: Attribute>(
        character: &CharacterCommonData,
        config: &WeaponConfig,
    ) -> Option<Box<dyn WeaponEffect<A>>> {
        match *config {
            WeaponConfig::ReliquaryOfTruth { spectral_stack } => {
                // Reuse existing config: spectral_stack >= 1 = skill buff, >= 2 = both buffs
                Some(Box::new(ReliquaryOfTruthEffect {
                    has_skill_buff: spectral_stack >= 1.0,
                    has_lunar_bloom_buff: spectral_stack >= 2.0,
                }))
            }
            _ => None,
        }
    }
}
