<template>
  <v-card title="Last game">
    <template v-if="lastGame">
      <v-card-text>vs {{ lastGame.opponent_name }} ({{ lastGame.opponent_rating }})</v-card-text>
      <v-card-text>{{ lastGame.player_result }}</v-card-text>

      <v-card-actions>
        <v-spacer></v-spacer>

        <v-btn
          class="button-width-12"
          color="secondary"
          variant="elevated"
          @click="onAnalizeButtonClick"
        >
          Analize
        </v-btn>
      </v-card-actions>
    </template>

    <v-skeleton-loader v-else type="list-item-two-line" />
  </v-card>
</template>

<script setup lang="ts">
import { computed } from 'vue';

const router = useRouter();

import { useRouter } from 'vue-router';

import { useSyncGamesQuery } from '@/entities/game';

const { games } = useSyncGamesQuery();

const lastGame = computed(() => games.value[0]);

function onAnalizeButtonClick(): void {
  router.push('/analize-board');
}
</script>
