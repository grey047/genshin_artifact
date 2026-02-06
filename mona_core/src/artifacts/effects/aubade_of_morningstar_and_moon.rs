use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct AubadeOfMorningstarAndMoonEffect;

impl<A: Attribute> ArtifactEffect<A> for AubadeOfMorningstarAndMoonEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.set_value_by(crate::attribute::AttributeName::CriticalDamageBase, "晨星与月之歌2", 0.12);
    }

    fn effect4(&self, attribute: &mut A) {
        // Lunar reaction damage bonus - placeholder
        attribute.set_value_by(crate::attribute::AttributeName::USER1, "晨星与月之歌4", 0.32);
    }
}

pub struct AubadeOfMorningstarAndMoon;

impl ArtifactTrait for AubadeOfMorningstarAndMoon {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(AubadeOfMorningstarAndMoonEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::AubadeOfMorningstarAndMoon,
        name_mona: "AubadeOfMorningstarAndMoon",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "晨星与月之歌",
            en: "Aubade of Morningstar and Moon",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "晨星之花",
            en: "Flower of Morningstar",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "晨星之羽",
            en: "Feather of Morningstar",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "晨星之沙",
            en: "Sands of Morningstar",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "晨星之杯",
            en: "Goblet of Morningstar",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "晨星之冠",
            en: "Crown of Morningstar",
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "暴击伤害提高12%。",
            en: "CRIT DMG +12%.",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "「月」反应伤害提高32%。",
            en: "Lunar reaction damage +32%.",
        )),
        effect5: None,
        internal_id: 99904,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = None;
}
