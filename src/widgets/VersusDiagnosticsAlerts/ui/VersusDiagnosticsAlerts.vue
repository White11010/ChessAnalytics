<template>
  <div class="versus-diagnostics">
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
import type { VersusSpeedSlice } from '@/entities/versus';
import { useI18n } from '@/shared/lib/i18n';

defineProps<{
  slice: VersusSpeedSlice;
  opponentGamesInApiSample: number;
  speedLabel: string;
}>();

const { t } = useI18n();
</script>
