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
      <div class="versus-game-plan-layout pa-4">
        <div class="versus-game-plan-divider-rail d-none d-md-flex">
          <v-divider vertical class="versus-game-plan-divider" />
        </div>
        <div class="versus-game-plan-content">
          <div class="versus-game-plan-row mb-4 pb-2 border-b-sm">
            <div class="text-subtitle-2 d-flex align-center ga-2">
              <span aria-hidden="true">♔</span>
              {{ t('versusPage.gpWhite') }}
            </div>
            <div class="text-subtitle-2 text-medium-emphasis d-flex align-center ga-2">
              <span aria-hidden="true">♚</span>
              {{ t('versusPage.gpBlack') }}
            </div>
          </div>
          <VersusGamePlanTierGrid
            :label="t('versusPage.gpPlay')"
            tier="play"
            :white-entries="slice.gamePlan.asWhite.play"
            :black-entries="slice.gamePlan.asBlack.play"
          />
          <VersusGamePlanTierGrid
            root-class="mt-6"
            :label="t('versusPage.gpAvoid')"
            tier="avoid"
            :white-entries="slice.gamePlan.asWhite.avoid"
            :black-entries="slice.gamePlan.asBlack.avoid"
          />
        </div>
      </div>
    </template>
    <v-divider />
    <v-alert class="mt-6" variant="tonal" type="info" :text="t('versusPage.gpFooterHint')" />
  </v-card>
</template>

<script setup lang="ts">
import type { VersusSpeedSlice } from '@/entities/versus';
import { useI18n } from '@/shared/lib/i18n';
import { computed } from 'vue';

import VersusGamePlanTierGrid from './VersusGamePlanTierGrid.vue';

const props = defineProps<{
  slice: VersusSpeedSlice;
}>();

const { t } = useI18n();

const gamePlanEmpty = computed(() => {
  const g = props.slice.gamePlan;
  if (!g) return true;
  const sum =
    g.asWhite.play.length +
    g.asWhite.avoid.length +
    g.asBlack.play.length +
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

.versus-game-plan-layout {
  position: relative;
}

.versus-game-plan-divider-rail {
  position: absolute;
  left: 50%;
  top: 0;
  bottom: 0;
  transform: translateX(-50%);
  align-items: stretch;
  justify-content: center;
  z-index: 1;
  pointer-events: none;
}

.versus-game-plan-divider {
  height: 100%;
  opacity: 1;
}

.versus-game-plan-content {
  position: relative;
  z-index: 0;
}

.versus-game-plan-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  column-gap: 48px;
  align-items: stretch;
}

@media (max-width: 959px) {
  .versus-game-plan-row {
    grid-template-columns: 1fr;
    row-gap: 12px;
    column-gap: 0;
  }
}
</style>
