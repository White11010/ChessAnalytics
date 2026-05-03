<template>
  <v-card title="Recent games">
    <v-card-text>
      <v-data-table
        v-if="games.length"
        :headers="headers"
        :items="getRecentGames"
        hide-default-footer
      />

      <v-skeleton-loader v-else type="table-thead, table-tbody" />
    </v-card-text>
    <v-card-actions v-if="games.length">
      <v-spacer></v-spacer>

      <v-btn
        class="button-width-12"
        color="secondary"
        variant="elevated"
        @click="onWatchAllButtonClick"
      >
        Watch all games
      </v-btn>
    </v-card-actions>
  </v-card>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useRouter } from 'vuetify/lib/composables/router.mjs';

import { useSyncGamesQuery } from '@/entities/game';

const router = useRouter();

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

const getRecentGames = computed(() => {
  return games.value.slice(0, 10);
});

function onWatchAllButtonClick(): void {
  router?.push('/my-games');
}
</script>
