// Whether an insight kind shows the headline metric row on insight cards.

const KINDS_WITHOUT_HEADLINE_METRIC = new Set([
  'opening_trend',
  'opponent_rating_performance',
  'psychology_tilt',
  'tactics_middlegame_vs_endgame',
]);

export function insightShowsHeadlineMetric(kind: string): boolean {
  return !KINDS_WITHOUT_HEADLINE_METRIC.has(kind);
}

const PRELINE_SUMMARY_KINDS = new Set([
  'opponent_rating_performance',
  'opening_trend',
  'tactics_middlegame_vs_endgame',
]);

export function insightSummaryUsesPreLine(kind: string): boolean {
  return PRELINE_SUMMARY_KINDS.has(kind);
}
