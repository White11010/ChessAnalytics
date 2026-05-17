<template>
  <div class="settings-page">
    <v-container class="settings-page__inner pa-4 pa-md-6" fluid>
      <div class="mx-auto settings-page__max">
        <appearance-settings />
        <analysis-settings class="mt-6" />
      </div>
    </v-container>
  </div>
</template>

<script setup lang="ts">
// Route-level page: composes features/widgets for this screen and wires page-local queries or navigation.

import { AnalysisSettings } from '@/widgets/AnalysisSettings';
import { AppearanceSettings } from '@/widgets/AppearanceSettings';
</script>

<style scoped lang="scss">
.settings-page {
  align-self: stretch;
  width: 100%;
  flex: 1;
  min-height: 100%;
  overflow-y: auto;
}

.settings-page__max {
  max-width: 58rem;
}
</style>
