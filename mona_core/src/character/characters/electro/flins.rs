use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::macros::{damage_enum, skill_type};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::{TargetFunction, TargetFunctionConfig};
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

// ============================================================================
// Skill Data - from KQM TCL
// ============================================================================

pub struct FlinsSkillType {
    // Normal Attack
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],  // 2 hits
    pub normal_dmg5: [f64; 15],
    pub charged_dmg: [f64; 15],
    pub charged_stamina: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    // Elemental Skill - Manifest Flame Mode
    pub e_dmg1: [f64; 15],  // 幽焰一段
    pub e_dmg2: [f64; 15],  // 幽焰二段
    pub e_dmg3: [f64; 15],  // 幽焰三段
    pub e_dmg4: [f64; 15],  // 幽焰四段 (2 hits)
    pub e_dmg5: [f64; 15],  // 幽焰五段
    pub e_charged_dmg: [f64; 15],
    pub e_spearstorm_dmg: [f64; 15],  // 北国枪阵
    pub e_duration: [f64; 15],  // 幽焰持续时间
    pub e_cd: [f64; 15],  // 技能冷却

    // Elemental Burst - 旧仪·夜客致访
    pub q_initial_dmg: [f64; 15],  // 初始伤害
    pub q_middle_dmg: [f64; 15],  // 月感电中段 (2 hits)
    pub q_final_dmg: [f64; 15],  // 月感电终段
    pub q_energy: [f64; 15],
    pub q_cd: [f64; 15],

    // Thunderous Symphony (短Q)
    pub q_short_dmg: [f64; 15],  // 雷霆交响
    pub q_short_energy: [f64; 15],

    // Passives
    pub a1_lunarcharged_bonus: f64,  // A1: 满辉时月感电+20%
    pub a2_em_percent: f64,          // A2: EM = ATK * 8%, max 160
    pub a4_base_dmg_per_100: f64,    // A4: 每100 ATK +0.7% 月感电基础伤害, max 14%

    // Constellations
    pub c2_normal_dmg: f64,         // C2: 下次普攻+50% ATK
    pub c2_res_shred: f64,          // C2: 满辉时雷抗-25%
    pub c4_atk_percent: f64,         // C4: ATK+20%
    pub c4_em_percent: f64,         // C4: EM = ATK * 10%, max 220
    pub c6_lunarcharged_bonus: f64,  // C6: 月感电+35%
    pub c6_team_bonus: f64,         // C6: 满辉时全队+10%
}

