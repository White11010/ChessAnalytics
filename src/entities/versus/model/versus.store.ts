import { invoke } from '@tauri-apps/api/core';
import { defineStore } from 'pinia';

import type {
  VersusCompareResponse,
  VersusProgressPayload,
  VersusSpeedSlice,
} from './versus.types';

export type VersusSpeed = 'bullet' | 'blitz' | 'rapid';

export const useVersusStore = defineStore('versus', {
  state: () => ({
    opponentUsernameInput: '',
    selectedSpeed: 'blitz' as VersusSpeed,
    loading: false,
    errorMessage: null as string | null,
    progress: null as VersusProgressPayload | null,
    result: null as VersusCompareResponse | null,
  }),

  getters: {
    activeSlice(state): VersusSpeedSlice | null {
      if (!state.result) return null;
      return state.result.slices[state.selectedSpeed];
    },
  },

  actions: {
    setProgress(payload: VersusProgressPayload | null): void {
      this.progress = payload;
    },

    reset(): void {
      this.errorMessage = null;
      this.progress = null;
      this.result = null;
    },

    cancel(): void {
      void invoke('versus_cancel_compare');
    },

    async compare(): Promise<void> {
      const trimmed = this.opponentUsernameInput.trim();
      if (!trimmed || this.loading) return;

      this.loading = true;
      this.errorMessage = null;
      this.result = null;
      this.progress = null;
      this.selectedSpeed = 'blitz';

      try {
        const res = await invoke<VersusCompareResponse>('versus_compare', {
          opponentUsername: trimmed,
        });
        this.result = res;
      } catch (e) {
        const msg = e instanceof Error ? e.message : String(e);
        this.errorMessage = msg;
      } finally {
        this.loading = false;
        this.progress = null;
      }
    },
  },
});
