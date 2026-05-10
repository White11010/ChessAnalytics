<template>
  <v-card class="mb-4" :title="t('versusPage.title')">
    <v-card-subtitle class="my-4 px-0 text-body-2 text-medium-emphasis">
      {{ t('versusPage.subtitle') }}
    </v-card-subtitle>

    <v-card-text>
      <div class="d-flex flex-column flex-lg-row flex-wrap align-stretch align-lg-center ga-3">
        <v-text-field
          v-model="versusStore.opponentUsernameInput"
          :label="t('versusPage.usernameLabel')"
          density="compact"
          variant="outlined"
          hide-details
          :disabled="versusStore.loading"
          autocomplete="off"
          class="flex-grow-1"
          style="min-width: 200px"
          @keydown.enter.prevent="runCompare"
        />
        <div class="d-flex flex-wrap ga-2 flex-shrink-0">
          <v-btn
            color="secondary"
            :loading="versusStore.loading"
            :disabled="!versusStore.opponentUsernameInput.trim() || versusStore.loading"
            @click="runCompare"
          >
            {{ t('versusPage.compare') }}
          </v-btn>
          <v-btn v-if="versusStore.loading" variant="text" @click="versusStore.cancel()">
            {{ t('versusPage.cancel') }}
          </v-btn>
        </div>
      </div>
    </v-card-text>
  </v-card>

  <v-alert
    v-if="versusStore.errorMessage"
    type="error"
    variant="tonal"
    density="compact"
    class="mb-4"
    closable
    @click:close="versusStore.errorMessage = null"
  >
    {{
      versusStore.errorMessage.includes('RATE_LIMITED')
        ? t('versusPage.rateLimited')
        : versusStore.errorMessage
    }}
  </v-alert>

  <div v-if="versusStore.progress && versusStore.loading" class="mb-4">
    <p class="text-caption text-medium-emphasis mb-1">
      {{
        versusStore.progress.phase === 'analyze_opponent'
          ? t('versusPage.analyzingProgress', {
              current: versusStore.progress.current,
              total: versusStore.progress.total,
            })
          : t('versusPage.fetchingProgress')
      }}
    </p>
    <v-progress-linear
      height="6"
      rounded
      color="primary"
      :indeterminate="versusStore.progress.phase === 'fetch_opponent'"
      :model-value="
        versusStore.progress.phase !== 'fetch_opponent' && versusStore.progress.total > 0
          ? (100 * versusStore.progress.current) / versusStore.progress.total
          : undefined
      "
    />
    <p class="text-caption text-medium-emphasis mt-2 mb-0">
      {{ t('versusPage.fetchingMayTakeTimeHint') }}
    </p>
  </div>
</template>

<script setup lang="ts">
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { onMounted, onUnmounted } from 'vue';

import { useVersusStore } from '@/entities/versus';
import { useI18n } from '@/shared/lib/i18n';

const { t } = useI18n();
const versusStore = useVersusStore();

let unlistenVersus: UnlistenFn | null = null;

onMounted(async () => {
  unlistenVersus = await listen('versus://progress', (e) => {
    const p = e.payload as { phase?: string; current?: number; total?: number };
    if (
      p &&
      typeof p.phase === 'string' &&
      typeof p.current === 'number' &&
      typeof p.total === 'number'
    ) {
      versusStore.setProgress({ phase: p.phase, current: p.current, total: p.total });
    }
  });
});

onUnmounted(() => {
  unlistenVersus?.();
  unlistenVersus = null;
});

function runCompare(): void {
  void versusStore.compare();
}
</script>
