<template>
  <v-card :title="'Daily insight'">
    <template v-if="heroInsight">
      <v-card-text>
        {{ heroInsight.title }}
      </v-card-text>
      <v-card-text>
        {{ heroInsight.summary }}
      </v-card-text>
      <v-card-text>
        {{ heroInsight.recommendation }}
      </v-card-text>
    </template>
    <v-skeleton-loader v-else type="list-item-two-line" />
  </v-card>
</template>

<script setup lang="ts">
import { computed } from 'vue';

import { useSyncGamesQuery } from '@/entities/game';
import { useRegenerateInsightsQuery } from '@/entities/insight';

const gamesQuery = useSyncGamesQuery();
const { insights } = useRegenerateInsightsQuery(gamesQuery.isSuccess);
const heroInsight = computed(() => insights.value[0]);
</script>
