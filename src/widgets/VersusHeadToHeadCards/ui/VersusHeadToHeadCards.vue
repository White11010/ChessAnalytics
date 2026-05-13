<template>
  <v-row dense class="mb-4" align="stretch">
    <v-col cols="12" md>
      <v-card
        variant="tonal"
        color="info"
        class="h-100 pa-3 d-flex align-center justify-space-between ga-3"
      >
        <span class="text-subtitle-1 font-weight-medium text-truncate">
          {{ slice.selfSide.username }}
        </span>
        <v-chip
          size="small"
          variant="flat"
          class="font-weight-medium versus-mono flex-shrink-0"
        >
          {{
            slice.selfSide.rating != null
              ? `${slice.selfSide.rating} · ${speedLabel}`
              : `— · ${speedLabel}`
          }}
        </v-chip>
      </v-card>
    </v-col>
    <v-col cols="12" md="auto" class="d-flex align-center justify-center py-2 py-md-0">
      <span class="text-h5 text-disabled font-weight-bold">VS</span>
    </v-col>
    <v-col cols="12" md>
      <v-card
        variant="tonal"
        color="warning"
        class="h-100 pa-3 d-flex align-center justify-space-between ga-3"
      >
        <span class="text-subtitle-1 font-weight-medium text-truncate">
          {{ slice.opponentSide.username }}
        </span>
        <v-chip
          size="small"
          variant="flat"
          class="font-weight-medium versus-mono flex-shrink-0"
        >
          {{
            slice.opponentSide.rating != null
              ? `${slice.opponentSide.rating} · ${speedLabel}`
              : `— · ${speedLabel}`
          }}
        </v-chip>
      </v-card>
    </v-col>
  </v-row>
</template>

<script setup lang="ts">
// Composite widget: presents a focused dashboard block; reads shared Pinia stores and Tauri invoke where needed.

import type { VersusSpeedSlice } from '@/entities/versus';

defineProps<{
  slice: VersusSpeedSlice;
  speedLabel: string;
}>();
</script>

<style scoped lang="scss">
.versus-mono {
  font-family: 'JetBrains Mono', ui-monospace, monospace;
  font-variant-numeric: tabular-nums;
}
</style>
