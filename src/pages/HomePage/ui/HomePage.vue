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
  --home-page-gap: 1.25rem;
  box-sizing: border-box;
  width: 100%;
  flex: 1;
  height: 100%;
  min-height: 0;
  overflow-y: auto;
  overflow-x: hidden;
  display: flex;
  flex-direction: column;
  border: none;
  /* Padding on the scrollport so the bottom inset stays visible at scroll end */
  padding: var(--home-page-gap);
}

.home-page__container {
  width: 100%;
  max-width: none;
  margin: 0;
  flex: 0 0 auto;
  display: flex;
  flex-direction: column;
  min-height: 0;
  border: none;
  container-type: inline-size;
  container-name: home-page;
}
</style>
