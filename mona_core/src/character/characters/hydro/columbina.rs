use num_derive::FromPrimitive;
use num::FromPrimitive;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};

use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::common::i18n::{locale, hit_n_dmg, charged_dmg, plunging_dmg};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::character::macros::{skill_type, damage_enum, skill_map, damage_ratio};

// Auto-generated from Genshin Fandom Wiki and HHW Data
// Character: Columbina (Hydro / Catalyst)

pub struct ColumbinaSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub e_dmg1: [f64; 15],
    pub e_dmg2: [f64; 15],
    pub e_dmg3_lc: [f64; 15],
    pub e_dmg3_lb: [f64; 15],
    pub e_dmg3_lcrys: [f64; 15],

    pub q_dmg1: [f64; 15],
}

pub const COLUMBINA_SKILL: ColumbinaSkillType = ColumbinaSkillType {
    // Normal Attack: Moondew Cascade (Lv10: 79.55%, 62.26%, 99.42%)
    normal_dmg1: [0.4473, 0.4839, 0.5205, 0.5665, 0.6031, 0.6397, 0.6858, 0.7319, 0.7780, 0.7955, 0.8388, 0.8821, 0.9254, 0.9687, 1.0120],
    normal_dmg2: [0.3501, 0.3787, 0.4073, 0.4433, 0.4720, 0.5006, 0.5366, 0.5727, 0.6087, 0.6226, 0.6565, 0.6904, 0.7243, 0.7582, 0.7921],
    normal_dmg3: [0.5590, 0.6047, 0.6504, 0.7080, 0.7536, 0.7993, 0.8569, 0.9145, 0.9720, 0.9942, 1.0483, 1.1024, 1.1565, 1.2106, 1.2647],
    
    // Charged Attack (Lv10: 197.34%)
    charged_dmg1: [1.1096, 1.2003, 1.2910, 1.4053, 1.4960, 1.5867, 1.7011, 1.8154, 1.9298, 1.9734, 2.0807, 2.1880, 2.2953, 2.4026, 2.5099],
    
    // Plunging Attack (Lv10: 104.41% / 208.77% / 260.76%)
    plunging_dmg1: [0.5871, 0.6351, 0.6831, 0.7436, 0.7916, 0.8395, 0.9000, 0.9605, 1.0210, 1.0441, 1.1009, 1.1576, 1.2144, 1.2711, 1.3279],
    plunging_dmg2: [1.1740, 1.2700, 1.3660, 1.4871, 1.5831, 1.6790, 1.8000, 1.9210, 2.0420, 2.0877, 2.2014, 2.3151, 2.4288, 2.5425, 2.6562],
    plunging_dmg3: [1.4661, 1.5859, 1.7057, 1.8571, 1.9769, 2.0966, 2.2476, 2.3985, 2.5494, 2.6076, 2.7496, 2.8916, 3.0336, 3.1756, 3.3176],
    
    // Elemental Skill: Eternal Tides - HP scaling
    // Lv10: Skill DMG 30.10% / Ripple 16.85% / LC 8.47% / LB 2.53% / LCrys 15.88%
    e_dmg1: [0.1692, 0.1830, 0.1967, 0.2141, 0.2278, 0.2415, 0.2589, 0.2763, 0.2937, 0.3010, 0.3174, 0.3338, 0.3502, 0.3666, 0.3830],
    e_dmg2: [0.0947, 0.1024, 0.1101, 0.1198, 0.1275, 0.1352, 0.1450, 0.1547, 0.1644, 0.1685, 0.1777, 0.1869, 0.1961, 0.2053, 0.2145],
    e_dmg3_lc: [0.0476, 0.0514, 0.0553, 0.0601, 0.0640, 0.0679, 0.0728, 0.0777, 0.0826, 0.0847, 0.0893, 0.0939, 0.0985, 0.1031, 0.1077],
    e_dmg3_lb: [0.0142, 0.0154, 0.0165, 0.0180, 0.0191, 0.0203, 0.0217, 0.0232, 0.0246, 0.0253, 0.0267, 0.0281, 0.0294, 0.0308, 0.0322],
    e_dmg3_lcrys: [0.0892, 0.0964, 0.1037, 0.1129, 0.1201, 0.1274, 0.1366, 0.1459, 0.1551, 0.1588, 0.1674, 0.1760, 0.1846, 0.1932, 0.2018],
    
    // Elemental Burst: Moonlit Melancholy - HP scaling (Lv10: 58.03%)
    q_dmg1: [0.3261, 0.3526, 0.3791, 0.4127, 0.4392, 0.4657, 0.4993, 0.5329, 0.5665, 0.5803, 0.6119, 0.6435, 0.6751, 0.7067, 0.7383],
};

