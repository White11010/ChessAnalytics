<template>
  <v-card
    variant="outlined"
    density="compact"
    class="versus-game-plan-card pa-3 h-100 d-flex flex-column"
  >
    <div class="text-body-2 font-weight-medium flex-shrink-0">{{ entry.title }}</div>
    <div class="versus-game-plan-card__body text-caption text-medium-emphasis flex-grow-1 d-flex align-center">
      {{ reasonText(entry) }}
    </div>
  </v-card>
</template>

<script setup lang="ts">
import type { VersusPlanEntry, VersusPlanReasonKind } from '@/entities/versus';
import { useI18n } from '@/shared/lib/i18n';

defineProps<{
  entry: VersusPlanEntry;
}>();

const { t } = useI18n();

function reasonText(entry: VersusPlanEntry): string {
  const key = `versusPage.gpReason.${entry.reasonKind}` as `versusPage.gpReason.${VersusPlanReasonKind}`;
  return t(key, entry.reasonParams);
}
</script>

<style scoped>
.versus-game-plan-card {
  min-height: 4.5rem;
}

.versus-game-plan-card__body {
  min-height: 0;
}
</style>
