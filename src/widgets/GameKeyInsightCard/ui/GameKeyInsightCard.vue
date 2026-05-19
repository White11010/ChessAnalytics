<template>
  <v-card class="game-key-insight-card border-s-lg" border="secondary opacity-75">
    <v-card-text class="pa-6">
      <v-row align="center">
        <v-col cols="12" md>
          <div class="d-flex align-center ga-2 mb-2">
            <v-icon :icon="insightIcon" size="small" />
            <span class="text-h6 font-weight-semibold text-secondary">{{ keyInsightTitle }}</span>
          </div>
          <p class="text-body-2 text-medium-emphasis mb-0">{{ keyInsightDescription }}</p>
        </v-col>
        <v-col cols="12" md="auto" class="d-flex justify-center justify-md-end">
          <v-sheet rounded="lg" border class="insight-metric pa-4 text-center" color="surface">
            <div class="text-h4 font-weight-bold">{{ maxAdvantageDisplay }}</div>
            <div class="text-caption text-uppercase font-weight-bold text-medium-emphasis mt-1">
              {{ t('analysis.overviewStats.maxAdvantage') }}
            </div>
          </v-sheet>
        </v-col>
      </v-row>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import { computed } from 'vue';

import { getKeyInsightDescription, getKeyInsightTitle } from '@/entities/game-analysis';
import type { GameAnalysis } from '@/entities/game-analysis';
import { useI18n } from '@/shared/lib/i18n';

const props = defineProps<{
  analysis: GameAnalysis;
}>();

const { t, te } = useI18n();

const keyInsightTitle = computed(() => getKeyInsightTitle(props.analysis, t, te));
const keyInsightDescription = computed(() => getKeyInsightDescription(props.analysis, t, te));

const insightColor = computed(() => {
  switch (props.analysis.key_insight.severity) {
    case 'high':
      return 'error';
    case 'warning':
      return 'warning';
    case 'good':
      return 'success';
    default:
      return 'warning';
  }
});

const insightIcon = computed(() => {
  switch (props.analysis.key_insight.severity) {
    case 'high':
      return 'mdi-alert-circle-outline';
    case 'good':
      return 'mdi-lightbulb-on-outline';
    default:
      return 'mdi-alert-outline';
  }
});

const maxAdvantageDisplay = computed(() => formatPawns(props.analysis.max_advantage_cp));

function formatPawns(cp: number): string {
  const pawns = cp / 100;
  const sign = pawns > 0 ? '+' : '';
  return `${sign}${pawns.toFixed(1)}`;
}
</script>

<style scoped lang="scss">
.game-key-insight-card {
  // background: linear-gradient(
  //   135deg,
  //   rgba(var(--v-theme-warning), 0.12) 0%,
  //   rgba(var(--v-theme-surface), 0.4) 100%
  // );
}

.insight-metric {
  min-width: 120px;
}
</style>
