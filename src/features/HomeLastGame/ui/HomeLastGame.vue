<template>
  <v-card title="Last game">
    <template v-if="lastGame">
      <v-card-text>
        <v-row>
          <v-col cols="5" class="d-flex pa-2">
            <ChessStaticBoard
              :fen="lastGame.last_fen!"
              size="256px"
              :last-move="getLastMoveFromMoves(lastGame.moves!)"
              :orientation="lastGame.player_color"
              :winner="lastGame.winner!"
            />
          </v-col>
          <v-col cols="7">
            <v-list density="compact">
              <v-list-item>
                <v-list-item-title> vs {{ lastGame.opponent_name }} </v-list-item-title>
                <v-list-item-subtitle>
                  Rating: {{ lastGame.opponent_rating }}
                </v-list-item-subtitle>
              </v-list-item>

              <v-list-item>
                <v-list-item-title> Result: {{ lastGame.player_result }} </v-list-item-title>
              </v-list-item>

              <v-list-item>
                <v-list-item-title> Color: {{ lastGame.player_color }} </v-list-item-title>
              </v-list-item>

              <v-list-item>
                <v-list-item-title> Opening: {{ lastGame.opening_name }} </v-list-item-title>
              </v-list-item>
            </v-list>
          </v-col>
        </v-row>
      </v-card-text>

      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn
          class="button-width-12"
          color="secondary"
          variant="elevated"
          @click="onAnalizeButtonClick(lastGame)"
        >
          Analize
        </v-btn>
      </v-card-actions>
    </template>

    <v-skeleton-loader v-else type="list-item-two-line" />
  </v-card>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useRouter } from 'vue-router';

import { Game, useSyncGamesQuery } from '@/entities/game';
import { useBoardStore } from '@/entities/board';
import { ChessStaticBoard } from '@/shared/ui';
import { Chess } from 'chess.js';
import { Key } from 'chessground/types';

const router = useRouter();
const boardStore = useBoardStore();

const { games } = useSyncGamesQuery();

const lastGame = computed(() => games.value[2]);

function onAnalizeButtonClick(game: Game): void {
  if (!game.pgn) return;

  boardStore.loadPgn(game.pgn);

  router.push('/analize-board');
}

function getLastMoveFromMoves(moves: string): [Key, Key] {
  const game = new Chess();

  const movesArr = moves.split(' ').filter(Boolean);

  let lastMove = null;

  for (const san of movesArr) {
    const move = game.move(san);
    lastMove = move;
  }

  return [lastMove!.from, lastMove!.to];
}
</script>
