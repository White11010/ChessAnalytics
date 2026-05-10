/** Relative label for "last games sync" using `gamesSync.*` keys in app shell locale. */
export function formatLastGamesSyncLabel(
  t: (key: string, values?: Record<string, number>) => string,
  lastSyncedAtMs: number | null,
  nowMs: number,
): string {
  if (lastSyncedAtMs == null) {
    return String(t('gamesSync.neverSynced'));
  }
  const diffMin = Math.floor((nowMs - lastSyncedAtMs) / 60_000);
  if (diffMin < 1) {
    return String(t('gamesSync.justNow'));
  }
  if (diffMin < 60) {
    return String(t('gamesSync.minutesAgo', { n: diffMin }));
  }
  const diffH = Math.floor(diffMin / 60);
  if (diffH < 48) {
    return String(t('gamesSync.hoursAgo', { n: diffH }));
  }
  const diffD = Math.floor(diffH / 24);
  return String(t('gamesSync.daysAgo', { n: diffD }));
}
