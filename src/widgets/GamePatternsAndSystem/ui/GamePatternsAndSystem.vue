<template>
  <v-card
    variant="tonal"
    color="warning"
    rounded="lg"
    class="game-pattern-card border-s-lg"
    border="secondary opacity-75"
  >
    <v-card-text class="pa-5">
      <div v-if="cardTitle" class="text-subtitle-1 font-weight-semibold text-secondary mb-3">
        {{ cardTitle }}
      </div>

      <template v-if="hasSystemContent">
        <div v-if="titleText" class="text-body-1 font-weight-bold mb-2">{{ titleText }}</div>
        <p class="text-body-2 text-medium-emphasis mb-0">{{ primaryText }}</p>
        <p v-if="ctaText" class="text-body-2 font-weight-medium mt-3 mb-0">{{ ctaText }}</p>
        <p v-if="secondaryText" class="text-caption text-medium-emphasis mt-2 mb-0">
          {{ secondaryText }}
        </p>
      </template>
      <v-alert v-else type="info" variant="tonal" density="compact" class="mt-0">
        {{ t('gameDetails.noSystemConnection') }}
      </v-alert>

      <div v-if="analysis.pattern_tags.length" class="d-flex flex-wrap ga-2 mt-4">
        <v-chip
          v-for="tag in analysis.pattern_tags"
          :key="tag"
          size="small"
          color="secondary"
          variant="outlined"
        >
          {{ formatPatternTagLabel(tag, t, te) }}
        </v-chip>
      </div>
      <p v-else class="text-caption text-medium-emphasis mt-4 mb-0">
        {{ t('gameDetails.noPatternTags') }}
      </p>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import { computed } from 'vue';

import {
  getSystemConnectionCta,
  getSystemConnectionPrimary,
  getSystemConnectionSecondary,
  getSystemConnectionTitle,
  type GameAnalysis,
} from '@/entities/game-analysis';
import { formatPatternTagLabel } from '@/shared/lib';
import { useI18n } from '@/shared/lib/i18n';

const props = defineProps<{
  analysis: GameAnalysis;
}>();

const { t, te } = useI18n();

function patternLabel(tag: string): string {
  return formatPatternTagLabel(tag, t, te);
}

const sc = computed(() => props.analysis.system_connection);

const titleText = computed(() => getSystemConnectionTitle(sc.value, t, te));

const primaryText = computed(() => getSystemConnectionPrimary(sc.value, t, te, patternLabel));

const ctaText = computed(() => getSystemConnectionCta(sc.value, t, te));

const secondaryText = computed(
  () => getSystemConnectionSecondary(sc.value, t, te, patternLabel) ?? '',
);

const hasSystemContent = computed(() => Boolean(primaryText.value?.trim()));

const cardTitle = computed(() => {
  const primaryTag = props.analysis.pattern_tags[0];
  if (primaryTag) {
    return patternLabel(primaryTag);
  }
  if (sc.value.tag && sc.value.tag !== 'general') {
    return patternLabel(sc.value.tag);
  }
  return titleText.value ?? '';
});
</script>

<style scoped lang="scss">
.game-pattern-card {
  background: linear-gradient(
    135deg,
    rgba(var(--v-theme-warning), 0.1) 0%,
    rgba(var(--v-theme-surface), 0.5) 100%
  );
}
</style>
