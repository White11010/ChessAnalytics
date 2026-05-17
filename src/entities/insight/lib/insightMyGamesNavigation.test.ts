import { describe, expect, it } from 'vitest';

import { MY_GAMES_LONG_LOSS_FILTER_TAG } from '@/shared/lib/patternTags';

import type { Insight } from '../model/insight.types';
import { buildMyGamesFiltersFromInsight } from './insightMyGamesNavigation';

function insight(kind: string, payload: Record<string, unknown>): Insight {
  return {
    id: '1',
    user_id: 'u',
    kind,
    title: '',
    summary: '',
    severity: 'info',
    confidence: 50,
    category: 'openings',
    sort_priority: 0,
    created_at: 0,
    payload_json: JSON.stringify({
      subject_key: `opening:${payload.opening ?? 'X'}`,
      params: payload,
    }),
  };
}

describe('buildMyGamesFiltersFromInsight', () => {
  it('opening_rare_gem sets openingNamesExact', () => {
    const patch = buildMyGamesFiltersFromInsight(
      insight('opening_rare_gem', { opening: 'Caro-Kann Defense' }),
    );
    expect(patch.openingNamesExact).toEqual(['Caro-Kann Defense']);
    expect(patch.openingValues).toEqual([]);
  });

  it('opening_color_split sets opening and player color', () => {
    const patch = buildMyGamesFiltersFromInsight(
      insight('opening_color_split', {
        opening: 'Sicilian Defense',
        stronger_color: 'white',
      }),
    );
    expect(patch.openingNamesExact).toEqual(['Sicilian Defense']);
    expect(patch.playerColors).toEqual(['white']);
  });

  it('time_control_best sets speeds', () => {
    const patch = buildMyGamesFiltersFromInsight(
      insight('time_control_best', { speed_label: 'Blitz', pct: 60, n: 20 }),
    );
    expect(patch.speeds).toEqual(['blitz']);
  });

  it('tactics_late_game_losses sets pattern tag', () => {
    const ins: Insight = {
      id: '1',
      user_id: 'u',
      kind: 'tactics_late_game_losses',
      title: '',
      summary: '',
      severity: 'warning',
      confidence: 50,
      category: 'tactics',
      sort_priority: 0,
      created_at: 0,
      payload_json: JSON.stringify({ subject_key: 'tactics:late_long_loss', params: {} }),
    };
    const patch = buildMyGamesFiltersFromInsight(ins);
    expect(patch.patternTag).toBe(MY_GAMES_LONG_LOSS_FILTER_TAG);
  });

  it('tactics_side_performance sets playerColors', () => {
    const ins: Insight = {
      id: '1',
      user_id: 'u',
      kind: 'tactics_side_performance',
      title: '',
      summary: '',
      severity: 'info',
      confidence: 50,
      category: 'tactics',
      sort_priority: 0,
      created_at: 0,
      payload_json: JSON.stringify({
        subject_key: 'side:white_vs_black',
        params: { white_stronger: true },
      }),
    };
    const patch = buildMyGamesFiltersFromInsight(ins);
    expect(patch.playerColors).toEqual(['white']);
  });
});
