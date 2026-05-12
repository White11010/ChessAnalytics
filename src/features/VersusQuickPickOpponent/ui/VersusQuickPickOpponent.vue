<template>
  <div class="d-flex flex-wrap ga-2">
    <v-btn
      v-for="preset in presets"
      :key="preset.username"
      variant="tonal"
      color="secondary"
      size="small"
      rounded="lg"
      class="text-none"
      :disabled="versusStore.loading"
      @click="pick(preset.username)"
    >
      {{ preset.label }}
    </v-btn>
  </div>
</template>

<script setup lang="ts">
// Feature slice: encapsulates one user flow or form; parent pages/widgets compose it and pass props/events.

import { useVersusStore } from '@/entities/versus';

interface OpponentPreset {
  label: string;
  username: string;
}

const presets: ReadonlyArray<OpponentPreset> = [
  { label: 'Magnus Carlsen', username: 'DrNykterstein' },
  { label: 'Levy Rozman', username: 'GothamChess' },
  { label: 'Hikaru Nakamura', username: 'Hikaru' },
  { label: 'Alireza Firouzja', username: 'Firouzja2003' },
];

const versusStore = useVersusStore();

function pick(username: string): void {
  if (versusStore.loading) return;
  versusStore.opponentUsernameInput = username;
  void versusStore.compare();
}
</script>
