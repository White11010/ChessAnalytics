<template>
  <v-card v-if="slice.gamePlan">
    <v-card-title class="d-flex flex-wrap align-center justify-space-between ga-2">
      <span class="d-flex align-center ga-2">
        <v-icon icon="mdi-chess-pawn" />
        {{
          t('versusPage.gamePlanSubtitle', {
            name: slice.opponentSide.username,
          })
        }}
      </span>
      <v-chip size="small" variant="outlined" class="versus-mono">
        {{ t('versusPage.gamePlanBadge', { n: slice.gamePlan.oppGamesInOpeningSlice }) }}
      </v-chip>
    </v-card-title>
    <v-divider />
    <v-card-text v-if="gamePlanEmpty" class="text-caption text-medium-emphasis py-6 text-center">
      {{ t('versusPage.gpEmpty') }}
    </v-card-text>
    <template v-else>
      <v-row no-gutters>
        <v-col cols="12" md class="pa-4">
          <div class="text-subtitle-2 d-flex align-center ga-2 mb-4 pb-2 border-b-sm">
            <span aria-hidden="true">♔</span>
            {{ t('versusPage.gpWhite') }}
          </div>
          <VersusGamePlanBlock
            :label="t('versusPage.gpAttack')"
            tier="attack"
            :entries="slice.gamePlan.asWhite.attack"
          />
          <VersusGamePlanBlock
            class="mt-4"
            :label="t('versusPage.gpAvoid')"
            tier="avoid"
            :entries="slice.gamePlan.asWhite.avoid"
          />
        </v-col>
        <v-col cols="12" md="auto" class="d-none d-md-flex align-stretch pa-0 justify-center">
          <v-divider vertical />
        </v-col>
        <v-col cols="12" md class="pa-4">
          <div
            class="text-subtitle-2 text-medium-emphasis d-flex align-center ga-2 mb-4 pb-2 border-b-sm"
          >
            <span aria-hidden="true">♚</span>
            {{ t('versusPage.gpBlack') }}
          </div>
          <VersusGamePlanBlock
            :label="t('versusPage.gpAttack')"
            tier="attack"
            :entries="slice.gamePlan.asBlack.attack"
          />
          <VersusGamePlanBlock
            class="mt-4"
            :label="t('versusPage.gpAvoid')"
            tier="avoid"
            :entries="slice.gamePlan.asBlack.avoid"
          />
        </v-col>
      </v-row>
    </template>
    <v-divider />
    <v-card-actions class="text-caption text-medium-emphasis px-4 py-3 bg-surface-variant">
      <v-icon icon="mdi-lightbulb-outline" size="small" class="me-2 flex-shrink-0" />
      <span>{{ t('versusPage.gpFooterHint') }}</span>
    </v-card-actions>
  </v-card>
</template>

<script setup lang="ts">
import type { VersusSpeedSlice } from '@/entities/versus';
import { useI18n } from '@/shared/lib/i18n';
import { computed } from 'vue';

import VersusGamePlanBlock from './VersusGamePlanBlock.vue';

const props = defineProps<{
  slice: VersusSpeedSlice;
}>();

const { t } = useI18n();

const gamePlanEmpty = computed(() => {
  const g = props.slice.gamePlan;
  if (!g) return true;
  const sum =
    g.asWhite.attack.length +
    g.asWhite.avoid.length +
    g.asBlack.attack.length +
    g.asBlack.avoid.length;
  return sum === 0;
});
</script>

<style scoped lang="scss">
.versus-mono {
  font-family: 'JetBrains Mono', ui-monospace, monospace;
  font-variant-numeric: tabular-nums;
}

.border-b-sm {
  border-bottom: thin solid rgba(var(--v-theme-on-surface), 0.08);
}
</style>
