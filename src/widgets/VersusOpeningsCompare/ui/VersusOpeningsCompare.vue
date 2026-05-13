<template>
  <div>
    <h3 class="text-overline text-medium-emphasis mb-2">{{ t('versusPage.openingPrefs') }}</h3>
    <v-row class="mb-6">
      <v-col cols="12" md="6">
        <v-card class="h-100">
          <v-card-title class="text-subtitle-2 text-info pb-0">
            {{ t('versusPage.colYou') }}
          </v-card-title>
          <v-divider class="mt-3" />
          <v-card-text class="pt-2 pb-2">
            <div class="text-caption font-weight-medium text-medium-emphasis mb-1">
              {{ t('versusPage.gpWhite') }}:
            </div>
            <v-list density="compact" class="py-0">
              <v-list-item
                v-for="(o, i) in slice.selfSide.openingsAsWhite"
                :key="'sw-' + i + '-' + o.name"
                :title="o.name"
              >
                <template #append>
                  <opening-stat-append :o="o" />
                </template>
              </v-list-item>
              <v-list-item v-if="!slice.selfSide.openingsAsWhite.length">
                <v-list-item-title class="text-caption text-medium-emphasis">—</v-list-item-title>
              </v-list-item>
            </v-list>
            <div class="text-caption font-weight-medium text-medium-emphasis mb-1 mt-3">
              {{ t('versusPage.gpBlack') }}:
            </div>
            <v-list density="compact" class="py-0">
              <v-list-item
                v-for="(o, i) in slice.selfSide.openingsAsBlack"
                :key="'sb-' + i + '-' + o.name"
                :title="o.name"
              >
                <template #append>
                  <opening-stat-append :o="o" />
                </template>
              </v-list-item>
              <v-list-item v-if="!slice.selfSide.openingsAsBlack.length">
                <v-list-item-title class="text-caption text-medium-emphasis">—</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-card-text>
        </v-card>
      </v-col>
      <v-col cols="12" md="6">
        <v-card class="h-100">
          <v-card-title class="text-subtitle-2 text-warning pb-0">
            {{ slice.opponentSide.username }}
          </v-card-title>
          <v-divider class="mt-3" />
          <v-card-text class="pt-2 pb-2">
            <div class="text-caption font-weight-medium text-medium-emphasis mb-1">
              {{ t('versusPage.gpWhite') }}:
            </div>
            <v-list density="compact" class="py-0">
              <v-list-item
                v-for="(o, i) in slice.opponentSide.openingsAsWhite"
                :key="'ow-' + i + '-' + o.name"
                :title="o.name"
              >
                <template #append>
                  <opening-stat-append :o="o" />
                </template>
              </v-list-item>
              <v-list-item v-if="!slice.opponentSide.openingsAsWhite.length">
                <v-list-item-title class="text-caption text-medium-emphasis">—</v-list-item-title>
              </v-list-item>
            </v-list>
            <div class="text-caption font-weight-medium text-medium-emphasis mb-1 mt-3">
              {{ t('versusPage.gpBlack') }}:
            </div>
            <v-list density="compact" class="py-0">
              <v-list-item
                v-for="(o, i) in slice.opponentSide.openingsAsBlack"
                :key="'ob-' + i + '-' + o.name"
                :title="o.name"
              >
                <template #append>
                  <opening-stat-append :o="o" />
                </template>
              </v-list-item>
              <v-list-item v-if="!slice.opponentSide.openingsAsBlack.length">
                <v-list-item-title class="text-caption text-medium-emphasis">—</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>
  </div>
</template>

<script setup lang="ts">
// Composite widget: presents a focused dashboard block; reads shared Pinia stores and Tauri invoke where needed.

import type { VersusSpeedSlice } from '@/entities/versus';
import { useI18n } from '@/shared/lib/i18n';

import OpeningStatAppend from './OpeningStatAppend.vue';

defineProps<{
  slice: VersusSpeedSlice;
}>();

const { t } = useI18n();
</script>
