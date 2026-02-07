use super::character_static_data::{CharacterStaticData};
use super::character_name::CharacterName;
use super::character_sub_stat::{CharacterSubStat};
use crate::attribute::{AttributeName, Attribute};
use crate::common::ChangeAttribute;
use super::characters::get_static_data;


pub struct CharacterCommonData {
    pub name: CharacterName,
    pub level: usize,
    pub ascend: bool,
    pub constellation: i32,
    pub base_atk: f64,
    pub base_def: f64,
    pub base_hp: f64,
    pub has_talent1: bool,
    pub has_talent2: bool,
    pub skill1: usize,
    pub skill2: usize,
    pub skill3: usize,

    pub static_data: CharacterStaticData,
}

impl CharacterCommonData {
    pub fn new(
        name: CharacterName,
        level: usize,
        ascend: bool,
        constellation: i32,
        skill1: usize,
        skill2: usize,
        skill3: usize
    ) -> CharacterCommonData {
        let data = get_static_data(name);

        let base_atk = CharacterCommonData::base_value(&data.atk, level, ascend);
        let base_def = CharacterCommonData::base_value(&data.def, level, ascend);
        let base_hp = CharacterCommonData::base_value(&data.hp, level, ascend);

        CharacterCommonData {
            name,
            level, ascend, constellation,
            base_atk, base_def, base_hp,
            static_data: data,
            has_talent1: (level == 20 && ascend) || level > 20,
            has_talent2: (level == 60 && ascend) || level > 60,
            skill1,
            skill2,
            skill3,
        }
    }

    pub fn base_value(array: &[i32; 15], level: usize, _ascend: bool) -> f64 {
        // 数组结构: [Lv1, Lv20, Lv20+, Lv40, Lv40+, Lv50, Lv50+, Lv60, Lv60+, Lv70, Lv70+, Lv80, Lv80+, Lv90, Lv100]
        // 索引:       0     1      2      3      4      5      6      7      8      9      10     11     12     13      14
        // 注意: Lv90 以后没有突破，所以 Lv91-100 使用线性插值
        
        if level < 20 {
            (level - 1) as f64 * (array[1] - array[0]) as f64 / 19.0 + array[0] as f64
        } else if level < 40 {
            (level - 20) as f64 * (array[3] - array[2]) as f64 / 20.0 + array[2] as f64
        } else if level < 50 {
            (level - 40) as f64 * (array[5] - array[4]) as f64 / 10.0 + array[4] as f64
        } else if level < 60 {
            (level - 50) as f64 * (array[7] - array[6]) as f64 / 10.0 + array[6] as f64
        } else if level < 70 {
            (level - 60) as f64 * (array[9] - array[8]) as f64 / 10.0 + array[8] as f64
        } else if level < 80 {
            (level - 70) as f64 * (array[11] - array[10]) as f64 / 10.0 + array[10] as f64
        } else if level < 90 {
            (level - 80) as f64 * (array[13] - array[12]) as f64 / 10.0 + array[12] as f64
        } else if level <= 100 {
            // Lv90 到 Lv100: 线性插值
            let lv90 = array[13] as f64;
            let lv100 = array[14] as f64;
            (level - 90) as f64 * (lv100 - lv90) / 10.0 + lv90
        } else {
            array[14] as f64
        }
    }

    pub fn get_3_skill(&self) -> (usize, usize, usize) {
        (self.skill1, self.skill2, self.skill3)
    }
}

impl<T: Attribute> ChangeAttribute<T> for CharacterCommonData {
    fn change_attribute(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::ATKBase, "角色基础攻击", self.base_atk);
        attribute.set_value_by(AttributeName::DEFBase, "角色基础防御", self.base_def);
        attribute.set_value_by(AttributeName::HPBase, "角色基础生命", self.base_hp);

        let sub_stat = CharacterSubStat::new(self.static_data.sub_stat, self.level, self.ascend);
        sub_stat.change_attribute(attribute);
    }
}