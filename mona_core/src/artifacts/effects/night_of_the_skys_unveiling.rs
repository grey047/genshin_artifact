use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct NightOfTheSkysUnveilingEffect;

impl<A: Attribute> ArtifactEffect<A> for NightOfTheSkysUnveilingEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.set_value_by(crate::attribute::AttributeName::ElementalMastery, "穹境示现之夜2", 80.0);
    }

    fn effect4(&self, attribute: &mut A) {
        // Gleaming Moon: Intent — CR +15/30% (Nascent/Ascendant Gleam)
        // + Lunar DMG +10% per unique Gleaming Moon effect
        // Simplified: use placeholder for conditional CR buff
        attribute.set_value_by(crate::attribute::AttributeName::CriticalBase, "穹境示现之夜4", 0.15);
    }
}

pub struct NightOfTheSkysUnveiling;

impl ArtifactTrait for NightOfTheSkysUnveiling {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(NightOfTheSkysUnveilingEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::NightOfTheSkysUnveiling,
        name_mona: "NightOfTheSkysUnveiling",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "穹境示现之夜",
            en: "Night of the Sky's Unveiling",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "穹境示现之花",
            en: "Flower of the Unveiled Sky",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "穹境示现之羽",
            en: "Feather of the Unveiled Sky",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "穹境示现之沙",
            en: "Sands of the Unveiled Sky",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "穹境示现之杯",
            en: "Goblet of the Unveiled Sky",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "穹境示现之冠",
            en: "Crown of the Unveiled Sky",
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "元素精通提高80点。",
            en: "Elemental Mastery +80.",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "辉月·心意：获得初月辉光时暴击率+15%，升华辉光时暴击率+30%；此外每拥有一种不同的辉月效果，月反应伤害+10%。",
            en: "Gleaming Moon: Intent — When Nascent Gleam is obtained, CRIT Rate +15%; when Ascendant Gleam is obtained, CRIT Rate +30%. Additionally, Lunar DMG +10% per unique Gleaming Moon effect.",
        )),
        effect5: None,
        internal_id: 15041,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = None;
}
