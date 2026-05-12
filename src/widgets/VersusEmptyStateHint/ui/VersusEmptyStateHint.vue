<template>
  <v-card class="mb-4" :title="t('versusPage.emptyTitle')">
    <v-card-text>
      <h3 class="text-overline text-medium-emphasis mb-2">
        {{ t('versusPage.emptyBenefitsTitle') }}
      </h3>
      <v-list density="compact" class="bg-transparent pa-0 mb-4" lines="one">
        <v-list-item
          v-for="benefit in benefits"
          :key="benefit.key"
          class="px-0"
          min-height="36"
        >
          <template #prepend>
            <v-icon :icon="benefit.icon" color="secondary" size="20" class="me-3" />
          </template>
          <v-list-item-title class="text-body-2 text-wrap">
            {{ benefit.text }}
          </v-list-item-title>
        </v-list-item>
      </v-list>

      <h3 class="text-overline text-medium-emphasis mb-2">
        {{ t('versusPage.emptyQuickStart') }}
      </h3>
      <VersusQuickPickOpponent />
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
// Composite widget: presents a focused dashboard block; reads shared Pinia stores and Tauri invoke where needed.

import { VersusQuickPickOpponent } from '@/features/VersusQuickPickOpponent';
import { useI18n } from '@/shared/lib/i18n';
import { computed } from 'vue';

const { t } = useI18n();

const benefits = computed(() => [
  {
    key: 'compare',
    icon: 'mdi-pentagon-outline',
    text: t('versusPage.emptyBenefitCompare'),
  },
  {
    key: 'metrics',
    icon: 'mdi-chart-box-outline',
    text: t('versusPage.emptyBenefitMetrics'),
  },
  {
    key: 'gameplan',
    icon: 'mdi-target',
    text: t('versusPage.emptyBenefitGameplan'),
  },
]);
</script>
