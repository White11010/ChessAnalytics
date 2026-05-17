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
            <th class="text-center text-caption text-medium-emphasis text-uppercase text-info">
              {{ t('versusPage.colYou') }}
            </th>
            <th class="text-center text-caption text-medium-emphasis text-uppercase text-warning">
              {{ t('versusPage.colOpp', { name: slice.opponentSide.username }) }}
            </th>
            <th class="text-center text-caption text-medium-emphasis text-uppercase" aria-hidden="true">
              <v-icon icon="mdi-trophy" size="small" />
            </th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="m in metricTable" :key="m.key">
            <td class="text-body-2 text-medium-emphasis">{{ m.label }}</td>
            <td class="text-center versus-mono" :class="youCellClass(m.win)">
              {{ m.youFmt }}
            </td>
            <td class="text-center versus-mono" :class="oppCellClass(m.win)">
              {{ m.oppFmt }}
            </td>
            <td class="text-center">
              <div
                v-if="m.win === 'you'"
                class="d-inline-flex align-center justify-center ga-1 flex-wrap"
              >
                <v-icon icon="mdi-trophy" size="small" color="info" aria-hidden="true" />
                <v-chip size="small" color="info" variant="tonal">
                  {{ t('versusPage.verdictYou') }}
                </v-chip>
              </div>
              <div
                v-else-if="m.win === 'opp'"
                class="d-inline-flex align-center justify-center ga-1 flex-wrap"
              >
                <v-icon icon="mdi-trophy" size="small" color="warning" aria-hidden="true" />
                <v-chip size="small" color="warning" variant="tonal">
                  {{ t('versusPage.verdictOpp', { name: slice.opponentSide.username }) }}
                </v-chip>
              </div>
              <v-chip v-else-if="m.win === 'tie'" size="small" color="secondary" variant="tonal">
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
// Composite widget: presents a focused dashboard block; reads shared Pinia stores and Tauri invoke where needed.

import type { VersusSpeedSlice } from '@/entities/versus';
import { buildVersusMetricTable, type WinCell } from '@/entities/versus/lib/versusMetrics';
import { useI18n } from '@/shared/lib/i18n';
import { computed } from 'vue';

const props = defineProps<{
  slice: VersusSpeedSlice;
  speedLabel: string;
}>();

const { t } = useI18n();

const metricTable = computed(() =>
  buildVersusMetricTable(props.slice.selfSide, props.slice.opponentSide, props.speedLabel, t),
);

function youCellClass(win: WinCell): string {
  if (win === 'you') return 'text-success font-weight-bold';
  if (win === 'tie') return 'text-success';
  if (win === 'opp') return 'text-error';
  return '';
}

function oppCellClass(win: WinCell): string {
  if (win === 'opp') return 'text-success font-weight-bold';
  if (win === 'tie') return 'text-success';
  if (win === 'you') return 'text-error';
  return '';
}
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
