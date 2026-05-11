// Locates the standard 30-day rating growth insight in a list for dashboard KPIs.

import type { Insight } from '../model/insight.types';

const RATING_GROWTH_30D_KIND = 'time_rating_growth_30d' as const;

export function findRatingGrowth30dInsight(items: readonly Insight[]): Insight | undefined {
  return items.find((i) => i.kind === RATING_GROWTH_30D_KIND);
}
