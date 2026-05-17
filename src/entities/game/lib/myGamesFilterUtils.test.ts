import { describe, expect, it } from 'vitest';

import type { Game } from '../model/games.types';
import { filterMyGames, openingValuesForExactNames } from './myGamesFilterUtils';

function game(overrides: Partial<Game> = {}): Game {
  return {
    id: 'g1',
    username: 'u',
    platform: 'Lichess',
    rated: true,
    speed: 'blitz',
    time_control: '180+0',
    created_at: Date.now(),
    player_name: 'u',
    player_id: 'u',
    opponent_name: 'o',
    opponent_id: 'o',
    white_name: 'u',
    white_id: 'u',
    black_name: 'o',
    black_id: 'o',
    white_rating: null,
    black_rating: null,
    player_rating: null,
    opponent_rating: null,
    winner: 'white',
    player_color: 'white',
    player_result: 'win',
    opening_eco: 'B20',
    opening_name: 'Sicilian Defense',
    moves: null,
    last_fen: null,
    pgn: '',
    ...overrides,
  };
}

const emptyOpts = {
  searchText: '',
  results: [] as Array<'win' | 'loss' | 'draw'>,
  speeds: [] as string[],
  periods: [] as Array<'7' | '30' | '90' | 'all'>,
  patternTag: null,
  openingValues: [] as string[],
  openingNamesExact: [] as string[],
  playerColors: [] as Array<'white' | 'black'>,
};

describe('openingValuesForExactNames', () => {
  it('returns all eco|name keys for matching opening names', () => {
    const games = [
      game({ id: 'a', opening_eco: 'B20', opening_name: 'Sicilian Defense' }),
      game({ id: 'b', opening_eco: 'B21', opening_name: 'Sicilian Defense' }),
      game({ id: 'c', opening_eco: 'C00', opening_name: 'French Defense' }),
    ];
    const values = openingValuesForExactNames(games, ['Sicilian Defense']);
    expect(values).toEqual(['B20|Sicilian Defense', 'B21|Sicilian Defense']);
  });
});

describe('filterMyGames opening filters', () => {
  const games = [
    game({ id: 'a', opening_eco: 'B20', opening_name: 'Sicilian Defense' }),
    game({ id: 'b', opening_eco: 'C00', opening_name: 'French Defense' }),
  ];

  it('filters by openingNamesExact', () => {
    const out = filterMyGames(games, {
      ...emptyOpts,
      openingNamesExact: ['Sicilian Defense'],
    });
    expect(out.map((g) => g.id)).toEqual(['a']);
  });

  it('filters by multiple openingValues', () => {
    const out = filterMyGames(games, {
      ...emptyOpts,
      openingValues: ['B20|Sicilian Defense', 'C00|French Defense'],
    });
    expect(out.map((g) => g.id)).toEqual(['a', 'b']);
  });

  it('prefers openingNamesExact over openingValues', () => {
    const out = filterMyGames(games, {
      ...emptyOpts,
      openingNamesExact: ['Sicilian Defense'],
      openingValues: ['C00|French Defense'],
    });
    expect(out.map((g) => g.id)).toEqual(['a']);
  });
});
