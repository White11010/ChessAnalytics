// Game entity public surface: sync/query helpers, My Games filters, and v-data-table sort maps for the games list.
export { computeCurrentResultStreak, type ResultStreak } from './lib/computeCurrentResultStreak';
export { averageAnalysisAccuracyPercentRounded } from './lib/averageAnalysisAccuracy';
export { useSyncGamesQuery } from './lib/gamesQuery';
export type { AccuracyTone } from './lib/myGamesDisplay';
export {
  accuracyPercentRounded,
  accuracyToneFromRounded,
  getExternalGameUrl,
  openExternalGame,
  openingTooltipText,
  resultLetter,
  shortOpeningDisplay,
  speedChipLabel,
} from './lib/myGamesDisplay';
export { MY_GAMES_TABLE_CUSTOM_KEY_SORT } from './lib/myGamesTableSort';
export {
  filterMyGames,
  normalizePatternTagId,
  patternTagOptions,
  uniqueOpeningOptions,
  uniquePatternTags,
} from './lib/myGamesFilterUtils';
export type { OpeningOption, PatternOption } from './lib/myGamesFilterUtils';
export { useGamesStore } from './model/game.store';
export type { Game, MyGamesPeriod, MyGamesPlayerColor } from './model/games.types';
export type { MyGamesTableSortItem } from './model/myGamesFilters.store';
export { useMyGamesFiltersStore } from './model/myGamesFilters.store';
