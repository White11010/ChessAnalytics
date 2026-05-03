<template>
  <v-card
    v-if="appStore.status === 'ready' && userStore.user === null"
    title="Connect your chess account"
    subtitle="Import your games from Lichess to start querying"
  >
    <v-card-text>
      <form class="d-flex ga-4 mt-4" @submit.prevent="onSubmit">
        <v-text-field
          v-model="tokenInput"
          density="comfortable"
          variant="outlined"
          hide-details
          :loading="isLoading"
        ></v-text-field>
        <v-btn type="submit" class="button--medium" size="large">Connect with Lichess</v-btn>
      </form>
    </v-card-text>
  </v-card>
  <p v-if="appStore.status === 'loading'">Loading</p>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';

import { useAppStore } from '@/entities/app';
import type { User } from '@/entities/user';
import { useUserStore } from '@/entities/user';

const isLoading = ref(false);
const userStore = useUserStore();
const appStore = useAppStore();

const tokenInput = ref<null | string>(null);
async function saveToken(): Promise<void> {
  isLoading.value = true;
  try {
    // Вызываем Rust-команду и передаем ей имя пользователя
    await invoke('save_token', { token: tokenInput.value });
    const user = await invoke('sync_me');
    userStore.setUser(user as User);
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