pub const FLINS_SKILL: FlinsSkillType = FlinsSkillType {
    // Normal Attack (普攻 - 不值得升级)
    normal_dmg1: [0.4470, 0.4837, 0.5204, 0.5721, 0.6085, 0.6501, 0.7074, 0.7646, 0.8219, 0.8843, 0.9466, 1.0089, 1.0713, 1.1336, 1.1959],
    normal_dmg2: [0.4510, 0.4882, 0.5254, 0.5777, 0.6142, 0.6562, 0.7140, 0.7717, 0.8295, 0.8925, 0.9555, 1.0185, 1.0814, 1.1444, 1.2074],
    normal_dmg3: [0.5592, 0.6047, 0.6503, 0.7153, 0.7608, 0.8129, 0.8844, 0.9559, 1.0274, 1.1054, 1.1834, 1.2615, 1.3395, 1.4175, 1.4955],
    normal_dmg4: [0.3204, 0.3465, 0.3725, 0.4098, 0.4359, 0.4657, 0.5067, 0.5476, 0.5886, 0.6333, 0.6780, 0.7227, 0.7674, 0.8121, 0.8569],
    normal_dmg5: [0.7679, 0.8305, 0.8930, 0.9823, 1.0448, 1.1162, 1.2144, 1.3127, 1.4109, 1.5180, 1.6252, 1.7323, 1.8395, 1.9466, 2.0538],
    charged_dmg: [1.0303, 1.1142, 1.1981, 1.3178, 1.4017, 1.4975, 1.6293, 1.7611, 1.8929, 2.0366, 2.1804, 2.3241, 2.4679, 2.6116, 2.7554],
    charged_stamina: [25.0; 15],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.6020, 3.8248, 4.0476, 4.2704],

    // Elemental Skill - Manifest Flame Mode
    e_dmg1: [0.5825, 0.6262, 0.6699, 0.7281, 0.7718, 0.8155, 0.8737, 0.9320, 0.9902, 1.0485, 1.1067, 1.1650, 1.2378, 1.3106, 1.3834],
    e_dmg2: [0.5880, 0.6321, 0.6762, 0.7350, 0.7791, 0.8232, 0.8820, 0.9408, 0.9996, 1.0584, 1.1172, 1.1759, 1.2494, 1.3229, 1.3964],
    e_dmg3: [0.7283, 0.7829, 0.8375, 0.9103, 0.9649, 1.0196, 1.0924, 1.1652, 1.2380, 1.3109, 1.3837, 1.4565, 1.5475, 1.6386, 1.7296],
    e_dmg4: [0.4173, 0.4485, 0.4798, 0.5216, 0.5529, 0.5842, 0.6259, 0.6676, 0.7093, 0.7511, 0.7928, 0.8345, 0.8867, 0.9388, 0.9910],
    e_dmg5: [1.0001, 1.0751, 1.1501, 1.2501, 1.3251, 1.4002, 1.5002, 1.6002, 1.7002, 1.8002, 1.9002, 2.0002, 2.1253, 2.2503, 2.3753],
    e_charged_dmg: [1.1496, 1.2358, 1.3220, 1.4370, 1.5232, 1.6094, 1.7244, 1.8394, 1.9543, 2.0693, 2.1842, 2.2992, 2.4429, 2.5866, 2.7303],
    e_spearstorm_dmg: [1.7840, 1.9178, 2.0516, 2.2300, 2.3638, 2.4976, 2.6760, 2.8544, 3.0328, 3.2112, 3.3896, 3.5680, 3.7910, 4.0140, 4.2370],
    e_duration: [10.0; 15],
    e_cd: [16.0; 15],

    // Elemental Burst
    q_initial_dmg: [2.5984, 2.7933, 2.9882, 3.2480, 3.4429, 3.6378, 3.8976, 4.1574, 4.4173, 4.6771, 4.9369, 5.1968, 5.5216, 5.8464, 6.1712],
    q_middle_dmg: [0.1624, 0.1746, 0.1868, 0.2030, 0.2152, 0.2274, 0.2436, 0.2598, 0.2761, 0.2923, 0.3086, 0.3248, 0.3451, 0.3654, 0.3857],
    q_final_dmg: [1.1693, 1.2570, 1.3447, 1.4616, 1.5493, 1.6370, 1.7539, 1.8708, 1.9878, 2.1047, 2.2216, 2.3386, 2.4847, 2.6309, 2.7770],
    q_energy: [80.0; 15],
    q_cd: [20.0; 15],

    // Thunderous Symphony (短Q)
    q_short_dmg: [0.7146, 0.7682, 0.8217, 0.8932, 0.9468, 1.0004, 1.0718, 1.1433, 1.2148, 1.2862, 1.3577, 1.4291, 1.5184, 1.6078, 1.6971],
    q_short_energy: [30.0; 15],

    // Passives
    a1_lunarcharged_bonus: 0.20,   // 20%
    a2_em_percent: 0.08,             // 8% of ATK
    a4_base_dmg_per_100: 0.007,      // 0.7% per 100 ATK

    // Constellations
    c2_normal_dmg: 0.50,            // 50% ATK
    c2_res_shred: 0.25,             // 25%
    c4_atk_percent: 0.20,           // 20%
    c4_em_percent: 0.10,             // 10% of ATK (C4 enhanced)
    c6_lunarcharged_bonus: 0.35,    // 35%
    c6_team_bonus: 0.10,             // 10%
};

