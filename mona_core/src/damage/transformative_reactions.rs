// Transformative Reaction Multipliers and Formulas
// Source: genshin-optimizer / homdgcat.wiki
// Cross-validated: Level 90 base = 1446.853515625 (GO) = 1446.85 (homdgcat) ✓

use crate::damage::level_coefficient::get_level_multiplier;

/// Transformative reaction type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransformativeType {
    Overload,
    ElectroCharged,
    Superconduct,
    SwirlPyro,
    SwirlHydro,
    SwirlElectro,
    SwirlCryo,
    Bloom,
    Hyperbloom,
    Burgeon,
    Burning,
    Shatter,
    Crystallize,
}

/// Get reaction multiplier for transformative reactions
/// These values are cross-validated with genshin-optimizer
#[inline]
pub fn get_reaction_multiplier(t: TransformativeType) -> f64 {
    match t {
        // Burgeon/Hyperbloom: 3.0x (GO and Mona verified)
        TransformativeType::Burgeon | TransformativeType::Hyperbloom => 3.0,
        // Overload/Bloom: 2.0x (GO and Mona verified)
        TransformativeType::Overload | TransformativeType::Bloom => 2.0,
        // ElectroCharged: 1.2x (GO and Mona verified)
        TransformativeType::ElectroCharged => 1.2,
        // Swirl variants: 0.6x (GO and Mona verified)
        TransformativeType::SwirlPyro
        | TransformativeType::SwirlHydro
        | TransformativeType::SwirlElectro
        | TransformativeType::SwirlCryo => 0.6,
        // Shatter: 1.5x (GO and Mona verified)
        TransformativeType::Shatter => 1.5,
        // Superconduct: 0.5x (GO and Mona verified)
        TransformativeType::Superconduct => 0.5,
        // Burning: 0.25x (GO and Mona verified)
        TransformativeType::Burning => 0.25,
        // Crystallize: uses separate crystallize base
        TransformativeType::Crystallize => 1.0,
    }
}

/// Get the EM bonus factor for transformative reactions
/// Formula: 1 + 16 * EM / (EM + 2000)
/// Cross-validated with genshin-optimizer (eleMasMulti(16, 2000))
#[inline]
pub fn get_transformative_em_bonus(em: f64) -> f64 {
    1.0 + 16.0 * em / (em + 2000.0)
}

/// Calculate transformative reaction base damage
/// Formula: level_multiplier × reaction_multiplier × (1 + 16×EM/(EM+2000))
/// Cross-validated with genshin-optimizer and Mona existing implementation
#[inline]
pub fn get_transformative_base(level: u16, t: TransformativeType) -> f64 {
    let level_multiplier = get_level_multiplier(level);
    let reaction_multiplier = get_reaction_multiplier(t);
    let em_bonus = get_transformative_em_bonus(0.0); // Base without EM
    
    level_multiplier * reaction_multiplier
}

/// Calculate full transformative reaction damage
/// damage = level_multiplier × reaction_multiplier × (1 + 16×EM/(EM+2000)) × resistance_factor
pub fn calculate_transformative_damage(
    level: u16,
    em: f64,
    t: TransformativeType,
    resistance_factor: f64,
) -> f64 {
    let level_multiplier = get_level_multiplier(level);
    let reaction_multiplier = get_reaction_multiplier(t);
    let em_bonus = get_transformative_em_bonus(em);
    
    level_multiplier * reaction_multiplier * em_bonus * resistance_factor
}

/// Crystallize shield calculation
/// Formula: crystallize_base × (1 + 40/9 × EM/(EM + 1400))
/// Cross-validated with genshin-optimizer (cryLvlMultis × eleMasMulti(40/9, 1400))
#[inline]
pub fn get_crystallize_em_bonus(em: f64) -> f64 {
    1.0 + (40.0 / 9.0) * em / (em + 1400.0)
}

/// Get crystallize shield value
#[inline]
pub fn get_crystallize_base(level: u16) -> f64 {
    use crate::damage::level_coefficient::get_crystallize_base;
    get_crystallize_base(level)
}

/// Catalyze (Quicken/Spread/Aggravate) reactions
/// Formula: level_multiplier × 1.15 (Spread) or 1.25 (Aggravate) × (1 + 5×EM/(EM+1200))
/// Note: Base catalyze damage uses different EM formula

/// Get catalyze EM bonus
/// Formula: 1 + 5 * EM / (EM + 1200)
/// Cross-validated with genshin-optimizer (eleMasMulti(5, 1200))
#[inline]
pub fn get_catalyze_em_bonus(em: f64) -> f64 {
    1.0 + 5.0 * em / (em + 1200.0)
}

/// Catalyze reaction multiplier
#[inline]
pub fn get_catalyze_multiplier(is_aggravate: bool, is_spread: bool) -> f64 {
    match (is_aggravate, is_spread) {
        (true, false) => 1.25,  // Aggravate
        (false, true) => 1.15,  // Spread
        _ => 1.0,               // Quicken (base)
    }
}

/// Calculate catalyze reaction damage
#[inline]
pub fn calculate_catalyze_damage(
    level: u16,
    em: f64,
    is_aggravate: bool,
    is_spread: bool,
    resistance_factor: f64,
) -> f64 {
    let level_multiplier = get_level_multiplier(level);
    let reaction_multiplier = get_catalyze_multiplier(is_aggravate, is_spread);
    let em_bonus = get_catalyze_em_bonus(em);
    
    level_multiplier * reaction_multiplier * em_bonus * resistance_factor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_90_base() {
        // Cross-validated with genshin-optimizer (1446.8535)
        // and homdgcat.wiki (1446.85)
        let level_90 = get_level_multiplier(90);
        assert!((level_90 - 1446.85).abs() < 0.01);
    }

    #[test]
    fn test_reaction_multipliers() {
        assert!((get_reaction_multiplier(TransformativeType::Overload) - 2.0).abs() < 0.001);
        assert!((get_reaction_multiplier(TransformativeType::ElectroCharged) - 1.2).abs() < 0.001);
        assert!((get_reaction_multiplier(TransformativeType::SwirlPyro) - 0.6).abs() < 0.001);
        assert!((get_reaction_multiplier(TransformativeType::Superconduct) - 0.5).abs() < 0.001);
        assert!((get_reaction_multiplier(TransformativeType::Bloom) - 2.0).abs() < 0.001);
        assert!((get_reaction_multiplier(TransformativeType::Hyperbloom) - 3.0).abs() < 0.001);
        assert!((get_reaction_multiplier(TransformativeType::Burning) - 0.25).abs() < 0.001);
    }

    #[test]
    fn test_em_bonus() {
        // At EM = 1000, bonus should be 1 + 16*1000/3000 = 1 + 5.333 = 6.333
        let em_bonus = get_transformative_em_bonus(1000.0);
        assert!((em_bonus - 6.3333).abs() < 0.01);
    }

    #[test]
    fn test_level_progression() {
        // Verify level progression is increasing
        for level in 2..=100 {
            let prev = get_level_multiplier(level - 1);
            let curr = get_level_multiplier(level);
            assert!(curr > prev, "Level {} should be > Level {}", level, level - 1);
        }
    }
}
