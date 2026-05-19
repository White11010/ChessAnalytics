export type SimilarGamesTableTranslate = (key: string) => string;

export function getSimilarGamesTableHeaders(t: SimilarGamesTableTranslate) {
  return [
    { key: 'opponent', title: t('myGames.table.opponent'), sortable: false },
    { key: 'opening', title: t('myGames.table.opening'), sortable: false },
    { key: 'speed', title: t('myGames.table.time'), width: '100px', sortable: false },
    { key: 'result', title: t('gameDetails.listResult'), width: '96px', sortable: false },
    { key: 'accuracy', title: t('myGames.table.accuracy'), width: '88px', sortable: false },
    { key: 'date', title: t('myGames.table.date'), width: '144px', sortable: false },
    { key: 'actions', title: '', width: '56px', sortable: false },
  ];
}
