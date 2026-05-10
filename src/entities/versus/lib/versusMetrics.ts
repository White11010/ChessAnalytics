import type { VersusSideSummary } from '../model/versus.types';

export type WinCell = 'you' | 'opp' | 'tie' | 'none';

export type VersusMetricRow = {
  key: string;
  label: string;
  youFmt: string;
  oppFmt: string;
  win: WinCell;
};

type TFn = (key: string, params?: Record<string, unknown>) => string;

export function higherWins(
  a: number | null | undefined,
  b: number | null | undefined,
): WinCell {
  if (a == null || b == null) return 'none';
  const d = a - b;
  if (Math.abs(d) < 0.35) return 'tie';
  return d > 0 ? 'you' : 'opp';
}

export function lowerWins(
  a: number | null | undefined,
  b: number | null | undefined,
): WinCell {
  if (a == null || b == null) return 'none';
  const d = a - b;
  if (Math.abs(d) < 0.35) return 'tie';
  return d < 0 ? 'you' : 'opp';
}

export function buildVersusMetricTable(
  sy: VersusSideSummary,
  so: VersusSideSummary,
  speedLabel: string,
  t: TFn,
): VersusMetricRow[] {
  const fmtPct = (v: number | null | undefined) =>
    v == null ? '—' : `${v.toFixed(0)}%`;
  const fmtNum = (v: number | null | undefined, d = 1) =>
    v == null ? '—' : v.toFixed(d);
  const convYou = sy.conversionPct;
  const convOpp = so.conversionPct;
  const convWin = higherWins(convYou ?? null, convOpp ?? null);

  return [
    {
      key: 'acc',
      label: String(t('versusPage.metricAvgAccuracy')),
      youFmt: fmtPct(sy.avgAccuracyPct),
      oppFmt: fmtPct(so.avgAccuracyPct),
      win: higherWins(sy.avgAccuracyPct, so.avgAccuracyPct),
    },
    {
      key: 'acpl',
      label: String(t('versusPage.metricAcpl')),
      youFmt: fmtNum(sy.avgAcpl, 1),
      oppFmt: fmtNum(so.avgAcpl, 1),
      win: lowerWins(sy.avgAcpl, so.avgAcpl),
    },
    {
      key: 'wr',
      label: String(t('versusPage.metricWinRate')),
      youFmt: fmtPct(sy.winRatePct),
      oppFmt: fmtPct(so.winRatePct),
      win: higherWins(sy.winRatePct, so.winRatePct),
    },
    {
      key: 'bl',
      label: String(t('versusPage.metricBlundersPg')),
      youFmt: fmtNum(sy.blundersPerGame, 1),
      oppFmt: fmtNum(so.blundersPerGame, 1),
      win: lowerWins(sy.blundersPerGame, so.blundersPerGame),
    },
    {
      key: 'cv',
      label: String(t('versusPage.metricConversion')),
      youFmt: convYou == null ? '—' : `${convYou.toFixed(0)}%`,
      oppFmt: convOpp == null ? '—' : `${convOpp.toFixed(0)}%`,
      win: convWin,
    },
    {
      key: 'spd',
      label: String(t('versusPage.metricBestSpeed')),
      youFmt: speedLabel,
      oppFmt: speedLabel,
      win: 'none' as WinCell,
    },
  ];
}
