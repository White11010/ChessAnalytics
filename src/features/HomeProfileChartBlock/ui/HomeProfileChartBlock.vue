<template>
  <v-card class="home-profile-card h-100">
    <v-card-title class="d-flex flex-wrap justify-space-between align-center ga-2">
      <span>{{ t('home.profileChartTitle') }}</span>
      <v-chip
        v-if="payload?.ratingUsed != null && speedChipLabel"
        size="small"
        variant="tonal"
        color="secondary"
        class="font-mono"
      >
        {{ payload.ratingUsed }} · {{ speedChipLabel }}
      </v-chip>
    </v-card-title>
    <v-card-text class="pb-2 pt-4">
      <v-btn-toggle
        v-model="speed"
        class="d-flex flex-wrap"
        mandatory
        base-color="secondary"
        variant="text"
        density="compact"
      >
        <v-btn value="bullet">{{ t('myGames.speed.bullet') }}</v-btn>
        <v-btn value="blitz">{{ t('myGames.speed.blitz') }}</v-btn>
        <v-btn value="rapid">{{ t('myGames.speed.rapid') }}</v-btn>
      </v-btn-toggle>
    </v-card-text>

    <v-card-text class="pt-2 home-profile-card__content">
      <div v-if="isPending" class="d-flex flex-column ga-3">
        <v-skeleton-loader type="button, button, button" />
        <v-skeleton-loader type="image" />
      </div>

      <template v-else-if="isError">
        <v-alert type="error" variant="tonal" density="compact">
          {{ t('detailsPage.tryRefresh') }}
        </v-alert>
      </template>

      <template v-else-if="payload">
        <div class="d-flex flex-wrap align-center ga-4 mb-2">
          <div v-if="payload.player" class="d-flex align-center ga-2 text-caption">
            <v-icon size="14" color="secondary" icon="mdi-circle" />
            <span>{{ t('home.profileChartLegendYours') }}</span>
          </div>
          <div class="d-flex align-center ga-2 text-caption text-medium-emphasis">
            <v-icon size="14" :color="'outline'" icon="mdi-circle-outline" />
            <span>{{ t('home.profileChartLegendAvg', { label: avgBucketSnippet }) }}</span>
          </div>
        </div>

        <v-alert
          v-if="!payload.player"
          density="compact"
          variant="tonal"
          type="info"
          class="mb-4"
          :text="String(t('home.profileChartLowSample', { n: payload.sampleSize }))"
        />

        <v-row dense class="home-profile-card__layout">
          <v-col cols="12" lg="8" class="d-flex justify-center align-center overflow-visible">
            <!-- ApexCharts Radar: stroke.width must be a number; arrays break radius (NaN) and collapse labels. -->
            <div class="home-profile-radar-wrap w-100 d-flex justify-center">
              <apexchart
                v-if="series.length"
                :key="chartKey"
                width="100%"
                height="460"
                type="radar"
                :options="chartOptions"
                :series="series"
              />
            </div>
          </v-col>
          <v-col cols="12" lg="4" class="d-flex">
            <div class="home-profile-metrics ps-lg-2">
              <div v-for="row in metricRows" :key="row.key" class="home-profile-metric-row">
                <div class="text-overline text-medium-emphasis">{{ row.label }}</div>
                <div class="text-h6 font-weight-bold mb-1" :style="{ color: row.accentRgb }">
                  {{ row.displayPlayer }}
                </div>
                <v-progress-linear
                  :model-value="row.barValue"
                  :color="row.barColor"
                  height="4"
                  rounded
                />
                <div class="mt-2 text-caption">
                  <span class="text-medium-emphasis"
                    >{{ t('home.profileChartAvgPrefix') }}: {{ row.roundedBench }}</span
                  >
                  <span class="text-medium-emphasis"> · </span>
                  <span :class="row.deltaClass">{{ row.deltaFormatted }}</span>
                </div>
              </div>
            </div>
          </v-col>
        </v-row>
      </template>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import type { ApexOptions } from 'apexcharts';

import { keepPreviousData, useQuery } from '@tanstack/vue-query';
import { invoke } from '@tauri-apps/api/core';
import { computed, ref, watch } from 'vue';
import { useTheme } from 'vuetify';

import { useI18n } from '@/shared/lib/i18n';

