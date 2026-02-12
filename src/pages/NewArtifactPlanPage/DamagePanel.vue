<template>
    <div>
        <el-table
            :data="tableData"
        >
            <el-table-column
                prop="name"
                :label="t('misc.type1')"
            >
            </el-table-column>
            <el-table-column :label="t('dmg.expect')">
                <template #default="{ row }">
                    <el-tooltip placement="top" :show-after="200" :disabled="!row.breakdown">
                        <template #content>
                            <div class="breakdown-tooltip" v-if="row.breakdown">
                                <template v-if="row.breakdown.isHeal">
                                    <div class="breakdown-item">
                                        <span class="breakdown-label">基础治疗量</span>
                                        <span class="breakdown-value">{{ row.breakdown.baseDamage.toFixed(1) }}</span>
                                    </div>
                                    <div class="breakdown-item">
                                        <span class="breakdown-label">× 治疗加成</span>
                                        <span class="breakdown-value">{{ row.breakdown.healingBonusMult.toFixed(4) }}</span>
                                    </div>
                                </template>
                                <template v-else-if="row.breakdown.isShield">
                                    <div class="breakdown-item">
                                        <span class="breakdown-label">基础护盾量</span>
                                        <span class="breakdown-value">{{ row.breakdown.baseDamage.toFixed(1) }}</span>
                                    </div>
                                    <div class="breakdown-item">
                                        <span class="breakdown-label">× 护盾强效</span>
                                        <span class="breakdown-value">{{ row.breakdown.shieldStrengthMult.toFixed(4) }}</span>
                                    </div>
                                </template>
                                <template v-else>
                                    <div class="breakdown-item">
                                        <span class="breakdown-label">基础伤害</span>
                                        <span class="breakdown-value">{{ row.breakdown.baseDamage.toFixed(1) }}</span>
                                    </div>
                                    <div class="breakdown-item">
                                        <span class="breakdown-label">× 伤害加成</span>
                                        <span class="breakdown-value">{{ row.breakdown.bonusMult.toFixed(4) }}</span>
                                    </div>
                                    <div class="breakdown-item">
                                        <span class="breakdown-label">× 暴击期望</span>
                                        <span class="breakdown-value">{{ (1 + row.breakdown.critRate * row.breakdown.critDmg).toFixed(4) }}</span>
                                    </div>
                                    <div class="breakdown-item">
                                        <span class="breakdown-label">× 防御乘区</span>
                                        <span class="breakdown-value">{{ row.breakdown.defMult.toFixed(4) }}</span>
                                    </div>
                                    <div class="breakdown-item">
                                        <span class="breakdown-label">× 抗性乘区</span>
                                        <span class="breakdown-value">{{ row.breakdown.resMult.toFixed(4) }}</span>
                                    </div>
                                    <div class="breakdown-item" v-if="row.breakdown.reactionMult !== 1">
                                        <span class="breakdown-label">× 反应乘区</span>
                                        <span class="breakdown-value">{{ row.breakdown.reactionMult.toFixed(4) }}</span>
                                    </div>
                                </template>
                                <div class="breakdown-divider"></div>
                                <div class="breakdown-item breakdown-result">
                                    <span class="breakdown-label">= 最终</span>
                                    <span class="breakdown-value">{{ row.expectation }}</span>
                                </div>
                            </div>
                        </template>
                        <span class="damage-cell">{{ row.expectation }}</span>
                    </el-tooltip>
                </template>
            </el-table-column>
            <el-table-column :label="t('dmg.crit')">
                <template #default="{ row }">
                    <el-tooltip placement="top" :show-after="200" :disabled="!row.breakdown">
                        <template #content>
                            <div class="breakdown-tooltip" v-if="row.breakdown">
                                <template v-if="row.breakdown.isHeal || row.breakdown.isShield">
                                    <div class="breakdown-item">
                                        <span class="breakdown-label">—</span>
                                    </div>
                                </template>
                                <template v-else>
                                    <div class="breakdown-item">
                                        <span class="breakdown-label">基础伤害</span>
                                        <span class="breakdown-value">{{ row.breakdown.baseDamage.toFixed(1) }}</span>
                                    </div>
                                    <div class="breakdown-item">
                                        <span class="breakdown-label">× 伤害加成</span>
                                        <span class="breakdown-value">{{ row.breakdown.bonusMult.toFixed(4) }}</span>
                                    </div>
                                    <div class="breakdown-item">
                                        <span class="breakdown-label">× 暴击伤害</span>
                                        <span class="breakdown-value">{{ (1 + row.breakdown.critDmg).toFixed(4) }}</span>
                                    </div>
                                    <div class="breakdown-item">
                                        <span class="breakdown-label">× 防御乘区</span>
                                        <span class="breakdown-value">{{ row.breakdown.defMult.toFixed(4) }}</span>
                                    </div>
                                    <div class="breakdown-item">
                                        <span class="breakdown-label">× 抗性乘区</span>
                                        <span class="breakdown-value">{{ row.breakdown.resMult.toFixed(4) }}</span>
                                    </div>
                                    <div class="breakdown-item" v-if="row.breakdown.reactionMult !== 1">
                                        <span class="breakdown-label">× 反应乘区</span>
                                        <span class="breakdown-value">{{ row.breakdown.reactionMult.toFixed(4) }}</span>
                                    </div>
                                </template>
                                <div class="breakdown-divider"></div>
                                <div class="breakdown-item breakdown-result">
                                    <span class="breakdown-label">= 最终</span>
                                    <span class="breakdown-value">{{ row.critical }}</span>
                                </div>
                            </div>
                        </template>
                        <span class="damage-cell">{{ row.critical }}</span>
                    </el-tooltip>
                </template>
            </el-table-column>
            <el-table-column :label="t('dmg.nonCrit')">
                <template #default="{ row }">
                    <el-tooltip placement="top" :show-after="200" :disabled="!row.breakdown">
                        <template #content>
                            <div class="breakdown-tooltip" v-if="row.breakdown">
                                <template v-if="row.breakdown.isHeal || row.breakdown.isShield">
                                    <div class="breakdown-item">
                                        <span class="breakdown-label">—</span>
                                    </div>
                                </template>
                                <template v-else>
                                    <div class="breakdown-item">
                                        <span class="breakdown-label">基础伤害</span>
                                        <span class="breakdown-value">{{ row.breakdown.baseDamage.toFixed(1) }}</span>
                                    </div>
                                    <div class="breakdown-item">
                                        <span class="breakdown-label">× 伤害加成</span>
                                        <span class="breakdown-value">{{ row.breakdown.bonusMult.toFixed(4) }}</span>
                                    </div>
                                    <div class="breakdown-item">
                                        <span class="breakdown-label">× 防御乘区</span>
                                        <span class="breakdown-value">{{ row.breakdown.defMult.toFixed(4) }}</span>
                                    </div>
                                    <div class="breakdown-item">
                                        <span class="breakdown-label">× 抗性乘区</span>
                                        <span class="breakdown-value">{{ row.breakdown.resMult.toFixed(4) }}</span>
                                    </div>
                                    <div class="breakdown-item" v-if="row.breakdown.reactionMult !== 1">
                                        <span class="breakdown-label">× 反应乘区</span>
                                        <span class="breakdown-value">{{ row.breakdown.reactionMult.toFixed(4) }}</span>
                                    </div>
                                </template>
                                <div class="breakdown-divider"></div>
                                <div class="breakdown-item breakdown-result">
                                    <span class="breakdown-label">= 最终</span>
                                    <span class="breakdown-value">{{ row.nonCritical }}</span>
                                </div>
                            </div>
                        </template>
                        <span class="damage-cell">{{ row.nonCritical }}</span>
                    </el-tooltip>
                </template>
            </el-table-column>
        </el-table>
    </div>
