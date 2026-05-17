// Versus page: holds opponent input, selected speed tab, progress events, and last `versus_compare` payload from Tauri.
import { invoke } from '@tauri-apps/api/core';
import { defineStore } from 'pinia';

import type {
  VersusCompareResponse,
  VersusProgressPayload,
  VersusSpeedSlice,
} from './versus.types';

export type VersusSpeed = 'bullet' | 'blitz' | 'rapid';

export const useVersusStore = defineStore('versus', {
  // Written by `compare`/`reset`/`setProgress`; Versus widgets read `result`, `loading`, and `activeSlice` getter.
  state: () => ({
    opponentUsernameInput: '',
    selectedSpeed: 'blitz' as VersusSpeed,
    loading: false,
    errorMessage: null as string | null,
    progress: null as VersusProgressPayload | null,
    result: null as VersusCompareResponse | null,
    compareSeq: 0,
  }),

  getters: {
    activeSlice(state): VersusSpeedSlice | null {
      if (!state.result) return null;
      return state.result.slices[state.selectedSpeed];
    },
  },

  actions: {
    setProgress(payload: VersusProgressPayload | null): void {
      // Ignore late progress events that arrive after a cancel or after the compare finished.
      if (payload && !this.loading) return;
      this.progress = payload;
    },

    reset(): void {
      this.errorMessage = null;
      this.progress = null;
      this.result = null;
    },

    cancel(): void {
      // Invalidate the in-flight compare so its late resolution cannot repopulate state.
      this.compareSeq++;
      this.loading = false;
      this.progress = null;
      this.errorMessage = null;
      this.result = null;
      void invoke('versus_cancel_compare');
    },

    async compare(): Promise<void> {
      const trimmed = this.opponentUsernameInput.trim();
      if (!trimmed || this.loading) return;

      const mySeq = ++this.compareSeq;
      this.loading = true;
      this.errorMessage = null;
      this.result = null;
      this.progress = null;
      this.selectedSpeed = 'blitz';

      try {
        const res = await invoke<VersusCompareResponse>('versus_compare', {
          opponentUsername: trimmed,
        });
        if (mySeq !== this.compareSeq) return;
        this.result = res;
        console.log('compare', res);
      } catch (e) {
        if (mySeq !== this.compareSeq) return;
        const msg = e instanceof Error ? e.message : String(e);
        this.errorMessage = msg;
      } finally {
        if (mySeq === this.compareSeq) {
          this.loading = false;
          this.progress = null;
        }
      }
    },
  },
});
