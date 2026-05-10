<template>
  <v-list-item
    class="sidebar-games-sync px-4 py-2"
    density="comfortable"
    :disabled="listItemDisabled"
    @click="onClick"
  >
    <template #prepend>
      <v-progress-circular v-if="syncStore.isSyncing" indeterminate size="22" width="2" />
      <v-icon v-else-if="syncStore.phase === 'error'" icon="mdi-alert-circle" color="error" />
      <v-icon v-else icon="mdi-refresh" :color="iconColor" />
    </template>
    <v-list-item-title class="text-body-2 text-medium-emphasis">
      {{ primaryLine }}
    </v-list-item-title>
    <v-list-item-subtitle v-if="subtitleLine" class="text-caption">
      {{ subtitleLine }}
    </v-list-item-subtitle>
    <v-tooltip v-if="tooltipText" activator="parent" location="end">
      {{ tooltipText }}
    </v-tooltip>
  </v-list-item>
</template>

<script setup lang="ts">
import { useGamesSyncStore } from '@/entities/games-sync';
import { formatLastGamesSyncLabel } from '@/entities/games-sync/lib/formatLastGamesSync';
import { useI18n } from '@/shared/lib/i18n';
import { storeToRefs } from 'pinia';
import { computed, onMounted, onUnmounted, ref } from 'vue';

const { t } = useI18n();
const syncStore = useGamesSyncStore();
const { phase, lastSyncedAt } = storeToRefs(syncStore);

const nowTick = ref(Date.now());
let tickId: ReturnType<typeof setInterval> | undefined;

onMounted(() => {
  tickId = setInterval(() => {
    nowTick.value = Date.now();
  }, 1000);
});

onUnmounted(() => {
  if (tickId != null) {
    clearInterval(tickId);
  }
});

const updatedLabel = computed(() =>
  formatLastGamesSyncLabel(t, lastSyncedAt.value, nowTick.value),
);

const primaryLine = computed(() => {
  if (syncStore.isSyncing) {
    return t('gamesSync.syncing');
  }
  if (phase.value === 'error') {
    return t('gamesSync.errorRetry');
  }
  if (phase.value === 'rate_limited' && syncStore.rateLimitRemainingSec() > 0) {
    return t('gamesSync.rateLimited');
  }
  return updatedLabel.value;
});

const subtitleLine = computed(() => {
  if (
    syncStore.isSyncing ||
    phase.value === 'error' ||
    (phase.value === 'rate_limited' && syncStore.rateLimitRemainingSec() > 0)
  ) {
    return '';
  }
  const cd = syncStore.manualCooldownRemainingSec();
  if (cd > 0) {
    return String(t('gamesSync.cooldownWait', { s: cd }));
  }
  const rl = syncStore.rateLimitRemainingSec();
  if (rl > 0) {
    return String(t('gamesSync.cooldownWait', { s: rl }));
  }
  return '';
});

const listItemDisabled = computed(() => {
  if (syncStore.isSyncing) {
    return true;
  }
  if (syncStore.rateLimitRemainingSec() > 0) {
    return true;
  }
  if (phase.value === 'error') {
    return false;
  }
  if (!syncStore.canManualSync()) {
    return true;
  }
  return false;
});

const iconColor = computed(() => {
  if (phase.value === 'error') {
    return 'error';
  }
  return undefined;
});

const tooltipText = computed(() => String(t('gamesSync.hoverRefresh')));

function onClick(): void {
  if (listItemDisabled.value) {
    return;
  }
  if (phase.value === 'error') {
    syncStore.clearTransientError();
  }
  void syncStore.manualSync();
}
</script>
