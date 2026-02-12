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
        attribute.add_atk_percentage("风起之日2", 0.18);
    }

    fn effect4(&self, attribute: &mut A) {
        // ATK +25% for 6s after hit; upgraded to also give CR +20% when Witch's Homework is completed
        attribute.add_atk_percentage("风起之日4", 0.25);
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
            zh_cn: "风起之日",
            en: "A Day Carved From Rising Winds",
        ),
        flower: Some(crate::common::i18n::locale!(
            zh_cn: "风起之花",
            en: "Flower of Rising Winds",
        )),
        feather: Some(crate::common::i18n::locale!(
            zh_cn: "风起之羽",
            en: "Feather of Rising Winds",
        )),
        sand: Some(crate::common::i18n::locale!(
            zh_cn: "风起之沙",
            en: "Sands of Rising Winds",
        )),
        goblet: Some(crate::common::i18n::locale!(
            zh_cn: "风起之杯",
            en: "Goblet of Rising Winds",
        )),
        head: Some(crate::common::i18n::locale!(
            zh_cn: "风起之冠",
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
            zh_cn: "受到攻击后6秒内攻击力+25%；完成魔女的功课后还会额外获得暴击率+20%。",
            en: "ATK +25% for 6s after being hit; additionally gains CRIT Rate +20% when Witch's Homework is completed.",
        )),
        effect5: None,
        internal_id: 15044,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = None;
}
