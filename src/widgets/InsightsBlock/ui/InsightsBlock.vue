<template>
  <div class="d-flex flex-column ga-6 w-100 min-w-0">
    <header>
      <h1 class="text-h5 font-weight-medium mb-1">{{ t('insightsPage.title') }}</h1>
      <p class="text-body-2 text-medium-emphasis mb-0">{{ pageSubtitle }}</p>
    </header>

    <template v-if="showKpiSkeleton">
      <v-row dense>
        <v-col v-for="i in 4" :key="i" cols="12" sm="6" md="3">
          <v-skeleton-loader type="card" class="rounded" />
        </v-col>
      </v-row>
    </template>

    <v-row v-else dense>
      <v-col cols="12" sm="6" md="3">
        <v-card variant="tonal" class="h-100 pa-4 d-flex flex-column">
          <div class="text-overline text-medium-emphasis">{{ t('insightsPage.kpiTotalLabel') }}</div>
          <div class="text-h4 font-weight-bold mt-1">{{ insightsStore.items.length }}</div>
          <div class="text-caption text-medium-emphasis mt-2">
            {{ t('insightsPage.kpiTotalHint', { n: insightsStore.warningInsights.length }) }}
          </div>
        </v-card>
      </v-col>
      <v-col cols="12" sm="6" md="3">
        <v-card variant="tonal" class="h-100 pa-4 d-flex flex-column">
          <div class="text-overline text-medium-emphasis">{{ t('insightsPage.kpiAccuracyLabel') }}</div>
          <div class="text-h4 font-weight-bold mt-1">
            {{ avgAccuracyDisplay }}
          </div>
          <div
            v-if="accuracyDeltaFromBench != null"
            class="text-caption mt-2"
            :class="accuracyDeltaClass"
          >
            {{ accuracyDeltaArrow }}
            {{ t('insightsPage.kpiAccuracyDeltaFromAvg', { n: accuracyDeltaAbs }) }}
          </div>
          <div v-else class="text-caption text-medium-emphasis mt-2">
            {{ avgAccuracySubtext }}
          </div>
        </v-card>
      </v-col>
      <v-col cols="12" sm="6" md="3">
        <v-card variant="tonal" class="h-100 pa-4 d-flex flex-column">
          <div class="text-overline text-medium-emphasis">{{ t('insightsPage.kpiRatingLabel') }}</div>
          <div
            class="text-h4 font-weight-bold mt-1"
            :class="ratingGrowthInsight ? 'text-success' : undefined"
          >
            {{ ratingGrowthDisplay }}
          </div>
          <div class="text-caption text-medium-emphasis mt-2">
            {{ ratingGrowthSubtext }}
          </div>
        </v-card>
      </v-col>
      <v-col cols="12" sm="6" md="3">
        <v-card variant="tonal" class="h-100 pa-4 d-flex flex-column">
          <div class="text-overline text-medium-emphasis">{{ t('insightsPage.kpiBestLabel') }}</div>
          <div class="text-h6 font-weight-bold mt-1">{{ bestAreaLabel }}</div>
          <div class="text-caption text-medium-emphasis mt-2">{{ bestAreaHint }}</div>
        </v-card>
      </v-col>
    </v-row>

    <v-card v-if="insightsStore.items.length === 0" variant="tonal" class="pa-6">
      <p class="text-body-1 text-medium-emphasis mb-4">{{ t('insightsPage.emptyHint') }}</p>
      <generate-insights />
    </v-card>
    <openings-insights v-else />
  </div>
</template>

<script setup lang="ts">
// Composite widget: presents a focused dashboard block; reads shared Pinia stores and Tauri invoke where needed.

import { computed, onMounted, onUnmounted, ref } from 'vue';

import { useGamesStore } from '@/entities/game';
import { useInsightsStore } from '@/entities/insight';
import { formatLastGamesSyncLabel } from '@/entities/games-sync/lib/formatLastGamesSync';
import { GenerateInsights } from '@/features/GenerateInsights';
import { OpeningsInsights } from '@/features/OpeningsInsights';
import { useI18n } from '@/shared/lib/i18n';

import { useInsightsBlockKpis } from '../lib/useInsightsBlockKpis';

const { t } = useI18n();

const insightsStore = useInsightsStore();
const gamesStore = useGamesStore();

const {
  showKpiSkeleton,
  avgAccuracyDisplay,
  accuracyDeltaFromBench,
  accuracyDeltaAbs,
  accuracyDeltaArrow,
  accuracyDeltaClass,
  avgAccuracySubtext,
  bestAreaLabel,
  bestAreaHint,
  ratingGrowthInsight,
  ratingGrowthDisplay,
  ratingGrowthSubtext,
} = useInsightsBlockKpis();

const nowTick = ref(Date.now());
let tickId: ReturnType<typeof setInterval> | undefined;

onMounted(() => {
  void insightsStore.load();
  tickId = setInterval(() => {
    nowTick.value = Date.now();
  }, 30_000);
});

onUnmounted(() => {
  if (tickId != null) {
    clearInterval(tickId);
  }
});

const gamesCount = computed(() => gamesStore.games.length);

const pageSubtitle = computed(() => {
  const gamesPart = t('insightsPage.basedOnGames', { n: gamesCount.value });
  if (!insightsStore.lastLoadedAt) {
    return `${gamesPart} · ${t('insightsPage.insightsNotLoadedYet')}`;
  }
  const relative = formatLastGamesSyncLabel(t, insightsStore.lastLoadedAt, nowTick.value);
  return `${gamesPart} · ${t('insightsPage.updatedRelative', { relative })}`;
});
</script>
