<template>
  <div>
    <h3 class="text-overline text-medium-emphasis mb-2">{{ t('versusPage.conclusion') }}</h3>
    <v-row class="mb-6">
      <v-col cols="12" md="6">
        <v-card variant="outlined" class="h-100 versus-conc-you">
          <v-card-title class="d-flex align-center ga-2 text-subtitle-1">
            <v-icon icon="mdi-circle" size="12" color="primary" />
            {{ slice.selfSide.username }}
          </v-card-title>
          <v-card-text>
            <div
              v-for="(line, i) in conclusionsYou"
              :key="'y' + i"
              class="d-flex align-start ga-2 py-2 border-b-sm text-medium-emphasis"
            >
              <v-icon
                :icon="line.good ? 'mdi-arrow-up-bold' : 'mdi-arrow-down-bold'"
                size="18"
                :color="line.good ? 'success' : 'error'"
                class="flex-shrink-0 mt-0"
              />
              <span class="text-body-2">{{ line.text }}</span>
            </div>
            <p v-if="!conclusionsYou.length" class="text-caption text-medium-emphasis mb-0">
              {{ t('versusPage.conclusionsFromData') }}
            </p>
          </v-card-text>
        </v-card>
      </v-col>
      <v-col cols="12" md="6">
        <v-card variant="outlined" class="h-100 versus-conc-opp">
          <v-card-title class="d-flex align-center ga-2 text-subtitle-1">
            <v-icon icon="mdi-circle" size="12" color="warning" />
            {{ slice.opponentSide.username }}
          </v-card-title>
          <v-card-text>
            <div
              v-for="(line, i) in conclusionsOpp"
              :key="'o' + i"
              class="d-flex align-start ga-2 py-2 border-b-sm"
            >
              <v-icon
                :icon="line.good ? 'mdi-arrow-up-bold' : 'mdi-arrow-down-bold'"
                size="18"
                :color="line.good ? 'success' : 'error'"
                class="flex-shrink-0 mt-0"
              />
              <span class="text-body-2">{{ line.text }}</span>
            </div>
            <p v-if="!conclusionsOpp.length" class="text-caption text-medium-emphasis mb-0">
              {{ t('versusPage.conclusionsFromData') }}
            </p>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>
  </div>
</template>

<script setup lang="ts">
import type { VersusSpeedSlice } from '@/entities/versus';
import {
  buildConclusionsOpp,
  buildConclusionsYou,
} from '@/entities/versus/lib/versusConclusions';
import { useI18n } from '@/shared/lib/i18n';
import { computed } from 'vue';

const props = defineProps<{
  slice: VersusSpeedSlice;
}>();

const { t } = useI18n();

const conclusionsYou = computed(() =>
  buildConclusionsYou(props.slice.selfSide, props.slice.opponentSide, t),
);
const conclusionsOpp = computed(() =>
  buildConclusionsOpp(props.slice.selfSide, props.slice.opponentSide, t),
);
</script>

<style scoped lang="scss">
.border-b-sm:not(:last-child) {
  border-bottom: thin solid rgba(var(--v-theme-on-surface), 0.08);
}

.versus-conc-you {
  border-top: 3px solid rgb(var(--v-theme-primary)) !important;
}

.versus-conc-opp {
  border-top: 3px solid rgb(var(--v-theme-warning)) !important;
}
</style>
