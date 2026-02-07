pub struct Reaction {}

impl Reaction {
    pub fn amp(em: f64) -> f64 {
        em * 25.0 / (9.0 * (em + 1400.0))
    }

    pub fn transformative(em: f64) -> f64 {
        em * 16.0 / (em + 2000.0)
    }

    pub fn catalyze(em: f64) -> f64 {
        em * 5.0 / (em + 1200.0)
    }

    /// Lunar reaction EM bonus formula
    /// Same as transformative: 1 + 16*EM/(EM+2000)
    #[inline]
    pub fn lunar_em_bonus(em: f64) -> f64 {
        1.0 + 16.0 * em / (em + 2000.0)
    }

    /// Lunar-Charged (Electro + Hydro) direct damage multiplier
    /// Multiplier: 3.0
    #[inline]
    pub fn lunar_charged_multiplier() -> f64 {
        3.0
    }

    /// Lunar-Bloom (Hydro + Dendro) direct damage multiplier
    /// Multiplier: 3.0
    #[inline]
    pub fn lunar_bloom_multiplier() -> f64 {
        3.0
    }

    /// Lunar-Crystallize (Geo + Hydro) direct damage multiplier
    /// Multiplier: 1.6
    #[inline]
    pub fn lunar_crystallize_multiplier() -> f64 {
        1.6
    }

    /// Reaction Lunar-Charged multiplier (for transformative-like calculations)
    /// Base multiplier: 1.0, adjusted by ranking
    /// Ranking: 1st×1, 2nd×½, others×1/12
    #[inline]
    pub fn lunar_charged_reaction_multiplier(rank: usize) -> f64 {
        match rank {
            1 => 1.0,
            2 => 0.5,
            _ => 1.0 / 12.0,
        }
    }

    /// Reaction Lunar-Crystallize multiplier
    /// Base multiplier: 0.96 of Lunar-Charged
    #[inline]
    pub fn lunar_crystallize_reaction_multiplier() -> f64 {
        0.96
    }
}