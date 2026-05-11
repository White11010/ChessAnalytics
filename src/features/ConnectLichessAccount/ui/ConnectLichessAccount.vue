<template>
  <v-card
    v-if="appStore.status === 'ready' && userStore.user === null"
    :title="t('home.connectTitle')"
    :subtitle="t('home.connectSubtitle')"
  >
    <v-card-text>
      <form class="d-flex ga-4 mt-4" @submit.prevent="onSubmit">
        <v-text-field
          v-model="tokenInput"
          density="comfortable"
          variant="outlined"
          hide-details
          :label="t('home.connectTokenLabel')"
          :loading="isLoading"
        />
        <v-btn type="submit" class="button--medium" size="large">{{ t('home.connectButton') }}</v-btn>
      </form>
    </v-card-text>
  </v-card>
  <p v-if="appStore.status === 'loading'">{{ t('home.loading') }}</p>
</template>

<script setup lang="ts">
// Feature slice: encapsulates one user flow or form; parent pages/widgets compose it and pass props/events.

import { invoke } from '@tauri-apps/api/core';
import { useQueryClient } from '@tanstack/vue-query';
import { ref } from 'vue';

import { useAppStore } from '@/entities/app';
import { useGamesSyncStore } from '@/entities/games-sync';
import type { User } from '@/entities/user';
import { useUserStore } from '@/entities/user';
import { useI18n } from '@/shared/lib/i18n';

const { t } = useI18n();
const queryClient = useQueryClient();

const isLoading = ref(false);
const userStore = useUserStore();
const appStore = useAppStore();
const gamesSyncStore = useGamesSyncStore();

const tokenInput = ref<null | string>(null);
async function saveToken(): Promise<void> {
  isLoading.value = true;
  try {
    await invoke('save_token', { token: tokenInput.value });
    const user = await invoke('sync_me');
    userStore.setUser(user as User);
    gamesSyncStore.hydrateFromUser((user as User).last_sync_completed_at_ms ?? null);
    void queryClient.invalidateQueries({ queryKey: ['user'] });
  } finally {
    isLoading.value = false;
  }
}

function onSubmit(): void {
  if (tokenInput.value) {
    saveToken();
  }
}

if (!userStore.user) {
  appStore.bootstrap();
}
</script>

<style scoped></style>
