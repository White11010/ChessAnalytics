<template>
  <v-card class="h-100 d-flex flex-column game-eval-history">
    <v-card-item class="pb-0">
      <v-card-title class="text-subtitle-1 font-weight-semibold pa-0">
        {{ t('analysis.evalHistoryTitle') }}
      </v-card-title>
    </v-card-item>
    <v-card-text class="flex-grow-1 d-flex flex-column game-eval-history__body">
      <apexchart
        v-if="series[0].data.length"
        type="line"
        class="flex-grow-1"
        :height="chartHeight"
        :options="chartOptions"
        :series="series"
      />
      <v-alert
        v-else
        type="info"
        variant="tonal"
        class="flex-grow-1 d-flex align-center justify-center"
      >
        {{ t('analysis.evalHistoryEmpty') }}
      </v-alert>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useTheme } from 'vuetify';

import type { GameAnalysis } from '@/entities/game-analysis';
import { useI18n } from '@/shared/lib/i18n';

const props = defineProps<{
  analysis: GameAnalysis;
}>();

const { t } = useI18n();
const theme = useTheme();

const chartHeight = '400px';

const isDark = computed(() => theme.global.current.value.dark);
const textColor = computed(() => theme.current.value.colors['on-surface']);
const borderColor = computed(() => theme.current.value.colors.outline);
const lineColor = computed(() => theme.current.value.colors.secondary);

const series = computed(() => [
  {
    name: t('analysis.evalSeriesName'),
    data: props.analysis.eval_history.map((value, index) => ({
      x: index + 1,
      y: value / 100,
    })),
    color: lineColor.value,
  },
]);

const chartOptions = computed(() => ({
  chart: {
    toolbar: { show: false },
    background: 'transparent',
    zoom: { enabled: false },
  },
  theme: {
    mode: isDark.value ? 'dark' : 'light',
  },
  xaxis: {
    title: { text: t('analysis.evalAxisPly') },
    labels: { style: { colors: textColor.value } },
    axisBorder: { color: borderColor.value },
    axisTicks: { color: borderColor.value },
  },
  yaxis: {
    title: { text: t('analysis.evalAxisPawns') },
    labels: { style: { colors: textColor.value } },
  },
  grid: {
    borderColor: borderColor.value,
  },
  stroke: {
    width: 3,
    curve: 'smooth',
  },
  tooltip: {
    theme: isDark.value ? 'dark' : 'light',
  },
}));
</script>

<style scoped lang="scss">
.game-eval-history__body {
  min-height: 280px;
}
</style>
