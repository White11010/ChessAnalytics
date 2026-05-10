<template>
  <template v-if="gamesStore.games.length">
    <div class="my-games-section">
      <aside class="my-games-section__sidebar">
        <my-games-toolbar :pattern-options="patternOptions" :opening-options="openingOptions" />
      </aside>
      <div class="my-games-section__main">
        <p class="text-body-2 text-medium-emphasis mb-2 mt-0 d-flex flex-wrap align-center ga-1">
          <span v-if="showResultCount">
            {{
              t('myGames.section.shown', {
                shown: displayedGames.length,
                total: gamesStore.games.length,
              })
            }}
          </span>
          <span v-else>
            {{ t('gamesSync.myGamesTotalGames', { total: gamesStore.games.length }) }}
          </span>
          <span>· {{ myGamesUpdatedLabel }} ·</span>
          <v-btn
            variant="text"
            size="small"
            class="text-none px-1"
            :loading="syncStore.isSyncing"
            :disabled="!syncStore.canManualSync()"
            @click="onRefreshGames"
          >
            {{ t('gamesSync.refresh') }}
          </v-btn>
        </p>
        <div class="my-games-section__table-wrap">
          <my-games-list
            v-if="displayedGames.length"
            v-model:sort-by="sortBy"
            :games="displayedGames"
          />
          <v-card v-else variant="tonal" class="pa-6">
            <p class="text-body-1 text-center text-medium-emphasis mb-0">
              {{ t('myGames.section.noMatching') }}
            </p>
          </v-card>
        </div>
      </div>
    </div>
  </template>
</template>

<script setup lang="ts">
import { useMyGamesFiltersStore } from '@/entities/game';
import { useGamesSyncStore } from '@/entities/games-sync';
import { formatLastGamesSyncLabel } from '@/entities/games-sync/lib/formatLastGamesSync';
import { MyGamesList } from '@/features/MyGamesList';
import { MyGamesToolbar } from '@/features/MyGamesToolbar';
import { useI18n } from '@/shared/lib/i18n';
import { storeToRefs } from 'pinia';
import { computed, onMounted, onUnmounted, ref } from 'vue';

import { useMyGamesSection } from '../lib/useMyGamesSection';

const { t } = useI18n();
const syncStore = useGamesSyncStore();
const { lastSyncedAt } = storeToRefs(syncStore);

const nowTick = ref(Date.now());
let tickId: ReturnType<typeof setInterval> | undefined;

onMounted(() => {
  tickId = setInterval(() => {
    nowTick.value = Date.now();
  }, 30_000);
});

onUnmounted(() => {
  if (tickId != null) {
    clearInterval(tickId);
  }
});

const myGamesUpdatedLabel = computed(() =>
  formatLastGamesSyncLabel(t, lastSyncedAt.value, nowTick.value),
);

function onRefreshGames(): void {
  void syncStore.manualSync();
}

const filtersStore = useMyGamesFiltersStore();
const { sortBy } = storeToRefs(filtersStore);

const { gamesStore, patternOptions, openingOptions, displayedGames, showResultCount } =
  useMyGamesSection();
</script>

<style scoped lang="scss">
.my-games-section {
  width: 100%;
  display: grid;
  gap: 1.5rem;
  align-items: start;
}

.my-games-section__table-wrap {
  max-width: 100%;
  overflow-x: auto;
}

@media (min-width: 1200px) {
  .my-games-section {
    grid-template-columns: 1fr minmax(280px, 22rem);
  }

  .my-games-section__main {
    grid-column: 1;
    grid-row: 1;
    min-width: 0;
  }

  .my-games-section__sidebar {
    margin-top: 36px;
    grid-column: 2;
    grid-row: 1;
  }
}

@media (max-width: 1199.98px) {
  .my-games-section {
    display: flex;
    flex-direction: column;
  }

  .my-games-section__main,
  .my-games-section__sidebar {
    align-self: stretch;
    max-width: 100%;
  }
}
</style>
