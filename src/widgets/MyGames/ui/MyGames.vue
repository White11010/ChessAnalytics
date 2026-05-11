<template>
  <div class="my-games">
    <my-games-sync />
    <my-games-section />
    <no-games-found />
  </div>
</template>

<script setup lang="ts">
// Composite widget: presents a focused dashboard block; reads shared Pinia stores and Tauri invoke where needed.

import MyGamesSection from './MyGamesSection.vue';
import { MyGamesSync } from '@/features/MyGamesSync';
import { NoGamesFound } from '@/features/NoGamesFound';
</script>
