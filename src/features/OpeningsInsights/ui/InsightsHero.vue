<template>
  <v-row dense>
    <v-col cols="12">
      <v-card variant="tonal" color="secondary" class="overflow-hidden position-relative w-100">
        <v-sheet
          class="position-absolute start-0 top-0 bottom-0"
          width="4"
          :color="insightCategoryStripeColor(insight.category)"
          rounded="0"
        />
        <v-card-text class="ps-6 pb-0">
          <div class="d-flex flex-wrap align-center ga-2 mb-3">
            <v-chip color="secondary" size="small" variant="flat">
              <v-icon icon="mdi-star" start size="16" />
              {{ t('insightsPage.featuredBadge') }}
            </v-chip>
            <v-chip
              size="small"
              variant="tonal"
              :color="insightCategoryStripeColor(insight.category)"
            >
              {{ getInsightCategoryLabel(insight.category, t) }}
            </v-chip>
          </div>
          <div class="text-h6 font-weight-medium">
            {{ getInsightTitle(insight, t, te) }}
          </div>
        </v-card-text>
        <v-card-text class="ps-6 d-flex flex-column flex-md-row ga-4 pt-0">
          <div class="flex-grow-1">
            <p class="text-body-1 text-medium-emphasis mb-0">
              {{ getInsightSummary(insight, t, te) }}
            </p>
            <p
              v-if="recommendationText(insight)"
              class="text-body-2 text-medium-emphasis mt-3 mb-0"
            >
              {{ recommendationText(insight) }}
            </p>
          </div>
          <div
            v-if="insight.metric_value || insight.metric_number != null"
            class="d-flex flex-column align-md-end flex-shrink-0"
          >
            <span v-if="metricLabel(insight)" class="text-caption text-medium-emphasis">{{
              metricLabel(insight)
            }}</span>
            <span class="text-h3 font-weight-bold text-secondary">{{
              insight.metric_value ?? 'РІР‚вЂќ'
            }}</span>
            <MetricDelta
              v-if="insight.metric_number != null"
              :current="insight.metric_number"
              :prev="insight.metric_prev"
              :percent-points="insightDeltaUsesPercentPoints(insight.kind)"
            />
          </div>
        </v-card-text>
        <v-card-actions v-if="canNavigateInsightToMyGames(insight)" class="ps-6 pb-4">
          <v-btn
            variant="text"
            color="primary"
            class="text-none px-0"
            @click="emit('view-games', insight)"
          >
            {{ t('insightsPage.viewGames') }}
            <v-icon icon="mdi-arrow-right" end size="18" />
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-col>
  </v-row>
</template>

<script setup lang="ts">
// Featured hero card for the top insight on the insights page.

import {
  canNavigateInsightToMyGames,
  getInsightCategoryLabel,
  getInsightMetricLabel,
  getInsightRecommendation,
  getInsightSummary,
  getInsightTitle,
  insightCategoryStripeColor,
  insightDeltaUsesPercentPoints,
  type Insight,
} from '@/entities/insight';
import { useI18n } from '@/shared/lib/i18n';
import { MetricDelta } from '@/shared/ui';

defineProps<{
  insight: Insight;
}>();

const emit = defineEmits<{
  'view-games': [insight: Insight];
}>();

const { t, te } = useI18n();

function metricLabel(ins: Insight): string | null {
  return getInsightMetricLabel(ins, t, te);
}

function recommendationText(ins: Insight): string | null {
  return getInsightRecommendation(ins, t, te);
}
</script>
