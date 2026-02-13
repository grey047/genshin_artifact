use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::macros::{damage_enum, damage_ratio, skill_map, skill_type};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::common::i18n::{charged_dmg, hit_n_dmg, locale, plunging_dmg};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::{ChangeAttribute, Element, SkillType, StatName, WeaponType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::{TargetFunction, TargetFunctionConfig};
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

// Auto-generated from Research Data
// Character: Lauma (Dendro / Catalyst)

pub struct LaumaSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub charged_dmg: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],
    pub e_dmg1: [f64; 15],
    pub e_dmg2: [f64; 15],
    pub e_dmg3: [f64; 15],
    pub q_dmg1: [f64; 15],
    pub pale_hymn_bloom: [f64; 15],  // Bloom/Hyperbloom/Burgeon
    pub pale_hymn_lunar: [f64; 15],  // Lunar-Bloom
}

pub const LAUMA_SKILL: LaumaSkillType = LaumaSkillType {
    // Normal Attack Multipliers (Peregrination of Linnunrata)
    normal_dmg1: [
        0.432, 0.467, 0.502, 0.552, 0.587, 0.622, 0.672, 0.722, 0.772, 0.822, 0.872, 0.922, 0.972,
        1.022, 1.072,
    ],
    normal_dmg2: [
        0.384, 0.415, 0.446, 0.491, 0.522, 0.553, 0.598, 0.643, 0.688, 0.733, 0.778, 0.823, 0.868,
        0.913, 0.958,
    ],
    normal_dmg3: [
        0.528, 0.570, 0.612, 0.673, 0.715, 0.757, 0.819, 0.881, 0.943, 1.005, 1.067, 1.129, 1.191,
        1.253, 1.315,
    ],
    normal_dmg4: [
        0.528, 0.570, 0.612, 0.673, 0.715, 0.757, 0.819, 0.881, 0.943, 1.005, 1.067, 1.129, 1.191,
        1.253, 1.315,
    ],
    charged_dmg: [
        0.528, 0.570, 0.612, 0.673, 0.715, 0.757, 0.819, 0.881, 0.943, 1.005, 1.067, 1.129, 1.191,
        1.253, 1.315,
    ],
    // Plunging Attack Multipliers (standard Catalyst values)
    plunging_dmg1: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
    plunging_dmg2: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
    plunging_dmg3: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.602, 3.8248, 4.0476, 4.2704,
    ],
    // Elemental Skill Multipliers (Runo: Dawnless Rest of Karsikko)
    e_dmg1: [
        2.048, 2.214, 2.380, 2.618, 2.784, 2.950, 3.188, 3.426, 3.664, 3.974, 4.284, 4.594, 4.904,
        5.214, 5.524,
    ],
    e_dmg2: [
        // 2-Hit Hold DMG - EM per Verdant Dew (Lunar-Bloom DMG)
        // Lv1: 152%, Lv10: 273.6%, Lv13: 323%, Lv15: 361%
        1.52, 1.634, 1.748, 1.90, 2.014, 2.128, 2.28, 2.432, 2.584, 2.736, 2.888, 3.04, 3.23, 3.42, 3.61,
    ],
    e_dmg3: [
        // Frostgrove Sanctuary: ATK% + EM% (EM% = ATK% × 2)
        // Lv1: 96% ATK + 192% EM, Lv10: 172.8% ATK + 345.6% EM, Lv13: 204% ATK + 408% EM
        0.96, 1.032, 1.104, 1.20, 1.272, 1.344, 1.44, 1.536, 1.632, 1.728, 1.824, 1.92, 2.04, 2.16, 2.28,
    ],
    // Elemental Burst Multipliers (Runo: All Hearts Become the Beating Moon)
    q_dmg1: [
        4.160, 4.496, 4.832, 5.312, 5.648, 5.984, 6.464, 6.944, 7.424, 8.048, 8.672, 9.296, 9.920,
        10.544, 11.168,
    ],
    // Pale Hymn: Bloom/Hyperbloom/Burgeon DMG Increase (% EM)
    // Lv1: 277.76%, Lv10: 499.97%, Lv13: 590.24%, Lv15: 659.68%
    pale_hymn_bloom: [
        2.7776, 2.9859, 3.1942, 3.4720, 3.6803, 3.8886, 4.1664, 4.4442, 4.7219, 4.9997,
        5.2774, 5.5552, 5.9024, 6.2496, 6.5968,
    ],
    // Pale Hymn: Lunar-Bloom DMG Increase (% EM)
    // Lv1: 222.24%, Lv10: 400.03%, Lv13: 472.26%, Lv15: 527.82%
    pale_hymn_lunar: [
        2.2224, 2.3891, 2.5558, 2.7780, 2.9447, 3.1114, 3.3336, 3.5558, 3.7781, 4.0003,
        4.2226, 4.4448, 4.7226, 5.0004, 5.2782,
    ],
};

