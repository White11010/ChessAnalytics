import type { MyGamesFiltersSnapshot } from '@/entities/game/model/myGamesFilters.store';
import type { MyGamesPlayerColor } from '@/entities/game/model/games.types';
import { MY_GAMES_LONG_LOSS_FILTER_TAG } from '@/shared/lib/patternTags';

import type { Insight } from '../model/insight.types';
import { parseInsightPayload, speedLabelFromSubjectKey } from './insightPayload';

export type InsightMyGamesFilterPatch = Pick<
  MyGamesFiltersSnapshot,
  | 'searchText'
  | 'results'
  | 'speeds'
  | 'periods'
  | 'patternTag'
  | 'openingValues'
  | 'openingNamesExact'
  | 'playerColors'
>;

function emptyPatch(): InsightMyGamesFilterPatch {
  return {
    searchText: '',
    results: [],
    speeds: [],
    periods: [],
    patternTag: null,
    openingValues: [],
    openingNamesExact: [],
    playerColors: [],
  };
}

function playerColorFromParam(raw: unknown): MyGamesPlayerColor | null {
  if (raw === 'white' || raw === 'black') {
    return raw;
  }
  return null;
}

export function canNavigateInsightToMyGames(insight: Insight): boolean {
  const k = insight.kind;
  if (
    k.startsWith('opening_') ||
    k === 'time_control_best' ||
    k === 'time_control_worst' ||
    k === 'time_rating_growth_30d' ||
    k === 'tactics_late_game_losses'
  ) {
    return true;
  }
  if (k === 'tactics_side_performance') {
    const { params } = parseInsightPayload(insight.payload_json);
    return typeof params.white_stronger === 'boolean';
  }
  return false;
}

function speedSlugFromLabel(label: string): string | null {
  const x = label.trim().toLowerCase();
  if (x === 'bullet' || x === 'ultrabullet') {
    return 'bullet';
  }
  if (x === 'blitz' || x === 'rapid' || x === 'classical') {
    return x;
  }
  return null;
}

export function buildMyGamesFiltersFromInsight(insight: Insight): InsightMyGamesFilterPatch {
  if (!canNavigateInsightToMyGames(insight)) {
    return emptyPatch();
  }

  const { subject_key, params } = parseInsightPayload(insight.payload_json);
  const k = insight.kind;

  if (k.startsWith('opening_')) {
    const openingRaw = params.opening;
    const openingName = typeof openingRaw === 'string' ? openingRaw : '';
    if (!openingName.trim()) {
      return emptyPatch();
    }
    // Insights aggregate by exact `opening_name` in Rust; match that here (substring search would include longer Lichess names).
    const patch: InsightMyGamesFilterPatch = {
      ...emptyPatch(),
      openingNamesExact: [openingName.trim()],
    };
    if (k === 'opening_color_split') {
      const color = playerColorFromParam(params.stronger_color);
      if (color) {
        patch.playerColors = [color];
      }
    }
    return patch;
  }

  if (k === 'time_control_best' || k === 'time_control_worst' || k === 'time_rating_growth_30d') {
    const labelFromParams =
      typeof params.speed_label === 'string'
        ? params.speed_label
        : typeof params.label === 'string'
          ? params.label
          : speedLabelFromSubjectKey(subject_key);
    const slug = labelFromParams ? speedSlugFromLabel(labelFromParams) : null;
    const speeds = slug != null ? [slug] : [];
    const periods = k === 'time_rating_growth_30d' ? (['30'] as MyGamesFiltersSnapshot['periods']) : [];
    return { ...emptyPatch(), speeds, periods };
  }

  if (k === 'tactics_late_game_losses') {
    // Virtual pattern in My Games toolbar (same cohort as the insight, clearable via the pattern control).
    return { ...emptyPatch(), patternTag: MY_GAMES_LONG_LOSS_FILTER_TAG };
  }

  if (k === 'tactics_side_performance') {
    const ws = params.white_stronger;
    if (typeof ws !== 'boolean') {
      return emptyPatch();
    }
    return {
      ...emptyPatch(),
      playerColors: ws ? ['white'] : ['black'],
    };
  }

  return emptyPatch();
}