</template>

<script>
import {useI18n} from "@/i18n/i18n";
import { LEVEL_MULTIPLIER } from "@/constants/levelMultiplier"

function sumObj(obj) {
    if (!obj) return 0
    let s = 0
    for (const key in obj) {
        s += obj[key]
    }
    return s
}

export default {
    name: "DamageList",
    props: {
        analysisFromWasm: {},
        enemyConfig: {
            type: Object,
            default: () => ({ level: 90, electro_res: 0.1, pyro_res: 0.1, hydro_res: 0.1, cryo_res: 0.1, geo_res: 0.1, anemo_res: 0.1, dendro_res: 0.1, physical_res: 0.1 })
        },
        characterLevel: {
            type: Number,
            default: 90
        }
    },
    computed: {
        element() {
            return this.analysisFromWasm.element
        },

        normalDamageTitle() {
            if (this.analysisFromWasm.is_heal) {
                return this.t("dmg.heal")
            } else {
                return this.t("dmg", this.element)
            }
        },

        // Compute all breakdown factors from the DamageAnalysis data
        breakdownBase() {
            const a = this.analysisFromWasm
            if (!a) return null

            const atk = sumObj(a.atk)
            const atkRatio = sumObj(a.atk_ratio)
            const def = sumObj(a.def)
            const defRatio = sumObj(a.def_ratio)
            const hp = sumObj(a.hp)
            const hpRatio = sumObj(a.hp_ratio)
            const em = sumObj(a.em)
            const emRatio = sumObj(a.em_ratio)
            const extraDamage = sumObj(a.extra_damage)
            const baseDamage = atk * atkRatio + def * defRatio + hp * hpRatio + em * emRatio + extraDamage

            const bonus = sumObj(a.bonus)
            const bonusMult = 1 + bonus

            const healingBonus = sumObj(a.healing_bonus)
            const healingBonusMult = 1 + healingBonus

            const shieldStrength = sumObj(a.shield_strength)
            const shieldStrengthMult = 1 + shieldStrength

            const critRate = Math.min(sumObj(a.critical), 1)
            const critDmg = sumObj(a.critical_damage)

            const defMinus = sumObj(a.def_minus)
            const defPen = sumObj(a.def_penetration)
            const charLvl = this.characterLevel
            const enemyLvl = this.enemyConfig.level
            const c = 100 + charLvl
            const defMult = c / ((1 - defPen) * (1 - defMinus) * (100 + enemyLvl) + c)

            const elementKey = this.element.toLowerCase() + "_res"
            const enemyRes = this.enemyConfig[elementKey] ?? 0.1
            const resMinus = sumObj(a.res_minus)
            const res = enemyRes - resMinus
            let resMult
            if (res > 0.75) {
                resMult = 1 / (1 + res * 4)
            } else if (res > 0) {
                resMult = 1 - res
            } else {
                resMult = 1 - res / 2
            }

            const meltEnhance = sumObj(a.melt_enhance)
            const meltRatio = this.element === "Pyro" ? 2.0 : 1.5
            const meltMult = meltRatio * (1 + meltEnhance)

            const vapEnhance = sumObj(a.vaporize_enhance)
            const vapRatio = this.element === "Hydro" ? 2.0 : 1.5
            const vapMult = vapRatio * (1 + vapEnhance)

            const spreadEnhance = sumObj(a.spread_compose)
            const spreadBaseDamage = baseDamage + LEVEL_MULTIPLIER[charLvl - 1] * 1.25 * (1 + spreadEnhance)

            const aggravateEnhance = sumObj(a.aggravate_compose)
            const aggravateBaseDamage = baseDamage + LEVEL_MULTIPLIER[charLvl - 1] * 1.15 * (1 + aggravateEnhance)

            return {
                baseDamage,
                bonusMult,
                healingBonusMult,
                shieldStrengthMult,
                critRate,
                critDmg,
                defMult,
                resMult,
                meltMult,
                vapMult,
                spreadBaseDamage,
                aggravateBaseDamage,
                isHeal: !!a.is_heal,
                isShield: !!a.is_shield,
            }
        },

        tableData() {
            let temp = []
            const r = (x) => Math.round(x)
            const base = this.breakdownBase

            const makeBreakdown = (reactionType) => {
                if (!base) return null
                let baseDmg = base.baseDamage
                let reactionMult = 1

                if (reactionType === "melt") {
                    reactionMult = base.meltMult
                } else if (reactionType === "vaporize") {
                    reactionMult = base.vapMult
                } else if (reactionType === "spread") {
                    baseDmg = base.spreadBaseDamage
                } else if (reactionType === "aggravate") {
                    baseDmg = base.aggravateBaseDamage
                }

                return {
                    baseDamage: baseDmg,
                    bonusMult: base.bonusMult,
                    healingBonusMult: base.healingBonusMult,
                    shieldStrengthMult: base.shieldStrengthMult,
                    critRate: base.critRate,
                    critDmg: base.critDmg,
                    defMult: base.defMult,
                    resMult: base.resMult,
                    reactionMult,
                    isHeal: base.isHeal,
                    isShield: base.isShield,
                }
            }

            const push = (name, title, reactionType) => {
                temp.push({
                    expectation: r(this.analysisFromWasm[name]?.expectation) ?? "—",
                    critical: r(this.analysisFromWasm[name]?.critical) ?? "—",
                    nonCritical: r(this.analysisFromWasm[name]?.non_critical) ?? "—",
                    name: title,
                    breakdown: makeBreakdown(reactionType || "normal"),
                })
            }

            push("normal", this.normalDamageTitle, "normal")

            if (this.analysisFromWasm.melt) {
                push("melt", this.t("dmg.melt"), "melt")
            }
            if (this.analysisFromWasm.vaporize) {
                push("vaporize", this.t("dmg.vaporize"), "vaporize")
            }
            if (this.analysisFromWasm.spread) {
                push("spread", this.t("dmg.spread"), "spread")
            }
            if (this.analysisFromWasm.aggravate) {
                push("aggravate", this.t("dmg.aggravate"), "aggravate")
            }

            return temp
        }
    },
    setup() {
        const { t } = useI18n()

        return {
            t
        }
    }
}
</script>

<style scoped lang="scss">
.damage-cell {
    cursor: default;
}

.breakdown-tooltip {
    min-width: 200px;
    font-size: 13px;
    line-height: 1.6;
}

.breakdown-item {
    display: flex;
    justify-content: space-between;
    gap: 24px;

    .breakdown-label {
        white-space: nowrap;
    }

    .breakdown-value {
        font-family: monospace;
        white-space: nowrap;
    }
}

.breakdown-divider {
    border-top: 1px solid rgba(255, 255, 255, 0.3);
    margin: 4px 0;
}

.breakdown-result {
    font-weight: bold;
}
</style>