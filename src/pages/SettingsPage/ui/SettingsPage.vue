<template>
  <div class="settings-page">
    <appearance-settings />
  </div>
</template>

<script setup lang="ts">
import { AppearanceSettings } from '@/widgets/AppearanceSettings';
</script>

<style scoped>
.settings-page {
  padding: 2rem;
  max-width: 58rem;
  flex: 1;
  height: 100%;
}
</style>
