import { invoke } from '@tauri-apps/api/core';
import { defineStore } from 'pinia';

import type { User } from './user.types';

interface State {
  user: User | null;
  isLoading: boolean;
}

export const useUserStore = defineStore('user', {
  state: (): State => ({
    user: null,
    isLoading: false,
  }),

  actions: {
    setUser(data: { username: string; id: string } | null) {
      this.user = data;
    },
    async getCurrentUser() {
      this.isLoading = true;
      this.user = await invoke('fetch_lichess_player');
      this.isLoading = false;

      return this.user;
    },
    async syncMe() {
      this.isLoading = true;
      this.user = await invoke('sync_me');
      this.isLoading = false;

      return this.user as User;
    },
  },
});
