<template>
  <div
    class="d-flex flex-column flex-md-row align-stretch align-md-center ga-4 justify-space-between"
  >
    <div class="d-flex flex-column flex-sm-row align-sm-center ga-3 min-w-0">
      <span class="text-caption text-medium-emphasis text-no-wrap flex-shrink-0">{{
        t('insightsPage.categoriesLabel')
      }}</span>
      <v-chip-group
        v-model="selectedFilter"
        class="flex-grow-1"
        mandatory
        selected-class="text-secondary"
      >
        <v-chip value="all" color="secondary" variant="tonal">{{
          t('insightsPage.filterAll')
        }}</v-chip>

        <v-chip value="openings" color="secondary" variant="tonal">{{
          t('insightsPage.filterOpenings')
        }}</v-chip>
        <v-chip value="time" color="secondary" variant="tonal">{{
          t('insightsPage.filterTime')
        }}</v-chip>
        <v-chip value="tactics" color="secondary" variant="tonal">{{
          t('insightsPage.filterTactics')
        }}</v-chip>
        <v-chip value="psychology" color="secondary" variant="tonal">{{
          t('insightsPage.filterPsychology')
        }}</v-chip>
        <v-chip value="attention" color="warning" variant="tonal">
          <v-icon icon="mdi-alert-circle-outline" start size="16" />
          {{ t('insightsPage.filterAttention') }}
        </v-chip>
      </v-chip-group>
    </div>
    <v-select
      v-model="sortOrder"
      density="compact"
      hide-details
      :items="sortItems"
      item-title="title"
      item-value="value"
      :label="t('insightsPage.sortByPriority')"
      variant="outlined"
      style="max-width: 280px"
    />
  </div>
</template>

<script setup lang="ts">
// Category chips and sort order for the insights list; state lives in insightsFilters store.

import { storeToRefs } from 'pinia';
import { computed, watch } from 'vue';

import { useInsightsFiltersStore } from '@/entities/insight';
import { useI18n } from '@/shared/lib/i18n';

const { t } = useI18n();
const insightsFiltersStore = useInsightsFiltersStore();
const { selectedFilter, sortOrder } = storeToRefs(insightsFiltersStore);

watch(
  () => insightsFiltersStore.$state,
  () => {
    insightsFiltersStore.persist();
  },
  { deep: true },
);

const sortItems = computed(() => [
  { value: 'highFirst' as const, title: t('insightsPage.sortPriorityHighFirst') },
  { value: 'lowFirst' as const, title: t('insightsPage.sortPriorityLowFirst') },
]);
</script>