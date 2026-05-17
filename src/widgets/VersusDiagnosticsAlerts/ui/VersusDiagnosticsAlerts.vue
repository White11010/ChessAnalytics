<template>
  <div class="versus-diagnostics">
    <v-alert
      v-if="ratingGap != null && ratingGap > 200"
      density="compact"
      variant="tonal"
      type="warning"
      class="mb-3"
      :text="
        String(
          t('versusPage.ratingGapWarning', {
            gap: ratingGap,
            speed: speedLabel,
          }),
        )
      "
    />
    <v-alert
      v-if="slice.selfSide.sampleSizeMetrics < 5"
      density="compact"
      variant="tonal"
      type="info"
      class="mb-3"
      :text="String(t('versusPage.selfMetricsLow', { n: slice.selfSide.sampleSizeMetrics }))"
    />
    <v-alert
      v-if="opponentGamesInApiSample === 0"
      density="compact"
      variant="tonal"
      type="warning"
      class="mb-2"
      :text="String(t('versusPage.diagOppGamesEmptyApi'))"
    />
    <v-alert
      v-else-if="
        slice.diagnostic.opponentGamesMatchingSpeed === 0 &&
        ((slice.diagnostic.sampleLichessSpeedsWhenNoMatch?.length ?? 0) > 0)
      "
      density="compact"
      variant="tonal"
      type="warning"
      class="mb-2"
      :text="
        String(
          t('versusPage.diagOppWrongSpeed', {
            speed: speedLabel,
            speeds: (slice.diagnostic.sampleLichessSpeedsWhenNoMatch ?? []).join(', '),
          }),
        )
      "
    />
    <v-alert
      v-else-if="slice.diagnostic.opponentGamesMatchingSpeed === 0"
      density="compact"
      variant="tonal"
      type="warning"
      class="mb-2"
      :text="String(t('versusPage.diagOppWrongSpeedNoSpeeds'))"
    />
    <v-alert
      v-else-if="
        slice.diagnostic.opponentAnalysesAttempted > 0 &&
        slice.diagnostic.opponentAnalysesSucceeded === 0
      "
      density="compact"
      variant="tonal"
      type="error"
      class="mb-2"
      :text="
        String(
          t('versusPage.diagOppAnalysisFailed', {
            attempted: slice.diagnostic.opponentAnalysesAttempted,
            error: slice.diagnostic.firstAnalysisError ?? '—',
          }),
        )
      "
    />
    <v-alert
      v-if="
        slice.opponentSide.sampleSizeMetrics > 0 && slice.opponentSide.sampleSizeMetrics < 5
      "
      density="compact"
      variant="tonal"
      type="info"
      class="mb-4"
      :text="
        String(t('versusPage.oppLowSampleHint', { n: slice.opponentSide.sampleSizeMetrics }))
      "
    />
  </div>
</template>

<script setup lang="ts">
// Composite widget: presents a focused dashboard block; reads shared Pinia stores and Tauri invoke where needed.

import type { VersusSpeedSlice } from '@/entities/versus';
import { useI18n } from '@/shared/lib/i18n';
import { computed } from 'vue';

const props = defineProps<{
  slice: VersusSpeedSlice;
  opponentGamesInApiSample: number;
  speedLabel: string;
}>();

const { t } = useI18n();

const ratingGap = computed(() => {
  const a = props.slice.selfSide.rating;
  const b = props.slice.opponentSide.rating;
  if (a == null || b == null) return null;
  return Math.abs(a - b);
});
</script>
