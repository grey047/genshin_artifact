//! Hexenzirkel Buff System
//!
//! Implements detailed buff values for each Hexenzirkel character.
//! Each character has unique buffs that scale with constellation level.

use crate::common::UnsafeDamageContext;

/// Hexenzirkel Buff Values
///
/// Contains all buff multipliers for each Hexenzirkel character.
/// Values scale with constellation level.
#[derive(Clone, Debug)]
pub struct HexenzirkelBuffs {
    /// Venti: Anemo arrow normal attack bonus
    pub venti_normal_bonus: f64,
    /// Venti: Vortex/Burst damage bonus after Swirl
    pub venti_burst_bonus: f64,
    /// Klee: Charged attack bonus with Boom Boost stacks
    pub klee_charged_bonus: f64,
    /// Klee: Boom Boost stack count (0-4)
    pub klee_boom_stacks: u8,
    /// Albedo: DEF-based damage bonus for team
    pub albedo_def_bonus: f64,
    /// Mona: Damage amplification bonus
    pub mona_amp_bonus: f64,
    /// Fischl: Oz attack frequency multiplier
    pub fischl_oz_frequency: f64,
    /// Fischl: C6 CRIT bonus
    pub fischl_crit_bonus: f64,
    /// Razor: Constellation bonus multiplier
    pub razor_constellation_bonus: f64,
    /// Razor: C6 extra sigils and crit
    pub razor_c6_bonus: f64,
    /// Sucrose: Elemental Mastery bonus
    pub sucrose_em_bonus: f64,
    /// Sucrose: Elemental damage bonus
    pub sucrose_elemental_bonus: f64,
}

impl Default for HexenzirkelBuffs {
    fn default() -> Self {
        HexenzirkelBuffs {
            venti_normal_bonus: 0.0,
            venti_burst_bonus: 0.0,
            klee_charged_bonus: 0.0,
            klee_boom_stacks: 0,
            albedo_def_bonus: 0.0,
            mona_amp_bonus: 0.0,
            fischl_oz_frequency: 1.0,
            fischl_crit_bonus: 0.0,
            razor_constellation_bonus: 0.0,
            razor_c6_bonus: 0.0,
            sucrose_em_bonus: 0.0,
            sucrose_elemental_bonus: 0.0,
        }
    }
}

impl HexenzirkelBuffs {
    /// Create new empty buffs
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate Venti buffs based on constellation level
    ///
    /// C0: 10% Anemo DMG Bonus
    /// C1: 10% → 12% Anemo DMG Bonus
    /// C2: +10 Energy on burst hit
    /// C4: 20% Vortex ATK increase
    /// C6: Further Anemo DMG Bonus
    pub fn calculate_venti(&mut self, constellation: u8) {
        self.venti_normal_bonus = match constellation {
            0 | 1 => 0.10,
            2 | 3 => 0.12,
            4 | 5 => 0.14,
            _ => 0.15,
        };
        
        self.venti_burst_bonus = match constellation {
            0 | 1 | 2 | 3 => 0.0,
            4 => 0.20,
            _ => 0.25,
        };
    }

    /// Calculate Klee buffs based on constellation level
    ///
    /// C0: 12% Charged Attack DMG
    /// C1: +1 Boom Boost stack
    /// C2: 20% DEF ignore on charged
    /// C4: +2 max Boom Boost stacks
    /// C6: 25% Charged Attack DMG
    pub fn calculate_klee(&mut self, constellation: u8, boom_stacks: u8) {
        self.klee_boom_stacks = boom_stacks.min(4);
        
        let base_bonus = match constellation {
            0 => 0.12,
            1 | 2 => 0.15,
            3 | 4 => 0.18,
            _ => 0.25,
        };
        
        // Each Boom Boost stack adds 5% charged damage
        self.klee_charged_bonus = base_bonus + (self.klee_boom_stacks as f64 * 0.05);
    }

    /// Calculate Albedo buffs based on constellation level
    ///
    /// C0: 10% DMG Bonus based on DEF
    /// C2: +25% DEF to nearby allies
    /// C4: 20% burst DMG increase
    /// C6: 30% CRIT Rate on blossoms
    pub fn calculate_albedo(&mut self, constellation: u8, albedo_def: f64) {
        let def_ratio = match constellation {
            0 | 1 => 0.10,
            2 | 3 => 0.12,
            _ => 0.15,
        };
        
        // DEF-based bonus: typically ~30% of DEF becomes DMG Bonus
        self.albedo_def_bonus = albedo_def * 0.30 * def_ratio;
    }

    /// Calculate Mona buffs based on constellation level
    ///
    /// C0: 8% Damage Amplification
    /// C1: +20% Hydro DMG Bonus
    /// C2: Increased Illusory Torrent duration
    /// C4: 20% Omen extension
    /// C6: 15% additional amp on burst hit
    pub fn calculate_mona(&mut self, constellation: u8) {
        self.mona_amp_bonus = match constellation {
            0 | 1 => 0.08,
            2 | 3 => 0.12,
            4 => 0.15,
            _ => 0.20,
        };
    }

