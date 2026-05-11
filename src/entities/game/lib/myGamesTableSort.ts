// Vuetify `v-data-table` custom-key-sort comparators for My Games columns (nulls, empty strings, speed tier order).
export function nullableNumSort(a: unknown, b: unknown): number | null {
  const an = a == null || a === '' ? null : Number(a);
  const bn = b == null || b === '' ? null : Number(b);
  if (an === null && bn === null) {
    return 0;
  }
  if (an === null) {
    return 1;
  }
  if (bn === null) {
    return -1;
  }
  if (Number.isNaN(an) || Number.isNaN(bn)) {
    return null;
  }
  if (an === bn) {
    return 0;
  }
  return an < bn ? -1 : 1;
}

export function strEmptyLastSort(a: unknown, b: unknown): number | null {
  const sa = a == null ? '' : String(a).toLowerCase();
  const sb = b == null ? '' : String(b).toLowerCase();
  const ae = sa === '';
  const be = sb === '';
  if (ae && be) {
    return 0;
  }
  if (ae) {
    return 1;
  }
  if (be) {
    return -1;
  }
  if (sa < sb) {
    return -1;
  }
  if (sa > sb) {
    return 1;
  }
  return 0;
}

export function speedRank(s: unknown): number {
  const g = String(s ?? '').toLowerCase();
  if (g === 'ultrabullet') {
    return 0;
  }
  if (g === 'bullet') {
    return 1;
  }
  if (g === 'blitz') {
    return 2;
  }
  if (g === 'rapid') {
    return 3;
  }
  if (g === 'classical') {
    return 4;
  }
  return 99;
}

export function speedSort(a: unknown, b: unknown): number | null {
  const ra = speedRank(a);
  const rb = speedRank(b);
  if (ra !== rb) {
    return ra - rb;
  }
  return null;
}

export const MY_GAMES_TABLE_CUSTOM_KEY_SORT: Record<string, (a: unknown, b: unknown) => number | null> = {
  analysis_accuracy: nullableNumSort,
  analysis_acpl: nullableNumSort,
  player_rating: nullableNumSort,
  created_at: nullableNumSort,
  opponent_name: strEmptyLastSort,
  opening_name: strEmptyLastSort,
  speed: speedSort,
};
