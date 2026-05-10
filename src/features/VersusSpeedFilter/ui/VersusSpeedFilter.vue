<template>
  <div class="d-flex flex-column flex-sm-row align-stretch align-sm-center ga-2 mb-4">
    <span class="text-body-2 text-medium-emphasis flex-shrink-0 align-self-center">
      {{ t('versusPage.speedFilterLabel') }}
    </span>
    <v-btn-toggle
      :model-value="versusStore.selectedSpeed"
      mandatory
      density="compact"
      variant="outlined"
      color="secondary"
      class="versus-speed flex-shrink-0"
      @update:model-value="onSpeed"
    >
      <v-btn value="bullet" size="small">{{ t('myGames.speed.bullet') }}</v-btn>
      <v-btn value="blitz" size="small">{{ t('myGames.speed.blitz') }}</v-btn>
      <v-btn value="rapid" size="small">{{ t('myGames.speed.rapid') }}</v-btn>
    </v-btn-toggle>
  </div>
</template>

<script setup lang="ts">
import { useVersusStore, type VersusSpeed } from '@/entities/versus';
import { useI18n } from '@/shared/lib/i18n';

const { t } = useI18n();
const versusStore = useVersusStore();

function onSpeed(v: VersusSpeed): void {
  versusStore.selectedSpeed = v;
}
</script>

<style scoped lang="scss">
.versus-speed :deep(.v-btn) {
  text-transform: none;
  letter-spacing: normal;
}
</style>