import type { PentagonDto, PlayerProfileChartResponse } from '../model/types';

type MetricKey = keyof PentagonDto;

const speed = ref<'bullet' | 'blitz' | 'rapid'>('bullet');
const chartKey = ref(0);

const { t, locale } = useI18n();
const theme = useTheme();

const METRIC_ORDER: MetricKey[] = ['accuracy', 'stability', 'conversion', 'openings', 'endgame'];
const ACCENT_NAMES = ['secondary', 'primary', 'warning', 'success', 'error'] as const;

const { data, isPending, isError } = useQuery({
  queryKey: ['playerProfileChart', speed],
  queryFn: async () =>
    invoke<PlayerProfileChartResponse>('get_player_profile_chart', { speed: speed.value }),
  placeholderData: keepPreviousData,
});

watch(
  () => [theme.global.current.value.dark, theme.global.name.value],
  () => {
    chartKey.value += 1;
  },
);

watch(locale, () => {
  chartKey.value += 1;
});

const payload = computed(() => data.value);

const avgBucketSnippet = computed(() => payload.value?.bucketLabel ?? '');

const speedChipLabel = computed(() => {
  switch (speed.value) {
    case 'bullet':
      return t('myGames.speed.bullet');
    case 'blitz':
      return t('myGames.speed.blitz');
    default:
      return t('myGames.speed.rapid');
  }
});

function themeHex(name: string): string | undefined {
  const c = theme.current.value.colors as Record<string, string | undefined>;
  return c[name];
}

function clampPercent(n: number): number {
  return Math.min(100, Math.max(0, n));
}

/** Avoid division blow-ups on near-zero benchmarks; chart is index vs cohort baseline. */
const BENCH_DENOM_EPS = 0.05;

/** Must match `chartOptions.yaxis[0].max` / `tickAmount` so series sits on a real grid ring. */
const RADAR_Y_MAX = 100;
const RADAR_TICK_AMOUNT = 5;
const RADAR_GRID_STEP = RADAR_Y_MAX / RADAR_TICK_AMOUNT;
/** Second concentric ring inward from outer scale (100 → 80 → 60). */
const RADAR_BASELINE = RADAR_Y_MAX - 2 * RADAR_GRID_STEP;

function absBenchMetric(bench: PentagonDto, k: MetricKey): number {
  const raw = k === 'conversion' ? Number(bench.conversion ?? 0) : Number(bench[k] ?? 0);
  return clampPercent(raw);
}

function absPlayerMetric(p: PentagonDto, bench: PentagonDto, k: MetricKey): number {
  if (k === 'conversion') {
    const conv = p.conversion == null ? Number(bench.conversion ?? 0) : p.conversion;
    return clampPercent(conv);
  }
  return clampPercent(p[k] as number);
}

/** Radar Y values vs cohort baseline (baseline series is flat RADAR_BASELINE). */
function playerRadarChartValues(player: PentagonDto, bench: PentagonDto): number[] {
  return METRIC_ORDER.map((k) => {
    const pv = absPlayerMetric(player, bench, k);
    const bv = Math.max(absBenchMetric(bench, k), BENCH_DENOM_EPS);
    return clampPercent((RADAR_BASELINE * pv) / bv);
  });
}

function radarTooltipHtml(
  d: PlayerProfileChartResponse,
  metricIndex: number,
  seriesIndex: number,
): string {
  const key = METRIC_ORDER[metricIndex];
  if (!key) return '';
  const title = metricCategoryLabel(metricIndex);
  const bench = d.benchmark;
  const bv = Math.round(absBenchMetric(bench, key));
  if (!d.player) {
    const v = bv;
    return `<div class="apexcharts-tooltip-title" style="font-family: inherit">${title}</div>
<div class="apexcharts-tooltip-series-group apexcharts-active" style="order: 1">
  <span>${String(t('home.profileChartLegendAvg', { label: d.bucketLabel }))}: ${v}</span>
</div>`;
  }
  const isPlayerSer = seriesIndex === 0;
  const pv = Math.round(absPlayerMetric(d.player, bench, key));
  if (isPlayerSer) {
    return `<div class="apexcharts-tooltip-title" style="font-family: inherit">${title}</div>
<div class="apexcharts-tooltip-series-group apexcharts-active" style="order: 1">
  <span>${String(t('home.profileChartLegendYours'))}: ${pv}</span>
</div>`;
  }
  return `<div class="apexcharts-tooltip-title" style="font-family: inherit">${title}</div>
<div class="apexcharts-tooltip-series-group apexcharts-active" style="order: 1">
  <span>${String(t('home.profileChartLegendAvg', { label: d.bucketLabel }))}: ${bv}</span>
</div>`;
}

