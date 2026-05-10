<template>
  <v-row dense class="mb-4" align="stretch">
    <v-col cols="12" md>
      <v-card variant="tonal" color="primary" class="h-100 text-center pa-4">
        <v-avatar color="primary" size="48" class="mb-2 text-h6">
          {{ initialLetter(slice.selfSide.username) }}
        </v-avatar>
        <div class="text-subtitle-1 font-weight-medium text-truncate">
          {{ slice.selfSide.username }}
        </div>
        <v-chip size="small" variant="flat" class="mt-2 font-weight-medium versus-mono">
          {{
            slice.selfSide.rating != null
              ? `${slice.selfSide.rating} · ${speedLabel}`
              : `— · ${speedLabel}`
          }}
        </v-chip>
        <div class="mt-2">
          <v-chip size="x-small" variant="text" color="primary">
            {{ t('versusPage.youChip') }}
          </v-chip>
        </div>
      </v-card>
    </v-col>
    <v-col cols="12" md="auto" class="d-flex align-center justify-center py-2 py-md-0">
      <span class="text-h5 text-disabled font-weight-bold">VS</span>
    </v-col>
    <v-col cols="12" md>
      <v-card variant="tonal" color="warning" class="h-100 text-center pa-4">
        <v-avatar color="warning" size="48" class="mb-2 text-h6">
          {{ initialLetter(slice.opponentSide.username) }}
        </v-avatar>
        <div class="text-subtitle-1 font-weight-medium text-truncate">
          {{ slice.opponentSide.username }}
        </div>
        <v-chip size="small" variant="flat" class="mt-2 font-weight-medium versus-mono">
          {{
            slice.opponentSide.rating != null
              ? `${slice.opponentSide.rating} · ${speedLabel}`
              : `— · ${speedLabel}`
          }}
        </v-chip>
        <div class="mt-2">
          <v-chip size="x-small" variant="text" color="warning">
            {{ t('versusPage.oppChip') }}
          </v-chip>
        </div>
      </v-card>
    </v-col>
  </v-row>
</template>

<script setup lang="ts">
import type { VersusSpeedSlice } from '@/entities/versus';
import { useI18n } from '@/shared/lib/i18n';

defineProps<{
  slice: VersusSpeedSlice;
  speedLabel: string;
}>();

const { t } = useI18n();

function initialLetter(name: string): string {
  const c = name.trim()[0];
  return c ? c.toUpperCase() : '?';
}
</script>

<style scoped lang="scss">
.versus-mono {
  font-family: 'JetBrains Mono', ui-monospace, monospace;
  font-variant-numeric: tabular-nums;
}
</style>
