<template>
  <v-card class="h-100 d-flex flex-column game-key-moments">
    <v-card-item class="pb-0">
      <v-card-title class="text-subtitle-1 font-weight-semibold pa-0">
        {{ t('analysis.keyMomentsTitle') }}
      </v-card-title>
    </v-card-item>
    <v-card-text class="flex-grow-1 overflow-y-auto">
      <v-alert v-if="!analysis.key_moments.length" type="info" variant="tonal">
        {{ t('analysis.keyMomentsEmpty') }}
      </v-alert>
      <div v-else class="d-flex flex-column ga-3">
        <v-sheet
          v-for="moment in analysis.key_moments"
          :key="`${moment.ply}-${moment.kind}`"
          rounded="lg"
          class="pa-3"
          border="secondary opacity-75"
        >
          <div class="text-body-2 font-weight-semibold">{{ headline(moment) }}</div>
          <div v-if="description(moment)" class="text-body-2 text-medium-emphasis mt-1">
            {{ description(moment) }}
          </div>
          <div v-if="evalLine(moment)" class="text-caption text-medium-emphasis mt-1">
            {{ evalLine(moment) }}
          </div>
          <div class="text-body-2 mt-2">{{ movesLine(moment) }}</div>
        </v-sheet>
      </div>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import {
  getKeyMomentDescription,
  getKeyMomentEvalLine,
  getKeyMomentHeadline,
  getKeyMomentMovesLine,
} from '@/entities/game-analysis';
import type { GameAnalysis, KeyMoment } from '@/entities/game-analysis';
import { useI18n } from '@/shared/lib/i18n';

defineProps<{
  analysis: GameAnalysis;
}>();

const { t, te } = useI18n();

function headline(moment: KeyMoment): string {
  return getKeyMomentHeadline(moment, t, te);
}

function description(moment: KeyMoment): string {
  return getKeyMomentDescription(moment, t, te);
}

function movesLine(moment: KeyMoment): string {
  return getKeyMomentMovesLine(moment, t, te);
}

function evalLine(moment: KeyMoment): string | null {
  return getKeyMomentEvalLine(moment, t, te);
}
</script>
