// Rules for metric delta display: percent-point semantics by insight kind and optional delta captions.

import type { Insight } from '../model/insight.types';

type TFn = (key: string, ...args: unknown[]) => string;

export function insightDeltaUsesPercentPoints(kind: string): boolean {
  return (
    kind.startsWith('opening_') ||
    kind.startsWith('time_control_') ||
    kind === 'psychology_tilt' ||
    kind === 'psychology_comeback' ||
    kind === 'psychology_rest_effect' ||
    kind === 'tactics_conversion_advantage' ||
    kind === 'tactics_late_game_losses' ||
    kind === 'tactics_middlegame_vs_endgame' ||
    kind === 'tactics_side_performance' ||
    kind === 'tactics_accuracy_by_phase' ||
    kind === 'opponent_rating_performance' ||
    kind === 'time_games_per_day_pattern'
  );
}

export function getInsightDeltaCaption(insight: Insight, t: TFn): string | null {
  if (insight.metric_prev == null || insight.metric_number == null) {
    return null;
  }
  if (insight.kind === 'time_rating_growth_30d') {
    return t('insightsPage.deltaLast30');
  }
  if (insight.kind === 'time_morning_vs_evening') {
    return t('insightsPage.deltaRecentSample');
  }
  return null;
}
