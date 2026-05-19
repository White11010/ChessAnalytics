<template>
  <div class="game-details-sidebar d-flex flex-column ga-5 h-100">
    <v-card class="board-card">
      <v-card-text class="pa-0 d-flex align-center justify-center">
        <ChessStaticBoard
          v-if="canShowBoard"
          :fen="game.last_fen!"
          :last-move="lastMove"
          :orientation="game.player_color"
          :winner="game.winner"
          size="100%"
        />
        <v-sheet
          v-else
          rounded="lg"
          class="board-placeholder d-flex flex-column align-center justify-center ga-2 text-center px-4"
        >
          <v-icon icon="mdi-chess-king" color="secondary" size="40" />
          <span class="text-body-2 text-medium-emphasis">{{ t('gameDetails.boardEmpty') }}</span>
        </v-sheet>
      </v-card-text>
    </v-card>

    <v-card class="info-card flex-grow-1 d-flex flex-column">
      <v-card-text class="d-flex flex-column ga-3 flex-grow-1">
        <div class="info-block">
          <div class="info-label text-caption text-uppercase font-weight-bold text-medium-emphasis">
            {{ t('gameDetails.listOpponent') }}
          </div>
          <div class="text-body-1 font-weight-semibold text-secondary">
            {{ game.opponent_name }}
          </div>
        </div>

        <v-divider />

        <div class="info-block">
          <div class="info-label text-caption text-uppercase font-weight-bold text-medium-emphasis">
            {{ t('gameDetails.listResult') }}
          </div>
          <div class="text-body-1 font-weight-semibold" :class="resultClass">{{ resultLabel }}</div>
        </div>

        <v-divider />

        <div class="info-block">
          <div class="info-label text-caption text-uppercase font-weight-bold text-medium-emphasis">
            {{ t('gameDetails.listOpening') }}
          </div>
          <div class="text-body-2 font-weight-medium">
            {{ game.opening_name || t('gameDetails.unknownOpening') }}
          </div>
        </div>

        <v-divider />

        <div class="info-block">
          <div class="info-label text-caption text-uppercase font-weight-bold text-medium-emphasis">
            {{ t('gameDetails.listTimeControl') }}
          </div>
          <div class="text-body-2 font-weight-medium">
            {{ timeControlLabel }}
          </div>
        </div>

        <v-divider />

        <div class="info-block">
          <div class="info-label text-caption text-uppercase font-weight-bold text-medium-emphasis">
            {{ t('gameDetails.listColor') }}
          </div>
          <div class="text-body-2 font-weight-medium">{{ colorLabel }}</div>
        </div>

        <v-divider />

        <div class="info-block mt-auto">
          <div class="info-label text-caption text-uppercase font-weight-bold text-medium-emphasis">
            {{ t('gameDetails.playedAt') }}
          </div>
          <div class="text-body-2 text-medium-emphasis">{{ playedAtLabel }}</div>
        </div>
      </v-card-text>
    </v-card>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { Key } from 'chessground/types';

import type { Game } from '@/entities/game';
import { formatTimestamp } from '@/shared/lib/dates';
import { useI18n } from '@/shared/lib/i18n';
import { ChessStaticBoard } from '@/shared/ui';

const props = defineProps<{
  game: Game;
}>();

const { t, te, locale } = useI18n();

const canShowBoard = computed(() => Boolean(props.game.last_fen && props.game.moves));

const lastMove = computed<[Key, Key]>(() => {
  const tokens = (props.game.moves || '').trim().split(/\s+/).filter(Boolean);
  const lastToken = tokens[tokens.length - 1] || 'e2e4';
  const from = lastToken.slice(0, 2) as Key;
  const to = lastToken.slice(2, 4) as Key;
  return [from, to];
});

const resultLabel = computed(() => {
  const r = props.game.player_result;
  if (r === 'win') {
    return t('game.resultWin');
  }
  if (r === 'loss') {
    return t('game.resultLoss');
  }
  return t('game.resultDraw');
});

const resultClass = computed(() => {
  const r = props.game.player_result;
  if (r === 'win') {
    return 'text-success';
  }
  if (r === 'loss') {
    return 'text-error';
  }
  return '';
});

const colorLabel = computed(() =>
  props.game.player_color === 'white' ? t('gameDetails.colorWhite') : t('gameDetails.colorBlack'),
);

const timeControlLabel = computed(() => {
  const speedKey = `myGames.speed.${props.game.speed.toLowerCase()}`;
  const speedLabel = te(speedKey) ? t(speedKey) : props.game.speed;
  return `${speedLabel} · ${props.game.time_control}`;
});

const playedAtLabel = computed(() =>
  formatTimestamp(props.game.created_at, { locale: locale.value }),
);
</script>

<style scoped lang="scss">
.board-card {
  width: 100%;
  max-width: 380px;
}

.board-card :deep(.v-card-text) {
  aspect-ratio: 1;
  width: 100%;
}

.board-placeholder {
  width: 100%;
  aspect-ratio: 1;
}

.info-label {
  letter-spacing: 0.06em;
  margin-bottom: 4px;
}
</style>
