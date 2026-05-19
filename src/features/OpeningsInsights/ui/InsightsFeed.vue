<template>
  <v-row dense>
    <template v-for="item in insights" :key="item.id">
      <v-col cols="12" md="6">
        <v-card
          class="insight-card h-100 overflow-hidden position-relative d-flex flex-column"
          :variant="isFeatured(item) ? 'tonal' : 'elevated'"
        >
          <v-sheet
            class="position-absolute start-0 top-0 bottom-0"
            width="4"
            :color="insightCategoryStripeColor(item.category)"
            rounded="0"
          />
          <v-card-item class="ps-6 flex-shrink-0">
            <div class="d-flex flex-wrap align-center ga-2 mb-2">
              <v-chip v-if="isFeatured(item)" color="secondary" size="small" variant="flat">
                <v-icon icon="mdi-star" start size="16" />
                {{ t('insightsPage.featuredBadge') }}
              </v-chip>
              <v-chip
                size="small"
                variant="tonal"
                :color="insightCategoryStripeColor(item.category)"
              >
                {{ getInsightCategoryLabel(item.category, t) }}
              </v-chip>
            </div>
            <v-card-title class="text-wrap text-body-1 ps-0 pt-0">
              {{ getInsightTitle(item, t, te) }}
            </v-card-title>
          </v-card-item>
          <v-card-text class="insight-card__body pt-0 ps-6 d-flex flex-column flex-grow-1">
            <div class="insight-card__content flex-grow-1">
              <div
                v-if="metricBlockVisible(item)"
                class="d-flex flex-wrap align-baseline ga-1 mb-2"
              >
                <template v-if="metricHeadline(item)">
                  <span class="text-h5 font-weight-medium">{{ metricHeadline(item) }}</span>
                </template>
                <template v-else>
                  <span v-if="metricLabel(item)" class="text-caption text-medium-emphasis w-100">{{
                    metricLabel(item)
                  }}</span>
                  <span class="text-h5 font-weight-medium">{{
                    formatInsightMetricPrimaryValue(item)
                  }}</span>
                </template>
              </div>
              <div v-if="deltaCaption(item)" class="text-caption text-medium-emphasis mb-2">
                {{ deltaCaption(item) }}
              </div>
              <p class="mb-1" :class="{ 'insight-summary--preline': summaryUsesPreLine(item) }">
                {{ getInsightSummary(item, t, te) }}
              </p>
              <p v-if="recommendationText(item)" class="text-body-2 text-medium-emphasis mb-0">
                {{ recommendationText(item) }}
              </p>
            </div>
            <div
              class="insight-card__footer d-flex flex-wrap align-center justify-space-between ga-2 pt-3 mt-auto flex-shrink-0"
            >
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
// Two-column insight cards; optional featured card is the first item in the same grid.

import {
  canNavigateInsightToMyGames,
  formatInsightMetricPrimaryValue,
  getInsightCategoryLabel,
  getInsightDeltaCaption,
  getInsightMetricLabel,
  getInsightRecommendation,
  getInsightSummary,
  getInsightTitle,
  insightCategoryStripeColor,
  insightMetricBlockVisible,
  insightMetricHeadline,
  insightSummaryUsesPreLine,
  type Insight,
} from '@/entities/insight';
import { useI18n } from '@/shared/lib/i18n';

const props = defineProps<{
  insights: Insight[];
  featuredId?: string | null;
}>();

const emit = defineEmits<{
  'view-games': [insight: Insight];
}>();

const { t, te } = useI18n();

function isFeatured(item: Insight): boolean {
  return Boolean(props.featuredId && item.id === props.featuredId);
}

function metricBlockVisible(ins: Insight): boolean {
  return insightMetricBlockVisible(ins);
}

function metricLabel(ins: Insight): string | null {
  return getInsightMetricLabel(ins, t, te);
}

function metricHeadline(ins: Insight): string | null {
  return insightMetricHeadline(ins, metricLabel(ins));
}

function recommendationText(ins: Insight): string | null {
  return getInsightRecommendation(ins, t, te);
}

function deltaCaption(ins: Insight): string | null {
  return getInsightDeltaCaption(ins, t);
}

function summaryUsesPreLine(ins: Insight): boolean {
  return insightSummaryUsesPreLine(ins.kind);
}
</script>

<style scoped>
.insight-card__body {
  min-height: 0;
}

.insight-summary--preline {
  white-space: pre-line;
}
</style>
