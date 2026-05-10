<template>
  <v-card class="home-top-insight" :class="`home-top-insight--${severityColor}`">
    <div class="home-top-insight__accent" aria-hidden="true" />

    <template v-if="showSkeleton">
      <div class="home-top-insight__body">
        <v-skeleton-loader type="heading" class="mb-4" />
        <v-skeleton-loader type="paragraph" />
        <v-skeleton-loader type="button@2" class="mt-6" />
      </div>
    </template>

    <template v-else-if="ins">
      <div class="home-top-insight__body">
        <header class="home-top-insight__header">
          <div class="home-top-insight__chips">
            <span class="home-top-insight__chip home-top-insight__chip--day">
              {{ primaryChipLabel }}
            </span>
            <span class="home-top-insight__chip home-top-insight__chip--cat">
              {{ categoryLabel(ins.category) }}
            </span>
          </div>
          <span class="home-top-insight__confidence">
            {{ t('home.insightConfidence', { pct: Math.round(ins.confidence) }) }}
          </span>
        </header>

        <h2 class="home-top-insight__title">{{ displayTitle }}</h2>

        <div v-if="metricBlockVisible" class="home-top-insight__stat-bar">
          <span v-if="metricMain" class="home-top-insight__stat-bar-value">{{ metricMain }}</span>
          <span v-if="metricSecondary" class="home-top-insight__stat-bar-label">{{
            metricSecondary
          }}</span>
        </div>

        <p class="home-top-insight__summary">{{ displaySummary }}</p>

        <div v-if="recommendationText" class="home-top-insight__tip">
          <v-icon icon="mdi-lightbulb-on-outline" size="22" class="home-top-insight__tip-icon" />
          <span class="home-top-insight__tip-text">{{ recommendationText }}</span>
        </div>
      </div>

      <!-- <footer class="home-top-insight__footer">
        <v-btn
          variant="outlined"
          class="home-top-insight__btn"
          :disabled="nextDisabled"
          @click="goNextInsight"
        >
          <span class="home-top-insight__btn-label">{{ t('home.nextInsight') }}</span>
          <v-icon icon="mdi-arrow-right" size="18" class="home-top-insight__btn-arrow" />
        </v-btn>
        <v-btn variant="outlined" class="home-top-insight__btn" @click="goAllInsights">
          {{ t('home.allInsights') }}
        </v-btn>
      </footer> -->
    </template>

    <div v-else-if="showEmpty" class="home-top-insight__body home-top-insight__empty">
      <p class="home-top-insight__summary mb-4">{{ t('home.noInsightsYet') }}</p>
      <v-btn color="primary" variant="tonal" :to="{ name: 'Insights' }">
        {{ t('home.generateInsights') }}
      </v-btn>
    </div>
  </v-card>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { storeToRefs } from 'pinia';

import {
  getInsightRecommendation,
  getInsightSummary,
  getInsightTitle,
  useInsightsLoadQuery,
  useInsightsStore,
} from '@/entities/insight';
import type { Insight, InsightCategory } from '@/entities/insight/model/insight.types';
import { useSyncGamesQuery } from '@/entities/game';
import { useI18n } from '@/shared/lib/i18n';

const { t, te } = useI18n();

const gamesQuery = useSyncGamesQuery();
const insightsQuery = useInsightsLoadQuery(gamesQuery.isSuccess);

const insightsStore = useInsightsStore();
const { heroInsight } = storeToRefs(insightsStore);

const displayInsight = ref<Insight | null>(null);

watch(
  heroInsight,
  (hero) => {
    displayInsight.value = hero ?? null;
  },
  { immediate: true },
);

const ins = computed(() => displayInsight.value);

const queryEnabled = computed(() => gamesQuery.isSuccess.value);

const showSkeleton = computed(
  () =>
    queryEnabled.value &&
    (insightsQuery.isPending.value || insightsQuery.isFetching.value) &&
    ins.value === null,
);

const showEmpty = computed(
  () =>
    queryEnabled.value &&
    !insightsQuery.isPending.value &&
    !insightsQuery.isFetching.value &&
    ins.value === null,
);

