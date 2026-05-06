<template>
  <div ref="boardEl" :style="{ width: props.size, height: props.size }"></div>
</template>

<script setup lang="ts">
import { onMounted, ref, watch } from 'vue';
import { Chessground } from 'chessground';

import 'chessground/assets/chessground.base.css';
import 'chessground/assets/chessground.brown.css';

import type { Api } from 'chessground/api';
import { Key } from 'chessground/types';
import { Chess } from 'chess.js';

const props = withDefaults(
  defineProps<{
    fen: string;
    lastMove: [Key, Key];
    size?: string;
    orientation: 'white' | 'black';
    winner: 'white' | 'black';
  }>(),
  {
    size: '100%',
  },
);

const boardEl = ref<HTMLDivElement | null>(null);
let ground: Api | null = null;

onMounted(() => {
  if (!boardEl.value) return;

  const game = new Chess(props.fen);

  ground = Chessground(boardEl.value, {
    fen: props.fen,
    orientation: props.orientation,

    coordinates: false, // ✅ отключаем координаты

    movable: {
      free: false,
      color: undefined,
      dests: new Map(),
    },

    draggable: { enabled: false },
    selectable: { enabled: false },

    highlight: {
      lastMove: true,
      check: true, // ✅ подсветка шаха
    },

    drawable: {
      enabled: false,
      visible: false,
    },

    // ✅ подсветка последнего хода
    lastMove: props.lastMove,
  });

  highlightMate(game);
});

// ---------------- WATCH ----------------
watch(
  () => props.fen,
  (fen) => {
    if (!ground) return;

    const game = new Chess(fen);

    ground.set({
      fen,
      lastMove: props.lastMove,
    });

    highlightMate(game);
  },
);

// ---------------- CHECKMATE HIGHLIGHT ----------------
function highlightMate(game: Chess) {
  if (!ground) return;

  console.log(game.isCheckmate());

  if (game.isCheckmate()) {
    ground.set({
      check: props.winner === 'white' ? 'black' : 'white',
    });
  }
}
</script>

<style scoped>
.board {
  width: 100%;
  height: 200px;
}
</style>
