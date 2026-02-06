use serde::{Serialize, Deserialize};
use crate::common::Element;

#[derive(Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
pub enum TransformativeType {
    SwirlCryo,
    SwirlHydro,
    SwirlElectro,
    SwirlPyro,
    Superconduct,
    Overload,
    Burning,
    ElectroCharged,
    Shatter,

    Bloom,
    // 烈绽放
    Burgeon,
    // 超绽放
    Hyperbloom,
    
    // 月反应 (掷德卡莱)
    // 月岩充电
    LunarCharged,
    // 月华
    LunarBloom,
    // 月结晶
    LunarCrystallize,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
pub enum AmplifyingType {
    Melt(Element),
    Vaporize(Element)
}
