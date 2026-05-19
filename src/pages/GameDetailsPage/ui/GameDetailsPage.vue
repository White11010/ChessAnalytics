<template>
  <div
    class="game-details-page d-flex flex-column flex-1 h-100 min-h-0 overflow-y-auto overflow-x-hidden pa-5"
  >
    <div class="game-details-page__container">
      <v-container fluid class="pa-0">
        <div class="d-flex align-center justify-space-between flex-wrap ga-3 mb-8">
          <h1 class="text-h4 font-weight-bold mb-0">{{ t('gameDetails.pageTitle') }}</h1>
          <div class="d-flex ga-2">
            <RunGameAnalysisButton
              v-if="!showAnalysisProgressUi"
              :game-id="gameId"
              @done="onAnalysisDone"
              @failed="onRunFailed"
            />
            <v-btn
              variant="flat"
              color="secondary"
              prepend-icon="mdi-refresh"
              :loading="isFetching"
              @click="refetch"
            >
              {{ t('gameDetails.refresh') }}
            </v-btn>
          </div>
        </div>

        <v-row v-if="showAnalysisProgressUi">
          <v-col cols="12">
            <v-card variant="outlined" rounded="lg">
              <v-card-text class="py-10 d-flex flex-column align-center text-center ga-4">
                <v-progress-circular indeterminate color="secondary" size="48" width="4" />
                <div>
                  <p class="text-h6 mb-1">{{ analysisProgressTitle }}</p>
                  <p class="text-body-2 text-medium-emphasis mb-0">
                    {{ analysisProgressSubtitle }}
                  </p>
                </div>
              </v-card-text>
            </v-card>
          </v-col>
        </v-row>

        <v-row v-else-if="uiState !== 'ready'">
          <v-col cols="12">
            <GameAnalysisState :state="uiState" :message="stateMessage">
              <template #actions>
                <RunGameAnalysisButton
                  v-if="!showAnalysisProgressUi"
                  :game-id="gameId"
                  :label="t('gameDetails.runAnalysisNow')"
                  @done="onAnalysisDone"
                  @failed="onRunFailed"
                />
                <v-btn variant="text" color="secondary" @click="refetch">{{
                  t('gameDetails.tryRefresh')
                }}</v-btn>
              </template>
            </GameAnalysisState>
          </v-col>
        </v-row>

        <template v-else-if="analysis && game">
          <div class="game-details-grid">
            <GameDetailsHero :game="game" class="game-details-grid__sidebar" />

            <div class="game-details-main">
              <GameKeyInsightCard :analysis="analysis" />
              <GameMetricsGrid :analysis="analysis" :game="game" />

              <div class="game-details-analysis">
                <GameEvalHistory :analysis="analysis" class="game-details-analysis__chart" />
                <div class="game-details-side-stack">
                  <GamePatternsAndSystem :analysis="analysis" />
                  <GameKeyMoments :analysis="analysis" class="flex-grow-1" />
                </div>
              </div>
            </div>
          </div>

          <div class="mt-8">
            <GameSimilarGames :analysis="analysis" />
          </div>
        </template>
      </v-container>
    </div>
  </div>
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { computed, ref } from 'vue';
import { useI18n } from '@/shared/lib/i18n';
import { useRoute } from 'vue-router';

import { useAnalysisSettingsStore } from '@/entities/analysis-settings';
import {
  useGameAnalysisQuery,
  useGameAnalysisRunStore,
  type GameAnalysis,
} from '@/entities/game-analysis';
import { useSyncGamesQuery } from '@/entities/game';
import { GameAnalysisState } from '@/features/GameAnalysisState';
import { RunGameAnalysisButton } from '@/features/RunGameAnalysis';
import { GameDetailsHero } from '@/widgets/GameDetailsHero';
import { GameEvalHistory } from '@/widgets/GameEvalHistory';
import { GameKeyInsightCard } from '@/widgets/GameKeyInsightCard';
import { GameKeyMoments } from '@/widgets/GameKeyMoments';
import { GameMetricsGrid } from '@/widgets/GameMetricsGrid';
import { GamePatternsAndSystem } from '@/widgets/GamePatternsAndSystem';
import { GameSimilarGames } from '@/widgets/GameSimilarGames';