#[derive(Copy, Clone, FromPrimitive, Eq, PartialEq, EnumString, EnumCountMacro)]
pub enum ColumbinaDamageEnum {
    Normal1,
    Normal2,
    Normal3,
    Charged1,
    Plunging1,
    Plunging2,
    Plunging3,
    E1,
    E2,
    E3_LC,
    E3_LB,
    E3_LCrys,
    Q1,
}

impl ColumbinaDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use ColumbinaDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 => SkillType::NormalAttack,
            Charged1 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 | E3_LC | E3_LB | E3_LCrys => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst,
        }
    }

    pub fn get_element(&self) -> Element {
        Element::Hydro
    }
}

impl Into<usize> for ColumbinaDamageEnum {
    fn into(self) -> usize {
        self as usize
    }
}

pub struct ColumbinaEffect {
    pub has_talent1: bool,
    pub has_talent2: bool,
}

impl ColumbinaEffect {
    pub fn new(common_data: &CharacterCommonData) -> Self {
        ColumbinaEffect {
            has_talent1: common_data.has_talent1,
            has_talent2: common_data.has_talent2,
        }
    }
}

impl<A: Attribute> ChangeAttribute<A> for ColumbinaEffect {
    fn change_attribute(&self, attribute: &mut A) {
        // A4 Passive: Lunar DMG bonus based on Max HP
        if self.has_talent1 {
            let hp = attribute.get_value(AttributeName::HP);
            let bonus = (hp / 1000.0) * 0.002;
            let capped_bonus = bonus.min(0.07);
            attribute.set_value_by(AttributeName::BonusHydro, "Columbina A4: Moonsign Benediction", capped_bonus);
        }
    }
}

pub struct Columbina;

impl CharacterTrait for Columbina {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Columbina,
        internal_name: "Columbina",
        element: Element::Hydro,
        hp: [1469, 3797, 4515, 6768, 7390, 8505, 9451, 10568, 11284, 12401, 13117, 14234, 14950, 16067, 16067],
        atk: [514, 1327, 1578, 2366, 2586, 2975, 3305, 3696, 3946, 4340, 4586, 4979, 5226, 5619, 5619],
        def: [62, 160, 190, 285, 311, 358, 398, 445, 475, 523, 552, 600, 629, 677, 677],
        sub_stat: CharacterSubStatFamily::HP288,
        weapon_type: WeaponType::Catalyst,
        star: 5,
        skill_name1: locale!(
            zh_cn: "TODO",
            en: "Moondew Cascade",
        ),
        skill_name2: locale!(
            zh_cn: "TODO",
            en: "Eternal Tides",
        ),
        skill_name3: locale!(
            zh_cn: "TODO",
            en: "Moonlit Melancholy",
        ),
        name_locale: locale!(
            zh_cn: "哥伦比娅",
            en: "Columbina",
        )
    };
    type SkillType = ColumbinaSkillType;
    const SKILL: Self::SkillType = COLUMBINA_SKILL;
    type DamageEnumType = ColumbinaDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            ColumbinaDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 hit_n_dmg!(3)
            Charged1 charged_dmg!()
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            ColumbinaDamageEnum
            E1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
            E2 locale!(zh_cn: "Ripple伤害", en: "Ripple DMG")
            E3_LC locale!(zh_cn: "干涉LC", en: "Interference LC")
            E3_LB locale!(zh_cn: "干涉LB", en: "Interference LB")
            E3_LCrys locale!(zh_cn: "干涉结晶", en: "Interference Cryst")
        ),
        skill3: skill_map!(
            ColumbinaDamageEnum
            Q1 locale!(zh_cn: "爆发伤害", en: "Burst DMG")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = None;

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "moonsign_level",
            title: locale!(
                zh_cn: "月兆等级",
                en: "Moonsign Level"
            ),
            config: ItemConfigType::Int { min: 1, max: 2, default: 2 },
        },
        ItemConfig {
            name: "has_moonsign_benediction",
            title: locale!(
                zh_cn: "月兆祝福",
                en: "Moonsign Benediction"
            ),
            config: ItemConfigType::Bool { default: true },
        },
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, _config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: ColumbinaDamageEnum = FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let mut builder = D::new();
        
        // Normal and Charged attacks use ATK scaling
        match s {
            ColumbinaDamageEnum::Normal1 => builder.add_atk_ratio("技能倍率", COLUMBINA_SKILL.normal_dmg1[s1]),
            ColumbinaDamageEnum::Normal2 => builder.add_atk_ratio("技能倍率", COLUMBINA_SKILL.normal_dmg2[s1]),
            ColumbinaDamageEnum::Normal3 => builder.add_atk_ratio("技能倍率", COLUMBINA_SKILL.normal_dmg3[s1]),
            ColumbinaDamageEnum::Charged1 => builder.add_atk_ratio("技能倍率", COLUMBINA_SKILL.charged_dmg1[s1]),
            ColumbinaDamageEnum::Plunging1 => builder.add_atk_ratio("技能倍率", COLUMBINA_SKILL.plunging_dmg1[s1]),
            ColumbinaDamageEnum::Plunging2 => builder.add_atk_ratio("技能倍率", COLUMBINA_SKILL.plunging_dmg2[s1]),
            ColumbinaDamageEnum::Plunging3 => builder.add_atk_ratio("技能倍率", COLUMBINA_SKILL.plunging_dmg3[s1]),
            
            // E skills use HP scaling
            ColumbinaDamageEnum::E1 => builder.add_hp_ratio("E技能倍率", COLUMBINA_SKILL.e_dmg1[s2]),
            ColumbinaDamageEnum::E2 => builder.add_hp_ratio("Ripple倍率", COLUMBINA_SKILL.e_dmg2[s2]),
            ColumbinaDamageEnum::E3_LC => builder.add_hp_ratio("干涉LC倍率", COLUMBINA_SKILL.e_dmg3_lc[s2]),
            ColumbinaDamageEnum::E3_LB => builder.add_hp_ratio("干涉LB倍率", COLUMBINA_SKILL.e_dmg3_lb[s2]),
            ColumbinaDamageEnum::E3_LCrys => builder.add_hp_ratio("干涉结晶倍率", COLUMBINA_SKILL.e_dmg3_lcrys[s2]),
            
            // Q uses HP scaling
            ColumbinaDamageEnum::Q1 => builder.add_hp_ratio("Q技能倍率", COLUMBINA_SKILL.q_dmg1[s3]),
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            Element::Hydro,
            s.get_skill_type(),
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, _config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        Some(Box::new(ColumbinaEffect::new(common_data)))
    }

    fn get_target_function_by_role(_role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        Box::new(ColumbinaDefaultTargetFunction {
            recharge_demand: 1.0,
            use_skill: 0.5,
            use_burst: 0.5,
            use_e2: 0.3,
            use_e3_lc: 0.2,
            use_e3_lb: 0.1,
            use_e3_lcrys: 0.15,
            moonsign_level: 2,
        })
    }
}

