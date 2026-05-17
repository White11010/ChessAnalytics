<template>
  <section :class="rootClass">
    <div class="versus-game-plan-row">
      <v-chip
        size="small"
        class="label-chip text-uppercase font-weight-bold"
        :color="tier === 'play' ? 'success' : 'error'"
        variant="flat"
        label
      >
        {{ label }}
      </v-chip>
      <v-chip
        size="small"
        class="label-chip text-uppercase font-weight-bold"
        :color="tier === 'play' ? 'success' : 'error'"
        variant="flat"
        label
      >
        {{ label }}
      </v-chip>
    </div>
    <div
      v-for="i in rowCount"
      :key="i"
      class="versus-game-plan-row"
      :class="{ 'mt-3': i > 1 }"
    >
      <VersusGamePlanCard v-if="whiteEntries[i - 1]" :entry="whiteEntries[i - 1]!" />
      <VersusGamePlanCard v-if="blackEntries[i - 1]" :entry="blackEntries[i - 1]!" />
    </div>
  </section>
</template>

<script setup lang="ts">
import type { VersusPlanEntry } from '@/entities/versus';
import { computed } from 'vue';

import VersusGamePlanCard from './VersusGamePlanCard.vue';

const props = defineProps<{
  label: string;
  tier: 'play' | 'avoid';
  whiteEntries: VersusPlanEntry[];
  blackEntries: VersusPlanEntry[];
  rootClass?: string | string[] | Record<string, boolean>;
}>();

const rowCount = computed(() =>
  Math.max(props.whiteEntries.length, props.blackEntries.length, 0),
);
</script>

<style scoped lang="scss">
.versus-game-plan-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  column-gap: 48px;
  align-items: stretch;
}

.label-chip {
  letter-spacing: 0.06em;
  justify-self: start;
}
</style>
