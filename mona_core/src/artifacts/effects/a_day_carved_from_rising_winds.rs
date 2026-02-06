use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct ADayCarvedFromRisingWindsEffect;

impl<A: Attribute> ArtifactEffect<A> for ADayCarvedFromRisingWindsEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.add_atk_percentage("风之时刻2", 0.18);
    }

    fn effect4(&self, attribute: &mut A) {
        attribute.set_value_by(crate::attribute::AttributeName::BonusPlungingAttack, "风之时刻4", 0.30);
    }
}

pub struct ADayCarvedFromRisingWinds;

impl ArtifactTrait for ADayCarvedFromRisingWinds {
    fn create_effect<A: Attribute>(_config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(ADayCarvedFromRisingWindsEffect)
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::ADayCarvedFromRisingWinds,
        name_mona: "ADayCarvedFromRisingWinds",
        name_locale: crate::common::i18n::locale!(
            zh_cn: "风之时刻",
            en: "A Day Carved From Rising Winds",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "风之花",
            en: "Flower of Rising Winds",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "风之羽",
            en: "Feather of Rising Winds",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "风之沙",
            en: "Sands of Rising Winds",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "风之杯",
            en: "Goblet of Rising Winds",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "风之冠",
            en: "Crown of Rising Winds",
        )),
        star: (4, 5),
        effect1: None,
        effect2: Some(crate::common::i18n::locale!(
            zh_cn: "攻击力提高18%。",
            en: "ATK +18%.",
        )),
        effect3: None,
        effect4: Some(crate::common::i18n::locale!(
            zh_cn: "下落攻击伤害提高30%。",
            en: "Plunging Attack DMG +30%.",
        )),
        effect5: None,
        internal_id: 99903,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = None;
}
