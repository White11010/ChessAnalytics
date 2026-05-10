import { invoke } from '@tauri-apps/api/core';
import { defineStore } from 'pinia';

import type { Game } from './games.types';

interface State {
  games: Game[];
  wasLoadedEmptyList: boolean;
}

export const useGamesStore = defineStore('games', {
  state: (): State => ({
    games: [],
    wasLoadedEmptyList: false,
  }),

  actions: {
    async loadFromDb() {
      const games = await invoke('get_games');
      this.games = games as Game[];
      this.wasLoadedEmptyList = this.games.length === 0;
    },
  },
});
