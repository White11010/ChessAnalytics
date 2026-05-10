import { invoke } from '@tauri-apps/api/core';
import { defineStore } from 'pinia';

import { queryClient } from '@/app/providers/query/queryProvider';
import { useGamesSyncStore } from '@/entities/games-sync';
import type { User } from '@/entities/user';
import { useUserStore } from '@/entities/user';

type BootstrapState =
  | { state: 'Unauthorized' }
  | { state: 'Authorized'; data: { user: User; is_stale: boolean } };

export const useAppStore = defineStore('app', {
  state: () => ({
    status: 'loading' as 'loading' | 'ready',
    isStale: false,
  }),

  actions: {
    async bootstrap() {
      const userStore = useUserStore();

      const res = await invoke<BootstrapState>('bootstrap');

      if (res.state === 'Unauthorized') {
        this.status = 'ready';

        userStore.setUser(null);
        useGamesSyncStore().reset();
        return;
      }

      userStore.setUser(res.data.user);
      useGamesSyncStore().hydrateFromUser(res.data.user.last_sync_completed_at_ms ?? null);
      this.isStale = res.data.is_stale;
      this.status = 'ready';
      void queryClient.invalidateQueries({ queryKey: ['user'] });

      //   // optional UI refresh hint
      //   if (this.isStale) {
      //     this.refreshInBackground();
      //   }
    },

    async refreshInBackground() {
      const userStore = useUserStore();

      try {
        const res = await invoke('sync_me');
        userStore.setUser(res as User | null);
        useGamesSyncStore().hydrateFromUser((res as User).last_sync_completed_at_ms ?? null);
      } catch (e) {
        console.error(e);
      }
    },
  },
});
