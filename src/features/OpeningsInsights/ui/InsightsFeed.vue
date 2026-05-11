<template>
  <v-row dense>
    <template v-for="item in insights" :key="item.id">
      <v-col cols="12" md="6">
        <v-card class="h-100 overflow-hidden position-relative">
          <v-sheet
            class="position-absolute start-0 top-0 bottom-0"
            width="4"
            :color="insightCategoryStripeColor(item.category)"
            rounded="0"
          />
          <v-card-item class="ps-6">
            <v-chip
              size="small"
              variant="tonal"
              :color="insightCategoryStripeColor(item.category)"
              class="mb-2"
            >
              {{ getInsightCategoryLabel(item.category, t) }}
            </v-chip>
            <v-card-title class="text-wrap text-body-1 ps-0 pt-0">
              {{ getInsightTitle(item, t, te) }}
            </v-card-title>
          </v-card-item>
          <v-card-text class="pt-0 ps-6">
            <div
              v-if="item.metric_value || item.metric_number != null"
              class="d-flex flex-wrap align-baseline ga-1"
            >
              <span v-if="metricLabel(item)" class="text-caption text-medium-emphasis w-100">{{
                metricLabel(item)
              }}</span>
              <span class="text-h5 font-weight-medium">{{ item.metric_value ?? 'вЂ”' }}</span>
              <MetricDelta
                v-if="item.metric_number != null"
                :current="item.metric_number"
                :prev="item.metric_prev"
                :percent-points="insightDeltaUsesPercentPoints(item.kind)"
              />
            </div>
            <div v-if="deltaCaption(item)" class="text-caption text-medium-emphasis mt-1">
              {{ deltaCaption(item) }}
            </div>
            <p class="mt-2 mb-1">{{ getInsightSummary(item, t, te) }}</p>
            <p v-if="recommendationText(item)" class="text-body-2 text-medium-emphasis mb-0">
              {{ recommendationText(item) }}
            </p>
            <div class="d-flex flex-wrap align-center justify-space-between ga-2 mt-3">
              <span class="text-caption text-medium-emphasis">
                {{ t('home.insightConfidence', { pct: Math.round(item.confidence) }) }}
              </span>
              <v-btn
                v-if="canNavigateInsightToMyGames(item)"
                variant="text"
                size="small"
                class="text-none"
                @click="emit('view-games', item)"
              >
                {{ t('insightsPage.viewGames') }}
                <v-icon icon="mdi-arrow-right" end size="16" />
              </v-btn>
            </div>
          </v-card-text>
        </v-card>
      </v-col>
    </template>
  </v-row>
</template>

<script setup lang="ts">
// Two-column feed of insight cards below the hero on the insights page.

import {
  canNavigateInsightToMyGames,
  getInsightCategoryLabel,
  getInsightDeltaCaption,
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
  insights: Insight[];
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

function deltaCaption(ins: Insight): string | null {
  return getInsightDeltaCaption(ins, t);
}
</script>
