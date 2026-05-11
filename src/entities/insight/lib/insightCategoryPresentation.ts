// Maps insight categories to i18n labels and Vuetify stripe colors for cards and chips.

import type { InsightCategory } from '../model/insight.types';

type TFn = (key: string, ...args: unknown[]) => string;

export function getInsightCategoryLabel(category: InsightCategory, t: TFn): string {
  switch (category) {
    case 'openings':
      return t('insightsPage.filterOpenings');
    case 'time':
      return t('insightsPage.filterTime');
    case 'tactics':
      return t('insightsPage.filterTactics');
    case 'psychology':
      return t('insightsPage.filterPsychology');
    default:
      return category;
  }
}

export function insightCategoryStripeColor(category: InsightCategory): string {
  switch (category) {
    case 'openings':
      return 'teal';
    case 'time':
      return 'info';
    case 'tactics':
      return 'warning';
    case 'psychology':
      return 'purple';
    default:
      return 'surface-variant';
  }
}
