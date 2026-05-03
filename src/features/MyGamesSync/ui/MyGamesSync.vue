<template>
  <v-card v-if="!gamesStore.games.length" title="Expore your games" class="h-100">
    <v-card-text class="mt-2 d-flex justify-center">
      <v-btn :loading="gamesStore.loading" @click="loadPlayerData">Load your games</v-btn>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import { useGamesStore } from '@/entities/game';

const gamesStore = useGamesStore();

async function loadPlayerData(): Promise<void> {
  await gamesStore.sync();
}
</script>