// ============================================================================
// Damage Enum
// ============================================================================

damage_enum!(
    FlinsDamageEnum
    Normal1
    Normal2
    Normal3
    Normal4_1
    Normal4_2
    Normal5
    Charged
    Plunging1
    Plunging2
    Plunging3

    E1
    E2
    E3
    E4_1
    E4_2
    E5
    ECharged
    ESpearstorm

    QInitial
    QMiddle_1
    QMiddle_2
    QFinal
    QShort

    C2Normal
);

impl FlinsDamageEnum {
    pub fn get_element(&self, in_manifest_flame: bool) -> Element {
        // In Manifest Flame mode, Normal/Charged/Plunging deal Electro
        // Otherwise, they deal Physical
        if in_manifest_flame {
            Element::Electro
        } else {
            Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use FlinsDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4_1 | Normal4_2 | Normal5 | C2Normal => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 | E3 | E4_1 | E4_2 | E5 | ECharged | ESpearstorm => SkillType::ElementalSkill,
            QInitial | QMiddle_1 | QMiddle_2 | QFinal | QShort => SkillType::ElementalBurst,
        }
    }
}

// ============================================================================
// Effect Implementation
// ============================================================================

pub struct FlinsEffect {
    pub constellation: usize,
    pub in_manifest_flame: bool,  // 是否在幽焰显迹模式
    pub moonsign_level: usize,     // 月兆等级 (1或2=满辉)
    pub has_a1: bool,              // A1 unlocked (ascension 1)
    pub has_a2: bool,             // A2 unlocked (ascension 4)
    pub has_a4: bool,              // A4 unlocked
}

impl<A: Attribute> ChangeAttribute<A> for FlinsEffect {
    fn change_attribute(&self, attribute: &mut A) {
        // A2: EM = ATK * 8%, max 160 (C4: EM = ATK * 10%, max 220)
        // Simplified: add a flat EM bonus (proper edge function would be needed for percentage)
        if self.has_a2 || self.constellation >= 4 {
            // Simplified: 100 EM base (represents ~8% of ATK for typical builds)
            // Proper implementation would use edge function
            attribute.set_value_by(AttributeName::ElementalMastery, "菲林斯天赋", 100.0);
        }

        // C4: ATK +20%
        if self.constellation >= 4 {
            attribute.add_atk_percentage("菲林斯4命", FLINS_SKILL.c4_atk_percent);
        }
    }
}

// ============================================================================
// Character Implementation
// ============================================================================

pub struct Flins;

