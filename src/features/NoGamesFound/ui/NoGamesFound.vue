<template>
  <v-card v-if="!gamesStore.games.length && gamesStore.wasLoadedEmptyList" class="h-100">
    <v-card-text class="mt-2 d-flex justify-center">
      <p>There are no games found</p>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import { useGamesStore } from '@/entities/game';

const gamesStore = useGamesStore();
</script>
