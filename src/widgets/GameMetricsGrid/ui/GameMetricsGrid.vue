<template>
  <v-row class="flex-grow-0">
    <v-col v-for="item in stats" :key="item.key" cols="12" sm="6" md="3">
      <v-card class="h-100">
        <v-card-text class="pa-4">
          <div class="text-caption text-uppercase font-weight-bold text-medium-emphasis mb-2">
            {{ item.label }}
          </div>
          <div class="text-h5 font-weight-bold mb-2">{{ item.value }}</div>
          <div v-if="item.benchmarks?.length" class="d-flex flex-column ga-1">
            <div
              v-for="(line, idx) in item.benchmarks"
              :key="idx"
              class="text-caption"
              :class="line.toneClass"
            >
              {{ line.text }}
            </div>
          </div>
          <div v-else-if="item.hint" class="text-caption text-medium-emphasis">
            {{ item.hint }}
          </div>
        </v-card-text>
      </v-card>
    </v-col>
  </v-row>
</template>

<script setup lang="ts">
import { computed } from 'vue';

import type { Game } from '@/entities/game';
import type { GameAnalysis } from '@/entities/game-analysis';
import { useI18n } from '@/shared/lib/i18n';

import { useGameMetricsBenchmarks } from '../lib/useGameMetricsBenchmarks';

const props = defineProps<{
  analysis: GameAnalysis;
  game?: Game;
}>();

const { t } = useI18n();

const analysisRef = computed(() => props.analysis);
const gameRef = computed(() => props.game);

const { accuracyBenchmarks, acplBenchmarks } = useGameMetricsBenchmarks(analysisRef, gameRef);

const stats = computed(() => [
  {
    key: 'accuracy',
    label: t('analysis.overviewStats.accuracy'),
    value: `${Math.round(props.analysis.accuracy)}%`,
    benchmarks: accuracyBenchmarks.value,
    hint: undefined as string | undefined,
  },
  {
    key: 'acpl',
    label: t('analysis.overviewStats.acpl'),
    value: Math.round(props.analysis.avg_centipawn_loss),
    benchmarks: acplBenchmarks.value,
    hint: undefined,
  },
  {
    key: 'blunders',
    label: t('analysis.overviewStats.blunders'),
    value: props.analysis.blunders,
    benchmarks: undefined,
    hint: t('analysis.metricHint.blunders'),
  },
  {
    key: 'mistakes',
    label: t('analysis.overviewStats.mistakes'),
    value: props.analysis.mistakes,
    benchmarks: undefined,
    hint: t('analysis.metricHint.mistakes'),
  },
]);
</script>