damage_enum!(
    LaumaDamageEnum
    Normal1
    Normal2
    Normal3
    Normal4
    Charged
    Plunging1
    Plunging2
    Plunging3
    E1
    E2
    E3
    Q1
);

impl LaumaDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use LaumaDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 => SkillType::NormalAttack,
            Charged => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 | E3 => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst,
        }
    }
}

pub struct LaumaEffect {
    pub moonsign_level: usize,
    pub has_c2: bool,
    pub spirit_envoy_count: usize, // 草露数量 (1-3)
    pub has_c6: bool,
    pub pale_hymn_stacks: usize,   // 苍色岛格层数 (0-18, from Q)
    pub skill_level_q: usize,      // Q 技能等级 (用于 Pale Hymn 加成)
}

impl<A: Attribute> ChangeAttribute<A> for LaumaEffect {
    fn change_attribute(&self, attribute: &mut A) {
        let em = attribute.get_value(AttributeName::ElementalMastery);

        // A3: Moonsign Benediction: Nature's Chorus
        // EM × 0.0175% Lunar-Bloom Base DMG, max 14%
        let a3_bonus = (em * 0.000175).min(0.14);
        attribute.set_value_by(AttributeName::LunarBloomBaseDmg, "A3: 月兆祝赐·林风迷踪", a3_bonus);

        // A4: Cleansing for the Spring
        // EM × 0.04% E Skill DMG, max 32%
        let a4_bonus = (em * 0.0004).min(0.32);
        attribute.set_value_by(AttributeName::BonusElementalSkill, "A4: 奉向甘泉的沐濯", a4_bonus);

        // A1: Light for the Frosty Night
        // Moonsign: Ascendant Gleam - Lunar-Bloom Crit Rate +10%, Crit DMG +20%
        if self.moonsign_level >= 2 {
            attribute.set_value_by(AttributeName::LunarBloomCritRate, "A1: 月兆·满辉", 0.10);
            attribute.set_value_by(AttributeName::LunarBloomCritDMG, "A1: 月兆·满辉", 0.20);
        }

        // C2: Twine Warnings and Tales From the North
        // Lunar-Bloom DMG +40% when Moonsign: Ascendant Gleam
        if self.has_c2 && self.moonsign_level >= 2 {
            attribute.set_value_by(AttributeName::EnhanceLunarBloom, "C2: 月兆·满辉", 0.40);
        }

        // C6: I Offer Blood and Tears to the Moonlight
        // Elevated Lunar-Bloom DMG +25% when Moonsign: Ascendant Gleam
        if self.has_c6 && self.moonsign_level >= 2 {
            attribute.set_value_by(AttributeName::ElevateLunarBloom, "C6: 月兆·满辉", 0.25);
        }

        // Q: Pale Hymn - Party-wide reaction DMG buffs based on EM
        // Each stack provides full bonus, consumed on reaction
        if self.pale_hymn_stacks > 0 {
            let s3 = (self.skill_level_q - 1).min(14);
            
            // Base Pale Hymn values (C0)
            let bloom_ratio = LAUMA_SKILL.pale_hymn_bloom[s3];   // Bloom/Hyperbloom/Burgeon
            let lunar_ratio = LAUMA_SKILL.pale_hymn_lunar[s3];   // Lunar-Bloom
            
            // C2: Additional +500% EM for Bloom/Hyperbloom/Burgeon, +400% EM for Lunar-Bloom
            let (c2_bloom_bonus, c2_lunar_bonus) = if self.has_c2 {
                (5.0, 4.0)  // +500% EM, +400% EM
            } else {
                (0.0, 0.0)
            };
            
            // Total EM ratios
            let total_bloom_ratio = bloom_ratio + c2_bloom_bonus;
            let total_lunar_ratio = lunar_ratio + c2_lunar_bonus;
            
            // Apply as base DMG bonus (EM × ratio × stacks)
            // Note: Each reaction consumes 1 stack, but here we model max potential
            let bloom_dmg = em * total_bloom_ratio * self.pale_hymn_stacks as f64;
            let lunar_dmg = em * total_lunar_ratio * self.pale_hymn_stacks as f64;
            
            if bloom_dmg > 0.0 {
                // Use a custom attribute or existing one for reaction base DMG
                attribute.set_value_by(AttributeName::BloomBaseDmg, "Q: 苍色岛格", bloom_dmg);
                attribute.set_value_by(AttributeName::HyperbloomBaseDmg, "Q: 苍色岛格", bloom_dmg);
                attribute.set_value_by(AttributeName::BurgeonBaseDmg, "Q: 苍色岛格", bloom_dmg);
            }
            
            if lunar_dmg > 0.0 {
                attribute.set_value_by(AttributeName::LunarBloomBaseDmg, "Q: 苍色岛格", lunar_dmg);
            }
        }
    }
}

