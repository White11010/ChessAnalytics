<template>
  <div class="home-page">
    <lichess-account-connection />
    <home-dashboard />
  </div>
</template>

<script setup lang="ts">
import { HomeDashboard } from '@/widgets/HomeDashboard';
import { LichessAccountConnection } from '@/widgets/LichessAccountConnection';
</script>

<style scoped>
.home-page {
  padding: 0 2rem;
  max-width: 58rem;
  flex: 1;
  height: calc(100% - 2rem);
  overflow: auto;
  margin: 1rem 0;
}
</style>
