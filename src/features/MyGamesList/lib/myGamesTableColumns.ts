// Declarative My Games `v-data-table` column keys/widths/sortable flags + `getMyGamesTableHeaders(t)` for localized titles.

export type MyGamesTableTranslate = (key: string) => string;

type ColumnDef = {
  key: string;
  /** i18n key under `myGames.table.*`; omit for empty title (marker/actions columns). */
  titleKey?: string;
  width?: string;
  sortable?: boolean;
};

const MY_GAMES_TABLE_COLUMNS: readonly ColumnDef[] = [
  { key: 'result_marker', width: '40px', sortable: false },
  { key: 'opponent_name', titleKey: 'myGames.table.opponent', sortable: true },
  { key: 'opening_name', titleKey: 'myGames.table.opening', sortable: true },
  { key: 'speed', titleKey: 'myGames.table.time', width: '100px', sortable: true },
  { key: 'player_rating', titleKey: 'myGames.table.rating', width: '88px', sortable: true },
  { key: 'analysis_accuracy', titleKey: 'myGames.table.accuracy', width: '88px', sortable: true },
  { key: 'analysis_acpl', titleKey: 'myGames.table.acpl', width: '72px', sortable: true },
  { key: 'pattern_tags', titleKey: 'myGames.table.patterns', sortable: false },
  { key: 'created_at', titleKey: 'myGames.table.date', width: '144px', sortable: true },
  { key: 'actions', width: '96px', sortable: false },
];

export function getMyGamesTableHeaders(t: MyGamesTableTranslate) {
  return MY_GAMES_TABLE_COLUMNS.map((col) => ({
    key: col.key,
    title: col.titleKey ? t(col.titleKey) : '',
    ...(col.width ? { width: col.width } : {}),
    sortable: col.sortable !== false,
  }));
}
