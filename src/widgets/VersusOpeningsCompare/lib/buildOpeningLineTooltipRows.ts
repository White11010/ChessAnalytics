import type { VersusOpeningLineShare } from '@/entities/versus';

export interface OpeningLineTooltipRow {
  label: string;
  pctRounded: number;
}

/** Rows for the opening-family tooltip: all lines if ≤3, else top 3 by popularity + aggregated "others". */
export function buildOpeningLineTooltipRows(
  lines: VersusOpeningLineShare[] | undefined,
  othersLabel: string,
): OpeningLineTooltipRow[] | null {
  if (!lines?.length) return null;
  const roundPct = (x: number) => Math.round(x);
  if (lines.length <= 3) {
    return lines.map((l) => ({ label: l.name, pctRounded: roundPct(l.frequencyPct) }));
  }
  const top = lines.slice(0, 3).map((l) => ({ label: l.name, pctRounded: roundPct(l.frequencyPct) }));
  const restSum = lines.slice(3).reduce((s, l) => s + l.frequencyPct, 0);
  top.push({ label: othersLabel, pctRounded: roundPct(restSum) });
  return top;
}
