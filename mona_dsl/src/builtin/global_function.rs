use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use mona::character::CharacterName;
use crate::common::UnsafeDamageContext;
use crate::error::runtime_error::{RuntimeError, RuntimeErrorEnum};
use crate::object::builtin_function::{FunctionReturnType, MonaObjectBuiltinFunction, ParamVecType};
use crate::object::hexenzirkel::HexenzirkelBuffs;
use crate::object::mona_object::{MonaObject, MonaObjectEnum};
use crate::vm::env::MonaEnv;
use crate::vm::namespace::Namespace;

fn mona_print_internal(obj: &MonaObject, env: &mut MonaEnv) -> Result<(), RuntimeError> {
    match &obj.data {
        MonaObjectEnum::Number(x) => env.ostream.append_str(&format!("MONA: {}", x.value)),
        MonaObjectEnum::String(x) => env.ostream.append_str(&format!("MONA: {}", x.value)),
        MonaObjectEnum::BuiltinFunction(x) => env.ostream.append_str(&format!("MONA: [[function `{}`]]", x.name)),
        MonaObjectEnum::DamageNumber(x) => env.ostream.append_str(&format!("MONA: {:?}", x)),
        MonaObjectEnum::Bool(x) => env.ostream.append_str(&format!("MONA: {}", x.value)),
        // MonaObjectEnum::TransformativeDamage(x) => env.ostream.append_str(&format!("MONA: {:?}", x.damage)),
        _ => {
            return Err(RuntimeError::new(RuntimeErrorEnum::NotSupported, &format!("print type `{}` not implelented", obj.get_type())));
        }
    }

    Ok(())
}

pub fn mona_print(params: ParamVecType, env: &mut MonaEnv) -> FunctionReturnType {
    for item in params.iter() {
        mona_print_internal(&*item.borrow(), env)?;
    }

    Ok(None)
}

pub fn mona_type(params: ParamVecType, _env: &mut MonaEnv) -> FunctionReturnType {
    let item = params[0].clone();
    let name = item.borrow().get_type();

    let s = MonaObject::new_string(Rc::new(String::from(name)));

    Ok(Some(Rc::new(RefCell::new(s))))
}

pub fn mona_max(params: ParamVecType, _env: &mut MonaEnv) -> FunctionReturnType {
    let mut result = -f64::INFINITY;
    for item in params.iter() {
        let v = item.borrow().assert_number()?;
        if v > result {
            result = v;
        }
    }

    let obj = MonaObject::new_number(result);
    Ok(Some(Rc::new(RefCell::new(obj))))
}

pub fn mona_min(params: ParamVecType, _env: &mut MonaEnv) -> FunctionReturnType {
    let mut result = f64::INFINITY;
    for item in params.iter() {
        let v = item.borrow().assert_number()?;
        if v < result {
            result = v;
        }
    }

    let obj = MonaObject::new_number(result);
    Ok(Some(Rc::new(RefCell::new(obj))))
}

pub fn mona_select(params: ParamVecType, _env: &mut MonaEnv) -> FunctionReturnType {
    if params.len() != 3 {
        return Err(RuntimeError::new(RuntimeErrorEnum::ParamError, &format!("requiring exact 3 params, got {}", params.len())));
    }
    let flag = params[0].borrow().assert_bool()?;

    let obj = if flag {
        params[1].clone()
    } else {
        params[2].clone()
    };

    return Ok(Some(obj))
}

pub fn mona_abs(params: ParamVecType, _env: &mut MonaEnv) -> FunctionReturnType {
    if params.len() != 1 {
        return Err(RuntimeError::new(RuntimeErrorEnum::ParamError, &format!("requiring 1 param, got {}", params.len())));
    }

    let obj = &params[0];
    let number = obj.borrow().assert_number()?;

    let obj = MonaObject::new_number(number.abs());
    Ok(Some(Rc::new(RefCell::new(obj))))
}

pub fn get_moonsign_level(_params: ParamVecType, env: &mut MonaEnv) -> FunctionReturnType {
    // Get moonsign_level from the first available damage context
    if let Some((_, ctx)) = env.damage_ctx.iter().next() {
        let moonsign_level = ctx.moonsign_level as f64;
        let obj = MonaObject::new_number(moonsign_level);
        return Ok(Some(Rc::new(RefCell::new(obj))));
    }
    Err(RuntimeError::new(RuntimeErrorEnum::CharacterContextNotFound, "no damage context available"))
}

