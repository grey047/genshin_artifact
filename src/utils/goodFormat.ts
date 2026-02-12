/**
 * GOOD Format Support
 * 
 * Provides format detection and conversion from GOOD (Genshin Open Object Description)
 * format to mona.json format. GOOD is commonly exported by tools like Inventory Kamera.
 */

import type { ArtifactPosition, ArtifactStatName, ArtifactMainStatName } from "@/types/artifact"
import { artifactTags } from "@/constants/artifact"
import { getMainStatValue } from "@/utils/mainStatValues"

export type ImportFormat = "mona" | "good" | "unknown"

/**
 * Detect whether parsed JSON is mona.json or GOOD format.
 * - GOOD: has `format === "GOOD"` or has `artifacts` array at top level
 * - mona: has any of "flower", "feather", "sand", "cup", "head" keys
 */
export function detectFormat(obj: any): ImportFormat {
    if (obj.format === "GOOD" || (Array.isArray(obj.artifacts) && !obj.flower)) {
        return "good"
    }
    if (obj.flower || obj.feather || obj.sand || obj.cup || obj.head) {
        return "mona"
    }
    return "unknown"
}

/**
 * Maps GOOD slotKey → mona position
 */
const slotKeyMap: Record<string, ArtifactPosition> = {
    "flower": "flower",
    "plume": "feather",
    "sands": "sand",
    "goblet": "cup",
    "circlet": "head",
}

/**
 * Maps GOOD stat keys → mona ArtifactStatName
 */
const statNameMap: Record<string, ArtifactStatName> = {
    // Flat stats
    "hp": "lifeStatic",
    "atk": "attackStatic",
    "def": "defendStatic",
    // Percentage stats
    "hp_": "lifePercentage",
    "atk_": "attackPercentage",
    "def_": "defendPercentage",
    "eleMas": "elementalMastery",
    "enerRech_": "recharge",
    "critRate_": "critical",
    "critDMG_": "criticalDamage",
    "heal_": "cureEffect",
    // Elemental damage bonuses
    "pyro_dmg_": "fireBonus",
    "electro_dmg_": "thunderBonus",
    "hydro_dmg_": "waterBonus",
    "cryo_dmg_": "iceBonus",
    "anemo_dmg_": "windBonus",
    "geo_dmg_": "rockBonus",
    "dendro_dmg_": "dendroBonus",
    "physical_dmg_": "physicalBonus",
}

/**
 * Maps GOOD setKey (PascalCase) → mona setName
 * 
 * The mona system uses mixed casing:
 * - Older sets (~37 sets): camelCase (e.g., "gladiatorFinale")
 * - Newer sets (~22 sets from EchoesOfAnOffering onward): PascalCase (e.g., "EchoesOfAnOffering")
 */
const setNameMap: Record<string, string> = {
    // --- Older sets: GOOD PascalCase → mona camelCase ---
    "Adventurer": "adventurer",
    "ArchaicPetra": "archaicPetra",
    "Berserker": "berserker",
    "BlizzardStrayer": "blizzardStrayer",
    "BloodstainedChivalry": "bloodstainedChivalry",
    "BraveHeart": "braveHeart",
    "CrimsonWitchOfFlames": "crimsonWitch",
    "DefendersWill": "defenderWill",
    "EmblemOfSeveredFate": "emblemOfSeveredFate",
    "Gambler": "gambler",
    "GladiatorsFinale": "gladiatorFinale",
    "HeartOfDepth": "heartOfDepth",
    "HuskOfOpulentDreams": "huskOfOpulentDreams",
    "Instructor": "instructor",
    "Lavawalker": "lavaWalker",
    "LuckyDog": "luckyDog",
    "MaidenBeloved": "maidenBeloved",
    "MartialArtist": "martialArtist",
    "NoblesseOblige": "noblesseOblige",
    "OceanHuedClam": "oceanHuedClam",
    "PaleFlame": "paleFlame",
    "PrayersForDestiny": "prayersForDestiny",
    "PrayersForIllumination": "prayersForIllumination",
    "PrayersForWisdom": "prayersForWisdom",
    "PrayersToSpringtime": "prayersToSpringtime",
    "ResolutionOfSojourner": "resolutionOfSojourner",
    "RetracingBolide": "retracingBolide",
    "Scholar": "scholar",
    "ShimenawasReminiscence": "shimenawaReminiscence",
    "TenacityOfTheMillelith": "tenacityOfTheMillelith",
    "TheExile": "exile",
    "ThunderingFury": "thunderingFury",
    "Thundersoother": "thunderSmoother",
    "TinyMiracle": "tinyMiracle",
    "TravelingDoctor": "travelingDoctor",
    "ViridescentVenerer": "viridescentVenerer",
    "WanderersTroupe": "wandererTroupe",

    // --- Newer sets: GOOD PascalCase = mona PascalCase (identity mapping) ---
    "DeepwoodMemories": "DeepwoodMemories",
    "EchoesOfAnOffering": "EchoesOfAnOffering",
    "FlowerOfParadiseLost": "FlowerOfParadiseLost",
    "DesertPavilionChronicle": "DesertPavilionChronicle",
    "GildedDreams": "GildedDreams",
    "GoldenTroupe": "GoldenTroupe",
    "MarechausseeHunter": "MarechausseeHunter",
    "NymphsDream": "NymphsDream",
    "VermillionHereafter": "VermillionHereafter",
    "VourukashasGlow": "VourukashasGlow",
    "SongOfDaysPast": "SongOfDaysPast",
    "NighttimeWhispersInTheEchoingWoods": "NighttimeWhispersInTheEchoingWoods",
    "FragmentOfHarmonicWhimsy": "FragmentOfHarmonicWhimsy",
    "UnfinishedReverie": "UnfinishedReverie",
    "ScrollOfTheHeroOfCinderCity": "ScrollOfTheHeroOfCinderCity",
    "ObsidianCodex": "ObsidianCodex",
    "LongNightsOath": "LongNightsOath",
    "FinaleOfTheDeepGalleries": "FinaleOfTheDeepGalleries",
    "NightOfTheSkysUnveiling": "NightOfTheSkysUnveiling",
    "SilkenMoonsSerenade": "SilkenMoonsSerenade",
    "ADayCarvedFromRisingWinds": "ADayCarvedFromRisingWinds",
    "AubadeOfMorningstarAndMoon": "AubadeOfMorningstarAndMoon",
}

