<template>
  <Teleport to="body">
    <div
      v-show="preview"
      class="my-games-list__board-preview"
      :style="previewStyle"
      @mouseenter="emit('hide')"
    >
      <template v-if="preview && canShowFinalBoard(preview.game)">
        <ChessStaticBoard
          :fen="preview.game.last_fen!"
          :last-move="finalLastMove(preview.game)"
          :orientation="preview.game.player_color"
          :winner="preview.game.winner"
          size="200px"
        />
      </template>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
// Teleported floating ChessStaticBoard overlay for My Games table row hover preview.
import type { Game } from '@/entities/game';
import { ChessStaticBoard } from '@/shared/ui';
import type { CSSProperties } from 'vue';

import { canShowFinalBoard, finalLastMove } from '../lib/useMyGamesBoardRowPreview';

defineProps<{
  preview: { game: Game; rect: DOMRect } | null;
  previewStyle: CSSProperties;
}>();

const emit = defineEmits<{
  hide: [];
}>();
</script>

<style lang="scss" scoped>
.my-games-list__board-preview {
  padding: 0.5rem;
  border-radius: 0.5rem;
  background: rgb(var(--v-theme-surface));
  box-shadow:
    0 0.25rem 0.75rem rgba(0, 0, 0, 0.25),
    0 0 0 1px rgba(var(--v-border-color), var(--v-border-opacity));
  pointer-events: auto;
}
</style>
