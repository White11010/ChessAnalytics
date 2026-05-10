<template>
  <v-card class="h-100" :title="t('home.recentGamesTitle')">
    <v-card-text>
      <v-data-table
        v-if="games.length"
        class="home-recent-games-table"
        :headers="headers"
        :items="getRecentGames"
        hide-default-footer
      >
        <template #item.result_marker="{ item }">
          <div
            class="result-marker text-body-2 font-weight-medium"
            :class="'result-marker--' + item.player_result"
          >
            {{ resultLetter(item.player_result) }}
          </div>
        </template>

        <template #item.opponent_name="{ item }">
          <span class="d-inline-flex align-center ga-1 flex-wrap">
            <span class="color-piece text-medium-emphasis" aria-hidden="true">{{
              colorPiece(item.player_color)
            }}</span>
            <span>{{ item.opponent_name || emDash }}</span>
            <span v-if="item.opponent_rating != null" class="text-medium-emphasis text-body-2">
              ({{ item.opponent_rating }})
            </span>
          </span>
        </template>

        <template #item.opening_name="{ item }">
          <v-tooltip
            v-if="openingTooltipText(item.opening_name, item.opening_eco)"
            location="top"
            max-width="320"
          >
            <template #activator="{ props: tipProps }">
              <span
                v-bind="tipProps"
                class="opening-short text-truncate d-inline-block"
                style="max-width: 14rem"
              >
                {{ shortOpeningDisplay(item.opening_name) }}
              </span>
            </template>
            <span style="white-space: pre-wrap">{{
              openingTooltipText(item.opening_name, item.opening_eco)
            }}</span>
          </v-tooltip>
          <span v-else class="text-medium-emphasis">{{ emDash }}</span>
        </template>

        <template #item.analysis_accuracy="{ item }">
          <span
            v-if="accuracyPercentRounded(item.analysis_accuracy) != null"
            :class="accuracyClass(item.analysis_accuracy)"
          >
            {{ accuracyPercentRounded(item.analysis_accuracy) }}%
          </span>
          <v-progress-circular
            v-else-if="backgroundAnalysisEnabled && showAnalysisPendingLoader(item)"
            indeterminate
            size="20"
            width="2"
          />
          <span v-else class="text-medium-emphasis">{{ emDash }}</span>
        </template>

        <template #item.created_at="{ item }">
          {{ formatMyGamesTableDate(item.created_at, locale) }}
        </template>
      </v-data-table>

      <v-skeleton-loader v-else type="table-thead, table-tbody" />
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import { useAnalysisSettingsStore } from '@/entities/analysis-settings';
import {
  type Game,
  accuracyPercentRounded,
  accuracyToneFromRounded,
  openingTooltipText,
  resultLetter,
  shortOpeningDisplay,
  useSyncGamesQuery,
} from '@/entities/game';
import { formatMyGamesTableDate } from '@/shared/lib';
import { useI18n } from '@/shared/lib/i18n';
import { storeToRefs } from 'pinia';
import { computed } from 'vue';

const { t, locale } = useI18n();
const emDash = computed(() => t('common.emDash'));

const { games } = useSyncGamesQuery();

const { backgroundAnalysisEnabled } = storeToRefs(useAnalysisSettingsStore());

const headers = computed(() => [
  {
    key: 'result_marker',
    title: t('myGames.toolbar.result'),
    width: '40px',
    sortable: false,
  },
  {
    key: 'opponent_name',
    title: t('myGames.table.opponent'),
    sortable: false,
  },
  {
    key: 'opening_name',
    title: t('myGames.table.opening'),
    sortable: false,
  },
  {
    key: 'analysis_accuracy',
    title: t('myGames.table.accuracy'),
    width: '88px',
    sortable: false,
  },
  {
    key: 'created_at',
    title: t('myGames.table.date'),
    width: '144px',
    sortable: false,
  },
]);

const getRecentGames = computed(() => {
  return games.value.slice(0, 10);
});

function colorPiece(playerColor: Game['player_color']): string {
  return playerColor === 'white' ? '♔' : '♚';
}

function accuracyClass(acc: number | null | undefined): string {
  const r = accuracyPercentRounded(acc);
  const tone = accuracyToneFromRounded(r);
  if (tone === 'high') {
    return 'text-success';
  }
  if (tone === 'mid') {
    return 'text-warning';
  }
  if (tone === 'low') {
    return 'text-error';
  }
  return '';
}

function showAnalysisPendingLoader(item: Game): boolean {
  return accuracyPercentRounded(item.analysis_accuracy) == null;
}
</script>

<style lang="scss" scoped>
.home-recent-games-table {
  :deep(td) {
    height: 64px;
  }

  :deep(td:first-child) {
    padding-inline: 0;
  }
}

.result-marker {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 64px;
  margin-block: -8px;
  margin-inline-start: 0;
  margin-inline-end: 0;
  padding-inline: 8px;
  box-shadow: inset 3px 0 0 0 rgb(var(--v-theme-on-surface));
  opacity: 0.92;
}

.result-marker--win {
  box-shadow: inset 3px 0 0 0 rgb(var(--v-theme-success));
}

.result-marker--loss {
  box-shadow: inset 3px 0 0 0 rgb(var(--v-theme-error));
}

.result-marker--draw {
  box-shadow: inset 3px 0 0 0 rgb(var(--v-theme-outline));
}

.color-piece {
  font-size: 1rem;
  line-height: 1;
}
</style>
