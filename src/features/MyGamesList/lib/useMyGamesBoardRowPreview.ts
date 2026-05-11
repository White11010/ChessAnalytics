// Fixed-position final-FEN board preview on table row hover via `data-game-id`, without prop-driven churn on the table body.
import type { Game } from '@/entities/game';
import type { Key } from 'chessground/types';
import { computed, shallowRef, toValue, type CSSProperties, type MaybeRefOrGetter } from 'vue';

export function canShowFinalBoard(game: Game): boolean {
  return Boolean(game.last_fen && game.moves);
}

export function finalLastMove(game: Game): [Key, Key] {
  const tokens = (game.moves || '').trim().split(/\s+/).filter(Boolean);
  const lastToken = tokens[tokens.length - 1] || 'e2e4';
  return [lastToken.slice(0, 2) as Key, lastToken.slice(2, 4) as Key];
}

export function useMyGamesBoardRowPreview(games: MaybeRefOrGetter<Game[]>) {
  const boardPreview = shallowRef<{ game: Game; rect: DOMRect } | null>(null);

  function hideBoardPreview() {
    boardPreview.value = null;
  }

  function onGamesTableMouseOver(e: MouseEvent) {
    const host = (e.target as HTMLElement | null)?.closest?.('[data-game-id]') as HTMLElement | null;
    if (!host?.dataset.gameId) {
      hideBoardPreview();
      return;
    }
    const game = toValue(games).find((g) => g.id === host.dataset.gameId);
    if (!game || !canShowFinalBoard(game)) {
      hideBoardPreview();
      return;
    }
    const tr = host.closest('tr');
    const rect = (tr ?? host).getBoundingClientRect();
    boardPreview.value = { game, rect };
  }

  function onGamesTableMouseLeave() {
    hideBoardPreview();
  }

  const boardPreviewStyle = computed((): CSSProperties => {
    const b = boardPreview.value;
    if (!b) {
      return {};
    }
    const pad = 8;
    const boardPx = 200;
    const panelW = boardPx + pad * 2;
    const vw = typeof window !== 'undefined' ? window.innerWidth : 1200;
    const vh = typeof window !== 'undefined' ? window.innerHeight : 800;
    // Prefer floating board to the left of the row; fall back to the right if it would clip off-screen.
    let left = b.rect.left - pad - panelW;
    if (left < pad) {
      left = b.rect.right + pad;
    }
    if (left + panelW > vw - pad) {
      left = Math.max(pad, vw - pad - panelW);
    }
    const top = Math.max(pad, Math.min(b.rect.top, vh - boardPx - pad));
    return {
      position: 'fixed' as const,
      left: `${left}px`,
      top: `${top}px`,
      zIndex: 3000,
      width: `calc(${boardPx}px + ${pad * 2}px)`,
      height: `calc(${boardPx}px + ${pad * 2}px)`,
      overflow: 'hidden',
    };
  });

  return {
    boardPreview,
    boardPreviewStyle,
    onGamesTableMouseOver,
    onGamesTableMouseLeave,
    hideBoardPreview,
  };
}