function severityToColor(severity: Insight['severity']): 'success' | 'warning' | 'error' | 'info' {
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

const severityColor = computed(() => (ins.value ? severityToColor(ins.value.severity) : 'info'));

const showingDaily = computed(() => {
  const h = heroInsight.value;
  const d = displayInsight.value;
  return Boolean(h && d && h.id === d.id);
});

const primaryChipLabel = computed(() =>
  showingDaily.value ? t('home.insightOfDayChip') : t('home.insightPreviewChip'),
);

function categoryLabel(c: InsightCategory): string {
  switch (c) {
    case 'openings':
      return t('insightsPage.filterOpenings');
    case 'time':
      return t('insightsPage.filterTime');
    case 'tactics':
      return t('insightsPage.filterTactics');
    case 'psychology':
      return t('insightsPage.filterPsychology');
    default:
      return c;
  }
}

function displayTitleFor(i: Insight): string {
  const raw = i.title?.trim();
  if (raw) {
    return raw;
  }
  return getInsightTitle(i, t, te);
}

function displaySummaryFor(i: Insight): string {
  const raw = i.summary?.trim();
  if (raw) {
    return raw;
  }
  return getInsightSummary(i, t, te);
}

const displayTitle = computed(() => (ins.value ? displayTitleFor(ins.value) : ''));

const displaySummary = computed(() => (ins.value ? displaySummaryFor(ins.value) : ''));

const recommendationText = computed(() => {
  const i = ins.value;
  if (!i) {
    return null;
  }
  const raw = i.recommendation?.trim();
  if (raw) {
    return raw;
  }
  return getInsightRecommendation(i, t, te);
});

const metricMain = computed(() => {
  const i = ins.value;
  if (!i) {
    return '';
  }
  if (i.metric_value?.trim()) {
    return i.metric_value.trim();
  }
  if (i.metric_number != null && Number.isFinite(i.metric_number)) {
    const n = i.metric_number;
    return Number.isInteger(n) ? String(n) : n.toFixed(1);
  }
  return '';
});

const metricSecondary = computed(() => ins.value?.metric_label?.trim() ?? '');

const metricBlockVisible = computed(() => Boolean(metricMain.value || metricSecondary.value));
</script>

<style lang="scss" scoped>
.home-top-insight {
  position: relative;
  overflow: hidden;
  min-height: max-content;
  height: 100%;
  display: flex;
  flex-direction: column;
  background: rgb(var(--v-theme-surface));

  /* Accent channel for this card (matches Vuetify semantic colors) */
  &--success {
    --hti-accent: rgb(var(--v-theme-success));
  }
  &--warning {
    --hti-accent: rgb(var(--v-theme-warning));
  }
  &--error {
    --hti-accent: rgb(var(--v-theme-error));
  }
  &--info {
    --hti-accent: rgb(var(--v-theme-info));
  }
}

/* Thin top rule like the mock */
.home-top-insight__accent {
  height: 2px;
  width: 100%;
  flex-shrink: 0;
  background: var(--hti-accent);
  opacity: 0.95;
}

.home-top-insight__body {
  padding: 1.125rem 1.25rem 0 1rem;
  flex: 1 1 auto;
}

.home-top-insight__header {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 0.5rem 1rem;
  margin-bottom: 1.125rem;
}

.home-top-insight__chips {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 0.5rem;
  min-width: 0;
}

.home-top-insight__chip {
  display: inline-flex;
  align-items: center;
  border-radius: 9999px;
  font-size: 0.6875rem;
  font-weight: 600;
  letter-spacing: 0.02em;
  line-height: 1.2;
  padding: 0.35rem 0.65rem;
  white-space: nowrap;
}

.home-top-insight__chip--day {
  background: var(--hti-accent);
  color: #fff;
  box-shadow: 0 0 0 1px rgba(0, 0, 0, 0.12);
}

.home-top-insight__chip--cat {
  background: rgba(var(--v-theme-on-surface), 0.08);
  color: rgba(var(--v-theme-on-surface), 0.72);
}

.home-top-insight__confidence {
  font-size: 0.75rem;
  line-height: 1.25;
  color: rgba(var(--v-theme-on-surface), 0.5);
  text-align: right;
  flex-shrink: 0;
  margin-left: auto;
}

.home-top-insight__title {
  margin: 0 0 1rem;
  font-size: 1.125rem;
  font-weight: 700;
  line-height: 1.35;
  letter-spacing: -0.01em;
  color: rgb(var(--v-theme-on-surface));
}

/* Light “status” strip: value + label in one row */
.home-top-insight__stat-bar {
  display: flex;
  flex-wrap: wrap;
  align-items: baseline;
  gap: 0.5rem 0.75rem;
  margin-bottom: 0.875rem;
  padding: 0.65rem 1rem;
  border-radius: 0.625rem;
  background: #c8c8c8;
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.35);
}

