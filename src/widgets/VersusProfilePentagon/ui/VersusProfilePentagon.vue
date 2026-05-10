<template>
  <v-card class="mb-4">
    <v-card-title>{{ t('versusPage.sectionProfile') }}</v-card-title>
    <v-card-text>
      <v-row align="center">
        <v-col cols="12" lg="auto" class="d-flex justify-center">
          <div ref="pentagonSvgHost" class="versus-pentagon-host" />
        </v-col>
        <v-col cols="12" lg>
          <div class="d-flex flex-wrap ga-4 mb-4 text-caption">
            <span class="d-inline-flex align-center ga-1 text-primary">
              <v-icon icon="mdi-minus" size="18" />
              {{ t('versusPage.pentagonLegendYou') }}
            </span>
            <span class="d-inline-flex align-center ga-1 text-warning">
              <v-icon icon="mdi-minus" size="18" />
              {{
                t('versusPage.pentagonLegendOpp', {
                  name: slice.opponentSide.username,
                })
              }}
            </span>
          </div>
          <div
            v-for="row in pentagonLegendRows"
            :key="row.key"
            class="d-flex align-start align-sm-center ga-3 mb-3"
          >
            <span
              class="text-caption text-medium-emphasis flex-shrink-0"
              style="width: 6.5rem"
              >{{ row.label }}</span
            >
            <div class="flex-grow-1 d-flex flex-column ga-2" style="min-width: 0">
              <div class="d-flex align-center ga-2">
                <v-progress-linear
                  :model-value="row.you"
                  height="6"
                  rounded
                  color="primary"
                  class="flex-grow-1"
                />
                <span
                  class="text-caption flex-shrink-0 text-primary versus-mono"
                  style="width: 1.75rem; text-align: right"
                  >{{ rounded(row.youRaw) }}</span
                >
              </div>
              <div class="d-flex align-center ga-2">
                <v-progress-linear
                  :model-value="row.opp"
                  height="6"
                  rounded
                  color="warning"
                  class="flex-grow-1"
                />
                <span
                  class="text-caption flex-shrink-0 text-warning versus-mono"
                  style="width: 1.75rem; text-align: right"
                  >{{ rounded(row.oppRaw) }}</span
                >
              </div>
            </div>
          </div>
        </v-col>
      </v-row>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import type { VersusSpeedSlice } from '@/entities/versus';
import { pentagonAxisNumber } from '@/entities/versus/lib/versusConclusions';
import { useVersusPentagonChart } from '@/entities/versus/lib/useVersusPentagonChart';
import { useI18n } from '@/shared/lib/i18n';
import { computed } from 'vue';

const props = defineProps<{
  slice: VersusSpeedSlice;
}>();

const { t } = useI18n();

const sliceRef = computed(() => props.slice ?? null);

const { pentagonSvgHost } = useVersusPentagonChart(sliceRef);

const pentagonLegendRows = computed(() => {
  const x = props.slice;
  const py = x.selfSide.pentagon;
  const po = x.opponentSide.pentagon;
  const defs = [
    { key: 'accuracy' as const, label: t('home.profileMetric.accuracy'), axis: 'accuracy' as const },
    { key: 'stability' as const, label: t('home.profileMetric.stability'), axis: 'stability' as const },
    { key: 'conversion' as const, label: t('home.profileMetric.conversion'), axis: 'conversion' as const },
    { key: 'openings' as const, label: t('home.profileMetric.openings'), axis: 'openings' as const },
    { key: 'endgame' as const, label: t('home.profileMetric.endgame'), axis: 'endgame' as const },
  ];
  return defs.map(({ key, label, axis }) => {
    const youRaw =
      axis === 'conversion' ? (py?.conversion ?? 0) : pentagonAxisNumber(py, axis);
    const oppRaw =
      axis === 'conversion' ? (po?.conversion ?? 0) : pentagonAxisNumber(po, axis);
    return {
      key,
      label,
      youRaw,
      oppRaw,
      you: Math.min(100, Math.max(0, youRaw)),
      opp: Math.min(100, Math.max(0, oppRaw)),
    };
  });
});

function rounded(v: number): number {
  return Math.round(v);
}
</script>

<style scoped lang="scss">
.versus-mono {
  font-family: 'JetBrains Mono', ui-monospace, monospace;
  font-variant-numeric: tabular-nums;
}

.versus-pentagon-host :deep(svg) {
  display: block;
}
</style>
