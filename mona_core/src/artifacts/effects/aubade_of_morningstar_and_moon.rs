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
        attribute.set_value_by(crate::attribute::AttributeName::ElementalMastery, "晨星与月的晓歌2", 80.0);
    }

    fn effect4(&self, attribute: &mut A) {
        // Off-field Lunar Reaction DMG +20%; at Ascendant Gleam +40% more
        // Simplified: base off-field bonus
        attribute.set_value_by(crate::attribute::AttributeName::USER1, "晨星与月的晓歌4", 0.20);
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
            zh_cn: "晨星与月的晓歌",
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
            zh_cn: "元素精通提高80点。",
            en: "Elemental Mastery +80.",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "后台角色的月反应伤害+20%；处于升华辉光状态时额外+40%；前台角色上场3秒后移除。",
            en: "Off-field Lunar Reaction DMG +20%; at Ascendant Gleam, additionally +40%; removed 3s after going on-field.",
        )),
        effect5: None,
        internal_id: 15043,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = None;
}
