// My Games toolbar + table: filter/sort snapshot persisted in sessionStorage so navigation does not reset user choices.
import { defineStore } from 'pinia';

import type { MyGamesPeriod, MyGamesPlayerColor } from './games.types';

const STORAGE_KEY = 'chessanalytics:myGamesFilters'; // sessionStorage scope: per tab session, not long-lived like prefs.

/** Vuetify `v-data-table` sort model (single column). */
export type MyGamesTableSortItem = {
  key: string;
  order?: 'asc' | 'desc';
};

const DEFAULT_SORT_BY: MyGamesTableSortItem[] = [{ key: 'created_at', order: 'desc' }]; // Newest first matches Lichess habit.

const SORTABLE_KEYS = new Set([
  'created_at',
  'analysis_accuracy',
  'analysis_acpl',
  'player_rating',
  'opponent_name',
  'opening_name',
  'speed',
]);

function normalizeSortBy(raw: unknown): MyGamesTableSortItem[] {
  if (!Array.isArray(raw) || raw.length === 0) {
    return [...DEFAULT_SORT_BY];
  }
  const first = raw[0] as { key?: string; order?: string };
  const key = typeof first?.key === 'string' ? first.key : '';
  const order = first?.order === 'asc' || first?.order === 'desc' ? first.order : 'desc';
  if (!SORTABLE_KEYS.has(key)) {
    return [...DEFAULT_SORT_BY];
  }
  return [{ key, order }];
}

/** Legacy toolbar preset → table `sortBy` (rating → `player_rating` to match column). */
function sortPresetToSortBy(preset: string): MyGamesTableSortItem[] {
  const map: Record<string, MyGamesTableSortItem> = {
    date_desc: { key: 'created_at', order: 'desc' },
    date_asc: { key: 'created_at', order: 'asc' },
    accuracy_desc: { key: 'analysis_accuracy', order: 'desc' },
    accuracy_asc: { key: 'analysis_accuracy', order: 'asc' },
    acpl_asc: { key: 'analysis_acpl', order: 'asc' },
    acpl_desc: { key: 'analysis_acpl', order: 'desc' },
    rating_desc: { key: 'player_rating', order: 'desc' },
    rating_asc: { key: 'player_rating', order: 'asc' },
  };
  return [map[preset] ?? { ...DEFAULT_SORT_BY[0] }];
}

export interface MyGamesFiltersSnapshot {
  searchText: string;
  results: Array<'win' | 'loss' | 'draw'>;
  speeds: string[];
  periods: MyGamesPeriod[];
  patternTag: string | null;
  /** Selected `eco|name` keys from the opening autocomplete. */
  openingValues: string[];
  /** Exact Lichess `opening_name` (insight navigation); excludes longer-name supersets from substring search. */
  openingNamesExact: string[];
  /** Empty = all colors; matches `Game.player_color`. */
  playerColors: MyGamesPlayerColor[];
  sortBy: MyGamesTableSortItem[];
}

function defaultSnapshot(): MyGamesFiltersSnapshot {
  return {
    searchText: '',
    results: [],
    speeds: [],
    periods: [],
    patternTag: null,
    openingValues: [],
    openingNamesExact: [],
    playerColors: [],
    sortBy: [...DEFAULT_SORT_BY],
  };
}

function migrateOpeningFields(parsed: Record<string, unknown>): Pick<
  MyGamesFiltersSnapshot,
  'openingValues' | 'openingNamesExact'
> {
  const legacyValue = parsed.openingValue;
  const legacyExact = parsed.openingNameExact;
  const valuesFromParsed = parsed.openingValues;
  const namesFromParsed = parsed.openingNamesExact;

  let openingValues: string[] = [];
  if (Array.isArray(valuesFromParsed)) {
    openingValues = valuesFromParsed.filter((v): v is string => typeof v === 'string' && v !== '');
  } else if (typeof legacyValue === 'string' && legacyValue) {
    openingValues = [legacyValue];
  }

  let openingNamesExact: string[] = [];
  if (Array.isArray(namesFromParsed)) {
    openingNamesExact = namesFromParsed.filter((v): v is string => typeof v === 'string' && v !== '');
  } else if (typeof legacyExact === 'string' && legacyExact) {
    openingNamesExact = [legacyExact];
  }

  return { openingValues, openingNamesExact };
}

function loadSnapshot(): MyGamesFiltersSnapshot {
  try {
    const raw = sessionStorage.getItem(STORAGE_KEY);
    if (!raw) {
      return defaultSnapshot();
    }
    const parsed = JSON.parse(raw) as Record<string, unknown> & {
      sortPreset?: string;
      sortBy?: unknown;
    };
    const base = defaultSnapshot();
    const {
      sortPreset,
      sortBy: rawSortBy,
      openingValue: _ov,
      openingNameExact: _one,
      openingValues: _ovs,
      openingNamesExact: _ones,
      ...rest
    } = parsed;

    let sortBy = base.sortBy;
    if (Array.isArray(rawSortBy) && rawSortBy.length && (rawSortBy[0] as { key?: string })?.key) {
      sortBy = normalizeSortBy(rawSortBy);
    } else if (typeof sortPreset === 'string' && sortPreset) {
      sortBy = sortPresetToSortBy(sortPreset);
    }

    const openingFields = migrateOpeningFields(parsed);

    return { ...base, ...rest, ...openingFields, sortBy } as MyGamesFiltersSnapshot;
  } catch {
    return defaultSnapshot();
  }
}

function saveSnapshot(s: MyGamesFiltersSnapshot) {
  sessionStorage.setItem(STORAGE_KEY, JSON.stringify(s));
}

export const useMyGamesFiltersStore = defineStore('myGamesFilters', {
  // Entire snapshot is both state and persistence payload; `persist`/`reset` keep sessionStorage aligned with UI.
  state: (): MyGamesFiltersSnapshot => loadSnapshot(),

  actions: {
    persist() {
      saveSnapshot({
        searchText: this.searchText,
        results: this.results,
        speeds: this.speeds,
        periods: this.periods,
        patternTag: this.patternTag,
        openingValues: [...this.openingValues],
        openingNamesExact: [...this.openingNamesExact],
        playerColors: [...this.playerColors],
        sortBy: [...this.sortBy],
      });
    },

    reset() {
      Object.assign(this, defaultSnapshot());
      this.persist();
    },

    setOpeningFilters(values: string[], namesExact: string[]) {
      this.openingValues = [...values];
      this.openingNamesExact = [...namesExact];
    },
  },
});
