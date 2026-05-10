<template>
  <v-container ref="dashboardRootRef" fluid>
    <home-greeting-block />
    <v-row dense class="mt-2">
      <v-col ref="leftColRef" cols="12" lg="6">
        <home-profile-chart-block />
      </v-col>
      <v-col ref="rightColRef" cols="12" lg="6">
        <div class="d-flex flex-column ga-2">
          <home-top-insight-block />
          <home-last-game />
        </div>
      </v-col>
    </v-row>
    <v-row dense class="mt-2">
      <v-col cols="12" lg="6">
        <home-rate-chart-block />
      </v-col>
      <v-col cols="12" lg="6">
        <home-recent-games-block />
      </v-col>
    </v-row>
  </v-container>
</template>
<script setup lang="ts">
import { nextTick, onBeforeUnmount, onMounted, ref } from 'vue';
import { useDisplay } from 'vuetify';

import { HomeGreetingBlock } from '@/features/HomeGreetingBlock';
import { HomeLastGame } from '@/features/HomeLastGame';
import { HomeProfileChartBlock } from '@/features/HomeProfileChartBlock';
import { HomeRateChartBlock } from '@/features/HomeRateChartBlock';
import { HomeRecentGamesBlock } from '@/features/HomeRecentGamesBlock';
import { HomeTopInsightBlock } from '@/features/HomeTopInsightBlock';

type MaybeWithEl = { $el?: Element | null };
const dashboardRootRef = ref<HTMLElement | MaybeWithEl | null>(null);
const leftColRef = ref<HTMLElement | MaybeWithEl | null>(null);
const rightColRef = ref<HTMLElement | MaybeWithEl | null>(null);

const display = useDisplay();

function resolveElement(target: HTMLElement | MaybeWithEl | null): HTMLElement | null {
  if (!target) return null;
  if (target instanceof HTMLElement) return target;
  const el = target.$el;
  return el instanceof HTMLElement ? el : null;
}

function logHomeDashboardLayout(runId: string, reason: string): void {
  const dashboardEl = resolveElement(dashboardRootRef.value);
  const leftEl = resolveElement(leftColRef.value);
  const rightEl = resolveElement(rightColRef.value);
  const leftStyle = leftEl ? getComputedStyle(leftEl) : null;
  const rightStyle = rightEl ? getComputedStyle(rightEl) : null;
  // #region agent log
  fetch('http://127.0.0.1:7908/ingest/33c37ce5-42b3-43bc-a8f9-a7b804f9e401', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'X-Debug-Session-Id': '71c291',
    },
    body: JSON.stringify({
      sessionId: '71c291',
      runId,
      hypothesisId: 'H_LAYOUT_BREAKPOINT_OR_WIDTH',
      location: 'src/widgets/HomeDashboard/ui/HomeDashboard.vue:script',
      message: 'Home dashboard layout probe',
      data: {
        reason,
        viewportWidth: window.innerWidth,
        viewportHeight: window.innerHeight,
        displayName: display.name.value,
        lgAndUp: display.lgAndUp.value,
        mdAndUp: display.mdAndUp.value,
        dashboardWidth: dashboardEl?.clientWidth ?? null,
        leftColWidth: leftEl?.clientWidth ?? null,
        rightColWidth: rightEl?.clientWidth ?? null,
        leftFlexBasis: leftStyle?.flexBasis ?? null,
        rightFlexBasis: rightStyle?.flexBasis ?? null,
        leftMaxWidth: leftStyle?.maxWidth ?? null,
        rightMaxWidth: rightStyle?.maxWidth ?? null,
      },
      timestamp: Date.now(),
    }),
  }).catch(() => {});
  // #endregion
}

function onResize(): void {
  logHomeDashboardLayout('run-before-fix', 'window-resize');
}

onMounted(async () => {
  await nextTick();
  logHomeDashboardLayout('run-before-fix', 'mounted-after-next-tick');
  window.addEventListener('resize', onResize);
});

onBeforeUnmount(() => {
  window.removeEventListener('resize', onResize);
});
</script>