type UiState = 'loading' | 'empty' | 'pending' | 'failed' | 'error' | 'ready';

const { t } = useI18n();
const route = useRoute();
const externalErrorMessage = ref('');

const gameId = computed(() => String(route.params.id || ''));

const { games } = useSyncGamesQuery();
const game = computed(() => games.value.find((item) => item.id === gameId.value));

const { data, isLoading, isFetching, isError, refetch } = useGameAnalysisQuery(gameId);

const analysis = computed(() => data.value || null);

const { backgroundAnalysisEnabled } = storeToRefs(useAnalysisSettingsStore());
const { runningGameId } = storeToRefs(useGameAnalysisRunStore());

const lacksCompletedAnalysis = computed(() => {
  const a = analysis.value;
  if (a?.status === 'done') {
    return false;
  }
  if (a?.status === 'failed') {
    return false;
  }
  return true;
});

const showAnalysisProgressUi = computed(
  () =>
    Boolean(gameId.value) &&
    backgroundAnalysisEnabled.value &&
    !isLoading.value &&
    lacksCompletedAnalysis.value,
);

const isEngineWorkingOnThisGame = computed(
  () => Boolean(gameId.value) && runningGameId.value === gameId.value,
);

const analysisProgressTitle = computed(() =>
  isEngineWorkingOnThisGame.value
    ? t('gameDetails.progressInProgress')
    : t('gameDetails.progressQueued'),
);

const analysisProgressSubtitle = computed(() =>
  isEngineWorkingOnThisGame.value
    ? t('gameDetails.progressInProgressHint')
    : t('gameDetails.progressQueuedHint'),
);

const uiState = computed<UiState>(() => {
  if (isLoading.value && !analysis.value) {
    return 'loading';
  }
  if (isError.value) {
    return 'error';
  }
  if (!analysis.value) {
    return 'empty';
  }
  if (analysis.value.status === 'pending') {
    return 'pending';
  }
  if (analysis.value.status === 'failed') {
    return 'failed';
  }
  return 'ready';
});

const stateMessage = computed(() => {
  if (externalErrorMessage.value) {
    return t('errors.generic');
  }
  if (isError.value) {
    return t('errors.unknownLoad');
  }
  return analysis.value?.error ? t('errors.generic') : '';
});

function onAnalysisDone(_nextAnalysis: GameAnalysis) {
  externalErrorMessage.value = '';
  refetch();
}

function onRunFailed(message: string) {
  externalErrorMessage.value = message;
}
</script>

<style scoped lang="scss">
.game-details-page {
  width: 100%;
  flex: 1;
  height: 100%;
  overflow-y: auto;
  padding: 1.25rem;

  &__container {
    // max-width: 87.5rem;
    // margin: 0 auto;
    // padding: 2rem;
  }
}

.game-details-grid {
  display: grid;
  grid-template-columns: min(380px, 100%) 1fr;
  gap: 32px;
  align-items: stretch;
}

.game-details-grid__sidebar {
  position: sticky;
  top: 24px;
  align-self: stretch;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.game-details-main {
  display: flex;
  flex-direction: column;
  gap: 32px;
  min-height: 0;
  min-width: 0;
}

.game-details-analysis {
  display: grid;
  grid-template-columns: 1fr min(500px, 100%);
  gap: 24px;
  flex: 1;
  min-height: 400px;
  align-items: stretch;
}

.game-details-side-stack {
  display: flex;
  flex-direction: column;
  gap: 20px;
  min-height: 0;
}

@media (max-width: 1280px) {
  .game-details-grid {
    grid-template-columns: 1fr;
  }

  .game-details-grid__sidebar {
    position: static;
  }

  .game-details-analysis {
    grid-template-columns: 1fr;
  }
}
</style>
