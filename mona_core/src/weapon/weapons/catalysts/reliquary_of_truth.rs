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
    pub spectral_stack: f64,
}

impl<A: Attribute> WeaponEffect<A> for ReliquaryOfTruthEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        attribute.add_hp_percentage("Reliquary of Truth Passive", 0.16 + 0.04 * refine);
        attribute.set_value_by(
            AttributeName::SpeedNormalAttack,
            "Reliquary of Truth Passive",
            0.10 + 0.02 * refine,
        );
        attribute.set_value_by(
            AttributeName::BonusNormalAttack,
            "Reliquary of Truth Passive",
            0.06 * self.spectral_stack,
        );
        attribute.set_value_by(
            AttributeName::BonusChargedAttack,
            "Reliquary of Truth Passive",
            0.06 * self.spectral_stack,
        );
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
            zh_cn: "生命值上限提高<span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>。普通攻击速度提升<span style=\"color: #409EFF;\">10%-12%-14%-16%-18%</span>，此外，「光谱」每层使造成的伤害提升<span style=\"color: #409EFF;\">6%</span>。",
            en: "Max HP increased by <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>. Normal Attack SPD increased by <span style=\"color: #409EFF;\">10%-12%-14%-16%-18%</span>. Additionally, each stack of 'Spectrum' increases DMG by <span style=\"color: #409EFF;\">6%</span>."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "真理的圣匣",
            en: "Reliquary of Truth"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[ItemConfig {
        name: "spectral_stack",
        title: locale!(
            zh_cn: "光谱层数",
            en: "Spectral Stack"
        ),
        config: ItemConfigType::Float {
            default: 0.0,
            min: 0.0,
            max: 4.0,
        },
    }]);

    fn get_effect<A: Attribute>(
        character: &CharacterCommonData,
        config: &WeaponConfig,
    ) -> Option<Box<dyn WeaponEffect<A>>> {
        match *config {
            WeaponConfig::ReliquaryOfTruth { spectral_stack } => {
                Some(Box::new(ReliquaryOfTruthEffect { spectral_stack }))
            }
            _ => None,
        }
    }
}
