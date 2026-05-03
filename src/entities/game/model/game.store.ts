import { invoke } from '@tauri-apps/api/core';
import { defineStore } from 'pinia';

import type { Game } from './games.types';

interface State {
  games: Game[];
  loading: boolean;
  wasLoadedEmptyList: boolean;
}

export const useGamesStore = defineStore('games', {
  state: (): State => ({
    games: [],
    loading: false,
    wasLoadedEmptyList: false,
  }),

  actions: {
    async sync() {
      this.loading = true;
      await invoke('sync_games');
      const games = await invoke('get_games');
      this.games = games as Game[];
      this.loading = false;
      this.wasLoadedEmptyList = this.games.length === 0;

      return this.games;
    },
  },
});
