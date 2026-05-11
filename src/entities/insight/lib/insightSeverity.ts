// Maps insight severity levels to Vuetify semantic color names for UI accents.

import type { Insight } from '../model/insight.types';

export type InsightSeveritySemanticColor = 'success' | 'warning' | 'error' | 'info';

export function insightSeverityToSemanticColor(severity: Insight['severity']): InsightSeveritySemanticColor {
  switch (severity) {
    case 'good':
      return 'success';
    case 'warning':
      return 'warning';
    case 'critical':
      return 'error';
    case 'info':
    default:
      return 'info';
  }
}