pub struct Lauma;

impl CharacterTrait for Lauma {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Lauma,
        internal_name: "Lauma",
        name_locale: locale!(
            zh_cn: "菈乌玛",
            en: "Lauma",
        ),
        element: Element::Dendro,
        // Base HP: 829 (Lv1), ATK: 20 (Lv1), DEF: 52 (Lv1)
        // Verified against AvatarExcelConfigData
        hp: [
            829, 2151, 2861, 4280, 4785, 5505, 6178, 6906, 7410, 8146, 8650, 9393, 9897, 10646,
            11403,
        ],
        atk: [
            20, 52, 69, 103, 115, 133, 149, 167, 179, 197, 209, 227, 239, 257, 275,
        ],
        def: [
            52, 135, 180, 269, 300, 345, 388, 433, 465, 511, 543, 589, 621, 668, 715,
        ],
        sub_stat: CharacterSubStatFamily::ElementalMastery115,
        weapon_type: WeaponType::Catalyst,
        star: 5,
        skill_name1: locale!(
            zh_cn: "林纳塔之旅",
            en: "Peregrination of Linnunrata",
        ),
        skill_name2: locale!(
            zh_cn: "卢诺：卡尔西科的无尽休憩",
            en: "Runo: Dawnless Rest of Karsikko",
        ),
        skill_name3: locale!(
            zh_cn: "卢诺：万心成为跳动之月",
            en: "Runo: All Hearts Become the Beating Moon",
        ),
    };
    type SkillType = LaumaSkillType;
    const SKILL: Self::SkillType = LAUMA_SKILL;
    type DamageEnumType = LaumaDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            LaumaDamageEnum
            Normal1 hit_n_dmg!(1)
            Normal2 hit_n_dmg!(2)
            Normal3 hit_n_dmg!(3)
            Normal4 hit_n_dmg!(4)
            Charged charged_dmg!()
            Plunging1 plunging_dmg!(1)
            Plunging2 plunging_dmg!(2)
            Plunging3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            LaumaDamageEnum
            E1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
            E2 locale!(zh_cn: "强化伤害", en: "Enhanced DMG")
            E3 locale!(zh_cn: "灵菇伤害", en: "Spirit Envoy DMG")
        ),
        skill3: skill_map!(
            LaumaDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
        ),
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
            name: "spirit_envoy_count",
            title: locale!(
                zh_cn: "草露数量",
                en: "Spirit Envoy Count"
            ),
            config: ItemConfigType::Int { min: 0, max: 3, default: 3 },
        },
        ItemConfig {
            name: "pale_hymn_stacks",
            title: locale!(
                zh_cn: "苍色岛格层数(C6)",
                en: "Pale Hymn Stacks (C6)"
            ),
            config: ItemConfigType::Int { min: 0, max: 18, default: 0 },
        },
    ]);

    fn damage_internal<D: DamageBuilder>(
        context: &DamageContext<'_, D::AttributeType>,
        s: usize,
        config: &CharacterSkillConfig,
        fumo: Option<Element>,
    ) -> D::Result {
        let skill: LaumaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let mut builder = D::new();

        // Extract config values
        let (spirit_count, pale_hymn_stacks) = if let CharacterSkillConfig::Lauma { spirit_envoy_count, pale_hymn_stacks } = config {
            (*spirit_envoy_count as f64, *pale_hymn_stacks)
        } else {
            (3.0, 0)
        };

        match skill {
            // E2: Hold - 2-Hit Hold DMG (Lunar-Bloom DMG based on EM × ratio × Verdant Dew count)
            // From AnimeGameData: params[2] = 152%-361% EM per Verdant Dew
            LaumaDamageEnum::E2 => {
                // E2 Hold: EM × skill_ratio × spirit_count
                // Lv1: 152%, Lv10: 273.6%, Lv13: 323%, Lv15: 361%
                let skill_ratio = LAUMA_SKILL.e_dmg2[s2];
                let em_ratio = skill_ratio * spirit_count;
                builder.add_em_ratio("E2 Lunar Bloom Hold", em_ratio);
            }
            // C6: Normal attacks with Pale Hymn consume 1 stack to deal Lunar-Bloom DMG
            LaumaDamageEnum::Normal1 | LaumaDamageEnum::Normal2 | 
            LaumaDamageEnum::Normal3 | LaumaDamageEnum::Normal4 => {
                let ratio = match skill {
                    LaumaDamageEnum::Normal1 => LAUMA_SKILL.normal_dmg1[s1],
                    LaumaDamageEnum::Normal2 => LAUMA_SKILL.normal_dmg2[s1],
                    LaumaDamageEnum::Normal3 => LAUMA_SKILL.normal_dmg3[s1],
                    LaumaDamageEnum::Normal4 => LAUMA_SKILL.normal_dmg4[s1],
                    _ => 0.0,
                };
                
                // Base ATK damage
                builder.add_atk_ratio("Normal Attack", ratio);
                
                // C6: If has Pale Hymn stacks, add EM-based Lunar-Bloom damage
                if pale_hymn_stacks > 0 {
                    // C6: 150% EM as Lunar-Bloom DMG
                    builder.add_em_ratio("C6 Lunar Bloom", 1.5);
                }
            }
            _ => {
                // Normal skills use ATK
                let ratio = match skill {
                    LaumaDamageEnum::Charged => LAUMA_SKILL.charged_dmg[s1],
                    LaumaDamageEnum::Plunging1 => LAUMA_SKILL.plunging_dmg1[s1],
                    LaumaDamageEnum::Plunging2 => LAUMA_SKILL.plunging_dmg2[s1],
                    LaumaDamageEnum::Plunging3 => LAUMA_SKILL.plunging_dmg3[s1],
                    LaumaDamageEnum::E1 => LAUMA_SKILL.e_dmg1[s2],
                    LaumaDamageEnum::Q1 => LAUMA_SKILL.q_dmg1[s3],
                    _ => 0.0,
                };
                
                if ratio > 0.0 {
                    builder.add_atk_ratio("Skill Ratio", ratio);
                }
                
                // E3: Frostgrove Sanctuary - ATK + EM based
                if skill == LaumaDamageEnum::E3 {
                    // ATK part already added above, now add EM part
                    // Lv1: 192% EM, Lv10: 345.6% EM, Lv13: 408% EM, Lv15: 456% EM
                    let em_ratio = LAUMA_SKILL.e_dmg3[s2] * 2.0; // EM ratio is 2x the stored value (which is ATK%)
                    builder.add_em_ratio("Frostgrove Sanctuary EM", em_ratio);
                    
                    // C6: Additional 185% EM as Lunar-Bloom DMG
                    let constellation = context.character_common_data.constellation;
                    if constellation >= 6 {
                        builder.add_em_ratio("C6 Frostgrove Extra", 1.85);
                    }
                }
            }
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            Element::Dendro,
            skill.get_skill_type(),
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(
        common_data: &CharacterCommonData,
        config: &CharacterConfig,
    ) -> Option<Box<dyn ChangeAttribute<A>>> {
        let (moonsign_level, spirit_envoy_count, pale_hymn_stacks) = match *config {
            CharacterConfig::Lauma { moonsign_level, spirit_envoy_count, pale_hymn_stacks } => (moonsign_level, spirit_envoy_count, pale_hymn_stacks),
            _ => (2, 3, 0)
        };
        Some(Box::new(LaumaEffect {
            moonsign_level,
            has_c2: common_data.constellation >= 2,
            spirit_envoy_count,
            has_c6: common_data.constellation >= 6,
            pale_hymn_stacks,
            skill_level_q: common_data.skill3,
        }))
    }

    fn get_target_function_by_role(
        _role_index: usize,
        _team: &TeamQuantization,
        _c: &CharacterCommonData,
        _w: &WeaponCommonData,
    ) -> Box<dyn TargetFunction> {
        use crate::target_functions::target_function_config::TargetFunctionConfig;
        Box::new(crate::target_functions::target_functions::dendro::lauma_default::LaumaDefaultTargetFunction::new(&TargetFunctionConfig::LaumaDefault {
            recharge_demand: 1.0,
            use_skill: 0.5,
            use_burst: 0.5,
            moonsign_level: 2,
        }))
    }
}