/** Stable Apex categories (locale-independent) so axes never reorder relative to series data. */
function metricCategoryLabel(index: number): string {
  const key = METRIC_ORDER[index];
  if (!key) return '';
  if (key === 'accuracy') return t('home.profileMetric.accuracy');
  if (key === 'stability') return t('home.profileMetric.stability');
  if (key === 'conversion') return t('home.profileMetric.conversion');
  if (key === 'openings') return t('home.profileMetric.openings');
  return t('home.profileMetric.endgame');
}

const series = computed(() => {
  const d = payload.value;
  if (!d) {
    return [];
  }
  const benchRadar = METRIC_ORDER.map(() => RADAR_BASELINE);
  if (!d.player) {
    return [
      {
        name: String(t('home.profileChartLegendAvg', { label: d.bucketLabel })),
        data: benchRadar,
      },
    ];
  }
  const playerData = playerRadarChartValues(d.player, d.benchmark);
  return [
    { name: String(t('home.profileChartLegendYours')), data: playerData },
    { name: String(t('home.profileChartLegendAvg', { label: d.bucketLabel })), data: benchRadar },
  ];
});

const accentHexList = computed(() =>
  ACCENT_NAMES.map((n) => themeHex(n) ?? themeHex('secondary') ?? '#888888'),
);

function accentRgb(hex: string): string {
  if (!hex.startsWith('#') || hex.length < 7) {
    return 'rgb(var(--v-theme-secondary))';
  }
  const r = parseInt(hex.slice(1, 3), 16);
  const g = parseInt(hex.slice(3, 5), 16);
  const b = parseInt(hex.slice(5, 7), 16);
  return `rgb(${r}, ${g}, ${b})`;
}

const metricRows = computed(() => {
  const d = payload.value;
  if (!d) {
    return [];
  }
  const bench = d.benchmark;
  return METRIC_ORDER.map((key, i) => {
    const label =
      key === 'accuracy'
        ? t('home.profileMetric.accuracy')
        : key === 'stability'
          ? t('home.profileMetric.stability')
          : key === 'conversion'
            ? t('home.profileMetric.conversion')
            : key === 'openings'
              ? t('home.profileMetric.openings')
              : t('home.profileMetric.endgame');

    const bv = clampPercent(Number(bench[key] ?? 0));
    const hasPlayer = d.player != null;
    const pvRaw = !d.player
      ? null
      : key === 'conversion'
        ? d.player.conversion
        : (d.player[key] as number);

    const displayPlayer =
      key === 'conversion' && (pvRaw == null || !hasPlayer)
        ? String(t('common.emDash'))
        : `${Math.round(Number(pvRaw))}`;

    const barSource = pvRaw != null && hasPlayer ? Number(pvRaw) : bv;

    let deltaFormatted = String(t('common.emDash'));
    let deltaClass = 'text-medium-emphasis';
    let barColor: 'success' | 'error' | 'outline' = 'outline';
    if (pvRaw != null && hasPlayer) {
      const diff = Math.round(Number(pvRaw) - bv);
      deltaFormatted = diff > 0 ? `+${diff}` : String(diff);
      if (diff > 0) {
        deltaClass = 'text-success';
        barColor = 'success';
      } else if (diff < 0) {
        deltaClass = 'text-error';
        barColor = 'error';
      } else {
        deltaClass = 'text-medium-emphasis';
        barColor = 'outline';
      }
    }
    const hex = accentHexList.value[i]!;

    return {
      key,
      label,
      displayPlayer,
      roundedBench: Math.round(bv),
      barValue: clampPercent(barSource),
      barColor,
      deltaFormatted,
      deltaClass,
      accentRgb: accentRgb(hex),
    };
  });
});

const isDark = computed(() => theme.global.current.value.dark);

const textColor = computed(() => String(theme.current.value.colors['on-surface']));
const outlineSoft = computed(() => themeHex('outline-variant') ?? themeHex('outline') ?? '#666666');

