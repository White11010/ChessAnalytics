<template>
  <v-data-table v-if="games.length" :headers="headers" :items="games" />
</template>

<script setup lang="ts">
import { useSyncGamesQuery } from '@/entities/game';

const { games } = useSyncGamesQuery();

const headers = [
  {
    key: 'speed',
    title: '',
    width: '96px',
  },
  {
    key: 'opponent_name',
    title: 'Opponent',
  },
  {
    key: 'player_result',
    title: 'Result',
    width: '48px',
  },
  {
    key: 'created_at',
    title: 'Date',
    width: '144px',
  },
];
</script>
