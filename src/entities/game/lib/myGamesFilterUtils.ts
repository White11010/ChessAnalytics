import { MY_GAMES_LONG_LOSS_FILTER_TAG, normalizePatternTagId } from '@/shared/lib/patternTags';

import type { Game, MyGamesPeriod, MyGamesPlayerColor } from '../model/games.types';

export { normalizePatternTagId };

const DAY_MS = 86_400_000;

function isFileCode(c: number): boolean {
  return c >= 97 && c <= 104; // a–h
}

function isRankCode(c: number): boolean {
  return c >= 49 && c <= 56; // 1–8
}

/** UCI move token; mirrors Rust `move_count::is_uci_token` for ply counts aligned with backend insights. */
export function isUciToken(token: string): boolean {
  const u = token;
  const n = u.length;
  if (n === 4) {
    const c0 = u.charCodeAt(0);
    const c1 = u.charCodeAt(1);
    const c2 = u.charCodeAt(2);
    const c3 = u.charCodeAt(3);
    return isFileCode(c0) && isRankCode(c1) && isFileCode(c2) && isRankCode(c3);
  }
  if (n === 5) {
    const c4 = u.charCodeAt(4);
    return (
      isFileCode(u.charCodeAt(0)) &&
      isRankCode(u.charCodeAt(1)) &&
      isFileCode(u.charCodeAt(2)) &&
      isRankCode(u.charCodeAt(3)) &&
      (c4 === 113 || c4 === 114 || c4 === 98 || c4 === 110) // q r b n
    );
  }
  return false;
}

export function countUciHalfmovesFromMovesStr(moves: string | null | undefined): number {
  const s = (moves ?? '').trim();
  if (!s) {
    return 0;
  }
  return s.split(/\s+/).filter(isUciToken).length;
}

/** Prefer server `halfmoves_total` (matches insights) when the list row includes it. */
export function effectiveHalfmovesForFilter(game: Game): number {
  const n = game.halfmoves_total;
  if (typeof n === 'number' && Number.isFinite(n) && n >= 0) {
    return Math.floor(n);
  }
  return countUciHalfmovesFromMovesStr(game.moves);
}

/** `null` = no cutoff (all time). */
export function periodCutoffMs(periods: MyGamesPeriod[]): number | null {
  if (!periods.length || periods.includes('all')) {
    return null;
  }
  const days = periods
    .filter((p): p is Exclude<MyGamesPeriod, 'all'> => p !== 'all')
    .map((p) => Number(p));
  const maxDays = Math.max(...days);
  return Date.now() - maxDays * DAY_MS;
}

function speedMatchesFilter(gameSpeed: string, selected: string[]): boolean {
  if (!selected.length) {
    return true;
  }
  const g = gameSpeed.toLowerCase();
  return selected.some((s) => {
    const x = s.toLowerCase();
    if (x === 'bullet') {
      return g === 'bullet' || g === 'ultrabullet';
    }
    return g === x;
  });
}

function openingKey(game: Game): string {
  return `${game.opening_eco ?? ''}|${game.opening_name ?? ''}`;
}

/** Same threshold as `tactics_late_game_losses` / `move_count::total_halfmoves` on the backend. */
const LONG_LOSS_MIN_HALFMOVES = 40;

export function openingValuesForExactNames(games: Game[], names: string[]): string[] {
  const want = new Set(names.map((n) => n.trim()).filter(Boolean));
  if (!want.size) {
    return [];
  }
  const values = new Set<string>();
  for (const g of games) {
    const name = (g.opening_name ?? '').trim();
    if (want.has(name)) {
      values.add(openingKey(g));
    }
  }
  return [...values].sort((a, b) => a.localeCompare(b));
}

export function filterMyGames(
  games: Game[],
  opts: {
    searchText: string;
    results: Array<'win' | 'loss' | 'draw'>;
    speeds: string[];
    periods: MyGamesPeriod[];
    patternTag: string | null;
    openingValues: string[];
    openingNamesExact: string[];
    playerColors: MyGamesPlayerColor[];
  },
): Game[] {
  const q = opts.searchText.trim().toLowerCase();
  const cutoff = periodCutoffMs(opts.periods);
  const namesExact = opts.openingNamesExact.map((n) => n.trim()).filter(Boolean);
  const openingValues = opts.openingValues.filter(Boolean);

  return games.filter((g) => {
    if (cutoff !== null && g.created_at < cutoff) {
      return false;
    }
    if (opts.results.length && !opts.results.includes(g.player_result)) {
      return false;
    }
    if (!speedMatchesFilter(g.speed, opts.speeds)) {
      return false;
    }
    if (opts.patternTag) {
      const want = normalizePatternTagId(opts.patternTag);
      if (want === normalizePatternTagId(MY_GAMES_LONG_LOSS_FILTER_TAG)) {
        if (g.player_result !== 'loss' || effectiveHalfmovesForFilter(g) < LONG_LOSS_MIN_HALFMOVES) {
          return false;
        }
      } else {
        const tags = (g.pattern_tags ?? []).map((t) => normalizePatternTagId(t));
        if (!tags.includes(want)) {
          return false;
        }
      }
    }
    if (namesExact.length) {
      if (!namesExact.includes((g.opening_name ?? '').trim())) {
        return false;
      }
    } else if (openingValues.length && !openingValues.includes(openingKey(g))) {
      return false;
    }
    if (opts.playerColors.length && !opts.playerColors.includes(g.player_color)) {
      return false;
    }
    if (q) {
      const opp = (g.opponent_name ?? '').toLowerCase();
      const opName = (g.opening_name ?? '').toLowerCase();
      const opEco = (g.opening_eco ?? '').toLowerCase();
      if (!opp.includes(q) && !opName.includes(q) && !opEco.includes(q)) {
        return false;
      }
    }
    return true;
  });
}

export function uniquePatternTags(games: Game[]): string[] {
  const set = new Set<string>();
  for (const g of games) {
    for (const t of g.pattern_tags ?? []) {
      set.add(t);
    }
  }
  return [...set].sort((a, b) => a.localeCompare(b));
}

export type OpeningOption = { value: string; title: string };

export type PatternOption = { value: string; title: string };

export function patternTagOptions(games: Game[]): PatternOption[] {
  const synthetic: PatternOption = {
    value: MY_GAMES_LONG_LOSS_FILTER_TAG,
    title: '',
  };
  const fromGames = uniquePatternTags(games)
    .filter((tag) => normalizePatternTagId(tag) !== normalizePatternTagId(MY_GAMES_LONG_LOSS_FILTER_TAG))
    .map((tag) => ({
      value: tag,
      title: tag.replace(/_/g, ' '),
    }));
  return [synthetic, ...fromGames];
}

export function uniqueOpeningOptions(games: Game[]): OpeningOption[] {
  const map = new Map<string, OpeningOption>();
  for (const g of games) {
    if (!g.opening_name && !g.opening_eco) {
      continue;
    }
    const value = openingKey(g);
    if (map.has(value)) {
      continue;
    }
    const eco = g.opening_eco ?? '';
    const name = g.opening_name ?? '';
    const title = [eco, name].filter(Boolean).join(' — ') || name || eco;
    map.set(value, { value, title });
  }
  return [...map.values()].sort((a, b) => a.title.localeCompare(b.title));
}
