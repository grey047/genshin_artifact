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

struct NocturnesCurtainCallEffect {
    pub lunar_triggered: bool,
}

impl<A: Attribute> WeaponEffect<A> for NocturnesCurtainCallEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        // Static Max HP: 10/12/14/16/18%
        attribute.add_hp_percentage("Nocturne's Curtain Call Passive", 0.08 + 0.02 * refine);

        if self.lunar_triggered {
            // Lunar Reaction triggers: +14/16/18/20/22% HP
            attribute.add_hp_percentage("Nocturne's Curtain Call Lunar", 0.12 + 0.02 * refine);
            // Lunar CRIT DMG: +60/80/100/120/140%
            attribute.set_value_by(
                AttributeName::CriticalDamageBase,
                "Nocturne's Curtain Call Lunar",
                0.40 + 0.20 * refine,
            );
            // Energy recovery (14/15/16/17/18) cannot be modeled as attribute
        }
    }
}

pub struct NocturnesCurtainCall;

impl WeaponTrait for NocturnesCurtainCall {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::NocturnesCurtainCall,
        internal_name: "Catalyst_Brisingamen",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage192),
        weapon_base: WeaponBaseATKFamily::ATK542,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "生命值上限提高<span style=\"color: #409EFF;\">10%-12%-14%-16%-18%</span>。触发月反应后，生命值上限额外提高<span style=\"color: #409EFF;\">14%-16%-18%-20%-22%</span>，「月」反应暴击伤害提高<span style=\"color: #409EFF;\">60%-80%-100%-120%-140%</span>，并恢复<span style=\"color: #409EFF;\">14-15-16-17-18</span>点元素能量。该效果每18秒至多触发一次。",
            en: "Max HP increased by <span style=\"color: #409EFF;\">10%-12%-14%-16%-18%</span>. After triggering a Lunar Reaction, Max HP is additionally increased by <span style=\"color: #409EFF;\">14%-16%-18%-20%-22%</span>, Lunar Reaction CRIT DMG is increased by <span style=\"color: #409EFF;\">60%-80%-100%-120%-140%</span>, and <span style=\"color: #409EFF;\">14-15-16-17-18</span> Energy is recovered. This effect can trigger once every 18s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "夜幕的帷幕",
            en: "Nocturne's Curtain Call"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[ItemConfig {
        name: "lunar_triggered",
        title: locale!(
            zh_cn: "触发月反应",
            en: "Lunar Reaction Triggered"
        ),
        config: ItemConfigType::Bool { default: false },
    }]);

    fn get_effect<A: Attribute>(
        character: &CharacterCommonData,
        config: &WeaponConfig,
    ) -> Option<Box<dyn WeaponEffect<A>>> {
        match *config {
            WeaponConfig::NocturnesCurtainCall { stack } => {
                Some(Box::new(NocturnesCurtainCallEffect { lunar_triggered: stack > 0.0 }))
            }
            _ => None,
        }
    }
}
