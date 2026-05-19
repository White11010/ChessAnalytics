<template>
  <div class="d-flex flex-column ga-8">
    <GameKeyInsightCard :analysis="analysis" />
    <GameMetricsGrid :analysis="analysis" :game="game" />
  </div>
</template>

<script setup lang="ts">
import type { Game } from '@/entities/game';
import type { GameAnalysis } from '@/entities/game-analysis';
import { GameKeyInsightCard } from '@/widgets/GameKeyInsightCard';
import { GameMetricsGrid } from '@/widgets/GameMetricsGrid';

defineProps<{
  analysis: GameAnalysis;
  game?: Game;
}>();
</script>
