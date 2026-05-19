<template>
  <div class="d-flex flex-column ga-6">
    <section v-for="section in sections" :key="section.key">
      <h2 class="text-subtitle-1 font-weight-semibold mb-3 mt-0">{{ section.title }}</h2>
      <v-data-table
        :headers="headers"
        :items="section.rows"
        item-value="id"
        :items-per-page="10"
        class="similar-games-table"
        hover
        @click:row="onRowClick"
      >
        <template #item.opponent="{ item }">
          <template v-if="item.game">
            <span class="font-weight-medium">{{ item.game.opponent_name }}</span>
            <span
              v-if="item.game.opponent_rating != null"
              class="text-medium-emphasis text-caption ms-1"
            >
              ({{ item.game.opponent_rating }})
            </span>
          </template>
          <span v-else class="text-medium-emphasis">{{ item.id }}</span>
        </template>

        <template #item.opening="{ item }">
          <span
            v-if="item.game?.opening_name"
            class="text-truncate d-inline-block"
            style="max-width: 36rem"
          >
            {{ item.game.opening_name }}
          </span>
          <span v-else-if="!item.game" class="text-caption text-medium-emphasis">
            {{ t('gameDetails.similarGameNotSynced') }}
          </span>
          <span v-else class="text-medium-emphasis">{{ t('common.emDash') }}</span>
        </template>

        <template #item.speed="{ item }">
          <span v-if="item.game">{{ localizedSpeed(item.game.speed) }}</span>
          <span v-else>{{ t('common.emDash') }}</span>
        </template>

        <template #item.result="{ item }">
          <span v-if="item.game" :class="resultClass(item.game.player_result)">
            {{ resultLabel(item.game.player_result) }}
          </span>
          <span v-else>{{ t('common.emDash') }}</span>
        </template>

        <template #item.accuracy="{ item }">
          <span v-if="item.game?.analysis_accuracy != null">
            {{ Math.round(item.game.analysis_accuracy) }}%
          </span>
          <span v-else class="text-medium-emphasis">{{ t('common.emDash') }}</span>
        </template>

        <template #item.date="{ item }">
          <span v-if="item.game">{{ formatRowDate(item.game.created_at) }}</span>
          <span v-else>{{ t('common.emDash') }}</span>
        </template>

        <template #item.actions="{ item }">
          <v-btn
            icon="mdi-chevron-right"
            variant="text"
            size="small"
            color="secondary"
            :aria-label="t('myGames.table.ariaOpenDetails')"
            @click.stop="openGame(item.id)"
          />
        </template>

        <template #no-data>
          <div class="text-body-2 text-medium-emphasis py-6 text-center">
            {{ t('analysis.similarNoMatches') }}
          </div>
        </template>
      </v-data-table>
    </section>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useRouter } from 'vue-router';

import { useGamesStore } from '@/entities/game';
import type { GameAnalysis } from '@/entities/game-analysis';
import { formatTimestamp } from '@/shared/lib/dates';
import { useI18n } from '@/shared/lib/i18n';

import { getSimilarGamesTableHeaders } from '../lib/similarGamesTableColumns';
import { gamesByIdMap, resolveSimilarGameRows } from '../lib/resolveSimilarGameRows';

const props = defineProps<{
  analysis: GameAnalysis;
}>();

const { t, te, locale } = useI18n();
const router = useRouter();
const gamesStore = useGamesStore();

const headers = computed(() => getSimilarGamesTableHeaders(t));

const gamesMap = computed(() => gamesByIdMap(gamesStore.games));

const sections = computed(() => [
  {
    key: 'broad',
    title: t('analysis.similarBroad'),
    rows: resolveSimilarGameRows(props.analysis.similar_games.broad, gamesMap.value),
  },
  // {
  //   key: 'narrow',
  //   title: t('analysis.similarNarrow'),
  //   rows: resolveSimilarGameRows(props.analysis.similar_games.narrow, gamesMap.value),
  // },
]);

function localizedSpeed(speed: string): string {
  const key = `myGames.speed.${speed.toLowerCase()}`;
  return te(key) ? t(key) : speed;
}

function resultLabel(result: string): string {
  if (result === 'win') {
    return t('game.resultWin');
  }
  if (result === 'loss') {
    return t('game.resultLoss');
  }
  return t('game.resultDraw');
}

function resultClass(result: string): string {
  if (result === 'win') {
    return 'text-success';
  }
  if (result === 'loss') {
    return 'text-error';
  }
  return '';
}

function formatRowDate(createdAt: number): string {
  return formatTimestamp(createdAt, { locale: locale.value });
}

function openGame(gameId: string) {
  router.push(`/game-details/${gameId}`);
}

function onRowClick(_event: unknown, payload: { item: { id: string } }) {
  openGame(payload.item.id);
}
</script>

<style scoped lang="scss">
.similar-games-table :deep(tbody tr) {
  cursor: pointer;
}
</style>
