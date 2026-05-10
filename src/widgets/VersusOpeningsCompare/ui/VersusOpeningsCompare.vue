<template>
  <div>
    <h3 class="text-overline text-medium-emphasis mb-2">{{ t('versusPage.openingPrefs') }}</h3>
    <v-row class="mb-6">
      <v-col cols="12" md="6">
        <v-card variant="outlined" class="h-100">
          <v-card-title class="text-subtitle-2 text-primary pb-0">
            {{ t('versusPage.colYou') }}
          </v-card-title>
          <v-divider class="mt-3" />
          <v-list density="compact" class="py-0">
            <v-list-item v-for="o in slice.selfSide.openings" :key="o.name" :title="o.name">
              <template #append>
                <span
                  class="text-caption font-weight-medium versus-mono"
                  :class="wrTone(o.winRatePct)"
                >
                  {{ pct(o.winRatePct) }}
                </span>
              </template>
            </v-list-item>
            <v-list-item v-if="!slice.selfSide.openings.length">
              <v-list-item-title class="text-caption text-medium-emphasis">—</v-list-item-title>
            </v-list-item>
          </v-list>
        </v-card>
      </v-col>
      <v-col cols="12" md="6">
        <v-card variant="outlined" class="h-100">
          <v-card-title class="text-subtitle-2 text-warning pb-0">
            {{ slice.opponentSide.username }}
          </v-card-title>
          <v-divider class="mt-3" />
          <v-list density="compact" class="py-0">
            <v-list-item v-for="o in slice.opponentSide.openings" :key="o.name" :title="o.name">
              <template #append>
                <span
                  class="text-caption font-weight-medium versus-mono"
                  :class="wrTone(o.winRatePct)"
                >
                  {{ pct(o.winRatePct) }}
                </span>
              </template>
            </v-list-item>
            <v-list-item v-if="!slice.opponentSide.openings.length">
              <v-list-item-title class="text-caption text-medium-emphasis">—</v-list-item-title>
            </v-list-item>
          </v-list>
        </v-card>
      </v-col>
    </v-row>
  </div>
</template>

<script setup lang="ts">
import type { VersusSpeedSlice } from '@/entities/versus';
import { useI18n } from '@/shared/lib/i18n';

defineProps<{
  slice: VersusSpeedSlice;
}>();

const { t } = useI18n();

function pct(v: number): string {
  return `${Math.round(v)}%`;
}

function wrTone(wr: number): string {
  if (wr >= 58) return 'text-success';
  if (wr >= 48) return 'text-warning';
  return 'text-error';
}
</script>

<style scoped lang="scss">
.versus-mono {
  font-family: 'JetBrains Mono', ui-monospace, monospace;
  font-variant-numeric: tabular-nums;
}
</style>
