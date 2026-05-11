// Primary metric string and label helpers for compact insight cards (hero, home top).

import type { Insight } from '../model/insight.types';

export function formatInsightMetricPrimaryValue(insight: Insight): string {
  if (insight.metric_value?.trim()) {
    return insight.metric_value.trim();
  }
  const n = insight.metric_number;
  if (n != null && Number.isFinite(n)) {
    return Number.isInteger(n) ? String(n) : n.toFixed(1);
  }
  return '';
}

export function insightMetricSecondaryLabel(insight: Insight): string {
  return insight.metric_label?.trim() ?? '';
}

export function insightMetricBlockVisible(insight: Insight | null): boolean {
  if (!insight) {
    return false;
  }
  return Boolean(formatInsightMetricPrimaryValue(insight) || insightMetricSecondaryLabel(insight));
}
