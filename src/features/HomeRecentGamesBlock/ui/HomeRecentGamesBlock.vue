<template>
  <v-card title="Recent games">
    <v-card-text>
      <v-data-table
        v-if="games.length"
        :headers="headers"
        :items="getRecentGames"
        hide-default-footer
      >
        <template #item.player_result="{ item }">
          <v-chip :color="getResultBadgeColor(item.player_result)">
            {{ item.player_result }}
          </v-chip>
        </template>

        <template #item.created_at="{ item }">
          {{ formatTimestamp(item.created_at, { withRelativeDays: true }) }}
        </template>
      </v-data-table>

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

import { Game, useSyncGamesQuery } from '@/entities/game';
import { formatTimestamp } from '@/shared/lib';

const router = useRouter();

const { games } = useSyncGamesQuery();

const headers = [
  {
    key: 'speed',
    title: '',
    width: '96px',
    sortable: false,
  },
  {
    key: 'opponent_name',
    title: 'Opponent',
    sortable: false,
  },
  {
    key: 'player_result',
    title: 'Result',
    width: '48px',
    sortable: false,
  },
  {
    key: 'created_at',
    title: 'Date',
    width: '164px',
    sortable: false,
  },
];

const getRecentGames = computed(() => {
  return games.value.slice(0, 10);
});

function onWatchAllButtonClick(): void {
  router?.push('/my-games');
}

function getResultBadgeColor(result: Game['player_result']) {
  switch (result) {
    case 'win':
      return 'success';
    case 'loss':
      return 'error';
    case 'draw':
      return 'default';
  }
}
</script>
