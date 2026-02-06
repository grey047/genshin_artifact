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
        attribute.set_value_by(crate::attribute::AttributeName::CriticalBase, "天空之舞2", 0.08);
    }

    fn effect4(&self, attribute: &mut A) {
        // Lunar reaction damage bonus - placeholder, requires new attribute
        attribute.set_value_by(crate::attribute::AttributeName::USER1, "天空之舞4", 0.20);
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
            zh_cn: "天空之舞",
            en: "Night of the Sky's Unveiling",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "天空之花",
            en: "Flower of the Unveiled Sky",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "天空之羽",
            en: "Feather of the Unveiled Sky",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "天空之沙",
            en: "Sands of the Unveiled Sky",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "天空之杯",
            en: "Goblet of the Unveiled Sky",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "天空之冠",
            en: "Crown of the Unveiled Sky",
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "暴击率提高8%。",
            en: "CRIT Rate +8%.",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "「月」反应伤害提高20%。",
            en: "Lunar reaction damage +20%.",
        )),
        effect5: None,
        internal_id: 99901,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = None;
}
