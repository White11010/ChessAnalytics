<template>
  <div class="d-flex flex-column ga-3">
    <v-chip
      size="small"
      class="label-chip text-uppercase font-weight-bold"
      :color="tier === 'attack' ? 'success' : 'error'"
      variant="flat"
      label
    >
      {{ label }}
    </v-chip>
    <div v-if="!entries.length" class="text-caption text-medium-emphasis">—</div>
    <v-card
      v-for="(e, i) in entries"
      :key="i"
      variant="outlined"
      density="compact"
      class="pa-3"
    >
      <div class="text-body-2 font-weight-medium">{{ e.title }}</div>
      <div class="text-caption text-medium-emphasis mt-1">{{ e.subtitle }}</div>
    </v-card>
  </div>
</template>

<script setup lang="ts">
import type { VersusPlanEntry } from '@/entities/versus';

defineProps<{
  label: string;
  tier: 'attack' | 'avoid';
  entries: VersusPlanEntry[];
}>();
</script>

<style scoped>
.label-chip {
  letter-spacing: 0.06em;
  align-self: flex-start;
}
</style>
