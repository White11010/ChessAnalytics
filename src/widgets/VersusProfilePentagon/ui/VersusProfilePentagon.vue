<template>
  <v-card class="mb-4">
    <v-card-title>{{ t('versusPage.sectionProfile') }}</v-card-title>
    <v-card-text>
      <div class="versus-profile-grid">
        <div
          class="versus-pentagon-wrap d-flex justify-center align-center versus-pentagon-col w-100 h-100"
        >
          <div ref="pentagonSvgHost" class="versus-pentagon-host w-100" />
        </div>
        <div class="versus-bars-block w-100 h-100 d-flex flex-column">
          <div class="versus-bars-legend d-flex flex-wrap ga-4 mb-2 text-caption flex-shrink-0">
            <span class="d-inline-flex align-center ga-1 text-info">
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
          <div class="versus-bars-rows d-flex flex-column flex-grow-1">
            <div
              v-for="row in pentagonLegendRows"
              :key="row.key"
              class="versus-bars-row d-flex align-start align-sm-center ga-3"
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
                  color="info"
                  class="flex-grow-1"
                />
                <span
                  class="text-caption flex-shrink-0 text-info versus-mono"
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
        </div>
        </div>
      </div>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
// Composite widget: presents a focused dashboard block; reads shared Pinia stores and Tauri invoke where needed.

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

const pentagonAxisLabels = computed(() => [
  t('home.profileMetric.accuracy'),
  t('home.profileMetric.stability'),
  t('home.profileMetric.conversion'),
  t('home.profileMetric.openings'),
  t('home.profileMetric.endgame'),
]);

const { pentagonSvgHost } = useVersusPentagonChart(sliceRef, pentagonAxisLabels);

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
.versus-profile-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
  align-items: start;
}

@media (min-width: 1280px) {
  .versus-profile-grid {
    align-items: stretch;
  }
}

@media (max-width: 1279px) {
  .versus-profile-grid {
    grid-template-columns: 1fr;
  }
}

.versus-mono {
  font-family: 'JetBrains Mono', ui-monospace, monospace;
  font-variant-numeric: tabular-nums;
}

.versus-pentagon-host {
  box-sizing: border-box;
  width: 100%;
  max-width: 100%;
  aspect-ratio: 1 / 1;
  flex: 0 0 auto;
  align-self: center;
  display: flex;
  align-items: center;
  justify-content: center;
}

@media (min-width: 1280px) {
  .versus-pentagon-col {
    min-width: 0;
  }
}

.versus-pentagon-host :deep(svg) {
  display: block;
  width: 100%;
  height: 100%;
  max-width: 100%;
  max-height: 100%;
}

.versus-bars-rows {
  flex: 1 1 auto;
  min-height: 0;
  justify-content: space-between;
}

.versus-bars-row {
  flex: 1 1 0;
  min-height: 0;
  display: flex;
  align-items: center;
}
</style>