// Target Function Implementation
use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::SimpleAttributeGraph2;
use crate::character::Character;
use crate::common::StatName;
use crate::damage::SimpleDamageBuilder;
use crate::enemies::Enemy;
use crate::weapon::Weapon;

pub struct ColumbinaDefaultTargetFunction {
    pub recharge_demand: f64,
    pub use_skill: f64,
    pub use_burst: f64,
    pub use_e2: f64,
    pub use_e3_lc: f64,
    pub use_e3_lb: f64,
    pub use_e3_lcrys: f64,
    pub moonsign_level: usize,
}

impl ColumbinaDefaultTargetFunction {
    pub fn new(config: &TargetFunctionConfig) -> Self {
        match *config {
            TargetFunctionConfig::ColumbinaDefault {
                recharge_demand,
                use_skill,
                use_burst,
                use_e2,
                use_e3_lc,
                use_e3_lb,
                use_e3_lcrys,
                moonsign_level,
            } => Self {
                recharge_demand,
                use_skill,
                use_burst,
                use_e2,
                use_e3_lc,
                use_e3_lb,
                use_e3_lcrys,
                moonsign_level,
            },
            _ => Self {
                recharge_demand: 1.0,
                use_skill: 0.5,
                use_burst: 0.5,
                use_e2: 0.3,
                use_e3_lc: 0.2,
                use_e3_lb: 0.1,
                use_e3_lcrys: 0.15,
                moonsign_level: 2,
            }
        }
    }
}

