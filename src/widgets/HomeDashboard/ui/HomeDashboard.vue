<template>
  <v-container fluid class="home-dashboard pa-0">
    <home-greeting-block class="home-dashboard__full" />

    <v-row class="home-dashboard__row ma-0 ga-5" align="stretch">
      <v-col cols="12" class="home-dashboard__col-split pa-0 d-flex">
        <home-profile-chart-block class="h-100 w-100" />
      </v-col>
      <v-col cols="12" class="home-dashboard__col-split pa-0 d-flex flex-column">
        <div class="home-dashboard__stack">
          <div class="home-dashboard__stack-item">
            <home-top-insight-block />
          </div>
          <div class="home-dashboard__stack-item home-dashboard__stack-item--grow">
            <home-last-game class="flex-grow-1" />
          </div>
        </div>
      </v-col>
    </v-row>

    <v-row class="home-dashboard__row ma-0 ga-5" align="stretch">
      <v-col cols="12" class="home-dashboard__col-split pa-0">
        <home-rate-chart-block />
      </v-col>
      <v-col cols="12" class="home-dashboard__col-split pa-0 flex-grow-1">
        <home-recent-games-block />
      </v-col>
    </v-row>
  </v-container>
</template>
<script setup lang="ts">
import { HomeGreetingBlock } from '@/features/HomeGreetingBlock';
import { HomeLastGame } from '@/features/HomeLastGame';
import { HomeProfileChartBlock } from '@/features/HomeProfileChartBlock';
import { HomeRateChartBlock } from '@/features/HomeRateChartBlock';
import { HomeRecentGamesBlock } from '@/features/HomeRecentGamesBlock';
import { HomeTopInsightBlock } from '@/features/HomeTopInsightBlock';
</script>
<style scoped lang="scss">
.home-dashboard {
  --home-gap: var(--home-page-gap, 0.75rem);
  display: flex;
  flex-direction: column;
  gap: var(--home-gap);
  flex: 1;
}

.home-dashboard__full {
  width: 100%;
}

.home-dashboard__row {
  --v-gutter-x: var(--home-gap);
  --v-gutter-y: 0;
}

.home-dashboard__col-split {
  flex: 0 0 100%;
  max-width: 100%;
  min-width: 0;
}

.home-dashboard__stack {
  display: flex;
  flex-direction: column;
  gap: var(--home-gap);
  min-height: 0;
  /* Narrow home layout: intrinsic height — avoids collapse with height:100% children */
  flex: 0 1 auto;
}

.home-dashboard__stack-item {
  flex: 0 0 auto;
  &--grow {
    flex: 0 1 auto;
    /* allow card to grow with content; min-height:0 would clip tall LastGame */
    min-height: min-content;
    display: flex;
    flex-direction: column;
  }
}

@container home-page (min-width: 1570px) {
  .home-dashboard__row .home-dashboard__col-split {
    flex: 1 1 0;
    max-width: none;
  }

  .home-dashboard__stack {
    flex: 1 1 0;
    min-height: min-content;
  }

  .home-dashboard__stack-item--grow {
    flex: 1 1 auto;
    min-height: min-content;
  }
}

</style>
