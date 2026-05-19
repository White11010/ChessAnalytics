import { keepPreviousData, useQuery } from '@tanstack/vue-query';
import { invoke } from '@tauri-apps/api/core';
import { computed, type Ref } from 'vue';

import {
  averageAnalysisAccuracyPercentRounded,
  averageAnalysisAcplRounded,
  type Game,
  useGamesStore,
} from '@/entities/game';
import type { GameAnalysis } from '@/entities/game-analysis';
import type { PlayerProfileChartResponse } from '@/entities/player-profile-chart';
import {
  benchmarkAccuracyRounded,
  benchmarkAcplRounded,
} from '@/entities/player-profile-chart';
import { useI18n } from '@/shared/lib/i18n';

const PROFILE_SPEEDS = new Set(['bullet', 'blitz', 'rapid']);

function profileSpeedForGame(game: Game | undefined): string {
  const s = (game?.speed ?? 'rapid').toLowerCase();
  return PROFILE_SPEEDS.has(s) ? s : 'rapid';
}

export interface MetricBenchmarkLine {
  text: string;
  toneClass: string;
}

export function useGameMetricsBenchmarks(
  analysis: Ref<GameAnalysis>,
  game: Ref<Game | undefined>,
) {
  const { t } = useI18n();
  const gamesStore = useGamesStore();

  const profileSpeed = computed(() => profileSpeedForGame(game.value));

  const profileQuery = useQuery({
    queryKey: computed(() => ['playerProfileChart', 'gameDetails', profileSpeed.value]),
    queryFn: async () =>
      invoke<PlayerProfileChartResponse>('get_player_profile_chart', {
        speed: profileSpeed.value,
      }),
    staleTime: 1000 * 60 * 30,
    placeholderData: keepPreviousData,
  });

  const userAvgAccuracy = computed(() =>
    averageAnalysisAccuracyPercentRounded(gamesStore.games),
  );
  const ratingBenchAccuracy = computed(() => benchmarkAccuracyRounded(profileQuery.data.value));

  const userAvgAcpl = computed(() => averageAnalysisAcplRounded(gamesStore.games));
  const ratingBenchAcpl = computed(() => benchmarkAcplRounded(profileQuery.data.value));

  const accuracyBenchmarks = computed((): MetricBenchmarkLine[] => {
    const current = Math.round(analysis.value.accuracy);
    const lines: MetricBenchmarkLine[] = [];

    const userAvg = userAvgAccuracy.value;
    if (userAvg != null) {
      const delta = current - userAvg;
      if (delta !== 0) {
        const abs = Math.abs(delta);
        const key =
          delta < 0
            ? 'analysis.metricBenchmark.accuracyBelowUser'
            : 'analysis.metricBenchmark.accuracyAboveUser';
        lines.push({
          text: t(key, { delta: abs, avg: userAvg }),
          toneClass: delta < 0 ? 'text-error' : 'text-success',
        });
      }
    }

    const ratingAvg = ratingBenchAccuracy.value;
    if (ratingAvg != null) {
      const delta = current - ratingAvg;
      if (delta !== 0) {
        const abs = Math.abs(delta);
        const key =
          delta < 0
            ? 'analysis.metricBenchmark.accuracyBelowRating'
            : 'analysis.metricBenchmark.accuracyAboveRating';
        lines.push({
          text: t(key, { delta: abs, avg: ratingAvg }),
          toneClass: delta < 0 ? 'text-error' : 'text-success',
        });
      }
    }

    return lines;
  });

  const acplBenchmarks = computed((): MetricBenchmarkLine[] => {
    const current = Math.round(analysis.value.avg_centipawn_loss);
    const lines: MetricBenchmarkLine[] = [];

    const userAvg = userAvgAcpl.value;
    if (userAvg != null) {
      const delta = current - userAvg;
      if (delta !== 0) {
        const abs = Math.abs(delta);
        const key =
          delta > 0
            ? 'analysis.metricBenchmark.acplAboveUser'
            : 'analysis.metricBenchmark.acplBelowUser';
        lines.push({
          text: t(key, { delta: abs, avg: userAvg }),
          toneClass: delta > 0 ? 'text-error' : 'text-success',
        });
      }
    }

    const ratingAvg = ratingBenchAcpl.value;
    if (ratingAvg != null) {
      const delta = current - ratingAvg;
      if (delta !== 0) {
        const abs = Math.abs(delta);
        const key =
          delta > 0
            ? 'analysis.metricBenchmark.acplAboveRating'
            : 'analysis.metricBenchmark.acplBelowRating';
        lines.push({
          text: t(key, { delta: abs, avg: ratingAvg }),
          toneClass: delta > 0 ? 'text-error' : 'text-success',
        });
      }
    }

    return lines;
  });

  return {
    accuracyBenchmarks,
    acplBenchmarks,
  };
}
