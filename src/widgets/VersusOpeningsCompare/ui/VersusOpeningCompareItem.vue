<template>
  <v-tooltip v-if="tooltipRows" location="top" max-width="400">
    <template #activator="{ props: tipProps }">
      <v-list-item v-bind="tipProps" :title="o.name">
        <template #append>
          <opening-stat-append :o="o" />
        </template>
      </v-list-item>
    </template>
    <div class="pa-1">
      <div
        v-for="(row, idx) in tooltipRows"
        :key="idx + '-' + row.label"
        class="d-flex justify-space-between align-start ga-3 text-caption"
      >
        <span class="flex-grow-1" style="word-break: break-word">{{ row.label }}</span>
        <span class="versus-opening-tip-pct">{{ row.pctRounded }}%</span>
      </div>
    </div>
  </v-tooltip>
  <v-list-item v-else :title="o.name">
    <template #append>
      <opening-stat-append :o="o" />
    </template>
  </v-list-item>
</template>

<script setup lang="ts">
import type { VersusOpeningCard } from '@/entities/versus';
import { useI18n } from '@/shared/lib/i18n';
import { computed } from 'vue';

import { buildOpeningLineTooltipRows } from '../lib/buildOpeningLineTooltipRows';
import OpeningStatAppend from './OpeningStatAppend.vue';

const props = defineProps<{
  o: VersusOpeningCard;
}>();

const { t } = useI18n();

const tooltipRows = computed(() =>
  buildOpeningLineTooltipRows(props.o.lines, t('versusPage.openingLinesOthers')),
);
</script>

<style scoped lang="scss">
.versus-opening-tip-pct {
  font-family: 'JetBrains Mono', ui-monospace, monospace;
  font-variant-numeric: tabular-nums;
  flex-shrink: 0;
}
</style>
