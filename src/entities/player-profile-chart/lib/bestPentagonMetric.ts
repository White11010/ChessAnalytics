// Picks the strongest pentagon axis for the player vs benchmark for profile/KPI copy.

import type { PentagonDto, PlayerProfileChartResponse } from '../model/playerProfileChart.types';

export type PentagonMetricKey = keyof PentagonDto;

const METRIC_KEYS: PentagonMetricKey[] = ['accuracy', 'stability', 'conversion', 'openings', 'endgame'];

export interface BestPentagonMetricResult {
  key: PentagonMetricKey;
  player: number;
  bench: number | null;
}

function pentagonValue(dto: PentagonDto, key: PentagonMetricKey): number | null {
  const raw = key === 'conversion' ? dto.conversion : (dto[key] as number);
  if (raw == null || Number.isNaN(Number(raw))) {
    return null;
  }
  return Number(raw);
}

export function computeBestPentagonMetric(
  payload: PlayerProfileChartResponse | null | undefined,
): BestPentagonMetricResult | null {
  const player = payload?.player;
  if (!player) {
    return null;
  }
  let bestKey: PentagonMetricKey = 'accuracy';
  let bestVal = -1;
  for (const key of METRIC_KEYS) {
    const v = pentagonValue(player, key);
    if (v == null) {
      continue;
    }
    if (v > bestVal) {
      bestVal = v;
      bestKey = key;
    }
  }
  if (bestVal < 0) {
    return null;
  }
  const bench = payload.benchmark;
  const benchRaw = pentagonValue(bench, bestKey);
  const benchVal =
    benchRaw != null && !Number.isNaN(Number(benchRaw)) ? Math.round(Number(benchRaw)) : null;
  return {
    key: bestKey,
    player: Math.round(bestVal),
    bench: benchVal,
  };
}

export function benchmarkAccuracyRounded(
  payload: PlayerProfileChartResponse | null | undefined,
): number | null {
  const b = payload?.benchmark.accuracy;
  if (b == null || Number.isNaN(b)) {
    return null;
  }
  return Math.round(Number(b));
}
