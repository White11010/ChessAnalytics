<template>
  <v-card title="Month rate">
    <v-btn-toggle class="my-2" base-color="secondary" variant="text" v-model="speed" mandatory>
      <v-btn value="bullet"> Bullet </v-btn>

      <v-btn value="blitz"> Blitz </v-btn>

      <v-btn value="rapid"> Rapid </v-btn>
    </v-btn-toggle>
    <apexchart
      v-if="chartData.length"
      :key="speed"
      height="350"
      type="line"
      :options="chartOptions"
      :series="series"
    />

    <v-skeleton-loader v-else type="image" />
  </v-card>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import { useTheme } from 'vuetify';

import { useSyncGamesQuery } from '@/entities/game';

const theme = useTheme();
const { games } = useSyncGamesQuery();

const platform = 'Lichess';
const speed = ref('bullet');

const THIRTY_DAYS = 30 * 24 * 60 * 60 * 1000;

// ---------------- FILTERED GAMES ----------------
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

// ---------------- AGGREGATE BY DAY ----------------
const chartData = computed(() => {
  const dayMap = new Map<number, number>();

  filteredGames.value.forEach((game) => {
    const date = new Date(game.created_at);

    const dayStart = new Date(date.getFullYear(), date.getMonth(), date.getDate()).getTime();

    // последний рейтинг дня
    dayMap.set(dayStart, game.player_rating!);
  });

  return Array.from(dayMap.entries()).map(([x, y]) => ({
    x,
    y,
  }));
});

// ---------------- THEME ----------------
const isDark = computed(() => theme.global.current.value.dark);

const textColor = computed(() => theme.current.value.colors['on-surface']);
const borderColor = computed(() => theme.current.value.colors.outline);
const lineColor = computed(() => theme.current.value.colors.secondary);

// ---------------- SERIES ----------------
const series = computed(() => [
  {
    name: 'Rating',
    data: chartData.value,
    color: lineColor.value,
  },
]);

// ---------------- OPTIONS ----------------
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
