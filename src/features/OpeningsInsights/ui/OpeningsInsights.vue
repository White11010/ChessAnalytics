<template>
  <div class="d-flex flex-column ga-4 w-100 min-w-0">
    <InsightsFilters />
    <InsightsFeed
      :insights="displayInsights"
      :featured-id="featuredId"
      @view-games="goToMyGamesFromInsight"
    />
  </div>
</template>

<script setup lang="ts">
// Composes filters and the unified insights grid (featured card first when present).

import { storeToRefs } from 'pinia';
import { computed } from 'vue';
import { useRouter } from 'vue-router';

import { openingValuesForExactNames, useGamesStore, useMyGamesFiltersStore } from '@/entities/game';
import {
  buildMyGamesFiltersFromInsight,
  useInsightsFiltersStore,
  useInsightsStore,
} from '@/entities/insight';
import type { Insight } from '@/entities/insight';

import InsightsFeed from './InsightsFeed.vue';
import InsightsFilters from './InsightsFilters.vue';

const router = useRouter();
const insightsStore = useInsightsStore();
const gamesStore = useGamesStore();
const filtersStore = useMyGamesFiltersStore();
const insightsFiltersStore = useInsightsFiltersStore();
const { heroInsight } = storeToRefs(insightsStore);
const { selectedFilter, sortOrder } = storeToRefs(insightsFiltersStore);

function goToMyGamesFromInsight(insight: Insight): void {
  filtersStore.reset();
  const patch = buildMyGamesFiltersFromInsight(insight);
  if (patch.openingNamesExact.length) {
    patch.openingValues = openingValuesForExactNames(gamesStore.games, patch.openingNamesExact);
  }
  filtersStore.$patch(patch);
  filtersStore.persist();
  void router.push({ name: 'MyGames' });
}

const baseList = computed(() => {
  const list = [...insightsStore.items];
  const mult = sortOrder.value === 'highFirst' ? -1 : 1;
  return list.sort((a, b) => mult * (a.sort_priority - b.sort_priority));
});

const categoryFiltered = computed(() => {
  const filter = selectedFilter.value;
  if (filter === 'all') {
    return baseList.value;
  }
  if (filter === 'attention') {
    return baseList.value.filter((i) => i.severity === 'warning' || i.severity === 'critical');
  }
  return baseList.value.filter((i) => i.category === filter);
});

const heroVisible = computed(() => {
  if (!heroInsight.value) {
    return false;
  }
  return categoryFiltered.value.some((i) => i.id === heroInsight.value!.id);
});

const featuredId = computed(() =>
  heroVisible.value && heroInsight.value ? heroInsight.value.id : null,
);

const displayInsights = computed(() => {
  const heroId = featuredId.value;
  const rest = heroId
    ? categoryFiltered.value.filter((i) => i.id !== heroId)
    : categoryFiltered.value;
  if (heroId && heroInsight.value) {
    return [heroInsight.value, ...rest];
  }
  return rest;
});
</script>
