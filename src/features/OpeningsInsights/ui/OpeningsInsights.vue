<template>
  <div class="d-flex flex-column ga-4 w-100 min-w-0">
    <InsightsFilters />
    <InsightsHero
      v-if="heroInsight && heroVisible"
      :insight="heroInsight"
      @view-games="goToMyGamesFromInsight"
    />
    <InsightsFeed :insights="feedInsights" @view-games="goToMyGamesFromInsight" />
  </div>
</template>

<script setup lang="ts">
// Composes filters, featured hero, and the insights feed for the openings insights feature.

import { storeToRefs } from 'pinia';
import { computed } from 'vue';
import { useRouter } from 'vue-router';

import { useMyGamesFiltersStore } from '@/entities/game';
import {
  buildMyGamesFiltersFromInsight,
  useInsightsFiltersStore,
  useInsightsStore,
} from '@/entities/insight';
import type { Insight } from '@/entities/insight';

import InsightsFeed from './InsightsFeed.vue';
import InsightsFilters from './InsightsFilters.vue';
import InsightsHero from './InsightsHero.vue';

const router = useRouter();
const insightsStore = useInsightsStore();
const filtersStore = useMyGamesFiltersStore();
const insightsFiltersStore = useInsightsFiltersStore();
const { heroInsight } = storeToRefs(insightsStore);
const { selectedFilter, sortOrder } = storeToRefs(insightsFiltersStore);

function goToMyGamesFromInsight(insight: Insight): void {
  filtersStore.reset();
  const patch = buildMyGamesFiltersFromInsight(insight);
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

const feedInsights = computed(() => {
  const heroId = heroVisible.value && heroInsight.value ? heroInsight.value.id : null;
  return categoryFiltered.value.filter((i) => i.id !== heroId);
});
</script>
