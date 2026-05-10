import { invoke } from '@tauri-apps/api/core';
import { defineStore } from 'pinia';

import { queryClient } from '@/app/providers/query/queryProvider';
import { notifyGamesSynced } from '@/entities/analysis-settings';
import { useGamesStore } from '@/entities/game';
import { useInsightsStore } from '@/entities/insight';

export interface SyncGamesInvokeResult {
  inserted: number;
  username: string;
  last_sync_completed_at_ms: number;
}

const MANUAL_COOLDOWN_MS = 2 * 60 * 1000;
const RATE_LIMIT_RETRY_MS = 60 * 1000;
const AUTO_SYNC_DEBOUNCE_MS = 3000;

function messageIncludes(haystack: unknown, needle: string): boolean {
  return typeof haystack === 'string' && haystack.includes(needle);
}

export const useGamesSyncStore = defineStore('gamesSync', {
  state: () => ({
    lastSyncedAt: null as number | null,
    phase: 'idle' as 'idle' | 'syncing' | 'error' | 'rate_limited',
    errorMessage: null as string | null,
    manualCooldownUntil: null as number | null,
    rateLimitRetryAt: null as number | null,
    autoDebounceTimer: null as ReturnType<typeof setTimeout> | null,
    rateLimitRetryTimer: null as ReturnType<typeof setTimeout> | null,
    inFlight: false,
    snackbarMessage: null as string | null,
    snackbarColor: 'surface-variant' as string,
    snackbarVisible: false,
  }),

  getters: {
    isSyncing(state): boolean {
      return state.phase === 'syncing' || state.inFlight;
    },
  },

  actions: {
    shouldAutoSync(thresholdMinutes: number): boolean {
      if (this.lastSyncedAt == null) {
        return true;
      }
      return Date.now() - this.lastSyncedAt > thresholdMinutes * 60 * 1000;
    },

    canManualSync(): boolean {
      if (this.phase === 'syncing' || this.inFlight) {
        return false;
      }
      if (this.manualCooldownUntil != null && Date.now() < this.manualCooldownUntil) {
        return false;
      }
      if (this.rateLimitRetryAt != null && Date.now() < this.rateLimitRetryAt) {
        return false;
      }
      return true;
    },

    manualCooldownRemainingSec(): number {
      if (this.manualCooldownUntil == null) {
        return 0;
      }
      return Math.max(0, Math.ceil((this.manualCooldownUntil - Date.now()) / 1000));
    },

    rateLimitRemainingSec(): number {
      if (this.rateLimitRetryAt == null) {
        return 0;
      }
      return Math.max(0, Math.ceil((this.rateLimitRetryAt - Date.now()) / 1000));
    },

    hydrateFromUser(lastSyncCompletedAtMs: number | null | undefined): void {
      if (lastSyncCompletedAtMs != null && lastSyncCompletedAtMs > 0) {
        this.lastSyncedAt = lastSyncCompletedAtMs;
      }
    },

    reset(): void {
      this.lastSyncedAt = null;
      this.phase = 'idle';
      this.errorMessage = null;
      this.manualCooldownUntil = null;
      this.rateLimitRetryAt = null;
      this.inFlight = false;
      if (this.autoDebounceTimer != null) {
        clearTimeout(this.autoDebounceTimer);
        this.autoDebounceTimer = null;
      }
      if (this.rateLimitRetryTimer != null) {
        clearTimeout(this.rateLimitRetryTimer);
        this.rateLimitRetryTimer = null;
      }
    },

    showSnackbar(message: string, color: string = 'surface-variant'): void {
      this.snackbarMessage = message;
      this.snackbarColor = color;
      this.snackbarVisible = true;
    },

    dismissSnackbar(): void {
      this.snackbarVisible = false;
    },

    clearTransientError(): void {
      if (this.phase === 'error') {
        this.phase = 'idle';
        this.errorMessage = null;
      }
    },

    /** Coalesced auto-sync (plan §7.1): multiple triggers within a few seconds become one run. */
    scheduleAutoSync(thresholdMinutes: number, _reason: string): void {
      if (!this.shouldAutoSync(thresholdMinutes)) {
        return;
      }
      if (this.autoDebounceTimer != null) {
        clearTimeout(this.autoDebounceTimer);
      }
      this.autoDebounceTimer = setTimeout(() => {
        this.autoDebounceTimer = null;
        if (this.shouldAutoSync(thresholdMinutes)) {
          void this.runSync('auto', { isManual: false });
        }
      }, AUTO_SYNC_DEBOUNCE_MS);
    },

    async manualSync(): Promise<void> {
      if (!this.canManualSync()) {
        return;
      }
      await this.runSync('manual', { isManual: true });
    },

    async runSync(
      _reason: string,
      options?: { isManual?: boolean; isRateLimitRetry?: boolean },
    ): Promise<void> {
      const isManual = options?.isManual ?? false;
      const isRateLimitRetry = options?.isRateLimitRetry ?? false;
      if (this.inFlight) {
        return;
      }

      if (this.rateLimitRetryAt != null && Date.now() >= this.rateLimitRetryAt) {
        this.rateLimitRetryAt = null;
      }
      if (this.phase === 'rate_limited' && this.rateLimitRetryAt == null) {
        this.phase = 'idle';
      }

      this.inFlight = true;
      this.phase = 'syncing';
      this.errorMessage = null;

      try {
        const res = await invoke<SyncGamesInvokeResult>('sync_games');
        this.lastSyncedAt = res.last_sync_completed_at_ms;
        this.phase = 'idle';
        this.rateLimitRetryAt = null;
        if (this.rateLimitRetryTimer != null) {
          clearTimeout(this.rateLimitRetryTimer);
          this.rateLimitRetryTimer = null;
        }

        if (isManual) {
          this.manualCooldownUntil = Date.now() + MANUAL_COOLDOWN_MS;
        }

        const gamesStore = useGamesStore();
        await gamesStore.loadFromDb();
        notifyGamesSynced();
        await queryClient.invalidateQueries({ queryKey: ['games'] });

        if (res.inserted > 0) {
          void this.regenerateInsightsAfterSync();
        }
      } catch (e) {
        const msg = String(e);
        if (messageIncludes(msg, 'RATE_LIMITED')) {
          this.phase = 'rate_limited';
          this.errorMessage = msg;
          this.rateLimitRetryAt = Date.now() + RATE_LIMIT_RETRY_MS;
          this.showSnackbar(String(e), 'warning');
          if (!isRateLimitRetry && this.rateLimitRetryTimer == null) {
            this.rateLimitRetryTimer = setTimeout(() => {
              this.rateLimitRetryTimer = null;
              this.rateLimitRetryAt = null;
              void this.runSync('rate_limit_retry', {
                isManual: false,
                isRateLimitRetry: true,
              });
            }, RATE_LIMIT_RETRY_MS);
          }
        } else if (messageIncludes(msg, 'UNAUTHORIZED')) {
          this.phase = 'error';
          this.errorMessage = msg;
          this.showSnackbar(String(e), 'error');
        } else {
          this.phase = 'error';
          this.errorMessage = msg;
          this.showSnackbar(String(e), 'error');
        }
      } finally {
        this.inFlight = false;
      }
    },

    async regenerateInsightsAfterSync(): Promise<void> {
      try {
        const insightsStore = useInsightsStore();
        await insightsStore.regenerate();
        await queryClient.invalidateQueries({ queryKey: ['insights', 'load'] });
      } catch (err) {
        console.error('regenerate_insights after games sync', err);
      }
    },
  },
});