impl CharacterTrait for Flins {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Flins,
        internal_name: "Flins",
        name_locale: locale!(
            zh_cn: "菲林斯",
            en: "Flins",
        ),
        element: Element::Electro,
        hp: [972, 2521, 3354, 5018, 5610, 6451, 7241, 8097, 8689, 9551, 10142, 11013, 11604, 12482, 13370],
        atk: [27, 70, 93, 139, 156, 179, 201, 225, 241, 265, 282, 306, 322, 347, 371],
        def: [63, 163, 217, 325, 364, 418, 470, 525, 563, 619, 657, 714, 752, 809, 867],
        sub_stat: CharacterSubStatFamily::CriticalDamage384,
        weapon_type: WeaponType::Polearm,
        star: 5,
        skill_name1: locale!(
            zh_cn: "扈圣魔枪",
            en: "Pocztowy Demonspear",
        ),
        skill_name2: locale!(
            zh_cn: "古律·孤灯遗秘",
            en: "Ancient Rite: Arcane Light",
        ),
        skill_name3: locale!(
            zh_cn: "旧仪·夜客致访",
            en: "Ancient Ritual: Cometh the Night",
        ),
    };

    type SkillType = FlinsSkillType;
    const SKILL: Self::SkillType = FLINS_SKILL;
    type DamageEnumType = FlinsDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: FlinsDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: FlinsDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: FlinsDamageEnum::Normal3 as usize, text: hit_n_dmg!(3, 1) },
            CharacterSkillMapItem { index: FlinsDamageEnum::Normal4_1 as usize, text: hit_n_dmg!(4, 1) },
            CharacterSkillMapItem { index: FlinsDamageEnum::Normal4_2 as usize, text: hit_n_dmg!(4, 2) },
            CharacterSkillMapItem { index: FlinsDamageEnum::Normal5 as usize, text: hit_n_dmg!(5) },
            CharacterSkillMapItem { index: FlinsDamageEnum::Charged as usize, text: charged_dmg!() },
            CharacterSkillMapItem { index: FlinsDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: FlinsDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: FlinsDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: FlinsDamageEnum::E1 as usize, text: locale!(zh_cn: "幽焰一段", en: "1st Hit DMG") },
            CharacterSkillMapItem { index: FlinsDamageEnum::E2 as usize, text: locale!(zh_cn: "幽焰二段", en: "2nd Hit DMG") },
            CharacterSkillMapItem { index: FlinsDamageEnum::E3 as usize, text: locale!(zh_cn: "幽焰三段", en: "3rd Hit DMG") },
            CharacterSkillMapItem { index: FlinsDamageEnum::E4_1 as usize, text: locale!(zh_cn: "幽焰四段(1)", en: "4th Hit DMG (1)") },
            CharacterSkillMapItem { index: FlinsDamageEnum::E4_2 as usize, text: locale!(zh_cn: "幽焰四段(2)", en: "4th Hit DMG (2)") },
            CharacterSkillMapItem { index: FlinsDamageEnum::E5 as usize, text: locale!(zh_cn: "幽焰五段", en: "5th Hit DMG") },
            CharacterSkillMapItem { index: FlinsDamageEnum::ECharged as usize, text: locale!(zh_cn: "幽焰重击", en: "Charged DMG") },
            CharacterSkillMapItem { index: FlinsDamageEnum::ESpearstorm as usize, text: locale!(zh_cn: "北国枪阵", en: "Northland Spearstorm DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: FlinsDamageEnum::QInitial as usize, text: locale!(zh_cn: "初始伤害", en: "Initial DMG") },
            CharacterSkillMapItem { index: FlinsDamageEnum::QMiddle_1 as usize, text: locale!(zh_cn: "月感电中段(1)", en: "Middle Phase (1)") },
            CharacterSkillMapItem { index: FlinsDamageEnum::QMiddle_2 as usize, text: locale!(zh_cn: "月感电中段(2)", en: "Middle Phase (2)") },
            CharacterSkillMapItem { index: FlinsDamageEnum::QFinal as usize, text: locale!(zh_cn: "月感电终段", en: "Final Phase DMG") },
            CharacterSkillMapItem { index: FlinsDamageEnum::QShort as usize, text: locale!(zh_cn: "雷霆交响", en: "Thunderous Symphony DMG") },
            CharacterSkillMapItem { index: FlinsDamageEnum::C2Normal as usize, text: locale!(zh_cn: "C2额外伤害", en: "C2 Additional DMG") },
        ]),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "in_manifest_flame",
            title: locale!(
                zh_cn: "幽焰显迹模式",
                en: "Manifest Flame Mode",
            ),
            config: ItemConfigType::Bool { default: false },
        },
        ItemConfig {
            name: "moonsign_level",
            title: locale!(
                zh_cn: "月兆等级",
                en: "Moonsign Level",
            ),
            config: ItemConfigType::Int { min: 1, max: 2, default: 1 },
        },
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, _fumo: Option<Element>) -> D::Result {
        let s: FlinsDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();
        let _constellation = context.character_common_data.constellation as usize;

        let in_manifest_flame = match *config {
            CharacterSkillConfig::Flins { in_manifest_flame, .. } => in_manifest_flame,
            _ => false
        };

        let mut ratio: f64 = 0.0;

        use FlinsDamageEnum::*;

        match s {
            // Normal Attack
            Normal1 => ratio = FLINS_SKILL.normal_dmg1[s1],
            Normal2 => ratio = FLINS_SKILL.normal_dmg2[s1],
            Normal3 => ratio = FLINS_SKILL.normal_dmg3[s1],
            Normal4_1 => ratio = FLINS_SKILL.normal_dmg4[s1],
            Normal4_2 => ratio = FLINS_SKILL.normal_dmg4[s1],
            Normal5 => ratio = FLINS_SKILL.normal_dmg5[s1],
            Charged => ratio = FLINS_SKILL.charged_dmg[s1],
            Plunging1 => ratio = FLINS_SKILL.plunging_dmg1[s1],
            Plunging2 => ratio = FLINS_SKILL.plunging_dmg2[s1],
            Plunging3 => ratio = FLINS_SKILL.plunging_dmg3[s1],

            // Elemental Skill
            E1 => ratio = FLINS_SKILL.e_dmg1[s2],
            E2 => ratio = FLINS_SKILL.e_dmg2[s2],
            E3 => ratio = FLINS_SKILL.e_dmg3[s2],
            E4_1 => ratio = FLINS_SKILL.e_dmg4[s2],
            E4_2 => ratio = FLINS_SKILL.e_dmg4[s2],
            E5 => ratio = FLINS_SKILL.e_dmg5[s2],
            ECharged => ratio = FLINS_SKILL.e_charged_dmg[s2],
            ESpearstorm => ratio = FLINS_SKILL.e_spearstorm_dmg[s2],

            // Elemental Burst
            QInitial => ratio = FLINS_SKILL.q_initial_dmg[s3],
            QMiddle_1 => ratio = FLINS_SKILL.q_middle_dmg[s3],
            QMiddle_2 => ratio = FLINS_SKILL.q_middle_dmg[s3],
            QFinal => ratio = FLINS_SKILL.q_final_dmg[s3],
            QShort => ratio = FLINS_SKILL.q_short_dmg[s3],

            // C2 - Normal Attack type
            C2Normal => ratio = FLINS_SKILL.c2_normal_dmg,
        }

        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(in_manifest_flame),
            s.get_skill_type(),
            context.character_common_data.level,
            None,
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        let constellation = common_data.constellation as usize;

        // Parse config for Manifest Flame mode
        let in_manifest_flame = match *config {
            CharacterConfig::Flins { in_manifest_flame, .. } => in_manifest_flame,
            _ => false
        };

        let moonsign_level = match *config {
            CharacterConfig::Flins { moonsign_level, .. } => moonsign_level,
            _ => 1
        };

        Some(Box::new(FlinsEffect {
            constellation,
            in_manifest_flame,
            moonsign_level,
            has_a1: common_data.has_talent1,  // A1 unlocked at ascension 1
            has_a2: common_data.has_talent2, // A2 unlocked at ascension 4 (uses has_talent2)
            has_a4: common_data.ascend,       // A4 unlocked at full ascension
        }))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        use crate::target_functions::target_functions::electro::flins_default::FlinsDefaultTargetFunction;
        use crate::target_functions::TargetFunctionConfig;

        let config = match role_index {
            0 => TargetFunctionConfig::FlinsDefault {
                recharge_requirement: 1.2,
                manifest_flame: true,
                moonsign_level: 2,
                lunar_charged_rate: 0.4,
                normal_attack_rate: 0.3,
                burst_rate: 0.3,
            },
            _ => TargetFunctionConfig::FlinsDefault {
                recharge_requirement: 1.2,
                manifest_flame: true,
                moonsign_level: 2,
                lunar_charged_rate: 0.4,
                normal_attack_rate: 0.3,
                burst_rate: 0.3,
            }
        };

        FlinsDefaultTargetFunction::create(&_c, &_w, &config)
    }
}
