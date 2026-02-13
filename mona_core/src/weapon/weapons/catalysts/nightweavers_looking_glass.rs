use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::common::i18n::locale;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};

pub struct NightweaversLookingGlassEffect {
    pub has_prayer: bool,
    pub verse_stacks: usize,
}

impl<A: Attribute> WeaponEffect<A> for NightweaversLookingGlassEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as usize;
        let em_per_stack = [60.0, 75.0, 90.0, 105.0, 120.0][refine - 1];
        
        // Prayer of the Far North: 1 stack EM bonus
        let prayer_em = if self.has_prayer { em_per_stack } else { 0.0 };
        
        // New Moon Verse: 0-3 stacks EM bonus
        let verse_stacks = self.verse_stacks.min(3);
        let verse_em = em_per_stack * verse_stacks as f64;
        
        let total_em = prayer_em + verse_em;
        
        if total_em > 0.0 {
            attribute.set_value_by(AttributeName::ElementalMastery, "Millennial Hymn", total_em);
        }
        
        // At 3 stacks: Lunar-Bloom DMG +40%
        if verse_stacks >= 3 {
            attribute.set_value_by(AttributeName::EnhanceLunarBloom, "Millennial Hymn", 0.40);
        }
    }
}

pub struct NightweaversLookingGlass;

impl WeaponTrait for NightweaversLookingGlass {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::NightweaversLookingGlass,
        internal_name: "Catalyst_Nightweaver",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage96),
        weapon_base: WeaponBaseATKFamily::ATK542,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "装备者的元素战技命中敌人后，获得「极北祝颂」效果：元素精通提升60-75-90-105-120点，持续4.5秒。附近队伍触发月绽放反应时，装备者获得「新月诗篇」效果：元素精通提升60-75-90-105-120点，持续4.5秒，最多叠加3层。处于3层新月诗篇时，附近所有角色的月绽放伤害提升40%。",
            en: "When the equipping character's Elemental Skill deals Hydro or Dendro DMG, they will gain Prayer of the Far North: Elemental Mastery is increased by 60-75-90-105-120 for 4.5s. When nearby party members trigger Lunar-Bloom reactions, the equipping character gains New Moon Verse: Elemental Mastery is increased by 60-75-90-105-120 for 4.5s, max 3 stacks. At 3 stacks, nearby characters' Lunar-Bloom DMG is increased by 40%.",
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "织夜者的明镜",
            en: "Nightweaver's Looking Glass"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "has_prayer",
            title: locale!(
                zh_cn: "极北祝颂",
                en: "Prayer of the Far North",
            ),
            config: ItemConfigType::Bool { default: true },
        },
        ItemConfig {
            name: "verse_stacks",
            title: locale!(
                zh_cn: "新月诗篇层数",
                en: "New Moon Verse Stacks",
            ),
            config: ItemConfigType::Int { min: 0, max: 3, default: 3 },
        },
    ]);

    fn get_effect<A: Attribute>(
        _character: &CharacterCommonData,
        config: &WeaponConfig
    ) -> Option<Box<dyn WeaponEffect<A>>> {
        if let WeaponConfig::NightweaversLookingGlass { has_prayer, verse_stacks } = config {
            Some(Box::new(NightweaversLookingGlassEffect {
                has_prayer: *has_prayer,
                verse_stacks: *verse_stacks,
            }))
        } else {
            None
        }
    }
}
