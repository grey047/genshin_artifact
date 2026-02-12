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
        attribute.set_value_by(crate::attribute::AttributeName::Recharge, "纺月的夜歌2", 0.20);
    }

    fn effect4(&self, attribute: &mut A) {
        // Gleaming Moon: Devotion — EM +60/120 (Nascent/Ascendant Gleam)
        // + Lunar DMG +10% per unique Gleaming Moon effect
        // Simplified: use base Nascent value
        attribute.set_value_by(crate::attribute::AttributeName::ElementalMastery, "纺月的夜歌4", 60.0);
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
            zh_cn: "纺月的夜歌",
            en: "Silken Moon's Serenade",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "纺月夜歌之花",
            en: "Flower of Silken Moon",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "纺月夜歌之羽",
            en: "Feather of Silken Moon",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "纺月夜歌之沙",
            en: "Sands of Silken Moon",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "纺月夜歌之杯",
            en: "Goblet of Silken Moon",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "纺月夜歌之冠",
            en: "Crown of Silken Moon",
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "元素充能效率提高20%。",
            en: "Energy Recharge +20%.",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "辉月·奉献：获得初月辉光时元素精通+60，升华辉光时元素精通+120；此外每拥有一种不同的辉月效果，月反应伤害+10%。",
            en: "Gleaming Moon: Devotion — When Nascent Gleam is obtained, EM +60; when Ascendant Gleam is obtained, EM +120. Additionally, Lunar DMG +10% per unique Gleaming Moon effect.",
        )),
        effect5: None,
        internal_id: 15042,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = None;
}
