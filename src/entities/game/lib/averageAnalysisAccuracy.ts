// Average rounded analysis accuracy across games that have an accuracy field.

import type { Game } from '../model/games.types';

export function averageAnalysisAccuracyPercentRounded(games: readonly Game[]): number | null {
  const vals = games
    .map((g) => g.analysis_accuracy)
    .filter((x): x is number => x != null && !Number.isNaN(x));
  if (vals.length === 0) {
    return null;
  }
  return Math.round(vals.reduce((a, b) => a + b, 0) / vals.length);
}
