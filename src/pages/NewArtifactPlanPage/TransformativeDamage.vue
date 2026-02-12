<template>
    <el-table
        :data="tableDataForElementUI"
    >
        <el-table-column
            :label="t('misc.type1')"
        >
            <template #default="{ row }">
                {{ t("dmg", row.key) }}
            </template>
        </el-table-column>
        <el-table-column
            :label="t('misc.dmg')"
        >
            <template #default="{ row }">
                <template v-if="row && row.key && row.value">
                    <el-tooltip placement="top" :show-after="200" :disabled="!row.breakdown">
                        <template #content>
                            <div class="breakdown-tooltip" v-if="row.breakdown">
                                <div class="breakdown-item" v-if="row.breakdown.isCrystallize">
                                    <span class="breakdown-label">结晶护盾量（非伤害）</span>
                                </div>
                                <template v-else>
                                    <div class="breakdown-item">
                                        <span class="breakdown-label">反应基础伤害</span>
                                        <span class="breakdown-value">{{ row.breakdown.baseDamage.toFixed(1) }}</span>
                                    </div>
                                    <div class="breakdown-item">
                                        <span class="breakdown-label">× 精通+增伤</span>
                                        <span class="breakdown-value">{{ row.breakdown.emEnhanceMult.toFixed(4) }}</span>
                                    </div>
                                    <div class="breakdown-item">
                                        <span class="breakdown-label">× 抗性乘区</span>
                                        <span class="breakdown-value">{{ row.breakdown.resMult.toFixed(4) }}</span>
                                    </div>
                                </template>
                                <div class="breakdown-divider"></div>
                                <div class="breakdown-item breakdown-result">
                                    <span class="breakdown-label">= 最终</span>
                                    <span class="breakdown-value">{{ row.value.toFixed(1) }}</span>
                                </div>
                            </div>
                        </template>
                        <span class="damage-cell">
                            <span v-if="row.key === 'electroCharged'" style="color: #c250ff">{{ row.value.toFixed(1) }}</span>
                            <span v-else-if="row.key === 'overload'" style="color: #ff335a">{{ row.value.toFixed(1) }}</span>
                            <span v-else>{{ row.value.toFixed(1) }}</span>
                        </span>
                    </el-tooltip>
                </template>
            </template>
        </el-table-column>
    </el-table>
</template>

<script>
import {useI18n} from "@/i18n/i18n";
import { LEVEL_MULTIPLIER } from "@/constants/levelMultiplier"

const REACTION_META = {
    hyperbloom:    { coefficient: 3.0, element: 'dendro' },
    burgeon:       { coefficient: 3.0, element: 'dendro' },
    bloom:         { coefficient: 2.0, element: 'dendro' },
    overload:      { coefficient: 2.0, element: 'pyro' },
    shattered:     { coefficient: 1.5, element: 'physical' },
    electroCharged:{ coefficient: 1.2, element: 'electro' },
    swirlPyro:     { coefficient: 0.6, element: 'pyro' },
    swirlHydro:    { coefficient: 0.6, element: 'hydro' },
    swirlCryo:     { coefficient: 0.6, element: 'cryo' },
    swirlElectro:  { coefficient: 0.6, element: 'electro' },
    superConduct:  { coefficient: 0.5, element: 'cryo' },
    burning:       { coefficient: 0.25, element: 'pyro' },
    crystallize:   { coefficient: 0, element: null, isCrystallize: true },
}

function computeResRatio(enemyConfig, element) {
    if (!element || !enemyConfig) return 1
    const resKey = element + '_res'
    const res = enemyConfig[resKey] ?? 0.1
    if (res > 0.75) return 1 / (1 + res * 4)
    if (res > 0) return 1 - res
    return 1 - res / 2
}

export default defineComponent({
    name: "TransformativeDamage",
    props: {
        data: {},
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
        tableDataForElementUI() {
            const charLvl = this.characterLevel
            const levelMult = LEVEL_MULTIPLIER[charLvl - 1] || LEVEL_MULTIPLIER[89]

            const makeBreakdown = (key, value) => {
                const meta = REACTION_META[key]
                if (!meta) return null

                if (meta.isCrystallize) {
                    return { isCrystallize: true }
                }

                const baseDamage = levelMult * meta.coefficient
                const resMult = computeResRatio(this.enemyConfig, meta.element)

                // Back-calculate the combined (1 + em_bonus + enhance) multiplier
                const denominator = baseDamage * resMult
                const emEnhanceMult = denominator > 0 ? value / denominator : 0

                return {
                    baseDamage,
                    emEnhanceMult,
                    resMult,
                    isCrystallize: false,
                }
            }

            let results = []
            const push = (dataKey, key) => {
                const value = this.data[dataKey]
                results.push({
                    value,
                    key,
                    breakdown: makeBreakdown(key, value),
                })
            }

            push("hyperbloom", "hyperbloom")
            push("burgeon", "burgeon")
            push("bloom", "bloom")
            push("electro_charged", "electroCharged")
            push("overload", "overload")
            push("shatter", "shattered")
            push("superconduct", "superConduct")
            push("burning", "burning")
            push("swirl_electro", "swirlElectro")
            push("swirl_pyro", "swirlPyro")
            push("swirl_cryo", "swirlCryo")
            push("swirl_hydro", "swirlHydro")
            push("crystallize", "crystallize")
            return results
        }
    },
    setup() {
        const { t } = useI18n()
        return { t }
    }
})
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