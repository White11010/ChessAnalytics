import { defineStore } from 'pinia';
import { Chess } from 'chess.js';
import type { Board, MoveItem } from './board.types';

interface State {
  board: Board;
}

export const useBoardStore = defineStore('board', {
  state: (): State => ({
    board: {
      history: [],
      currentIndex: -1,
    },
  }),

  getters: {
    // ---------------- CURRENT FEN ----------------
    currentFen(state): string {
      const game = new Chess();

      for (let i = 0; i <= state.board.currentIndex; i++) {
        const move = state.board.history[i];
        if (!move) break;
        game.move(move.san);
      }

      return game.fen();
    },

    moves(state): MoveItem[] {
      return state.board.history;
    },

    currentIndex(state): number {
      return state.board.currentIndex;
    },
  },

  actions: {
    // ---------------- MAKE MOVE ----------------
    makeMove(from: string, to: string, promotion?: string) {
      const game = this.buildGame();

      const move = game.move({
        from,
        to,
        promotion,
      });
      if (!move) return null;

      const fen = game.fen();

      // если откатились назад → обрезаем будущее
      if (this.board.currentIndex < this.board.history.length - 1) {
        this.board.history = this.board.history.slice(0, this.board.currentIndex + 1);
      }

      this.board.history.push({
        san: move.san,
        fen,
      });

      this.board.currentIndex = this.board.history.length - 1;

      return { move, fen };
    },

    // ---------------- GO TO INDEX ----------------
    goToIndex(index: number) {
      if (index < -1 || index >= this.board.history.length) return;

      this.board.currentIndex = index;
    },

    // ---------------- RESET ----------------
    reset() {
      this.board.history = [];
      this.board.currentIndex = -1;
    },

    // ---------------- BUILD GAME ----------------
    buildGame(): Chess {
      const game = new Chess();

      for (let i = 0; i <= this.board.currentIndex; i++) {
        const move = this.board.history[i];
        if (!move) break;

        game.move(move.san);
      }

      return game;
    },

    // ---------------- TURN ----------------
    turn(): 'w' | 'b' {
      return this.buildGame().turn();
    },

    loadPgn(pgn: string) {
      const game = new Chess();

      game.loadPgn(pgn);

      const history = game.history({ verbose: true });

      // сброс
      this.board.history = [];
      this.board.currentIndex = -1;

      const replay = new Chess();

      history.forEach((move) => {
        replay.move(move);

        this.board.history.push({
          san: move.san,
          fen: replay.fen(),
        });
      });

      this.board.currentIndex = this.board.history.length - 1;
    },
  },
});
