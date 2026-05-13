<template>
  <span
    class="text-caption font-weight-medium versus-mono d-inline-flex align-center flex-wrap justify-end"
    style="gap: 0 2px"
  >
    <span :class="wrTone(o.winRatePct)">{{ pct(o.winRatePct) }}</span>
    <span class="text-medium-emphasis">(</span>
    <span class="text-success">{{ o.wins }}</span>
    <span class="text-medium-emphasis"> / </span>
    <span class="text-high-emphasis">{{ o.draws }}</span>
    <span class="text-medium-emphasis"> / </span>
    <span class="text-error">{{ o.losses }}</span>
    <span class="text-medium-emphasis">)</span>
  </span>
</template>

<script setup lang="ts">
import type { VersusOpeningCard } from '@/entities/versus';

defineProps<{
  o: VersusOpeningCard;
}>();

function pct(v: number): string {
  return `${Math.round(v)}%`;
}

function wrTone(wr: number): string {
  if (wr >= 50) return 'text-success';
  return 'text-error';
}
</script>

<style scoped lang="scss">
.versus-mono {
  font-family: 'JetBrains Mono', ui-monospace, monospace;
  font-variant-numeric: tabular-nums;
}
</style>