/** Muted polygon fill matching outline (semi-transparent gray). */
const benchmarkFillMuted = computed(() => {
  const hex = outlineSoft.value;
  if (!hex.startsWith('#') || hex.length < 7) {
    return 'rgba(128,128,128,0.22)';
  }
  const r = parseInt(hex.slice(1, 3), 16);
  const g = parseInt(hex.slice(3, 5), 16);
  const b = parseInt(hex.slice(5, 7), 16);
  return `rgba(${r},${g},${b},0.22)`;
});

const chartOptions = computed((): ApexOptions => {
  const d = payload.value;
  const hasPlayer = !!(d && d.player);
  const playerMarkerColors = [...accentHexList.value];
  const sec = themeHex('secondary') ?? '#F59E0B';
  const benchmarkStroke = outlineSoft.value;

  return {
    chart: {
      toolbar: { show: false },
      animations: { enabled: true },
      fontFamily: 'inherit',
      background: 'transparent',
      dropShadow: { enabled: false },
    },
    theme: { mode: isDark.value ? 'dark' : 'light' },
    plotOptions: {
      radar: {
        size: 120,
        offsetX: 0,
        offsetY: 2,
        polygons: {
          strokeColors: outlineSoft.value,
          fill: {
            colors: ['transparent'],
          },
        },
      },
    },
    stroke: {
      curve: 'straight',
      show: true,
      width: 2,
      dashArray: hasPlayer ? [0, 6] : 0,
    },
    markers: {
      size: hasPlayer ? [5, 0] : 0,
      hover: { size: 7 },
      strokeWidth: hasPlayer ? [1, 0] : 0,
      ...(hasPlayer
        ? {
            fillColors: playerMarkerColors,
            strokeColors: playerMarkerColors,
          }
        : {}),
    },
    fill: {
      type: 'solid',
      opacity: hasPlayer ? [0.14, 0.28] : [0.26],
      colors: hasPlayer ? [sec, benchmarkFillMuted.value] : [benchmarkFillMuted.value],
    },
    colors: hasPlayer ? [sec, benchmarkStroke] : [benchmarkStroke],
    xaxis: {
      categories: [...METRIC_ORDER],
      labels: {
        trim: false,
        hideOverlappingLabels: false,
        formatter: (
          value: string | number,
          _timestamp?: number,
          opts?: { dataPointIndex?: number },
        ) => {
          if (typeof opts?.dataPointIndex === 'number') {
            return metricCategoryLabel(opts.dataPointIndex);
          }
          const ki = METRIC_ORDER.indexOf(String(value) as MetricKey);
          return ki >= 0 ? metricCategoryLabel(ki) : String(value);
        },
        style: {
          colors: Array(METRIC_ORDER.length).fill(textColor.value),
          fontSize: '11px',
        },
      },
    },
    yaxis: [
      {
        show: false,
        min: 0,
        max: RADAR_Y_MAX,
        tickAmount: RADAR_TICK_AMOUNT,
        forceNiceScale: false,
        decimalsInFloat: 0,
        labels: {
          show: false,
        },
      },
    ],
    tooltip: {
      theme: isDark.value ? 'dark' : 'light',
      shared: false,
      intersect: true,
      custom: (opts: { seriesIndex?: number; dataPointIndex?: number }) => {
        const p = payload.value;
        if (!p) return '';
        const si = opts.seriesIndex ?? 0;
        const di = opts.dataPointIndex ?? 0;
        return radarTooltipHtml(p, di, si);
      },
    },
    legend: { show: false },
    dataLabels: {
      enabled: false,
    },
  };
});
</script>

<style scoped lang="scss">
.home-profile-radar-wrap {
  min-width: 560px;
  max-width: 760px;
  overflow: visible;
}

.home-profile-radar-wrap :deep(.apexcharts-canvas),
.home-profile-radar-wrap :deep(svg) {
  overflow: visible;
}

.home-profile-card {
  display: flex;
  flex-direction: column;
}

.home-profile-card__content {
  flex: 1 1 auto;
  display: flex;
  flex-direction: column;
}

.home-profile-card__layout {
  flex: 1 1 auto;
  min-height: 0;
}

.home-profile-metrics {
  height: 100%;
  width: 100%;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
}

.home-profile-metric-row {
  flex: 1 1 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
  min-height: 0;
}
</style>
