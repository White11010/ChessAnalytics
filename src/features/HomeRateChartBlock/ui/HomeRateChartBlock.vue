<template>
  <v-card class="h-100" :title="t('home.rateChartTitle')">
    <v-btn-toggle
      v-model="speed"
      class="d-flex flex-wrap mb-2 mt-4"
      mandatory
      base-color="secondary"
      variant="text"
      density="compact"
    >
      <v-btn value="bullet">{{ t('myGames.speed.bullet') }}</v-btn>
      <v-btn value="blitz">{{ t('myGames.speed.blitz') }}</v-btn>
      <v-btn value="rapid">{{ t('myGames.speed.rapid') }}</v-btn>
    </v-btn-toggle>
    <apexchart
      v-if="chartData.length"
      :key="speed"
      height="350"
      type="line"
      :options="chartOptions"
      :series="series"
    />

    <template v-else>
      <v-skeleton-loader type="button, button, button" />
      <v-skeleton-loader type="image" />
    </template>
  </v-card>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import { useTheme } from 'vuetify';

import { useSyncGamesQuery } from '@/entities/game';
import { useI18n } from '@/shared/lib/i18n';

const { t } = useI18n();
const theme = useTheme();
const { games } = useSyncGamesQuery();

const platform = 'Lichess';
const speed = ref('bullet');

const THIRTY_DAYS = 30 * 24 * 60 * 60 * 1000;

const filteredGames = computed(() => {
  const now = Date.now();
  const from = now - THIRTY_DAYS;

  return games.value
    .filter((game) => {
      return (
        game.rated &&
        game.platform === platform &&
        game.speed === speed.value &&
        game.created_at >= from
      );
    })
    .sort((a, b) => a.created_at - b.created_at);
});

const chartData = computed(() => {
  const dayMap = new Map<number, number>();

  filteredGames.value.forEach((game) => {
    const date = new Date(game.created_at);

    const dayStart = new Date(date.getFullYear(), date.getMonth(), date.getDate()).getTime();

    dayMap.set(dayStart, game.player_rating!);
  });

  return Array.from(dayMap.entries()).map(([x, y]) => ({
    x,
    y,
  }));
});

const isDark = computed(() => theme.global.current.value.dark);

const textColor = computed(() => theme.current.value.colors['on-surface']);
const borderColor = computed(() => theme.current.value.colors.outline);
const lineColor = computed(() => theme.current.value.colors.secondary);

const series = computed(() => [
  {
    name: t('home.rateSeriesName'),
    data: chartData.value,
    color: lineColor.value,
  },
]);

const chartOptions = computed(() => ({
  chart: {
    type: 'line',

    zoom: {
      enabled: false,
    },

    toolbar: {
      show: false,
    },

    animations: {
      enabled: true,
    },

    background: 'transparent',
  },

  theme: {
    mode: isDark.value ? 'dark' : 'light',
  },

  stroke: {
    curve: 'straight',
    width: 3,
  },

  grid: {
    borderColor: borderColor.value,
  },

  markers: {
    size: 3,
    hover: {
      size: 6,
    },
    strokeWidth: 0,
  },

  xaxis: {
    type: 'datetime',

    labels: {
      style: {
        colors: textColor.value,
      },
    },

    axisBorder: {
      color: borderColor.value,
    },

    axisTicks: {
      color: borderColor.value,
    },
  },

  yaxis: {
    labels: {
      style: {
        colors: textColor.value,
      },
    },
  },

  tooltip: {
    theme: isDark.value ? 'dark' : 'light',

    style: {
      fontSize: '14px',
    },

    x: {
      format: 'dd MMM',
    },
  },

  legend: {
    labels: {
      colors: textColor.value,
    },
  },
}));
</script>