.home-top-insight__stat-bar-value {
  font-size: 1.5rem;
  font-weight: 800;
  line-height: 1.1;
  color: var(--hti-accent);
  text-shadow: 0 1px 0 rgba(255, 255, 255, 0.25);
}

.home-top-insight__stat-bar-label {
  font-size: 0.8125rem;
  font-weight: 500;
  line-height: 1.35;
  color: rgba(0, 0, 0, 0.48);
  max-width: 100%;
}

.home-top-insight__summary {
  margin: 0 0 1rem;
  font-size: 0.8125rem;
  line-height: 1.45;
  font-weight: 400;
  color: rgba(var(--v-theme-on-surface), 0.58);
}

.home-top-insight__tip {
  display: flex;
  align-items: flex-start;
  gap: 0.625rem;
  padding: 0.875rem 1rem;
  border-radius: 0.625rem;
  border: 1px solid color-mix(in srgb, var(--hti-accent) 55%, transparent);
  background: color-mix(in srgb, var(--hti-accent) 14%, rgb(12, 12, 14));
  box-shadow: 0 0 0 1px rgba(0, 0, 0, 0.2);
}

.home-top-insight__tip-icon {
  flex-shrink: 0;
  margin-top: 1px;
  color: rgba(255, 255, 255, 0.92) !important;
  opacity: 0.95;
}

.home-top-insight__tip-text {
  font-size: 0.8125rem;
  line-height: 1.45;
  color: rgba(255, 255, 255, 0.92);
}

.home-top-insight__footer {
  display: flex;
  flex-direction: row;
  align-items: stretch;
  gap: 0.625rem;
  padding: 0 1.25rem 1.125rem;
}

.home-top-insight__btn {
  flex: 1 1 0;
  min-width: 0;
  height: auto !important;
  min-height: 2.5rem;
  padding-inline: 0.5rem !important;
  text-transform: none;
  letter-spacing: 0.01em;
  font-size: 0.8125rem;
  font-weight: 500;
  border-color: rgba(255, 255, 255, 0.42) !important;
  color: rgba(255, 255, 255, 0.92) !important;
}

.home-top-insight__btn:deep(.v-btn__content) {
  gap: 0.35rem;
}

.home-top-insight__btn-label {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.home-top-insight__btn-arrow {
  flex-shrink: 0;
  opacity: 0.9;
}

.home-top-insight__empty {
  padding-bottom: 1.25rem;
}
</style>
<!-- Theme overrides: parent has .v-theme--light, not this card -->
<style lang="scss">
.v-theme--light .home-top-insight {
  .home-top-insight__stat-bar {
    background: #d5d5d5;
  }

  .home-top-insight__stat-bar-label {
    color: rgba(0, 0, 0, 0.55);
  }

  .home-top-insight__tip {
    background: color-mix(in srgb, var(--hti-accent) 12%, rgb(var(--v-theme-surface)));
    box-shadow: none;
  }

  .home-top-insight__tip-text,
  .home-top-insight__tip-icon {
    color: rgba(var(--v-theme-on-surface), 0.88) !important;
  }

  .home-top-insight__btn {
    border-color: rgba(var(--v-theme-on-surface), 0.32) !important;
    color: rgb(var(--v-theme-on-surface)) !important;
  }
}
</style>
