import type { Game } from '@/entities/game';

export interface SimilarGameRow {
  id: string;
  game: Game | null;
}

export function resolveSimilarGameRows(
  ids: readonly string[],
  gamesById: ReadonlyMap<string, Game>,
): SimilarGameRow[] {
  return ids.map((id) => ({
    id,
    game: gamesById.get(id) ?? null,
  }));
}

export function gamesByIdMap(games: readonly Game[]): Map<string, Game> {
  const map = new Map<string, Game>();
  for (const g of games) {
    map.set(g.id, g);
  }
  return map;
}
