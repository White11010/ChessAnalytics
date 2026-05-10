<template>
  <div class="home-page">
    <div class="home-page__container">
      <lichess-account-connection />
      <home-dashboard />
    </div>
  </div>
</template>

<script setup lang="ts">
import { HomeDashboard } from '@/widgets/HomeDashboard';
import { LichessAccountConnection } from '@/widgets/LichessAccountConnection';
</script>

<style scoped>
.home-page {
  width: 100%;
  flex: 1;
  height: 100%;
  overflow-y: auto;
}

.home-page__container {
  padding: 1rem 2rem 2rem;
  width: 100%;
  max-width: none;
  margin: 0;
}
</style>