export interface ConvertGoodResult {
    result: any,
    skipped: number
}

/**
 * Convert a GOOD format JSON object into mona.json shape.
 * Returns an object with { flower: [...], feather: [...], ... }
 * suitable for passing to importMonaJson().
 */
export function convertGoodToMona(goodObj: any): ConvertGoodResult {
    const result: any = {
        flower: [],
        feather: [],
        sand: [],
        cup: [],
        head: [],
    }
    let skipped = 0

    for (const artifact of goodObj.artifacts ?? []) {
        // --- Structural validation ---
        if (!artifact || typeof artifact !== "object") {
            console.warn("[GOOD import] skipping non-object artifact entry")
            skipped++
            continue
        }

        // --- Mapping lookups (skip unknown values) ---
        const position = slotKeyMap[artifact.slotKey]
        if (!position) {
            console.warn(`[GOOD import] unknown slotKey: ${artifact.slotKey}`)
            skipped++
            continue
        }

        const setName = setNameMap[artifact.setKey]
        if (!setName) {
            console.warn(`[GOOD import] unknown setKey: ${artifact.setKey}`)
            skipped++
            continue
        }

        const mainStatName = statNameMap[artifact.mainStatKey] as ArtifactMainStatName
        if (!mainStatName) {
            console.warn(`[GOOD import] unknown mainStatKey: ${artifact.mainStatKey}`)
            skipped++
            continue
        }

        // --- Semantic validation (range checks) ---
        const star = typeof artifact.rarity === "number" ? artifact.rarity : 5
        const level = typeof artifact.level === "number" ? artifact.level : 0
        if (star < 1 || star > 5) {
            console.warn(`[GOOD import] invalid rarity: ${star}`)
            skipped++
            continue
        }
        if (level < 0 || level > star * 4) {
            console.warn(`[GOOD import] invalid level: ${level} for ${star}-star artifact`)
            skipped++
            continue
        }

        const mainStatValue = getMainStatValue(mainStatName, star, level)

        const normalTags = (artifact.substats ?? [])
            .filter((s: any) => {
                if (!s || typeof s !== "object") return false
                if (!s.key || s.key === "" || !statNameMap[s.key]) return false
                if (typeof s.value !== "number" || !isFinite(s.value)) return false
                return true
            })
            .map((s: any) => {
                const name = statNameMap[s.key]
                const tagInfo = artifactTags[name]
                const isPercentage = tagInfo?.percentage ?? false
                return {
                    name,
                    // GOOD uses display % (3.5 = 3.5%), mona uses decimal (0.035)
                    value: isPercentage ? s.value / 100 : s.value,
                }
            })

        const monaArtifact: any = {
            setName,
            position,
            mainTag: { name: mainStatName, value: mainStatValue },
            normalTags,
            star,
            level,
            omit: false,
        }

        // Map location → equip (transient property for kumi grouping only)
        if (artifact.location && artifact.location !== "") {
            monaArtifact.equip = artifact.location
        }

        result[position].push(monaArtifact)
    }

    if (skipped > 0) {
        console.warn(`[GOOD import] ${skipped} artifact(s) skipped due to validation errors`)
    }

    return { result, skipped }
}
