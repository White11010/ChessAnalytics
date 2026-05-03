<template>
  <v-card title="Month rate">
    <apexchart
      v-if="games.length"
      height="350"
      type="line"
      :options="chartOptions"
      :series="series"
    />
    <v-skeleton-loader v-else type="image" />
  </v-card>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useTheme } from 'vuetify';

import { useSyncGamesQuery } from '@/entities/game';

const theme = useTheme();

const { games } = useSyncGamesQuery();

const platform = 'Lichess';
const speed = 'bullet';

const THIRTY_DAYS = 30 * 24 * 60 * 60 * 1000;

const chartData = computed(() => {
  const now = Date.now();
  const from = now - THIRTY_DAYS;

  return games.value
    .filter((game) => {
      return (
        game.rated && game.platform === platform && game.speed === speed && game.created_at >= from
      );
    })
    .sort((a, b) => a.created_at - b.created_at)
    .map((game) => ({
      x: game.created_at,
      y: game.player_rating,
    }));
});

const series = computed(() => [
  {
    name: 'Rating',
    data: chartData.value,
    color: lineColor.value,
  },
]);

const isDark = computed(() => theme.global.current.value.dark);

// const surfaceColor = computed(() => {
//   return theme.current.value.colors.surface;
// });

const textColor = computed(() => {
  return theme.current.value.colors['on-surface'];
});

const borderColor = computed(() => {
  return theme.current.value.colors.outline;
});
const lineColor = computed(() => theme.current.value.colors.secondary);
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

    colors: [lineColor],
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
    size: 4,
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
      format: 'dd MMM HH:mm',
    },

    marker: {
      show: true,
    },
  },

  legend: {
    labels: {
      colors: textColor.value,
    },
  },
}));
</script>
