import { invoke } from '@tauri-apps/api/core';
import { defineStore } from 'pinia';

import type { Insight } from './insight.types';

interface State {
  items: Insight[];

  isLoading: boolean;
  isRefreshing: boolean;

  error: string | null;

  lastLoadedAt: number | null;
}

export const useInsightsStore = defineStore('insights', {
  state: (): State => ({
    items: [],

    isLoading: false,
    isRefreshing: false,

    error: null,

    lastLoadedAt: null,
  }),

  getters: {
    heroInsight(state): Insight | null {
      if (!state.items.length) {
        return null;
      }

      return state.items[0];
    },

    goodInsights(state): Insight[] {
      return state.items.filter((item) => item.severity === 'good');
    },

    warningInsights(state): Insight[] {
      return state.items.filter(
        (item) => item.severity === 'warning' || item.severity === 'critical',
      );
    },
  },

  actions: {
    async load() {
      this.isLoading = true;
      this.error = null;

      try {
        const data = await invoke<Insight[]>('get_insights');

        this.items = data;
        this.lastLoadedAt = Date.now();
      } catch (error) {
        this.error = String(error);
      } finally {
        this.isLoading = false;
      }
    },

    async regenerate() {
      this.isRefreshing = true;
      this.error = null;

      try {
        const data = await invoke<Insight[]>('regenerate_insights');

        this.items = data;
        this.lastLoadedAt = Date.now();
      } catch (error) {
        this.error = String(error);
      } finally {
        this.isRefreshing = false;
      }

      return this.items as Insight[];
    },

    // async refreshBackground() {
    //   try {
    //     await invoke('refresh_insights_background');
    //   } catch (error) {
    //     console.error(error);
    //   }
    // },

    clear() {
      this.items = [];
      this.error = null;
      this.lastLoadedAt = null;
    },
  },
});
