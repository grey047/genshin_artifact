use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct SilkenMoonsSerenadeEffect;

impl<A: Attribute> ArtifactEffect<A> for SilkenMoonsSerenadeEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.set_value_by(crate::attribute::AttributeName::ElementalMastery, "丝绸之月2", 30.0);
    }

    fn effect4(&self, attribute: &mut A) {
        // Lunar reaction damage bonus - placeholder
        attribute.set_value_by(crate::attribute::AttributeName::USER1, "丝绸之月4", 0.20);
    }
}

pub struct SilkenMoonsSerenade;

impl ArtifactTrait for SilkenMoonsSerenade {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(SilkenMoonsSerenadeEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::SilkenMoonsSerenade,
        name_mona: "SilkenMoonsSerenade",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "丝绸之月",
            en: "Silken Moon's Serenade",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "丝绸之花",
            en: "Flower of Silken Moon",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "丝绸之羽",
            en: "Feather of Silken Moon",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "丝绸之沙",
            en: "Sands of Silken Moon",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "丝绸之杯",
            en: "Goblet of Silken Moon",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "丝绸之冠",
            en: "Crown of Silken Moon",
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "元素精通提高30点。",
            en: "Elemental Mastery +30.",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "「月」反应伤害提高20%。",
            en: "Lunar reaction damage +20%.",
        )),
        effect5: None,
        internal_id: 99902,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = None;
}