impl TargetFunctionMetaTrait for ColumbinaDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::ColumbinaDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "Columbina-默认",
            en: "Columbina-Default"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "HP Scaling Hydro Support",
            en: "HP-scaling Hydro support character"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Columbina),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "recharge_demand",
            title: locale!(
                zh_cn: "充能需求",
                en: "Recharge Requirement"
            ),
            config: ItemConfigType::Float { default: 1.0, min: 1.0, max: 3.0 }
        },
        ItemConfig {
            name: "use_skill",
            title: locale!(
                zh_cn: "E技能使用比例",
                en: "Skill Usage Ratio"
            ),
            config: ItemConfigType::Float { default: 0.5, min: 0.0, max: 1.0 }
        },
        ItemConfig {
            name: "use_burst",
            title: locale!(
                zh_cn: "大招使用比例",
                en: "Burst Usage Ratio"
            ),
            config: ItemConfigType::Float { default: 0.5, min: 0.0, max: 1.0 }
        },
        ItemConfig {
            name: "use_e2",
            title: locale!(
                zh_cn: "Ripple使用比例",
                en: "Ripple Usage Ratio"
            ),
            config: ItemConfigType::Float { default: 0.3, min: 0.0, max: 1.0 }
        },
        ItemConfig {
            name: "use_e3_lc",
            title: locale!(
                zh_cn: "干涉LC使用比例",
                en: "Interference LC Ratio"
            ),
            config: ItemConfigType::Float { default: 0.2, min: 0.0, max: 1.0 }
        },
        ItemConfig {
            name: "use_e3_lb",
            title: locale!(
                zh_cn: "干涉LB使用比例",
                en: "Interference LB Ratio"
            ),
            config: ItemConfigType::Float { default: 0.1, min: 0.0, max: 1.0 }
        },
        ItemConfig {
            name: "use_e3_lcrys",
            title: locale!(
                zh_cn: "干涉结晶使用比例",
                en: "Interference Cryst Ratio"
            ),
            config: ItemConfigType::Float { default: 0.15, min: 0.0, max: 1.0 }
        },
        ItemConfig {
            name: "moonsign_level",
            title: locale!(
                zh_cn: "月兆等级",
                en: "Moonsign Level"
            ),
            config: ItemConfigType::Int { min: 1, max: 2, default: 2 },
        },
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(ColumbinaDefaultTargetFunction::new(config))
    }
}

impl TargetFunction for ColumbinaDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        TargetFunctionOptConfig {
            atk_fixed: 0.0,
            atk_percentage: 0.3,
            hp_fixed: 0.0,
            hp_percentage: 1.2,
            def_fixed: 0.0,
            def_percentage: 0.0,
            recharge: 1.5,
            elemental_mastery: 0.3,
            critical: 1.2,
            critical_damage: 1.0,
            healing_bonus: 0.0,
            bonus_physical: 0.0,
            bonus_pyro: 0.0,
            bonus_hydro: 1.0,
            bonus_anemo: 0.0,
            bonus_cryo: 0.0,
            bonus_geo: 0.0,
            bonus_dendro: 0.0,
            bonus_electro: 0.0,
            sand_main_stats: vec![
                StatName::Recharge,
                StatName::HPPercentage,
            ],
            goblet_main_stats: vec![
                StatName::HydroBonus,
                StatName::HPPercentage,
            ],
            head_main_stats: vec![
                StatName::CriticalRate,
                StatName::CriticalDamage,
                StatName::HPPercentage,
            ],
            set_names: Some(vec![
                ArtifactSetName::SilkenMoonsSerenade,
                ArtifactSetName::AubadeOfMorningstarAndMoon,
                ArtifactSetName::NightOfTheSkysUnveiling,
            ]),
            very_critical_set_names: None,
            normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
            critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
            very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD
        }
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfig::default()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], _enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute,
            enemy: _enemy,
        };

        let s_config = CharacterSkillConfig::NoConfig;

        type S = <Columbina as CharacterTrait>::DamageEnumType;

        // Normal attack damage
        let dmg_normal = Columbina::damage::<SimpleDamageBuilder>(&context, S::Normal1, &s_config, None).normal.expectation;
        
        // E skill damages (HP scaling)
        let dmg_e1 = Columbina::damage::<SimpleDamageBuilder>(&context, S::E1, &s_config, None).normal.expectation;
        let dmg_e2 = Columbina::damage::<SimpleDamageBuilder>(&context, S::E2, &s_config, None).normal.expectation;
        let dmg_e3_lc = Columbina::damage::<SimpleDamageBuilder>(&context, S::E3_LC, &s_config, None).normal.expectation;
        let dmg_e3_lb = Columbina::damage::<SimpleDamageBuilder>(&context, S::E3_LB, &s_config, None).normal.expectation;
        let dmg_e3_lcrys = Columbina::damage::<SimpleDamageBuilder>(&context, S::E3_LCrys, &s_config, None).normal.expectation;
        
        // Q burst damage
        let dmg_q = Columbina::damage::<SimpleDamageBuilder>(&context, S::Q1, &s_config, None).normal.expectation;

        let recharge = attribute.get_value(AttributeName::Recharge);
        let r = recharge.min(self.recharge_demand);

        let total_dmg = (dmg_normal * (1.0 - self.use_skill * 0.5)
                       + dmg_e1 * self.use_skill
                       + dmg_e2 * self.use_e2
                       + dmg_e3_lc * self.use_e3_lc
                       + dmg_e3_lb * self.use_e3_lb
                       + dmg_e3_lcrys * self.use_e3_lcrys
                       + dmg_q * self.use_burst) * r;

        total_dmg
    }
}