    /// Calculate Fischl buffs based on constellation level
    ///
    /// C0: 10% Oz frequency, 10% Electro DMG
    /// C2: Extra Sigil on Oz attacks
    /// C4: 20% Oz ATK increase
    /// C6: 60% CRIT DMG on Oz attacks
    pub fn calculate_fischl(&mut self, constellation: u8) {
        self.fischl_oz_frequency = match constellation {
            0 | 1 => 1.0,
            2 | 3 => 1.15,
            _ => 1.25,
        };
        
        self.fischl_crit_bonus = match constellation {
            0 | 1 | 2 => 0.0,
            3 | 4 => 0.30,
            _ => 0.60,
        };
    }

    /// Calculate Razor buffs based on constellation level
    ///
    /// C0: 8% Physical/Electro DMG Bonus
    /// C2: +1 Electro Sigil on hit
    /// C4: 15% ATK speed bonus
    /// C6: 30% CRIT Rate, extra Sigils
    pub fn calculate_razor(&mut self, constellation: u8) {
        self.razor_constellation_bonus = match constellation {
            0 | 1 => 0.08,
            2 | 3 => 0.12,
            _ => 0.18,
        };
        
        self.razor_c6_bonus = match constellation {
            6 => 0.30,
            _ => 0.0,
        };
    }

    /// Calculate Sucrose buffers based on constellation level
    ///
    /// C0: 12% Elemental DMG Bonus
    /// C1: +50 EM on reactions
    /// C2: +20% reaction multiplier
    /// C4: +80 EM to infused allies
    /// C6: Additional 20% Elemental DMG Bonus
    pub fn calculate_sucrose(&mut self, constellation: u8) {
        self.sucrose_elemental_bonus = match constellation {
            0 => 0.12,
            1 | 2 => 0.15,
            3 | 4 => 0.20,
            _ => 0.25,
        };
        
        self.sucrose_em_bonus = match constellation {
            0 => 0.0,
            1 => 50.0,
            2 => 60.0,
            3 | 4 => 80.0,
            _ => 100.0,
        };
    }

    /// Get total damage multiplier from all active buffs
    pub fn total_damage_bonus(&self) -> f64 {
        self.venti_normal_bonus 
            + self.venti_burst_bonus 
            + self.klee_charged_bonus 
            + self.albedo_def_bonus 
            + self.mona_amp_bonus 
            + self.sucrose_elemental_bonus
    }

    /// Get total CRIT bonus from all active buffs
    pub fn total_crit_bonus(&self) -> f64 {
        self.fischl_crit_bonus + self.razor_c6_bonus
    }
}

/// Calculate Hexenzirkel buff for a specific character
///
/// Returns the damage multiplier for the given character based on:
/// - Character's constellation level
/// - Character's stats (DEF for Albedo, etc.)
/// - Whether team has 2+ Hexenzirkel members
pub fn get_hexenzirkel_buff_for_character(
    character: &str,
    ctx: &UnsafeDamageContext,
    team_hexenzirkel_count: u8,
) -> f64 {
    // Only apply if 2+ Hexenzirkel members
    if team_hexenzirkel_count < 2 {
        return 0.0;
    }

    let constellation: u8 = unsafe { (*ctx.character_common_data).constellation as u8 };
    
    match character {
        "Venti" => {
            let mut buffs = HexenzirkelBuffs::new();
            buffs.calculate_venti(constellation);
            buffs.venti_normal_bonus + buffs.venti_burst_bonus
        }
        "Klee" => {
            let mut buffs = HexenzirkelBuffs::new();
            // Assuming 2 boom stacks as default
            buffs.calculate_klee(constellation, 2);
            buffs.klee_charged_bonus
        }
        "Albedo" => {
            let mut buffs = HexenzirkelBuffs::new();
            let def = unsafe { (*ctx.character_common_data).base_def };
            buffs.calculate_albedo(constellation, def);
            buffs.albedo_def_bonus
        }
        "Mona" => {
            let mut buffs = HexenzirkelBuffs::new();
            buffs.calculate_mona(constellation);
            buffs.mona_amp_bonus
        }
        "Fischl" => {
            let mut buffs = HexenzirkelBuffs::new();
            buffs.calculate_fischl(constellation);
            // Return Oz frequency as damage equivalent (10% freq ≈ 10% dmg)
            (buffs.fischl_oz_frequency - 1.0) * 2.0 + buffs.fischl_crit_bonus * 0.5
        }
        "Razor" => {
            let mut buffs = HexenzirkelBuffs::new();
            buffs.calculate_razor(constellation);
            buffs.razor_constellation_bonus + buffs.razor_c6_bonus * 0.5
        }
        "Sucrose" => {
            let mut buffs = HexenzirkelBuffs::new();
            buffs.calculate_sucrose(constellation);
            // EM bonus converts to damage: ~100 EM ≈ 10% bonus
            (buffs.sucrose_em_bonus / 1000.0) + buffs.sucrose_elemental_bonus
        }
        _ => 0.0,
    }
}
