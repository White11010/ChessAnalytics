<template>
  <div class="versus-page">
    <v-container class="versus-page__inner pa-4 pa-md-6" fluid>
      <div class="mx-auto versus-page__max">
        <VersusCompareForm />

        <template v-if="versusStore.result && activeSlice">
          <VersusSpeedFilter />
          <VersusHeadToHeadCards :slice="activeSlice" :speed-label="speedLabel" />
          <VersusProfilePentagon :slice="activeSlice" />
          <VersusDiagnosticsAlerts
            :slice="activeSlice"
            :opponent-games-in-api-sample="versusStore.result.opponentGamesInApiSample"
            :speed-label="speedLabel"
          />
          <VersusMetricsTable :slice="activeSlice" :speed-label="speedLabel" />
          <VersusOpeningsCompare :slice="activeSlice" />
          <VersusConclusionsBlock :slice="activeSlice" />
          <VersusGamePlanSection :slice="activeSlice" />
        </template>
      </div>
    </v-container>
  </div>
</template>

<script setup lang="ts">
import { VersusCompareForm } from '@/features/VersusCompareForm';
import { VersusSpeedFilter } from '@/features/VersusSpeedFilter';
import { useVersusStore } from '@/entities/versus';
import { useI18n } from '@/shared/lib/i18n';
import { VersusConclusionsBlock } from '@/widgets/VersusConclusionsBlock';
import { VersusDiagnosticsAlerts } from '@/widgets/VersusDiagnosticsAlerts';
import { VersusGamePlanSection } from '@/widgets/VersusGamePlanSection';
import { VersusHeadToHeadCards } from '@/widgets/VersusHeadToHeadCards';
import { VersusMetricsTable } from '@/widgets/VersusMetricsTable';
import { VersusOpeningsCompare } from '@/widgets/VersusOpeningsCompare';
import { VersusProfilePentagon } from '@/widgets/VersusProfilePentagon';
import { storeToRefs } from 'pinia';
import { computed } from 'vue';

const { t } = useI18n();
const versusStore = useVersusStore();
const { activeSlice } = storeToRefs(versusStore);

const speedLabel = computed(() => {
  const s = versusStore.selectedSpeed;
  if (s === 'bullet') return t('myGames.speed.bullet');
  if (s === 'rapid') return t('myGames.speed.rapid');
  return t('myGames.speed.blitz');
});
</script>

<style scoped lang="scss">
.versus-page {
  align-self: stretch;
  width: 100%;
  flex: 1;
  min-height: 100%;
  overflow-y: auto;
}

.versus-page__max {
  max-width: 58rem;
}
</style>
