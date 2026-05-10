<template>
  <v-card class="mb-4">
    <v-card-title>{{ t('versusPage.sectionMetrics') }}</v-card-title>
    <v-card-text class="pa-0">
      <v-table density="comfortable" hover class="versus-table rounded-b-xl">
        <thead>
          <tr>
            <th class="text-left text-caption text-medium-emphasis text-uppercase">
              {{ t('versusPage.colMetric') }}
            </th>
            <th
              class="text-center text-caption text-medium-emphasis text-uppercase text-primary"
            >
              {{ t('versusPage.colYou') }}
            </th>
            <th
              class="text-center text-caption text-medium-emphasis text-uppercase text-warning"
            >
              {{ t('versusPage.colOpp', { name: slice.opponentSide.username }) }}
            </th>
            <th class="text-caption text-medium-emphasis text-uppercase" />
          </tr>
        </thead>
        <tbody>
          <tr v-for="m in metricTable" :key="m.key">
            <td class="text-body-2 text-medium-emphasis">{{ m.label }}</td>
            <td
              class="text-center versus-mono"
              :class="m.win === 'you' ? 'text-success font-weight-bold' : 'text-primary'"
            >
              {{ m.youFmt }}
            </td>
            <td
              class="text-center versus-mono"
              :class="m.win === 'opp' ? 'text-success font-weight-bold' : 'text-warning'"
            >
              {{ m.oppFmt }}
            </td>
            <td class="text-center">
              <v-chip
                v-if="m.win === 'you'"
                size="x-small"
                color="primary"
                variant="tonal"
              >
                {{ t('versusPage.verdictYou') }}
              </v-chip>
              <v-chip
                v-else-if="m.win === 'opp'"
                size="x-small"
                color="warning"
                variant="tonal"
              >
                {{ t('versusPage.verdictOpp', { name: slice.opponentSide.username }) }}
              </v-chip>
              <v-chip
                v-else-if="m.win === 'tie'"
                size="x-small"
                color="secondary"
                variant="tonal"
              >
                {{ t('versusPage.verdictTie') }}
              </v-chip>
            </td>
          </tr>
        </tbody>
      </v-table>
    </v-card-text>
  </v-card>
</template>

<script setup lang="ts">
import type { VersusSpeedSlice } from '@/entities/versus';
import { buildVersusMetricTable } from '@/entities/versus/lib/versusMetrics';
import { useI18n } from '@/shared/lib/i18n';
import { computed } from 'vue';

const props = defineProps<{
  slice: VersusSpeedSlice;
  speedLabel: string;
}>();

const { t } = useI18n();

const metricTable = computed(() =>
  buildVersusMetricTable(
    props.slice.selfSide,
    props.slice.opponentSide,
    props.speedLabel,
    t,
  ),
);
</script>

<style scoped lang="scss">
.versus-mono {
  font-family: 'JetBrains Mono', ui-monospace, monospace;
  font-variant-numeric: tabular-nums;
}

.versus-table :deep(th),
.versus-table :deep(td) {
  vertical-align: middle;
}

.versus-table :deep(thead th) {
  border-bottom: thin solid rgba(var(--v-border-color), var(--v-border-opacity));
}
</style>
