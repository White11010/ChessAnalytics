// Public API for insight domain: display helpers, filters, stores, and My Games navigation.
export {
  getInsightMetricLabel,
  getInsightRecommendation,
  getInsightSummary,
  getInsightTitle,
  resolveInsightDisplayRecommendation,
  resolveInsightDisplaySummary,
  resolveInsightDisplayTitle,
} from './lib/formatInsightForDisplay';
export {
  getInsightCategoryLabel,
  insightCategoryStripeColor,
} from './lib/insightCategoryPresentation';
export {
  getInsightDeltaCaption,
  insightDeltaUsesPercentPoints,
} from './lib/insightDeltaPresentation';
export {
  formatInsightMetricPrimaryValue,
  insightMetricBlockVisible,
  insightMetricHeadline,
  insightMetricSecondaryLabel,
} from './lib/insightMetricLine';
export { insightShowsHeadlineMetric, insightSummaryUsesPreLine } from './lib/insightMetricPresentation';
export { findRatingGrowth30dInsight } from './lib/insightRatingGrowthKpi';
export {
  insightSeverityToSemanticColor,
  type InsightSeveritySemanticColor,
} from './lib/insightSeverity';
export {
  mergeInsightDisplayParams,
  openingNameFromSubjectKey,
  parseInsightPayload,
  speedLabelFromSubjectKey,
} from './lib/insightPayload';
export {
  buildMyGamesFiltersFromInsight,
  canNavigateInsightToMyGames,
} from './lib/insightMyGamesNavigation';
export type { InsightMyGamesFilterPatch } from './lib/insightMyGamesNavigation';
export { useInsightsLoadQuery, useRegenerateInsightsQuery } from './lib/insightsQuery';
export { useInsightsStore } from './model/insight.store';
export { useInsightsFiltersStore } from './model/insightsFilters.store';
export type {
  InsightsFilterKey,
  InsightsFiltersSnapshot,
  InsightsSortOrder,
} from './model/insightsFilters.store';
export type { Insight, InsightCategory } from './model/insight.types';
