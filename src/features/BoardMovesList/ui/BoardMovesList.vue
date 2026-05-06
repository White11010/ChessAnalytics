<template>
  <v-card class="moves-card" elevation="2">
    <v-card-title class="text-subtitle-2 px-3 py-2"> Moves </v-card-title>

    <v-divider />

    <div class="moves-list">
      <div v-for="(row, i) in rows" :key="i" class="move-row">
        <!-- номер -->
        <div class="move-number">{{ i + 1 }}.</div>

        <!-- белые -->
        <span
          class="move"
          :class="{ active: row.whiteIndex === currentIndex }"
          @click="select(row.whiteIndex)"
        >
          {{ row.white || '' }}
        </span>

        <!-- черные -->
        <span
          class="move"
          :class="{ active: row.blackIndex === currentIndex }"
          @click="select(row.blackIndex)"
        >
          {{ row.black || '' }}
        </span>
      </div>
    </div>
  </v-card>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useBoardStore } from '@/entities/board';

const store = useBoardStore();

const currentIndex = computed(() => store.currentIndex);

const rows = computed(() => {
  const res = [];

  for (let i = 0; i < store.moves.length; i += 2) {
    res.push({
      white: store.moves[i]?.san,
      black: store.moves[i + 1]?.san,
      whiteIndex: i,
      blackIndex: i + 1,
    });
  }

  return res;
});

function select(index: number) {
  if (index < -1 || index >= store.moves.length) return;
  store.goToIndex(index);
}
</script>

<style scoped>
.moves-card {
  width: 220px;
  max-height: 520px;
  display: flex;
  flex-direction: column;
}

.moves-list {
  overflow-y: auto;
  padding: 6px 0;
}

/* строка */
.move-row {
  margin-top: 4px;
  display: grid;
  grid-template-columns: 28px 1fr 1fr;
  align-items: center;
  font-size: 14px;
  line-height: 1.6;
  gap: 0.25rem;
}

/* номер */
.move-number {
  opacity: 0.5;
  text-align: right;
  padding-right: 6px;
}

/* ход */
.move {
  cursor: pointer;
  padding: 2px 6px;
  border-radius: 4px;
  white-space: nowrap;
  display: flex;
  align-items: center;
}

/* hover */
.move:hover {
  background: rgba(var(--v-theme-on-surface), 0.08);
}

/* активный */
.move.active {
  background: rgba(var(--v-theme-primary), 0.25);
}
</style>
