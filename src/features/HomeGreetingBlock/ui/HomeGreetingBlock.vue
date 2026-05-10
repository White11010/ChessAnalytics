<template>
  <v-card v-if="user" :title="t('home.greetingTitle')">
    <div class="d-flex align-center flex-wrap ga-2">
      <span>{{ t('home.greetingConnected', { username: user.username }) }}</span>
      <span class="text-body-2 text-medium-emphasis">
        · {{ updatedLabel }}
      </span>
      <v-tooltip :text="t('gamesSync.hoverRefresh')" location="bottom">
        <template #activator="{ props: tipProps }">
          <v-btn
            v-bind="tipProps"
            icon="mdi-refresh"
            variant="text"
            size="small"
            density="comfortable"
            :loading="syncStore.isSyncing"
            :disabled="!syncStore.canManualSync()"
            :aria-label="String(t('gamesSync.hoverRefresh'))"
            @click="onRefresh"
          />
        </template>
      </v-tooltip>
    </div>
  </v-card>
</template>

<script setup lang="ts">
import { useGamesSyncStore } from '@/entities/games-sync';
import { formatLastGamesSyncLabel } from '@/entities/games-sync/lib/formatLastGamesSync';
import { useGetUserQuery } from '@/entities/user';
import { useI18n } from '@/shared/lib/i18n';
import { storeToRefs } from 'pinia';
import { computed, onMounted, onUnmounted, ref } from 'vue';

const { t } = useI18n();
const { user } = useGetUserQuery();
const syncStore = useGamesSyncStore();
const { lastSyncedAt } = storeToRefs(syncStore);

const nowTick = ref(Date.now());
let tickId: ReturnType<typeof setInterval> | undefined;

onMounted(() => {
  tickId = setInterval(() => {
    nowTick.value = Date.now();
  }, 30_000);
});

onUnmounted(() => {
  if (tickId != null) {
    clearInterval(tickId);
  }
});

const updatedLabel = computed(() =>
  formatLastGamesSyncLabel(t, lastSyncedAt.value, nowTick.value),
);

function onRefresh(): void {
  void syncStore.manualSync();
}
</script>
