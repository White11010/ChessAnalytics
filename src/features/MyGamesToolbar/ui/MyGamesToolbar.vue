<template>
  <v-card class="my-games-toolbar" variant="tonal">
    <v-card-text class="d-flex flex-column ga-4">
      <v-text-field
        v-model="searchText"
        density="comfortable"
        variant="outlined"
        hide-details
        clearable
        prepend-inner-icon="mdi-magnify"
        :label="t('myGames.toolbar.searchLabel')"
      />

      <div class="d-flex flex-wrap ga-2">
        <v-select
          v-model="results"
          :items="resultItems"
          item-title="label"
          item-value="value"
          :label="t('myGames.toolbar.result')"
          density="comfortable"
          variant="outlined"
          hide-details
          multiple
          chips
          closable-chips
          class="toolbar-field toolbar-field--grow"
        />
        <v-select
          v-model="speeds"
          :items="speedItems"
          item-title="label"
          item-value="value"
          :label="t('myGames.toolbar.timeControl')"
          density="comfortable"
          variant="outlined"
          hide-details
          multiple
          chips
          closable-chips
          class="toolbar-field toolbar-field--grow"
        />
        <v-select
          v-model="periods"
          :items="periodItems"
          item-title="label"
          item-value="value"
          :label="t('myGames.toolbar.period')"
          density="comfortable"
          variant="outlined"
          hide-details
          multiple
          chips
          closable-chips
          class="toolbar-field toolbar-field--grow"
        />
        <v-select
          v-model="playerColors"
          :items="colorItems"
          item-title="label"
          item-value="value"
          :label="t('myGames.toolbar.pieceColor')"
          density="comfortable"
          variant="outlined"
          hide-details
          multiple
          chips
          closable-chips
          class="toolbar-field toolbar-field--grow"
        />
      </div>

      <div class="d-flex flex-wrap ga-2">
        <v-autocomplete
          v-model="patternTag"
          :items="patternSelectItems"
          item-title="title"
          item-value="value"
          :label="t('myGames.toolbar.pattern')"
          density="comfortable"
          variant="outlined"
          hide-details
          clearable
          class="toolbar-field toolbar-field--grow"
        />
        <v-autocomplete
          v-model="openingValues"
          :items="openingOptions"
          item-title="title"
          item-value="value"
          :label="t('myGames.toolbar.opening')"
          density="comfortable"
          variant="outlined"
          hide-details
          multiple
          chips
          closable-chips
          clearable
          class="toolbar-field toolbar-field--grow"
          @update:model-value="onOpeningValuesChange"
        />
      </div>

      <div class="d-flex justify-end">
        <v-btn variant="text" size="small" @click="onReset">{{
          t('myGames.toolbar.resetFilters')
        }}</v-btn>
      </div>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
// Feature slice: encapsulates one user flow or form; parent pages/widgets compose it and pass props/events.

import { storeToRefs } from 'pinia';
import { computed } from 'vue';

import type { OpeningOption, PatternOption } from '@/entities/game';
import { useMyGamesFiltersStore } from '@/entities/game';
import { formatPatternTagLabel } from '@/shared/lib';
import { useI18n } from '@/shared/lib/i18n';

const props = defineProps<{
  patternOptions: PatternOption[];
  openingOptions: OpeningOption[];
}>();

const { t, te } = useI18n();

const patternSelectItems = computed(() =>
  props.patternOptions.map((o) => ({
    value: o.value,
    title: formatPatternTagLabel(o.value, t, te),
  })),
);

const filtersStore = useMyGamesFiltersStore();
const { searchText, results, speeds, periods, patternTag, openingValues, playerColors } =
  storeToRefs(filtersStore);

function onOpeningValuesChange(values: string[] | null) {
  if (!values?.length) {
    filtersStore.setOpeningFilters([], []);
  } else {
    filtersStore.setOpeningFilters(values, []);
  }
}

const resultItems = computed(() => [
  { value: 'win' as const, label: t('myGames.toolbar.results.win') },
  { value: 'loss' as const, label: t('myGames.toolbar.results.loss') },
  { value: 'draw' as const, label: t('myGames.toolbar.results.draw') },
]);

const speedItems = computed(() => [
  { value: 'bullet', label: t('myGames.toolbar.speeds.bullet') },
  { value: 'blitz', label: t('myGames.toolbar.speeds.blitz') },
  { value: 'rapid', label: t('myGames.toolbar.speeds.rapid') },
  { value: 'classical', label: t('myGames.toolbar.speeds.classical') },
]);

const periodItems = computed(() => [
  { value: '7' as const, label: t('myGames.toolbar.periods.d7') },
  { value: '30' as const, label: t('myGames.toolbar.periods.d30') },
  { value: '90' as const, label: t('myGames.toolbar.periods.d90') },
  { value: 'all' as const, label: t('myGames.toolbar.periods.all') },
]);

const colorItems = computed(() => [
  { value: 'white' as const, label: t('myGames.toolbar.colors.white') },
  { value: 'black' as const, label: t('myGames.toolbar.colors.black') },
]);

function onReset() {
  filtersStore.reset();
}
</script>

<style scoped lang="scss">
.my-games-toolbar {
  .toolbar-field {
    min-width: 11rem;
    flex: 1 1 10rem;
  }

  .toolbar-field--md {
    flex: 1 1 12rem;
    max-width: 16rem;
  }

  .toolbar-field--grow {
    flex: 1 1 14rem;
    min-width: 12rem;
  }
}
</style>
