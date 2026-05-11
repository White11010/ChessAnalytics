// KPI row for the insights dashboard: games accuracy vs benchmark, pentagon best axis, and rating growth insight.

import { keepPreviousData, useQuery } from '@tanstack/vue-query';
import { invoke } from '@tauri-apps/api/core';
import { computed } from 'vue';

import {
  averageAnalysisAccuracyPercentRounded,
  useGamesStore,
  useSyncGamesQuery,
} from '@/entities/game';
import { findRatingGrowth30dInsight, useInsightsStore } from '@/entities/insight';
import type { PentagonMetricKey, PlayerProfileChartResponse } from '@/entities/player-profile-chart';
import {
  benchmarkAccuracyRounded,
  computeBestPentagonMetric,
} from '@/entities/player-profile-chart';
import { useI18n } from '@/shared/lib/i18n';

const PROFILE_SPEED = 'rapid' as const;

function profileMetricLabel(
  t: (key: string, ...args: unknown[]) => string,
  key: PentagonMetricKey,
): string {
  switch (key) {
    case 'accuracy':
      return t('home.profileMetric.accuracy');
    case 'stability':
      return t('home.profileMetric.stability');
    case 'conversion':
      return t('home.profileMetric.conversion');
    case 'openings':
      return t('home.profileMetric.openings');
    case 'endgame':
      return t('home.profileMetric.endgame');
    default:
      return String(key);
  }
}

export function useInsightsBlockKpis() {
  const { t } = useI18n();
  const gamesStore = useGamesStore();
  const gamesQuery = useSyncGamesQuery();
  const insightsStore = useInsightsStore();

  const profileQuery = useQuery({
    queryKey: ['playerProfileChart', 'insightsKpi', PROFILE_SPEED],
    queryFn: async () =>
      invoke<PlayerProfileChartResponse>('get_player_profile_chart', { speed: PROFILE_SPEED }),
    staleTime: 1000 * 60 * 30,
    placeholderData: keepPreviousData,
  });

  const showKpiSkeleton = computed(() => gamesQuery.isPending.value);

  const avgAccuracyRounded = computed(() =>
    averageAnalysisAccuracyPercentRounded(gamesStore.games),
  );

  const benchAccuracyRounded = computed(() => benchmarkAccuracyRounded(profileQuery.data.value));

  const avgAccuracyDisplay = computed(() => {
    if (avgAccuracyRounded.value == null) {
      return String(t('common.emDash'));
    }
    return `${avgAccuracyRounded.value}%`;
  });

  const accuracyDeltaFromBench = computed(() => {
    if (avgAccuracyRounded.value == null || benchAccuracyRounded.value == null) {
      return null;
    }
    return avgAccuracyRounded.value - benchAccuracyRounded.value;
  });

  const accuracyDeltaAbs = computed(() => {
    const d = accuracyDeltaFromBench.value;
    if (d == null) {
      return '';
    }
    return String(Math.abs(d));
  });

  const accuracyDeltaArrow = computed(() => {
    const d = accuracyDeltaFromBench.value;
    if (d == null || d === 0) {
      return '';
    }
    return d > 0 ? '▲' : '▼';
  });

  const accuracyDeltaClass = computed(() => {
    const d = accuracyDeltaFromBench.value;
    if (d == null || d === 0) {
      return 'text-medium-emphasis';
    }
    return d > 0 ? 'text-success' : 'text-error';
  });

  const avgAccuracySubtext = computed(() => {
    if (avgAccuracyRounded.value == null) {
      return t('insightsPage.kpiAccuracyNoData');
    }
    if (benchAccuracyRounded.value == null) {
      return String(t('common.emDash'));
    }
    return '';
  });

  const bestAreaFromProfile = computed(() => computeBestPentagonMetric(profileQuery.data.value));

  const bestAreaLabel = computed(() => {
    const b = bestAreaFromProfile.value;
    if (!b) {
      return String(t('common.emDash'));
    }
    return profileMetricLabel(t, b.key);
  });

  const bestAreaHint = computed(() => {
    const b = bestAreaFromProfile.value;
    if (!b) {
      return t('insightsPage.kpiBestNoData');
    }
    const benchStr = b.bench != null ? String(b.bench) : String(t('common.emDash'));
    return t('insightsPage.kpiBestHint', { player: b.player, bench: benchStr });
  });

  const ratingGrowthInsight = computed(() => findRatingGrowth30dInsight(insightsStore.items));

  const ratingGrowthDisplay = computed(() => {
    const ins = ratingGrowthInsight.value;
    if (ins?.metric_value) {
      return ins.metric_value;
    }
    return String(t('common.emDash'));
  });

  const ratingGrowthSubtext = computed(() => {
    if (ratingGrowthInsight.value) {
      return t('insightsPage.kpiRatingHint');
    }
    return t('insightsPage.kpiRatingNoData');
  });

  return {
    showKpiSkeleton,
    avgAccuracyDisplay,
    accuracyDeltaFromBench,
    accuracyDeltaAbs,
    accuracyDeltaArrow,
    accuracyDeltaClass,
    avgAccuracySubtext,
    bestAreaLabel,
    bestAreaHint,
    ratingGrowthInsight,
    ratingGrowthDisplay,
    ratingGrowthSubtext,
  };
}
