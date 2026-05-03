<template>
  <div class="my-games-page">
    <my-games />
  </div>
</template>

<script setup lang="ts">
import { MyGames } from '@/widgets/MyGames';
</script>

<style scoped lang="scss">
.my-games-page {
  padding: 2rem;
  max-width: 58rem;
  flex: 1;
  height: 100%;
}
</style>
