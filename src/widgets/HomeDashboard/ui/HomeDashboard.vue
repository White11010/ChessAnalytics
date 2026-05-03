<template>
  <v-container fluid>
    <home-greeting-block />
    <home-top-insight-block class="mt-2" />
    <!-- <home-last-games-stats-block /> -->
    <home-last-game class="mt-2" />
    <home-rate-chart-block class="mt-2" />
    <home-recent-games-block class="mt-2" />
  </v-container>
</template>
<script setup lang="ts">
import { HomeGreetingBlock } from '@/features/HomeGreetingBlock';
import { HomeLastGame } from '@/features/HomeLastGame';
// import { HomeLastGamesStatsBlock } from '@/features/HomeLastGamesStatsBlock';
import { HomeRateChartBlock } from '@/features/HomeRateChartBlock';
import { HomeRecentGamesBlock } from '@/features/HomeRecentGamesBlock';
import { HomeTopInsightBlock } from '@/features/HomeTopInsightBlock';
</script>
