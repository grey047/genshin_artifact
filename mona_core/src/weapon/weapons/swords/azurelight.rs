use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct AzurelightEffect {
    pub energy_zero: bool,
}

impl<A: Attribute> WeaponEffect<A> for AzurelightEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        
        // 基础 ATK 加成: 24% + 精炼 * 6%
        let base_atk_bonus = 0.24 + refine * 0.06;
        
        // 能量为0时: ATK 翻倍，且额外 CD
        let energy_multiplier = if self.energy_zero { 2.0 } else { 1.0 };
        
        // ATK 加成
        let atk_bonus = base_atk_bonus * energy_multiplier;
        attribute.set_value_by(AttributeName::ATK, "白山的馈赐", atk_bonus);
        
        // 能量为0时: 暴击伤害加成 40% + 精炼 * 10%
        if self.energy_zero {
            let cd_bonus = 0.40 + refine * 0.10;
            attribute.set_value_by(AttributeName::CriticalDamageBase, "白山的馈赐", cd_bonus);
        }
    }
}

pub struct Azurelight;

impl WeaponTrait for Azurelight {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::Azurelight,
        internal_name: "Sword_Regalis",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate96),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "施放元素战技后的12秒内，攻击力提升<span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span>。持续期间，装备者的元素能量为0时，攻击力还会提升相同值，且暴击伤害提升<span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span>。",
            en: "Within 12s after an Elemental Skill is used, ATK is increased by <span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span>. During this time, when the equipping character has 0 Energy, ATK will be further increased by the same amount, and CRIT DMG will be increased by <span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span>."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "苍耀",
            en: "Azurelight"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "energy_zero",
            title: locale!(
                zh_cn: "能量为0",
                en: "Energy = 0"
            ),
            config: ItemConfigType::Bool { default: false }
        },
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let energy_zero = match *config {
            WeaponConfig::Azurelight { energy_zero, .. } => energy_zero,
            _ => false
        };
        Some(Box::new(AzurelightEffect {
            energy_zero,
        }))
    }
}
