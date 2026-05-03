<template>
  <div class="insights-page">
    <insights-block />
  </div>
</template>

<script setup lang="ts">
import { InsightsBlock } from '@/widgets/InsightsBlock';
</script>

<style scoped>
.insights-page {
  padding: 2rem;
  max-width: 58rem;
  flex: 1;
  height: 100%;
}
</style>
