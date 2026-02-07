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
    pub stack: f64,
}

impl<A: Attribute> WeaponEffect<A> for NocturnesCurtainCallEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        attribute.add_hp_percentage("Nocturne's Curtain Call Passive", 0.28 + 0.04 * refine);
        attribute.set_value_by(
            AttributeName::BonusElementalSkill,
            "Nocturne's Curtain Call Passive",
            (0.10 + 0.02 * refine) * self.stack,
        );
        attribute.set_value_by(
            AttributeName::BonusElementalBurst,
            "Nocturne's Curtain Call Passive",
            (0.10 + 0.02 * refine) * self.stack,
        );
    }
}

pub struct NocturnesCurtainCall;

impl WeaponTrait for NocturnesCurtainCall {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::NocturnesCurtainCall,
        internal_name: "Catalyst_NocturneCurtainCall",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate68),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "生命值上限提高<span style=\"color: #409EFF;\">28%-32%-36%-40%-44%</span>。装备者元素战技和元素爆发造成的伤害提升，提高数值相当于装备者生命值上限的<span style=\"color: #409EFF;\">10%-12%-14%-16%-18%</span>。",
            en: "Max HP increased by <span style=\"color: #409EFF;\">28%-32%-36%-40%-44%</span>. Elemental Skill and Elemental Burst DMG increased by an amount equal to <span style=\"color: #409EFF;\">10%-12%-14%-16%-18%</span> of Max HP."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "夜幕的帷幕",
            en: "Nocturne's Curtain Call"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[ItemConfig::STACK04]);

    fn get_effect<A: Attribute>(
        character: &CharacterCommonData,
        config: &WeaponConfig,
    ) -> Option<Box<dyn WeaponEffect<A>>> {
        match *config {
            WeaponConfig::NocturnesCurtainCall { stack } => {
                Some(Box::new(NocturnesCurtainCallEffect { stack }))
            }
            _ => None,
        }
    }
}
