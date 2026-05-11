<template>
  <div
    v-if="games.length"
    class="my-games-list-hover-root"
    @mouseleave="onGamesTableMouseLeave"
    @mouseover="onGamesTableMouseOver"
  >
    <v-data-table
      v-model:sort-by="sortBy"
      :headers="headers"
      :items="games"
      class="my-games-list"
      :cell-props="cellDataAttrs"
      :custom-key-sort="MY_GAMES_TABLE_CUSTOM_KEY_SORT"
      items-per-page="25"
      must-sort
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

      <template #item.speed="{ item }">
        <v-tooltip v-if="item.time_control" location="top">
          <template #activator="{ props: tipProps }">
            <v-chip v-bind="tipProps" size="small" variant="tonal" class="text-none">
              {{ localizedSpeedChipLabel(item.speed) }}
            </v-chip>
          </template>
          {{ item.time_control }}
        </v-tooltip>
        <v-chip v-else size="small" variant="tonal" class="text-none">
          {{ localizedSpeedChipLabel(item.speed) }}
        </v-chip>
      </template>

      <template #item.player_rating="{ item }">
        {{ item.player_rating ?? emDash }}
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

      <template #item.analysis_acpl="{ item }">
        <span v-if="item.analysis_acpl != null">{{ formatAcpl(item.analysis_acpl) }}</span>
        <v-progress-circular
          v-else-if="backgroundAnalysisEnabled && showAnalysisPendingLoader(item)"
          indeterminate
          size="20"
          width="2"
        />
        <span v-else class="text-medium-emphasis">{{ emDash }}</span>
      </template>

      <template #item.pattern_tags="{ item }">
        <v-tooltip v-if="extraPatternTags(item).length" location="top">
          <template #activator="{ props: tipProps }">
            <span v-bind="tipProps" class="d-inline-flex flex-wrap ga-1 align-center">
              <v-chip
                v-for="tag in visiblePatternTags(item)"
                :key="tag"
                size="small"
                density="compact"
                variant="tonal"
                class="text-none"
              >
                {{ formatPatternTag(tag) }}
              </v-chip>
              <span v-if="extraPatternTags(item).length" class="text-caption text-medium-emphasis"
                >+{{ extraPatternTags(item).length }}</span
              >
            </span>
          </template>
          <span>{{ extraPatternTags(item).map(formatPatternTag).join(', ') }}</span>
        </v-tooltip>
        <span v-else class="d-inline-flex flex-wrap ga-1 align-center">
          <template v-if="visiblePatternTags(item).length">
            <v-chip
              v-for="tag in visiblePatternTags(item)"
              :key="tag"
              size="small"
              density="compact"
              variant="tonal"
              class="text-none"
            >
              {{ formatPatternTag(tag) }}
            </v-chip>
          </template>
          <v-progress-circular
            v-else-if="backgroundAnalysisEnabled && showAnalysisPendingLoader(item)"
            indeterminate
            size="20"
            width="2"
          />
          <span v-else class="text-medium-emphasis">{{ emDash }}</span>
        </span>
      </template>

      <template #item.created_at="{ item }">
        {{ formatMyGamesTableDate(item.created_at, locale) }}
      </template>

      <template #item.actions="{ item }">
        <div class="d-flex align-center ga-0">
          <v-btn
            variant="plain"
            color="secondary"
            icon="mdi-chart-timeline-variant-shimmer"
            :aria-label="t('myGames.table.ariaOpenDetails')"
            @click="onDetailsButtonClick(item.id)"
          />
          <v-btn
            v-if="getExternalGameUrl(item)"
            variant="plain"
            color="primary"
            icon="mdi-open-in-new"
            :aria-label="t('myGames.table.ariaOpenPlatform')"
            @click="onExternalClick(item)"
          />
        </div>
      </template>
    </v-data-table>
  </div>

  <MyGamesListBoardPreview
    :preview="boardPreview"
    :preview-style="boardPreviewStyle"
    @hide="hideBoardPreview"
  />
</template>

<script setup lang="ts">
// My Games `v-data-table`: column cell slots, row-hover board preview, and entity-backed sort/header config.
import { useAnalysisSettingsStore } from '@/entities/analysis-settings';
import {
  type Game,
  type MyGamesTableSortItem,
  MY_GAMES_TABLE_CUSTOM_KEY_SORT,
  accuracyPercentRounded,
  accuracyToneFromRounded,
  getExternalGameUrl,
  openExternalGame,
  openingTooltipText,
  resultLetter,
  shortOpeningDisplay,
} from '@/entities/game';
import { formatMyGamesTableDate, formatPatternTagLabel } from '@/shared/lib';
import { useI18n } from '@/shared/lib/i18n';
import { storeToRefs } from 'pinia';
import { computed } from 'vue';
import { useRouter } from 'vuetify/lib/composables/router.mjs';

import { getMyGamesTableHeaders } from '../lib/myGamesTableColumns';
import { useMyGamesBoardRowPreview } from '../lib/useMyGamesBoardRowPreview';
import MyGamesListBoardPreview from './MyGamesListBoardPreview.vue';

const props = defineProps<{
  games: Game[];
}>();

const sortBy = defineModel<MyGamesTableSortItem[]>('sortBy', { required: true });

const { t, te, locale } = useI18n();
const emDash = computed(() => t('common.emDash'));

const router = useRouter();

const { backgroundAnalysisEnabled } = storeToRefs(useAnalysisSettingsStore());

const {
  boardPreview,
  boardPreviewStyle,
  onGamesTableMouseOver,
  onGamesTableMouseLeave,
  hideBoardPreview,
} = useMyGamesBoardRowPreview(() => props.games);

function cellDataAttrs(data: { item: Game }) {
  return {
    'data-game-id': data.item.id,
  };
}

const headers = computed(() => getMyGamesTableHeaders(t));

function showAnalysisPendingLoader(item: Game): boolean {
  return accuracyPercentRounded(item.analysis_accuracy) == null;
}

function formatAcpl(v: number): string {
  return Number.isInteger(v) ? String(v) : v.toFixed(1);
}

function localizedSpeedChipLabel(speed: string): string {
  const s = speed.toLowerCase();
  const normalized = s === 'ultrabullet' ? 'bullet' : s;
  if (
    normalized === 'bullet' ||
    normalized === 'blitz' ||
    normalized === 'rapid' ||
    normalized === 'classical'
  ) {
    return t(`myGames.speed.${normalized}`);
  }
  return speed ? speed.charAt(0).toUpperCase() + speed.slice(1).toLowerCase() : t('common.emDash');
}

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

function visiblePatternTags(item: Game): string[] {
  return (item.pattern_tags ?? []).slice(0, 2);
}

function extraPatternTags(item: Game): string[] {
  return (item.pattern_tags ?? []).slice(2);
}

function formatPatternTag(tag: string): string {
  return formatPatternTagLabel(tag, t, te);
}

function onDetailsButtonClick(id: string) {
  router?.push(`/game-details/${id}`);
}

function onExternalClick(item: Game) {
  void openExternalGame(item);
}
</script>

<style lang="scss" scoped>
.my-games-list {
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

.my-games-list-hover-root {
  width: 100%;
}
</style>
