// Wires games sync and insights load query for the home top insight card content and loading states.

import { computed, ref, watch } from 'vue';
import { storeToRefs } from 'pinia';

import { useSyncGamesQuery } from '@/entities/game';
import {
  formatInsightMetricPrimaryValue,
  getInsightCategoryLabel,
  insightMetricBlockVisible,
  insightMetricSecondaryLabel,
  insightSeverityToSemanticColor,
  resolveInsightDisplayRecommendation,
  resolveInsightDisplaySummary,
  resolveInsightDisplayTitle,
  useInsightsLoadQuery,
  useInsightsStore,
  type Insight,
  type InsightCategory,
} from '@/entities/insight';
import { useI18n } from '@/shared/lib/i18n';

export function useHomeTopInsightBlock() {
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

  const severityColor = computed(() =>
    ins.value ? insightSeverityToSemanticColor(ins.value.severity) : 'info',
  );

  const showingDaily = computed(() => {
    const h = heroInsight.value;
    const d = displayInsight.value;
    return Boolean(h && d && h.id === d.id);
  });

  const primaryChipLabel = computed(() =>
    showingDaily.value ? t('home.insightOfDayChip') : t('home.insightPreviewChip'),
  );

  function categoryLabel(c: InsightCategory): string {
    return getInsightCategoryLabel(c, t);
  }

  const displayTitle = computed(() =>
    ins.value ? resolveInsightDisplayTitle(ins.value, t, te) : '',
  );

  const displaySummary = computed(() =>
    ins.value ? resolveInsightDisplaySummary(ins.value, t, te) : '',
  );

  const recommendationText = computed(() => {
    const i = ins.value;
    if (!i) {
      return null;
    }
    return resolveInsightDisplayRecommendation(i, t, te);
  });

  const metricMain = computed(() => (ins.value ? formatInsightMetricPrimaryValue(ins.value) : ''));

  const metricSecondary = computed(() =>
    ins.value ? insightMetricSecondaryLabel(ins.value) : '',
  );

  const metricBlockVisible = computed(() => insightMetricBlockVisible(ins.value));

  return {
    ins,
    showSkeleton,
    showEmpty,
    severityColor,
    primaryChipLabel,
    categoryLabel,
    displayTitle,
    displaySummary,
    recommendationText,
    metricMain,
    metricSecondary,
    metricBlockVisible,
  };
}