pub fn get_hexenzirkel_buff(params: ParamVecType, env: &mut MonaEnv) -> FunctionReturnType {
    // Get hexenzirkel buff for a specific character
    // Format: get_hexenzirkel_buff("CharacterName")
    if params.len() != 1 {
        return Err(RuntimeError::new(RuntimeErrorEnum::ParamError, &format!("requiring 1 param, got {}", params.len())));
    }

    let borrowed = params[0].borrow();
    let char_name = borrowed.get_string();
    
    // Get current character's constellation level and check Hexenzirkel count
    let mut constellation: u8 = 0;
    let mut team_hexenzirkel_count = 0;
    
    if let Some((_, ctx)) = env.damage_ctx.iter().next() {
        constellation = unsafe { (*ctx.character_common_data).constellation as u8 };
    }
    
    team_hexenzirkel_count = env.damage_ctx.values().filter(|ctx| ctx.is_hexenzirkel).count() as u8;

    // Only apply buffs if 2+ Hexenzirkel members
    if team_hexenzirkel_count < 2 {
        let obj = MonaObject::new_number(0.0);
        return Ok(Some(Rc::new(RefCell::new(obj))));
    }

    let buff_value = match char_name {
        // Venti: Anemo bonus and vortex damage
        // C0: 10%, C2: 12%, C4: 14% + 20% vortex, C6: 15% + 25% vortex
        "Venti" => {
            let mut buffs = HexenzirkelBuffs::new();
            buffs.calculate_venti(constellation);
            buffs.venti_normal_bonus + buffs.venti_burst_bonus
        }
        // Klee: Charged attack boost with Boom stacks
        // Base: 12%, C1: 15%, C4: 18%, C6: 25% + stacks
        "Klee" => {
            let mut buffs = HexenzirkelBuffs::new();
            buffs.calculate_klee(constellation, 2); // Assume 2 boom stacks
            buffs.klee_charged_bonus
        }
        // Albedo: DEF-based damage bonus for team
        // 10-15% of DEF converted to DMG Bonus
        "Albedo" => {
            let mut buffs = HexenzirkelBuffs::new();
            let def = if let Some(ctx) = env.damage_ctx.get(&CharacterName::Albedo) {
                unsafe { (*ctx.character_common_data).base_def }
            } else {
                800.0 // Default Albedo DEF
            };
            buffs.calculate_albedo(constellation, def);
            buffs.albedo_def_bonus
        }
        // Mona: Illusory Torrent enhanced dash + amp
        // C0: 8%, C2: 12%, C4: 15%, C6: 20%
        "Mona" => {
            let mut buffs = HexenzirkelBuffs::new();
            buffs.calculate_mona(constellation);
            buffs.mona_amp_bonus
        }
        // Fischl: Oz attack frequency + C6 crit
        // C0: 10% freq, C6: 60% crit
        "Fischl" => {
            let mut buffs = HexenzirkelBuffs::new();
            buffs.calculate_fischl(constellation);
            // Convert frequency boost to damage equivalent
            (buffs.fischl_oz_frequency - 1.0) * 2.0 + buffs.fischl_crit_bonus * 0.5
        }
        // Razor: Constellation bonuses
        // C0: 8%, C2: 12%, C6: 18% + 30% crit
        "Razor" => {
            let mut buffs = HexenzirkelBuffs::new();
            buffs.calculate_razor(constellation);
            buffs.razor_constellation_bonus + buffs.razor_c6_bonus * 0.5
        }
        // Sucrose: Elemental damage bonuses on reactions
        // C0: 12%, C2: 20%, C6: 25% + EM bonus
        "Sucrose" => {
            let mut buffs = HexenzirkelBuffs::new();
            buffs.calculate_sucrose(constellation);
            (buffs.sucrose_em_bonus / 1000.0) + buffs.sucrose_elemental_bonus
        }
        _ => 0.0,
    };

    let obj = MonaObject::new_number(buff_value);
    Ok(Some(Rc::new(RefCell::new(obj))))
}

pub fn team_hexenzirkel_count(_params: ParamVecType, env: &mut MonaEnv) -> FunctionReturnType {
    // Count Hexenzirkel members across all team damage contexts
    let count = env.damage_ctx.values().filter(|ctx| ctx.is_hexenzirkel).count() as f64;
    let obj = MonaObject::new_number(count);
    Ok(Some(Rc::new(RefCell::new(obj))))
}

pub fn is_hexenzirkel_member(_params: ParamVecType, env: &mut MonaEnv) -> FunctionReturnType {
    // Check if current character is a Hexenzirkel member
    if let Some((_, ctx)) = env.damage_ctx.iter().next() {
        let is_member = if ctx.is_hexenzirkel { 1.0 } else { 0.0 };
        let obj = MonaObject::new_number(is_member);
        return Ok(Some(Rc::new(RefCell::new(obj))));
    }
    Err(RuntimeError::new(RuntimeErrorEnum::CharacterContextNotFound, "no damage context available"))
}

macro insert_global($m:ident, $name:expr, $func:ident) {
    let t = MonaObjectBuiltinFunction {
        name: String::from($name),
        handler: Box::new($func)
    };
    ($m).insert(String::from($name), Rc::new(RefCell::new(MonaObject {
        data: MonaObjectEnum::BuiltinFunction(t)
    })));
}

pub fn setup_global_namespace() -> Namespace {
    let mut map = HashMap::new();

    insert_global!(map, "print", mona_print);
    insert_global!(map, "type", mona_type);
    insert_global!(map, "max", mona_max);
    insert_global!(map, "min", mona_min);
    insert_global!(map, "select", mona_select);
    insert_global!(map, "abs", mona_abs);
    insert_global!(map, "get_moonsign_level", get_moonsign_level);
    insert_global!(map, "get_hexenzirkel_buff", get_hexenzirkel_buff);
    insert_global!(map, "team_hexenzirkel_count", team_hexenzirkel_count);
    insert_global!(map, "is_hexenzirkel_member", is_hexenzirkel_member);

    Namespace {
        map
    }
}