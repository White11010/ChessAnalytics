<template>
  <div class="layout">
    <BoardEvaluationBar :eval-cp="evalCp" />

    <div ref="boardEl" class="board" @click="restoreArrow"></div>

    <BoardMovesList />
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref, watch } from 'vue';
import { Chessground } from 'chessground';
import { invoke } from '@tauri-apps/api/core';

import 'chessground/assets/chessground.base.css';
import 'chessground/assets/chessground.brown.css';

import type { Api } from 'chessground/api';
import type { Config } from 'chessground/config';
import type { Key } from 'chessground/types';

import { BoardMovesList } from '@/features/BoardMovesList';
import { BoardEvaluationBar } from '@/features/BoardEvaluationBar';
import { useBoardStore } from '@/entities/board';

// ---------------- STORE ----------------
const store = useBoardStore();

// ---------------- BOARD ----------------
let ground: Api;
const boardEl = ref<HTMLDivElement | null>(null);

// ---------------- STATE ----------------
const currentArrow = ref<{ orig: Key; dest: Key } | null>(null);
const evalCp = ref<number | null>(null);

// ---------------- INIT ----------------
onMounted(async () => {
  await invoke('init_engine');

  ground = Chessground(boardEl.value!, {
    fen: store.currentFen,
    orientation: 'white',

    movable: getMovable(),

    drawable: {
      enabled: true,
      visible: true,
      defaultSnapToValidMove: true,
    },
  });

  runEngine(store.currentFen);
});

// ---------------- MOVE ----------------
async function onMove(orig: string, dest: string) {
  if (isPromotion(orig, dest)) {
    const promotion = await askPromotion();
    if (!promotion) return;

    const result = store.makeMove(orig, dest, promotion);
    if (!result) return;

    syncBoard();
    await runEngine(result.fen);
    return;
  }

  const result = store.makeMove(orig, dest);
  if (!result) {
    syncBoard();
    return;
  }

  syncBoard();
  await runEngine(result.fen);
}
async function askPromotion(): Promise<string> {
  const piece = prompt('Promote to (q, r, b, n)?', 'q');
  return piece || 'q';
}

// ---------------- SYNC ----------------
function syncBoard() {
  const fen = store.currentFen;
  const lastMove = getLastMove();

  ground.set({
    fen,
    lastMove,
    movable: getMovable(),
  });

  restoreArrow();
}

// ---------------- WATCH ----------------
watch(
  () => store.currentIndex,
  () => {
    syncBoard();
    runEngine(store.currentFen);
  },
);

// ---------------- LEGAL MOVES ----------------
function getDests() {
  const map = new Map();
  const game = store.buildGame();

  game.moves({ verbose: true }).forEach((m: any) => {
    if (!map.has(m.from)) map.set(m.from, []);
    map.get(m.from).push(m.to);
  });

  return map;
}

// ---------------- ENGINE ----------------
async function runEngine(fen: string) {
  try {
    const res: any = await invoke('analyze_position', {
      fen,
      depth: 14,
    });

    if (!res) return;

    if (res.eval !== undefined) {
      const turn = store.turn();
      evalCp.value = turn === 'w' ? res.eval : -res.eval;
    }

    if (!res.best_move || res.best_move === '0000') return;

    drawBestMove(res.best_move);
  } catch (e) {
    console.error('ENGINE ERROR:', e);
  }
}

// ---------------- DRAW ARROW ----------------
function drawBestMove(move: string) {
  if (!move || move.length < 4) return;

  const orig = move.slice(0, 2);
  const dest = move.slice(2, 4);

  if (!isValidSquare(orig) || !isValidSquare(dest)) return;

  currentArrow.value = { orig, dest };

  ground.setShapes([
    {
      orig,
      dest,
      brush: 'blue',
      below: true,
    },
  ]);
}

// ---------------- RESTORE ARROW ----------------
function restoreArrow() {
  if (!currentArrow.value) return;

  ground.setShapes([
    {
      orig: currentArrow.value.orig,
      dest: currentArrow.value.dest,
      brush: 'blue',
    },
  ]);
}

// ---------------- LAST MOVE ----------------
function getLastMove(): [Key, Key] | undefined {
  const index = store.currentIndex;
  const move = store.moves[index];

  if (!move) return undefined;

  const game = store.buildGame();
  const history = game.history({ verbose: true });

  const last = history[history.length - 1];
  if (!last) return undefined;

  return [last.from as Key, last.to as Key];
}

// ---------------- UTILS ----------------
function isValidSquare(sq: string): boolean {
  return /^[a-h][1-8]$/.test(sq);
}

function getMovable(): Config['movable'] {
  return {
    color: store.turn() === 'w' ? 'white' : 'black',
    free: false,
    dests: getDests(),
    events: {
      after: onMove,
    },
  };
}

function isPromotion(from: string, to: string): boolean {
  return (from[1] === '7' && to[1] === '8') || (from[1] === '2' && to[1] === '1');
}

onMounted(async () => {
  await invoke('init_engine');

  // если нет партии → стартовая позиция
  if (!store.moves.length) {
    store.reset();
  }

  ground = Chessground(boardEl.value!, {
    fen: store.currentFen,
    orientation: 'white',
    movable: getMovable(),
    drawable: {
      enabled: true,
      visible: true,
      defaultSnapToValidMove: true,
    },
  });

  runEngine(store.currentFen);
});
</script>

<style scoped>
.layout {
  display: flex;
  gap: 16px;
  justify-content: center;
  align-items: flex-start;
}

.board {
  width: 520px;
  height: 520px;
}
</style>
