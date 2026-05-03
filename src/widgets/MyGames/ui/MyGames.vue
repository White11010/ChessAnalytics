<template>
  <div class="my-games">
    <my-games-sync />
    <my-games-list v-if="gamesStore.games.length" />
    <no-games-found />
  </div>
</template>

<script setup lang="ts">
import { useGamesStore } from '@/entities/game';
import { MyGamesList } from '@/features/MyGamesList';
import { MyGamesSync } from '@/features/MyGamesSync';
import { NoGamesFound } from '@/features/NoGamesFound';

const gamesStore = useGamesStore();
</script>
